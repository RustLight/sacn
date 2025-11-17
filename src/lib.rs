// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was modified as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

//! Implementation of the sACN network protocol.
//!
//! This crate implements the Streaming ACN (sACN) network protocol as specified in ANSI E1.31-2018. Streaming ACN is built on top of and is
//! compatible with the ACN protocol suite (ANSI E1.17-2015). This library supports sending and receiving data, universe synchronisation and
//! universe discovery. This library supports linux (fully) and windows (no receiving multicast) and IP unicast, multicast and broadcast.
//!
//! Installation instructions are detailed within the README file.
//!
//!
//!
//! This file was modified as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.
//!
//! # Examples
//!
//! Creating an sACN receiver and receiving data. This automatically handles receiving synchronised data at the right time with the array of received data
//! containing all the data which should be acted upon at the same time (so if there are 2 synchronised data packets the array will have length 2).
//! ```
//! use sacn::receive::SacnReceiver;
//! use sacn::packet::ACN_SDT_MULTICAST_PORT;
//!
//! use std::net::{IpAddr, Ipv4Addr, SocketAddr};
//! use std::time::Duration;
//!
//! const UNIVERSE1: u16 = 1;
//! const TIMEOUT: Option<Duration> = Some(Duration::from_secs(1)); // A timeout of None means blocking behaviour, some indicates the actual timeout.
//!
//! let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT);
//!
//! let mut dmx_rcv = SacnReceiver::with_ip(addr, None).unwrap();
//!
//! dmx_rcv.listen_universes(&[UNIVERSE1]).unwrap();
//!
//! // .recv(TIMEOUT) handles processing synchronised as-well as normal data.
//! match dmx_rcv.recv(TIMEOUT) {
//!     Err(e) => {
//!         // Print out the error.
//!         println!("{:?}", e);
//!     }
//!     Ok(p) => {
//!         // Print out the data.
//!         println!("{:?}", p);
//!     }
//! }
//! ```
//!
//! Creating an sACN receiver and checking for discovered sources through universe discovery.
//! ```
//! use sacn::receive::SacnReceiver;
//! use sacn::packet::ACN_SDT_MULTICAST_PORT;
//!
//! use std::net::{IpAddr, Ipv4Addr, SocketAddr};
//! use std::time::Duration;
//!
//! const TIMEOUT: Option<Duration> = Some(Duration::from_secs(1)); // A timeout of None means blocking behaviour, some indicates the actual timeout.
//!
//! let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT);
//!
//! // Creating the receiver, automatically listens for universe discovery packets.
//! let mut dmx_rcv = SacnReceiver::with_ip(addr, None).unwrap();
//!
//! // Cause source discovery to be announced by a returned error. If this isn't true then discovery packets are still handled by the user must poll
//! // the discovered sources list periodically as there will be no announcement that a discovery packet has been processed. By default this option is
//! // off (false) based on the assumption that when receiving data the majority of the time the receiver just wants to process more data from the same
//! // universe and doesn't want to discover sources.
//! dmx_rcv.set_announce_source_discovery(true);
//!
//! // Receive for a short period, no data is expected but this allows universe discovery packets to be received.
//! // This example will always timeout if run in isolation as there are no sources running on the network.
//! match dmx_rcv.recv(TIMEOUT) {
//!     Err(e) => {
//!         match e {
//!             sacn::error::errors::SacnError::SourceDiscovered(source_name) => {
//!                 println!("Source name: {} discovered!", source_name);
//!             }
//!             other => {
//!                 // Print out the error.
//!                 println!("{:?}", other);
//!             }
//!         }
//!     }
//!     Ok(p) => {
//!         // Print out the data. Note that no data is expected as no universes are registered for receiving data.
//!         println!("{:?}", p);
//!     }
//! }
//! ```
//!
//! Creating a sACN sender and sending some unsychronised data. An sACNSender automatically sends universe discovery packets.
//!
//! ```no_run
//! use sacn::source::SacnSource;
//! use sacn::packet::ACN_SDT_MULTICAST_PORT;
//! use std::net::{IpAddr, SocketAddr};
//!
//! let local_addr: SocketAddr = SocketAddr::new(IpAddr::V4("0.0.0.0".parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);
//!
//! let mut src = SacnSource::with_ip("Source", local_addr).unwrap();
//!
//! let universe: u16 = 1;                        // Universe the data is to be sent on.
//! let sync_uni: Option<u16> = None;             // Don't want the packet to be delayed on the receiver awaiting synchronisation.
//! let priority: u8 = 100;                       // The priority for the sending data, must be 1-200 inclusive,  None means use default.
//! let dst_ip: Option<SocketAddr> = None;        // Sending the data using IP multicast so don't have a destination IP.
//!
//! src.register_universe(universe).unwrap(); // Register with the source that will be sending on the given universe.
//!
//! let mut data: Vec<u8> = vec![0, 0, 0, 0, 255, 255, 128, 128]; // Some arbitrary data, must have length <= 513 (including start-code).
//!
//! src.send(&[universe], &data, Some(priority), dst_ip, sync_uni).unwrap(); // Actually send the data
//! ```
//!
//! Creating a sACN sender and sending some synchronised data.
//!
//! ```no_run
//! use sacn::source::SacnSource;
//! use sacn::packet::ACN_SDT_MULTICAST_PORT;
//!
//! use std::net::{IpAddr, SocketAddr};
//! use std::thread::sleep;
//! use std::time::Duration;
//!
//! let local_addr: SocketAddr = SocketAddr::new(IpAddr::V4("0.0.0.0".parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);
//!
//! let mut src = SacnSource::with_ip("Source", local_addr).unwrap();
//!
//! let universe: u16 = 1;                        // Universe the data is to be sent on.
//! let sync_uni: Option<u16> = Some(1);          // Data packets use a synchronisation address of 1.
//! let priority: u8 = 100;                       // The priority for the sending data, must be 1-200 inclusive,  None means use default.
//! let dst_ip: Option<SocketAddr> = None;        // Sending the data using IP multicast so don't have a destination IP.
//!
//! src.register_universe(universe).unwrap(); // Register with the source that will be sending on the given universe.
//!
//! let mut data: Vec<u8> = vec![0, 0, 0, 0, 255, 255, 128, 128]; // Some arbitrary data, must have length <= 513 (including start-code).
//!
//! // Actually send the data, since the sync_uni is not 0 the data will be synchronised at the receiver (if the receiver supports synchronisation).
//! src.send(&[universe], &data, Some(priority), dst_ip, sync_uni).unwrap();
//!
//! // A small delay between sending data and sending the sync packet as recommend in ANSI E1.31-2018 Section 11.2.2.
//! sleep(Duration::from_millis(10));
//!
//! // To actually trigger the data need to send a synchronisation packet like so.
//! src.send_sync_packet(sync_uni.unwrap(), dst_ip).unwrap();
//! ```
//!
//! Creating a sACN sender and sending data using unicast.
//!
//! ```no_run
//! use sacn::source::SacnSource;
//! use sacn::packet::ACN_SDT_MULTICAST_PORT;
//!
//! use std::net::{IpAddr, SocketAddr};
//! use std::thread::sleep;
//! use std::time::Duration;
//!
//! let local_addr: SocketAddr = SocketAddr::new(IpAddr::V4("0.0.0.0".parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);
//!
//! let mut src = SacnSource::with_ip("Source", local_addr).unwrap();
//!
//! let universe: u16 = 1;                        // Universe the data is to be sent on.
//! let sync_uni: Option<u16> = None;             // Data packets are unsynchronised in this example but unicast transmission supports synchronised and unsynchronised sending.
//! let priority: Option<u8> = Some(100);                       // The priority for the sending data, must be 1-200 inclusive,  None means use default.
//!
//! // To send using unicast the dst_ip argument is set to a Some() value with the address to send the data to. By default the port should be the
//! // ACN_SDT_MULTICAST_PORT but this can be configured differently if required in a specific situation. Change this address to the correct address for your
//! // application, 192.168.0.1 is just a stand-in.
//! let destination_address: SocketAddr = SocketAddr::new(IpAddr::V4("192.168.0.1".parse().unwrap()), ACN_SDT_MULTICAST_PORT);
//! let dst_ip: Option<SocketAddr> = Some(destination_address);
//!
//! src.register_universe(universe).unwrap(); // Register with the source that will be sending on the given universe.
//!
//! let mut data: Vec<u8> = vec![0, 0, 0, 0, 255, 255, 128, 128]; // Some arbitrary data, must have length <= 513 (including start-code).
//!
//! // Actually send the data, since the sync_uni is not 0 the data will be synchronised at the receiver (if the receiver supports synchronisation).
//! src.send(&[universe], &data, priority, dst_ip, sync_uni).unwrap();
//! ```

