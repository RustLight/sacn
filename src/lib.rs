// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was modified as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

//! A Rust implementation of the Streaming ACN (sACN) protocol, ANSI E1.31-2018.
//!
//! sACN carries DMX512 lighting control data over IP networks. This crate can send it ([`source::SacnSource`]) and
//! receive it ([`receive::SacnReceiver`]) over IPv4 and IPv6, using multicast, unicast or broadcast. It also handles
//! universe synchronisation and universe discovery.
//!
//! See the [README](https://github.com/RustLight/sacn#installation) for installation instructions and prerequisites.
//!
//! # Getting started
//!
//! Add the crate to your project:
//!
//! ```text
//! cargo add sacn
//! ```
//!
//! ## Key concepts
//!
//! - **Universe**: a numbered stream of up to 512 DMX slots (channels). Data universes are numbered 1 to 63999
//!   ([`packet::E131_MIN_MULTICAST_UNIVERSE`] to [`packet::E131_MAX_MULTICAST_UNIVERSE`]). Universe 64214
//!   ([`packet::E131_DISCOVERY_UNIVERSE`]) is reserved for universe discovery.
//! - **START code and slots**: in this crate a universe's data is a byte slice of up to 513 bytes
//!   ([`packet::UNIVERSE_CHANNEL_CAPACITY`]). The first byte is the DMX START code (`0` for ordinary dimmer data, the
//!   "Null START Code") and the remaining bytes are slots 1 to 512. So slot `n` is at index `n`.
//! - **Source**: a sender of sACN. Each source has a human readable name and a CID, a UUID that identifies it on the
//!   network. [`source::SacnSource`] generates a random CID unless you supply one.
//! - **Receiver**: a listener for one or more universes. A receiver may hear the same universe from several sources.
//! - **Priority**: every data packet carries a priority from 0 to 200 ([`packet::E131_MAX_PRIORITY`]), 100 by
//!   default ([`packet::E131_DEFAULT_PRIORITY`]). Receivers use it to choose between sources.
//! - **Synchronisation**: a source can send data that receivers hold until a synchronisation packet arrives on a
//!   chosen synchronisation universe, so several universes change at the same moment. Receivers start holding data
//!   once they have received the first synchronisation packet on that universe.
//! - **Universe discovery**: every source multicasts the list of universes it sends, every 10 seconds
//!   ([`packet::E131_UNIVERSE_DISCOVERY_INTERVAL`]). Receivers collect these to learn which sources are on the network.
//!
//! ## Modules
//!
//! | Module | What it holds |
//! | --- | --- |
//! | [`source`] | [`SacnSource`](source::SacnSource): sends data, synchronisation and stream termination packets, and sends universe discovery in the background. |
//! | [`receive`] | [`SacnReceiver`](receive::SacnReceiver): receives data; [`DMXData`](receive::DMXData): one universe of received data; [`DiscoveredSacnSource`](receive::DiscoveredSacnSource); the merge functions [`htp_dmx_merge`](receive::htp_dmx_merge) and [`discard_lowest_priority_then_previous`](receive::discard_lowest_priority_then_previous). |
//! | [`packet`] | Protocol constants (port, priorities, universe limits, timeouts) and the packet types, for parsing and packing packets yourself. |
//! | [`error`] | [`SacnError`](error::errors::SacnError) and the crate's [`Result`](error::errors::Result) type. |
//! | [`sacn_parse_pack_error`] | [`ParsePacketError`](sacn_parse_pack_error::ParsePacketError), returned when a packet can't be parsed or packed. |
//!
//! ## Sending data
//!
//! Create a [`SacnSource`](source::SacnSource), register the universes it will send on, then call
//! [`send`](source::SacnSource::send). Registered universes are advertised through universe discovery
//! automatically.
//!
//! ```no_run
//! use sacn::packet::UNIVERSE_CHANNEL_CAPACITY;
//! use sacn::source::SacnSource;
//!
//! # fn main() -> sacn::error::errors::Result<()> {
//! let mut src = SacnSource::new_v4("My lighting desk")?;
//!
//! let universe: u16 = 1;
//! src.register_universe(universe)?;
//!
//! // A full universe: the Null START Code followed by 512 slots.
//! let mut data = [0u8; UNIVERSE_CHANNEL_CAPACITY];
//! data[1] = 255; // Slot 1 at full.
//! data[2] = 128; // Slot 2 at half.
//!
//! // Arguments: universes, data, priority (None = default 100), destination (None = multicast),
//! // synchronisation universe (None = not synchronised).
//! src.send(&[universe], &data, None, None, None)?;
//! # Ok(())
//! # }
//! ```
//!
//! [`SacnSource::new_v4`](source::SacnSource::new_v4) (or [`new_v6`](source::SacnSource::new_v6) for IPv6) generates
//! a random CID. Use [`with_cid_v4`](source::SacnSource::with_cid_v4) with a CID you store to keep the same CID
//! across restarts, which the standard recommends. To choose the local address, see
//! [Binding to a specific address](#binding-to-a-specific-address).
//!
//! When you are done with a universe, call [`terminate_stream`](source::SacnSource::terminate_stream) (or
//! [`terminate_stream_to`](source::SacnSource::terminate_stream_to) for unicast receivers) so receivers stop using
//! its data straight away.
//!
//! ### Keeping a universe alive
//!
//! The source only sends when you call `send`. Your application is responsible for the transmission rate and
//! keep-alive required by ANSI E1.31-2018 Section 6.6:
//!
//! - Don't send a universe faster than the DMX512-A refresh rate (about 44 updates per second).
//! - When the data stops changing, send it three more times, then resend the last data every 800 to 1000 ms.
//!   Otherwise receivers decide the source has gone after [`packet::E131_NETWORK_DATA_LOSS_TIMEOUT`] (2.5 s).
//!
//! A simple way to do this is to give the source its own thread. The rest of the application sends it each new frame
//! over a channel, and the thread handles the timing. `SacnSource` is blocking and doesn't need an async runtime; in
//! async code, run a loop like this on its own thread and send frames to it from your tasks. Sending on a
//! [`std::sync::mpsc`] channel never blocks, so it is safe to call from async code.
//!
//! ```no_run
//! use sacn::packet::UNIVERSE_CHANNEL_CAPACITY;
//! use sacn::source::SacnSource;
//!
//! use std::sync::mpsc::{self, RecvTimeoutError};
//! use std::thread;
//! use std::time::Duration;
//!
//! type Frame = [u8; UNIVERSE_CHANNEL_CAPACITY];
//!
//! const UNIVERSE: u16 = 1;
//! // About 44 updates per second, the DMX512-A maximum.
//! const MIN_INTERVAL: Duration = Duration::from_millis(23);
//! // Within the 800 to 1000 ms keep-alive window, allowing for MIN_INTERVAL.
//! const KEEP_ALIVE_INTERVAL: Duration = Duration::from_millis(850);
//! // How many times to resend data after it stops changing.
//! const REPEATS: u32 = 3;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let (frames, rx) = mpsc::channel::<Frame>();
//!
//! let sender = thread::spawn(move || -> sacn::error::errors::Result<()> {
//!     let mut src = SacnSource::new_v4("My lighting desk")?;
//!     src.register_universe(UNIVERSE)?;
//!
//!     let mut data: Frame = [0; UNIVERSE_CHANNEL_CAPACITY];
//!     let mut repeats_left = 0;
//!     loop {
//!         // While repeats are due, don't wait for new data. Otherwise wait up to the keep-alive interval.
//!         let wait = if repeats_left > 0 { Duration::ZERO } else { KEEP_ALIVE_INTERVAL };
//!         match rx.recv_timeout(wait) {
//!             Ok(frame) => {
//!                 // Skip to the newest frame if several arrived while sleeping.
//!                 data = rx.try_iter().last().unwrap_or(frame);
//!                 repeats_left = REPEATS;
//!             }
//!             // No new data: this send is a repeat or a keep-alive.
//!             Err(RecvTimeoutError::Timeout) => repeats_left = repeats_left.saturating_sub(1),
//!             // The rest of the application has finished.
//!             Err(RecvTimeoutError::Disconnected) => break,
//!         }
//!         src.send(&[UNIVERSE], &data, None, None, None)?;
//!         thread::sleep(MIN_INTERVAL);
//!     }
//!
//!     src.terminate_stream(UNIVERSE, 0)?;
//!     Ok(())
//! });
//!
//! // The rest of the application sends a frame whenever the output changes.
//! let mut frame: Frame = [0; UNIVERSE_CHANNEL_CAPACITY];
//! for level in 0..=255 {
//!     frame[1] = level;
//!     frames.send(frame)?;
//!     thread::sleep(Duration::from_millis(10));
//! }
//!
//! // Dropping the channel stops the thread, which terminates the stream.
//! drop(frames);
//! sender.join().expect("sender thread panicked")?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Several universes in one call
//!
//! `send` splits `data` into [`UNIVERSE_CHANNEL_CAPACITY`](packet::UNIVERSE_CHANNEL_CAPACITY)-sized chunks and sends
//! each chunk to the next universe in the list. Each chunk starts with its own START code byte.
//!
//! ```no_run
//! # use sacn::packet::UNIVERSE_CHANNEL_CAPACITY;
//! # use sacn::source::SacnSource;
//! # fn main() -> sacn::error::errors::Result<()> {
//! # let mut src = SacnSource::new_v4("My lighting desk")?;
//! let universes = [1, 2];
//! src.register_universes(&universes)?;
//!
//! let data = [0u8; UNIVERSE_CHANNEL_CAPACITY * 2]; // Universe 1 is data[0..513], universe 2 is data[513..1026].
//! src.send(&universes, &data, None, None, None)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Unicast
//!
//! Pass a destination address to send to one receiver instead of the universe's multicast group.
//!
//! ```no_run
//! # use sacn::source::SacnSource;
//! use sacn::packet::ACN_SDT_MULTICAST_PORT;
//! use std::net::SocketAddr;
//!
//! # fn main() -> sacn::error::errors::Result<()> {
//! # let mut src = SacnSource::new_v4("My lighting desk")?;
//! # src.register_universe(1)?;
//! // Replace 192.168.0.1 with the receiver's address.
//! let receiver: SocketAddr = SocketAddr::new("192.168.0.1".parse().unwrap(), ACN_SDT_MULTICAST_PORT);
//! src.send(&[1], &[0, 255, 128], Some(100), Some(receiver), None)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Synchronised data
//!
//! Send each universe with the same synchronisation universe, wait briefly, then send one synchronisation packet.
//! Receivers that support synchronisation hold the data and act on all of it when the synchronisation packet arrives.
//! A receiver only starts holding data after it has received a synchronisation packet on that universe (ANSI E1.31-2016
//! Section 11.1.1), so the first frame a receiver gets is acted on straight away.
//! The synchronisation universe doesn't need to be registered; the first data universe is a reasonable choice.
//!
//! ```no_run
//! # use sacn::packet::UNIVERSE_CHANNEL_CAPACITY;
//! # use sacn::source::SacnSource;
//! use std::thread::sleep;
//! use std::time::Duration;
//!
//! # fn main() -> sacn::error::errors::Result<()> {
//! # let mut src = SacnSource::new_v4("My lighting desk")?;
//! let universes = [1, 2];
//! let sync_universe = 1;
//! src.register_universes(&universes)?;
//!
//! let data = [0u8; UNIVERSE_CHANNEL_CAPACITY * 2];
//! src.send(&universes, &data, None, None, Some(sync_universe))?;
//!
//! // ANSI E1.31-2018 Appendix B.1 recommends a short delay before the synchronisation packet.
//! sleep(Duration::from_millis(10));
//! src.send_sync_packet(sync_universe, None)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Binding to a specific address
//!
//! `new_v4` and `new_v6` bind to the unspecified address (`0.0.0.0` or `::`) on an ephemeral port, and the operating
//! system picks the interface to send from. Use [`with_ip`](source::SacnSource::with_ip) (or
//! [`with_cid_ip`](source::SacnSource::with_cid_ip)) to bind to an address of your choice, for example the address of
//! the network interface that faces your lighting network.
//!
//! ```no_run
//! use sacn::source::SacnSource;
//! use std::net::SocketAddr;
//!
//! # fn main() -> sacn::error::errors::Result<()> {
//! // Replace 192.168.0.6 with an address of the interface to send from.
//! let local_addr: SocketAddr = "192.168.0.6:0".parse().unwrap();
//! let mut src = SacnSource::with_ip("My lighting desk", local_addr)?;
//! # Ok(())
//! # }
//! ```
//!
//! Keep the port at 0 (an ephemeral port) unless you have a reason not to. If a source binds to the sACN port
//! ([`packet::ACN_SDT_MULTICAST_PORT`], 5568) and a [`SacnReceiver`](receive::SacnReceiver) runs on the same host,
//! the two sockets share the port, and on Linux some unicast sACN sent to the host is delivered to the source's socket
//! and lost.
//!
//! ## Receiving data
//!
//! Create a [`SacnReceiver`](receive::SacnReceiver), tell it which universes to listen to, then call
//! [`recv`](receive::SacnReceiver::recv) in a loop. `recv` returns a `Vec` of [`DMXData`](receive::DMXData), one per
//! universe that is ready to act on. Unsynchronised data is returned as soon as it arrives. Synchronised data is held
//! until its synchronisation packet arrives, then all the universes waiting on it are returned together. Until the
//! receiver has received a synchronisation packet on a data packet's synchronisation universe, that data is returned
//! as soon as it arrives too, so output doesn't wait on a synchronisation stream that hasn't started. Data is held
//! separately for each START code, so a universe whose source also sends per-address priority (START code `0xDD`)
//! is returned twice: once with its levels and once with its priorities.
//!
//! ```
//! use sacn::error::errors::SacnError;
//! use sacn::packet::ACN_SDT_MULTICAST_PORT;
//! use sacn::receive::SacnReceiver;
//!
//! use std::io::ErrorKind;
//! use std::net::{IpAddr, Ipv4Addr, SocketAddr};
//! use std::time::Duration;
//!
//! # fn main() -> sacn::error::errors::Result<()> {
//! // The address of the network interface to receive on, with the sACN port.
//! let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT);
//!
//! // None: no limit on the number of sources.
//! let mut dmx_rcv = SacnReceiver::with_ip(addr, None)?;
//! dmx_rcv.listen_universes(&[1])?;
//!
//! // A timeout of None blocks until data arrives.
//! match dmx_rcv.recv(Some(Duration::from_secs(1))) {
//!     Ok(universes) => {
//!         for dmx in universes {
//!             // dmx.values[0] is the START code, dmx.values[1..] are the slots.
//!             println!(
//!                 "Universe {} from {:?} at priority {}: {:?}",
//!                 dmx.universe, dmx.src_cid, dmx.priority, dmx.values
//!             );
//!         }
//!     }
//!     // No data arrived within the timeout: WouldBlock on Linux and macOS, TimedOut on Windows.
//!     Err(SacnError::Io(e)) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
//!     Err(e) => println!("Error: {e}"),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! `values` may be shorter than 513 bytes if the source sent fewer slots; the missing slots are zero.
//!
//! [`SacnReceiver::with_ip`](receive::SacnReceiver::with_ip) takes the address of the interface to receive on; use an
//! IPv6 address to receive IPv6. Use [`listen_universes`](receive::SacnReceiver::listen_universes) and
//! [`mute_universe`](receive::SacnReceiver::mute_universe) to change the universes at any time.
//!
//! ### Several sources on one universe
//!
//! For unsynchronised data, `recv` returns each source's data as it arrives, with its priority and source CID. Your
//! application decides which to use: normally the highest priority wins, and sources with the same priority are
//! merged, for example with [`htp_dmx_merge`](receive::htp_dmx_merge) (highest takes precedence). For data held for
//! synchronisation the receiver merges data for the same universe and START code for you with the function set by
//! [`set_merge_fn`](receive::SacnReceiver::set_merge_fn), which by default keeps the highest priority and then the
//! most recent data.
//!
//! ### Events reported as errors
//!
//! Some events are reported by `recv` as an `Err` so the application can react to them. Each is off by default:
//!
//! | Setter | Error returned |
//! | --- | --- |
//! | [`set_announce_source_discovery`](receive::SacnReceiver::set_announce_source_discovery) | [`SacnError::SourceDiscovered`](error::errors::SacnError::SourceDiscovered) when a source is first discovered (or returns after timing out), and again when its name or universe list changes. Periodic adverts that change nothing don't raise it. |
//! | [`set_announce_stream_termination`](receive::SacnReceiver::set_announce_stream_termination) | [`SacnError::UniverseTerminated`](error::errors::SacnError::UniverseTerminated) when a source terminates a universe. |
//! | [`set_announce_timeout`](receive::SacnReceiver::set_announce_timeout) | [`SacnError::UniverseTimeout`](error::errors::SacnError::UniverseTimeout) when a source stops sending a universe for longer than [`packet::E131_NETWORK_DATA_LOSS_TIMEOUT`]. |
//!
//! These errors don't stop the receiver: keep calling `recv`.
//!
//! Preview data (for visualisers, not live output) is ignored unless you call
//! [`set_process_preview_data(true)`](receive::SacnReceiver::set_process_preview_data).
//!
//! ### Discovering sources
//!
//! Every receiver listens for universe discovery packets. Call
//! [`get_discovered_sources`](receive::SacnReceiver::get_discovered_sources) at any time for the sources found so far
//! and the universes each one sends. Discovery packets are only processed while `recv` runs.
//! With `set_announce_source_discovery(true)`, `recv` reports each source once, and again only when its name or
//! universe list changes.
//!
//! ```
//! use sacn::error::errors::SacnError;
//! use sacn::packet::ACN_SDT_MULTICAST_PORT;
//! use sacn::receive::SacnReceiver;
//!
//! use std::net::{IpAddr, Ipv4Addr, SocketAddr};
//! use std::time::Duration;
//!
//! # fn main() -> sacn::error::errors::Result<()> {
//! let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT);
//! let mut dmx_rcv = SacnReceiver::with_ip(addr, None)?;
//! dmx_rcv.set_announce_source_discovery(true);
//!
//! // With no sources on the network this times out.
//! match dmx_rcv.recv(Some(Duration::from_secs(1))) {
//!     Err(SacnError::SourceDiscovered(name)) => println!("Discovered source {name}"),
//!     Err(e) => println!("Error: {e}"),
//!     Ok(data) => println!("{data:?}"),
//! }
//!
//! for source in dmx_rcv.get_discovered_sources() {
//!     println!("{} ({}) sends universes {:?}", source.name, source.cid, source.get_all_universes());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Platform notes
//!
//! - The crate is built and unit tested on Linux, Windows and macOS. The network integration tests run on Linux.
//! - On Windows, IPv6 multicast isn't supported, so an IPv6 receiver there only receives unicast and broadcast.
//!   [`is_multicast_enabled`](receive::SacnReceiver::is_multicast_enabled) tells you whether multicast is on.
//! - Receivers listen on UDP port 5568 ([`packet::ACN_SDT_MULTICAST_PORT`]). Allow it through the host firewall, and
//!   make sure the network passes multicast (IGMP snooping on managed switches, for example).
//! - The library never prints to stdout or stderr. With the default `logging` feature it reports failures it can't
//!   return, dropped packets and per-packet traces through the [`log`](https://docs.rs/log) crate under the
//!   `sacn::source` and `sacn::receive` targets. See [Logging](https://github.com/RustLight/sacn#logging) in the README
//!   for how to show or filter them.
//!
//! # Examples
//!
//! The `examples` folder in the repository has complete programs:
//!
//! - `sine-wave-sender`: sends a moving sine wave on universe 1 to localhost (`cargo run --example sine-wave-sender`).
//! - `simple-receiver`: shows the data received on universe 1 (`cargo run --example simple-receiver`).
//! - `demo_src` and `demo_rcv`: interactive source and receiver used for manual and conformance testing
//!   (`cargo run --example demo_src -- <interface ip>`).