#![doc(html_root_url = "https://docs.rs/sacn/")]
// #![warn(missing_docs)]
// Recursion limit for error_chain.
#![recursion_limit = "1024"]

/// The errors within the sACN crate related to parse/pack errors.
/// Error-chain is used for errors within the library to allow chaining errors together to provide more informative backtraces.
/// This completely replaces the old error system (sACN crate version 0.4.4) which relied on a simple Enum model without proper backtraces.
pub mod sacn_parse_pack_error;

/// The errors used within the sACN crate, parse/pack errors are seperated out into sacn_parse_pack_error.
pub mod error;

extern crate libc;
/// The library is built on top of socket2 to provide the underlying UDP networking interface.
extern crate socket2;

/// The core crate is used for string processing during packet parsing/packing as well as to provide access to the Hash trait.
extern crate core;

/// The byteorder crate is used for marshalling data on/off the network in Network Byte Order.
extern crate byteorder;

/// The uuid crate is used for working with/generating UUIDs which sACN uses as part of the cid field in the protocol.
extern crate uuid;

/// The packet module handles the sACN packets including parsing/packing and sACN related constants.
pub mod packet;

/// The source module handles generation of sACN on the network.
pub mod source;

/// The receive module handles the receiving of sACN on the network.
pub mod receive;