#![doc(html_root_url = "https://docs.rs/sacn/")]
// #![warn(missing_docs)]
// Recursion limit for error_chain.
#![recursion_limit = "1024"]

// Internal logging macros. With the `logging` feature they forward to the `log` crate. Without it they compile to nothing,
// but still type-check their arguments so that values used only for logging don't cause unused-variable warnings.
#[cfg(feature = "logging")]
macro_rules! log_warn { ($($arg:tt)+) => { log::warn!($($arg)+) } }
#[cfg(feature = "logging")]
macro_rules! log_debug { ($($arg:tt)+) => { log::debug!($($arg)+) } }
#[cfg(feature = "logging")]
macro_rules! log_trace { ($($arg:tt)+) => { log::trace!($($arg)+) } }
#[cfg(not(feature = "logging"))]
macro_rules! log_warn { ($($arg:tt)+) => { if false { let _ = format_args!($($arg)+); } } }
#[cfg(not(feature = "logging"))]
macro_rules! log_debug { ($($arg:tt)+) => { if false { let _ = format_args!($($arg)+); } } }
#[cfg(not(feature = "logging"))]
macro_rules! log_trace { ($($arg:tt)+) => { if false { let _ = format_args!($($arg)+); } } }

/// The errors within the sACN crate related to parse/pack errors.
/// Error-chain is used for errors within the library to allow chaining errors together to provide more informative backtraces.
/// This completely replaces the old error system (sACN crate version 0.4.4) which relied on a simple Enum model without proper backtraces.
pub mod sacn_parse_pack_error;

/// The errors used within the sACN crate, parse/pack errors are seperated out into sacn_parse_pack_error.
pub mod error;

/// The packet module handles the sACN packets including parsing/packing and sACN related constants.
pub mod packet;

/// The source module handles generation of sACN on the network.
pub mod source;

/// The receive module handles the receiving of sACN on the network.
pub mod receive;
