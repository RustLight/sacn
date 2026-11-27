#![warn(missing_docs)]

// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was modified as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.
//
// Documentation of private or crate local items that effects public items, such as errors from private functions which get passed up to a
// public function, should be copied into the documentation of the public item so that the public facing documentation is a complete documentation
// of each public item without relying on referring to private items.
//

use crate::error::errors::*;
use crate::interface::{self, MulticastIf, NetworkInterface};
use crate::packet::*;

use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// Socket2 used to create the underlying UDP socket that sACN is sent on.
use socket2::{Domain, SockAddr, Socket, Type};

/// UUID library used to handle the UUID's used in the CID fields.
use uuid::Uuid;

/// The name of the thread which runs periodically to perform various actions such as universe discovery adverts for the source.
const SND_UPDATE_THREAD_NAME: &str = "rust_sacn_snd_update_thread";

/// The default startcode used to send stream termination packets when the `SacnSource` is closed.
const DEFAULT_TERMINATE_START_CODE: u8 = 0;

/// The poll rate of the update thread.
/// Discovery updates are sent every `E131_UNIVERSE_DISCOVERY_INTERVAL` so the poll rate must be lower than or equal to this.
// const DEFAULT_POLL_PERIOD: Duration = E131_UNIVERSE_DISCOVERY_INTERVAL;
const DEFAULT_POLL_PERIOD: Duration = Duration::from_secs(1);

/// A DMX over sACN sender.
///
/// `SacnSource` is used for sending sACN packets over an IP network.
///
/// # Examples
///
/// ```no_run
/// // Example showing creation of a source and then sending some data.
/// use sacn::source::SacnSource;
/// use sacn::packet::ACN_SDT_MULTICAST_PORT;
/// use std::net::{IpAddr, SocketAddr};
///
/// let local_addr: SocketAddr = SocketAddr::new(IpAddr::V4("0.0.0.0".parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);
///
/// let mut src = SacnSource::with_ip("Source", local_addr).unwrap();
///
/// let universe: u16 = 1;                        // Universe the data is to be sent on.
/// let sync_uni: Option<u16> = None;             // Don't want the packet to be delayed on the receiver awaiting synchronisation.
/// let priority: u8 = 100;                       // The priority for the sending data, must be 1-200 inclusive,  None means use default.
/// let dst_ip: Option<SocketAddr> = None;        // Sending the data using IP multicast so don't have a destination IP.
///
/// src.register_universe(universe).unwrap(); // Register with the source that will be sending on the given universe.
///
/// let mut data: Vec<u8> = vec![0, 0, 0, 0, 255, 255, 128, 128]; // Some arbitrary data, must have length <= 513 (including start-code).
///
/// src.send(&[universe], &data, Some(priority), dst_ip, sync_uni).unwrap(); // Actually send the data
/// ```
///
/// An ANSI E1.31-2018 sACN source.
///
/// Allows sending DMX data over an IPv4 or IPv6 network using sACN.
#[derive(Debug)]
pub struct SacnSource {
    /// The DMX source used for actually sending the sACN packets.
    /// Protected by a Mutex lock to allow concurrent access between user threads and the update thread below.
    internal: Arc<Mutex<SacnSourceInternal>>,

    /// Update thread which performs actions every `DEFAULT_POLL_PERIOD` such as checking if a universe
    /// discovery packet should be sent.
    update_thread: Option<JoinHandle<()>>,
}

/// Internal sACN sender, this does most of the work however is encapsulated within `SacnSource`
/// to allow access by the `update_thread` which is used to manage sending periodic universe discovery packets.
#[derive(Debug)]
struct SacnSourceInternal {
    /// Underlying UDP socket used for sending sACN packets on the network.
    socket: Socket,

    /// The address of this `SacnSourceInternal` on the network.
    addr: SocketAddr,

    /// The unique ID of this `SacnSourceInternal`.
    /// It is the job of the user of the library to ensure that the cid is given on creation of the `SacnSourceInternal` is unique.
    cid: Uuid,

    /// The human readable name of this source.
    name: String,

    /// Flag which is included in sACN packets to indicate that the data shouldn't be used for live output
    /// (ie. on actual lighting fixtures). A receiver may or may not be compliant with this so it should not be relied
    /// upon in an untested environment.
    preview_data: bool,

    /// Flag which is included in sACN data packets that have a synchronisation address. It tells receivers what to do if
    /// synchronisation is lost, as per ANSI E1.31-2016 Section 6.2.6 (`Force_Synchronization`). If false receivers hold the
    /// data until synchronisation resumes, if true they may process the data without waiting for a synchronisation packet.
    force_synchronization: bool,

    /// The sequence numbers used for data packets, keeps a reference of the next sequence number to use for each universe.
    /// Sequence numbers are always in the range [0, 255].
    data_sequences: RefCell<HashMap<u16, u8>>,

    /// The sequence numbers used for sync packets, keeps a reference of the next sequence number to use for each universe.
    /// Sequence numbers are always in the range [0, 255].
    sync_sequences: RefCell<HashMap<u16, u8>>,

    /// A list of the universes registered to send by this source, used for universe discovery.
    /// Always sorted with lowest universe first to allow quicker usage.
    /// This may never contain duplicate universe values.
    universes: Vec<u16>,

    /// Flag that indicates if the `SacnSourceInternal` is running (the update thread should be triggering periodic discovery packets).
    running: bool,

    /// The time that the last universe discovery advert was send.
    last_discovery_advert_timestamp: Instant,

    /// Flag that is set to True to indicate that the source is sending periodic universe discovery packets.
    is_sending_discovery: bool,

    /// The interfaces that multicast packets are sent on, each packet being sent once per interface.
    /// Empty means the interface chosen by the operating system.
    multicast_interfaces: Vec<MulticastIf>,
}

impl SacnSource {
    /// Constructs a new `SacnSource` with the given name, binding to an IPv4 address on an ephemeral port.
    /// This generates a new CID automatically using random values.
    ///
    /// # Errors
    /// See [`with_cid_ip`](Self::with_cid_ip)
    pub fn new_v4(name: &str) -> Result<SacnSource> {
        let cid = Uuid::new_v4();
        SacnSource::with_cid_v4(name, cid)
    }

    /// Constructs a new `SacnSource` with the given name and specified CID binding to an IPv4 address.
    ///
    /// # Errors
    /// See [`with_cid_ip`](Self::with_cid_ip)
    pub fn with_cid_v4(name: &str, cid: Uuid) -> Result<SacnSource> {
        let ip = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            // Use an ephemeral port. Binding the ACN port would put this socket in the receivers' `SO_REUSEPORT` group,
            // and Linux would then hand some of the unicast sACN addressed to this host to this socket, where it is never read.
            0,
        );
        SacnSource::with_cid_ip(name, cid, ip)
    }

    /// Constructs a new `SacnSource` with the given name, binding to an IPv6 address on an ephemeral port.
    /// By default this will only receive IPv6 data but IPv4 can also be enabled by calling `set_ipv6_only(false)`.
    ///
    /// # Errors
    /// See [`with_cid_ip`](Self::with_cid_ip)
    pub fn new_v6(name: &str) -> Result<SacnSource> {
        let cid = Uuid::new_v4();
        SacnSource::with_cid_v6(name, cid)
    }

    /// Constructs a new `SacnSource` with the given name and specified CID binding to an IPv6 address.
    ///
    /// # Errors
    /// See [`with_cid_ip`](Self::with_cid_ip)
    pub fn with_cid_v6(name: &str, cid: Uuid) -> Result<SacnSource> {
        let ip = SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
            // Use an ephemeral port. Binding the ACN port would put this socket in the receivers' `SO_REUSEPORT` group,
            // and Linux would then hand some of the unicast sACN addressed to this host to this socket, where it is never read.
            0,
        );
        SacnSource::with_cid_ip(name, cid, ip)
    }

    /// Constructs a new `SacnSource` with the given name and binding to the supplied ip.
    ///
    /// Avoid binding to `ACN_SDT_MULTICAST_PORT` if a `SacnReceiver` may run on the same host. The socket would share the
    /// port with the receiver, and on Linux some unicast sACN sent to this host would be delivered to the source's socket
    /// and lost. Port 0 (an ephemeral port) is a good choice.
    ///
    /// # Errors
    /// See [`with_cid_ip`](Self::with_cid_ip)
    pub fn with_ip(name: &str, ip: SocketAddr) -> Result<SacnSource> {
        SacnSource::with_cid_ip(name, Uuid::new_v4(), ip)
    }

    /// Constructs a new `SacnSource` with the given name, cid and binding to the supplied ip.
    ///
    /// # Errors
    /// Io: Returned if the underlying UDP socket cannot be created and bound or if the thread used for sending periodic
    ///     discovery adverts fails to be created. Causes can be distinguished by looking at the error chain.
    ///
    /// `UnsupportedIpVersion`: Returned if the `SocketAddr` is not IPv4 or IPv6.
    ///
    /// `MalformedSourceName`: Returned if the given source name is longer than the maximum allowed size of `E131_SOURCE_NAME_MAX_LENGTH` (63 bytes, leaving room for the null terminator).
    pub fn with_cid_ip(name: &str, cid: Uuid, ip: SocketAddr) -> Result<SacnSource> {
        if name.len() > E131_SOURCE_NAME_MAX_LENGTH {
            return Err(SacnError::MalformedSourceName(
                "Source name provided is longer than maximum allowed".to_string(),
            ));
        }

        let trd_builder = thread::Builder::new().name(SND_UPDATE_THREAD_NAME.into());

        let internal_src = Arc::new(Mutex::new(SacnSourceInternal::with_cid_ip(name, cid, ip)?));

        let mut trd_src = internal_src.clone();

        let src = SacnSource {
            internal: internal_src,
            update_thread: Some(trd_builder.spawn(move || {
                // Stop if the source has been dropped or the mutex was poisoned by a panic elsewhere.
                while matches!(trd_src.lock().map(|s| s.running), Ok(true)) {
                    thread::sleep(DEFAULT_POLL_PERIOD);
                    // On error the source keeps running and tries again on the next period. With the `logging` feature the
                    // error is reported through the `log` crate, so the application decides whether and where it is shown.
                    if let Err(e) = perform_periodic_update(&mut trd_src) {
                        log_warn!("sACN source periodic update failed: {e:?}");
                    }
                }
            })?),
        };

        Ok(src)
    }

    /// Registers the given universes on this source in addition to already registered universes.
    ///
    /// This allows sending data to those universes or using them as synchronisation addresses as well as adding them to
    /// the list of universes that appear in universe discovery packets that are sent (depending on the
    /// `set_is_sending_discovery` flag) periodically.
    ///
    /// This is more efficient than repeated calls to `register_universe` as it means only 1 mutex unlock is required.
    ///
    /// # Arguments
    /// universes: The sACN universes to register for usage as data universes and/or synchronisation addresses. Note that sACN
    ///     universes start at 1 not 0.
    ///
    /// # Errors
    /// `IllegalUniverse`: Returned if a universe is outwith the range permitted by ANSI E1.31-2018.
    ///
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn register_universes(&mut self, universes: &[u16]) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.register_universes(universes)
    }

    /// Registers a single universe on this source in addition to already registered universes.
    ///
    /// This allows sending data to those universes or using them as synchronisation addresses as well as adding them to
    /// the list of universes that appear in universe discovery packets that are sent (depending on the
    /// `set_is_sending_discovery` flag) periodically.
    ///
    /// If registering multiple universes see [`register_universes`](Self::register_universes).
    ///
    /// # Arguments
    /// universe: The sACN universe to register for usage as a data universe and/or synchronisation address. Note that sACN
    ///     universes start at 1 not 0.
    ///
    /// # Errors
    /// `IllegalUniverse`: Returned if the universe is outwith the range permitted by ANSI E1.31-2018.
    ///
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn register_universe(&mut self, universe: u16) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.register_universe(universe)
    }

    /// Sends the given data to the given universes with the given priority, synchronisation address (universe) and destination ip.
    ///
    /// # Arguments
    ///
    /// universe:     The sACN universes that the data should be set on, the data will be split over these universes with each `UNIVERSE_CHANNEL_CAPACITY`
    ///                 sized chunk sent to the next universe.
    ///
    /// data:         The data that should be sent, must have a length greater than 0.
    ///
    /// priority:     The E131 priority that the data should be sent with, must be less than `E131_MAX_PRIORITY` (`const.E131_MAX_PRIORITY.packet`),
    ///                 if a value of None is provided then the default of `E131_DEFAULT_PRIORITY` (`const.E131_DEFAULT_PRIORITY.packet`) is used.
    ///
    /// `dst_ip`:       The destination IP, can be Ipv4 or Ipv6, None if should be sent using ip multicast.
    ///
    /// `sync_address`: The address to use for synchronisation, must be a valid universe, None indicates no synchronisation. If synchronisation is required a
    ///                 reasonable default address to use is the first universe that this data is being sent to.
    ///                 It doesn't need to be registered, so a universe used only for synchronisation isn't advertised in
    ///                 universe discovery.
    ///
    /// As per ANSI E1.31-2018 Section 6.6.1 this method shouldn't be called at a higher refresher rate than specified in ANSI E1.11 \[DMX\] unless
    ///     configured by the user to do so in an environment which doesn't contain any E1.31 to DMX512-A converters.
    ///
    /// Note as per ANSI-E1.31-2018 Appendix B.1 it is recommended to have a small delay before sending the follow up sync packet.
    ///
    /// # Errors
    /// `SenderAlreadyTerminated`: Returned if this method is called on an `SacnReceiverInternal` that has already terminated.
    ///
    /// `InvalidInput`: Returned if the data array has length 0 or if an insufficient number of universes for the given data are provided (each universe takes 513 bytes of data).
    ///
    /// `InvalidPriority`: Returned if the priority is greater than the allowed maximum priority of `E131_MAX_PRIORITY`.
    ///
    /// `IllegalUniverse`: Returned if the universe is outwith the allowed range as specified by ANSI E1.31-2018 Section 6.2.7.
    ///
    /// `UniverseNotRegistered`: Returned if the universe is not registered on the given `SacnSourceInternal`.
    ///
    /// `ExceedUniverseCapacity`: Returned if the data has a length greater than the maximum allowed within a universe (`packet::UNIVERSE_CHANNEL_CAPACITY`).
    ///
    /// Io: Returned if the data fails to be sent on the socket, see `send_to(fn.send_to.Socket)`.
    ///
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn send(
        &mut self,
        universes: &[u16],
        data: &[u8],
        priority: Option<u8>,
        dst_ip: Option<SocketAddr>,
        synchronisation_addr: Option<u16>,
    ) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.send(
            universes,
            data,
            priority,
            dst_ip,
            synchronisation_addr,
        )
    }

    /// Sends a synchronisation packet to trigger the sending of packets waiting to be sent together.
    ///
    /// A common pattern would be to use the send method to send data to all the universes that should be synchronised using a
    /// chosen synchronisation universe then wait for a small time as per the recommendation in ANSI-E1.31-2018 Appendix B.1 and
    /// then send a synchronisation packet with the address of the synchronisation universe chosen to trigger the packets.
    ///
    /// # Arguments
    /// universe: The universe of this synchronisation packet.
    /// `dst_ip`:   The destination IP address for this packet or None if it should be sent using multicast.
    ///
    /// The universe doesn't need to be registered: a synchronisation universe carries no data, so it is left out of universe discovery.
    ///
    /// # Errors
    /// `IllegalUniverse`: Returned if the universe is outwith the allowed range of sACN universes as defined in ANSI E1.31-2018 Section 6.2.7.
    ///
    /// Io: Returned if the packet fails to be sent using the underlying network socket.
    ///
    /// `SacnParsePackError`: Returned if the sync packet fails to be packed.
    ///
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn send_sync_packet(&mut self, universe: u16, dst_ip: Option<SocketAddr>) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.send_sync_packet(universe, dst_ip)
    }

    /// Terminates sending on the given universe.
    ///
    /// # Errors:
    /// `IllegalUniverse`: Returned if the universe is outwith the allowed range of sACN universes as defined in ANSI E1.31-2018 Section 6.2.7.
    ///
    /// `UniverseNotRegistered`: Returned if the universe is not registered on this source.
    ///
    /// Io: Returned if the termination packets fail to be sent on the socket.
    ///
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn terminate_stream(&mut self, universe: u16, start_code: u8) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.terminate_stream(universe, start_code)
    }

    /// Terminates sending on the given universe, sending the termination packets to the given unicast destinations.
    ///
    /// `terminate_stream` sends the termination packets only to the universe's multicast group, so a receiver that was
    /// sent the universe's data by unicast and hasn't joined that group never sees them. It then only notices the
    /// source has gone after the network data loss timeout. Pass the same destinations that were given to `send` here
    /// so those receivers are told straight away.
    ///
    /// The `E131_TERMINATE_STREAM_PACKET_COUNT` termination packets are sent to each destination in turn, then the
    /// universe is deregistered. If `dst_ips` is empty this behaves exactly like `terminate_stream` (multicast).
    ///
    /// # Arguments
    /// universe: The universe that is being terminated.
    ///
    /// `start_code`: used for the first byte of the otherwise empty data payload to indicate the `start_code` of the data.
    ///
    /// `dst_ips`: The unicast destinations to send the termination packets to.
    ///
    /// # Errors:
    /// `IllegalUniverse`: Returned if the universe is outwith the allowed range of sACN universes as defined in ANSI E1.31-2018 Section 6.2.7.
    ///
    /// `UniverseNotRegistered`: Returned if the universe is not registered on this source.
    ///
    /// Io: Returned if the termination packets fail to be sent on the socket. The packets are still sent to the
    /// remaining destinations and the universe is still deregistered; the first error is returned.
    ///
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn terminate_stream_to(
        &mut self,
        universe: u16,
        start_code: u8,
        dst_ips: &[SocketAddr],
    ) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.terminate_stream_to(universe, start_code, dst_ips)
    }

    /// Returns the ACN CID device identifier of the `SacnSourceInternal`.
    ///
    /// # Errors
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn cid(&self) -> Result<Uuid> {
        Ok(*unlock_internal(&self.internal)?.cid())
    }

    /// Sets the ACN CID device identifier.
    ///
    /// # Arguments
    /// cid: The new CID identifier for this source. It is left to the user to ensure that this is always unique within the network the source is in.
    ///
    /// # Errors
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn set_cid(&mut self, cid: Uuid) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.set_cid(cid);
        Ok(())
    }

    /// Returns the ACN source name.
    ///
    /// # Errors
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn name(&self) -> Result<String> {
        Ok(unlock_internal(&self.internal)?.name().into())
    }

    /// Sets ACN source name.
    ///
    /// # Argument
    /// name: The new name for the source, it is left to the user to ensure this is unique within the sACN network.
    ///
    /// # Errors
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    ///
    /// `MalformedSourceName`: Returned to indicate that the given source name is longer than the maximum allowed as per `E131_SOURCE_NAME_MAX_LENGTH` (63 bytes, leaving room for the null terminator).
    pub fn set_name(&mut self, name: &str) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.set_name(name)
    }

    /// Returns true if `SacnSourceInternal` is in preview mode, false if not.
    ///
    /// For details of `preview_mode` see [`set_preview_mode`](Self::set_preview_mode).
    ///
    /// # Errors
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn preview_mode(&self) -> Result<bool> {
        Ok(unlock_internal(&self.internal)?.preview_mode())
    }

    /// Sets the value of the `Preview_Data` flag in packets from this `SacnSource`.
    ///
    /// # Arguments
    /// `preview_mode`: If true then all data packets from this `SacnSource` will have the `Preview_Data` flag set to true indicating that the data is not
    ///     for live output. If false then the flag will be set to false.
    ///
    /// # Errors
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn set_preview_mode(&mut self, preview_mode: bool) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.set_preview_mode(preview_mode);
        Ok(())
    }

    /// Returns the value of the `Force_Synchronization` option set on data packets sent by this `SacnSource`.
    ///
    /// For details see [`set_force_synchronization`](Self::set_force_synchronization).
    ///
    /// # Errors
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn force_synchronization(&self) -> Result<bool> {
        Ok(unlock_internal(&self.internal)?.force_synchronization)
    }

    /// Sets the `Force_Synchronization` option on data packets sent by this `SacnSource`, as per ANSI E1.31-2016 Section 6.2.6.
    ///
    /// The option tells receivers what to do with synchronised data (data sent with a synchronisation address) if synchronisation is lost,
    /// meaning no synchronisation packet has been received on that address within `E131_NETWORK_DATA_LOSS_TIMEOUT`:
    /// - false (the default): receivers hold the data until synchronisation resumes.
    /// - true: receivers may process the data straight away without waiting for a synchronisation packet.
    ///
    /// # Errors
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn set_force_synchronization(&mut self, force_synchronization: bool) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.force_synchronization = force_synchronization;
        Ok(())
    }

    /// Sets the `is_sending_discovery` flag to the given value.
    ///
    /// # Arguments
    /// val: The new value for the `is_sending_discovery` flag, if true then source will send periodic universe discovery packets
    /// and if false it won't.
    pub fn set_is_sending_discovery(&mut self, val: bool) {
        self.internal.lock().unwrap().set_is_sending_discovery(val);
    }

    /// Returns the multicast time to live of the socket (the multicast hop limit for an IPv6 source).
    pub fn multicast_ttl(&self) -> Result<u32> {
        unlock_internal(&self.internal)?.multicast_ttl()
    }

    /// Sets the multicast time to live (the multicast hop limit for an IPv6 source).
    ///
    /// # Arguments
    /// `multicast_ttl`: The new time to live value for network packets sent using multicast.
    ///
    /// # Errors
    /// Io: Returned if the multicast TTL fails to be set on the underlying socket.
    ///
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn set_multicast_ttl(&mut self, multicast_ttl: u32) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.set_multicast_ttl(multicast_ttl)
    }

    /// Returns the current Time To Live for unicast packets send by this source (the unicast hop limit for an IPv6 source).
    ///
    /// # Errors
    /// Io: Returned if the TTL cannot be retrieved from the underlying socket.
    ///
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn ttl(&self) -> Result<u32> {
        unlock_internal(&self.internal)?.ttl()
    }

    /// Sets the Time To Live for packets sent by this source (the unicast hop limit for an IPv6 source).
    ///
    /// # Arguments
    /// ttl: The new time to live value for new packets.
    ///
    /// # Errors
    /// Io: Returned if the TTL value cannot be changed.
    ///
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn set_ttl(&mut self, ttl: u32) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.set_ttl(ttl)
    }

    /// Sets if multicast loop is enabled for an IPv4 source. Returns an error on an IPv6 source; use `set_multicast_loop` instead.
    ///
    /// # Arguments:
    /// `multicast_loop`: If true then multicast loop is enabled, if false it is not.
    ///
    /// # Errors
    /// Io: Returned if the `set_multicast_loop` option fails to be set on the socket.
    ///
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn set_multicast_loop_v4(&mut self, multicast_loop: bool) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.set_multicast_loop_v4(multicast_loop)
    }

    /// Sets if multicast loop is enabled, for either an IPv4 or an IPv6 source.
    ///
    /// Unlike `set_multicast_loop_v4`, this works on IPv6 sources too.
    ///
    /// # Arguments:
    /// `multicast_loop`: If true then multicast loop is enabled, if false it is not.
    ///
    /// # Errors
    /// Io: Returned if the multicast loop option fails to be set on the socket.
    ///
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn set_multicast_loop(&mut self, multicast_loop: bool) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.set_multicast_loop(multicast_loop)
    }

    /// Sets the network interfaces that multicast packets (data, synchronisation, termination and universe discovery)
    /// are sent on. Each packet is sent once on each interface. Unicast packets aren't affected: they follow the
    /// operating system's routing table.
    ///
    /// By default, and when given an empty list, packets are sent once on the interface the operating system chooses.
    ///
    /// Interfaces are given by name or by one of their addresses, see [`NetworkInterface`]. For an IPv4 source an
    /// interface named by name is used through its first IPv4 address; for an IPv6 source an interface must have an IPv6
    /// address, which may be link-local.
    ///
    /// # Examples
    /// ```no_run
    /// use sacn::interface::NetworkInterface;
    /// use sacn::source::SacnSource;
    ///
    /// let mut src = SacnSource::new_v4("Source").unwrap();
    /// src.set_multicast_interfaces(&["eth0".into(), "eth1".into()]).unwrap();
    /// ```
    ///
    /// # Errors
    /// Io: Returned with kind `NotFound` if an interface doesn't exist or has no address of the source's IP version, in
    /// which case the interfaces are left unchanged. Also returned if the socket's multicast interface can't be reset.
    ///
    /// `IpVersionError`: Returned if an interface is given by an address of the other IP version.
    ///
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn set_multicast_interfaces(&mut self, interfaces: &[NetworkInterface]) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.set_multicast_interfaces(interfaces)
    }

    /// Returns true if multicast loop is enabled, false if not.
    ///
    /// # Errors
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn multicast_loop(&self) -> Result<bool> {
        unlock_internal(&self.internal)?.multicast_loop()
    }

    /// Returns the universes currently registered on this source.
    ///
    /// # Errors
    /// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state.
    pub fn universes(&self) -> Result<Vec<u16>> {
        Ok(unlock_internal(&self.internal)?.universes())
    }
}

/// By implementing the Drop trait for `SacnSource` it means that the user doesn't have to explicitly clean up the source
/// and if it goes out of reference it will clean itself up and send the required termination packets etc.
impl Drop for SacnSource {
    fn drop(&mut self) {
        // The guard is dropped at the end of this statement so the update thread can see `running` change.
        if let Ok(mut i) = unlock_internal_mut(&mut self.internal) {
            i.running = false;
        } else {
            // As drop isn't always explicitly called and cannot return an error the error can't be returned. Memory safety is
            // maintained and this prevents causing a panic!.
            log_warn!(
                "sACN source dropped with a poisoned mutex; termination packets were not sent"
            );
            return;
        }

        if let Some(thread) = self.update_thread.take() {
            // Internal is accessed twice separately, this allows the discovery thread to interleave between running being set to false speeding up termination.
            if let Ok(mut i) = unlock_internal_mut(&mut self.internal) {
                // For same reasons as above a potential error can't be returned and a 'best attempt' is used to clean up.
                if let Err(e) = i.terminate(DEFAULT_TERMINATE_START_CODE) {
                    log_warn!(
                        "sACN source failed to send termination packets while being dropped: {e:?}"
                    );
                }
            } else {
                log_warn!(
                    "sACN source dropped with a poisoned mutex; termination packets were not sent"
                );
                // As drop isn't always explicitly called and cannot return an error the error is ignored. Memory safety is maintain and this prevents causing a panic!.
            };

            // The update thread only panics if the mutex was poisoned. Panicking here as well would abort the
            // process if the source is being dropped while unwinding, so the result is ignored.
            if thread.join().is_err() {
                log_warn!("sACN source update thread panicked");
            }
        }
    }
}

impl SacnSourceInternal {
    /// Constructs a new `SacnSourceInternal` with DMX START code set to 0 with specified CID and binding IP address.
    ///
    /// By default for an IPv6 address this will only receive IPv6 data but IPv4 can also be enabled by calling `set_ipv6_only(false)`.
    /// By default the TTL for ipv4 packets is 1 to keep them within the local network.
    ///
    /// # Arguments:
    /// name: The human readable name for this sacn source.
    /// cid:  The UUID for this source.
    /// ip:   The address that this source should bind to.
    ///
    /// # Errors
    /// Io: Returned if the underlying socket cannot be created or the IP cannot be bound to the underlying socket. See (`UdpBuilder::new_v4`)[`fn.new_v4.UdpBuilder`], (`UdpBuilder::new_v6`)[`fn.new_v6.UdpBuilder`] and (`Socket::bind`)[fn.bind.Socket2].
    ///
    /// `UnsupportedIpVersion`: Returned if the `SockAddr` is not IPv4 or IPv6.
    fn with_cid_ip(name: &str, cid: Uuid, ip: SocketAddr) -> Result<SacnSourceInternal> {
        let socket = if ip.is_ipv4() {
            Socket::new(Domain::IPV4, Type::DGRAM, None)?
        } else if ip.is_ipv6() {
            Socket::new(Domain::IPV6, Type::DGRAM, None)?
        } else {
            return Err(SacnError::UnsupportedIpVersion(
                "Address to create SacnSource is not IPv4 or IPv6".to_string(),
            ));
        };

        // Multiple different processes might want to send to the sACN stream so therefore need to allow re-using the ACN port.
        // Set reuse port is only supported on linux.
        #[cfg(target_os = "linux")]
        socket.set_reuse_port(true)?;

        // Set reuse address supported on linux and windows.
        socket.set_reuse_address(true)?;
        socket.bind(&ip.into())?;

        let ds = SacnSourceInternal {
            socket,
            addr: ip,
            cid,
            name: name.to_string(),
            preview_data: false,
            force_synchronization: false,
            data_sequences: RefCell::new(HashMap::new()),
            sync_sequences: RefCell::new(HashMap::new()),
            universes: Vec::new(),
            running: true,
            last_discovery_advert_timestamp: Instant::now(),
            is_sending_discovery: true,
            multicast_interfaces: Vec::new(),
        };

        Ok(ds)
    }

    /// Sets the `is_sending_discovery` flag to the given value.
    ///
    /// If `is_sending_discovery` is set to false then no discovery adverts for this source
    /// will be sent otherwise (and by default) they will be sent every `UNIVERSE_DISCOVERY_INTERVAL`.
    ///
    /// # Arguments:
    /// val: The new value of the `is_sending_discovery` flag.
    fn set_is_sending_discovery(&mut self, val: bool) {
        self.is_sending_discovery = val;
    }

    /// Registers the given array of universes with this source.
    ///
    /// Any universes already registered won't be re-registered and will have no effect.
    ///
    /// # Arguments:
    /// universes: The sACN universe to register. Note that sACN universes start at 1 not 0.
    ///
    /// # Errors
    /// See `register_universe(fn.register_universe.source)` for more details.
    fn register_universes(&mut self, universes: &[u16]) -> Result<()> {
        for u in universes {
            self.register_universe(*u)?;
        }
        Ok(())
    }

    /// Registers the given universe for sending with this source.
    ///
    /// If a universe is already registered then this method has no effect.
    ///
    /// # Errors
    /// `IllegalUniverse`: Returned if the universe can't carry data, see (`is_data_universe_in_range`)[`fn.is_data_universe_in_range.packet`].
    fn register_universe(&mut self, universe: u16) -> Result<()> {
        is_data_universe_in_range(universe)?;

        if self.universes.is_empty() {
            self.universes.push(universe);
        } else if let Err(i) = self.universes.binary_search(&universe) {
            // Value not found, i is the position it should be inserted
            self.universes.insert(i, universe);
        } else {
            // If value found then don't insert to avoid duplicates.
        }

        Ok(())
    }

    /// De-registers the given universe for sending with this source.
    ///
    /// # Errors
    /// `IllegalUniverse`: Returned if the universe is outwith the allowed range, see [`is_universe_in_range`](crate::packet::is_universe_in_range).
    ///
    /// `UniverseNotFound`: Returned if the given universe was never registered originally.
    fn deregister_universe(&mut self, universe: u16) -> Result<()> {
        is_universe_in_range(universe)?;

        match self.universes.binary_search(&universe) {
            Err(_i) => {
                // Value not found
                Err(SacnError::UniverseNotFound(universe))
            }
            Ok(i) => {
                // Value found, i is index.
                self.universes.remove(i);
                Ok(())
            }
        }
    }

    /// Checks if the given universe is a valid universe to send on (within allowed range) and that it is registered with this `SacnSourceInternal`.
    ///
    /// # Errors
    /// `IllegalUniverse`: Returned if the universe can't carry data, see (`is_data_universe_in_range`)[`fn.is_data_universe_in_range.packet`].
    ///
    /// `UniverseNotRegistered`: Returned if the universe is not registered on the given `SacnSourceInternal`.
    fn universe_allowed(&self, u: &u16) -> Result<()> {
        is_data_universe_in_range(*u)?;

        if !self.universes.contains(u) {
            return Err(SacnError::UniverseNotRegistered(*u));
        }

        Ok(())
    }

    /// Sends the given data to the given universes with the given priority, synchronisation address (universe) and destination ip.
    ///
    /// # Arguments
    ///
    /// universe:     The sACN universes that the data should be set on, the data will be split over these universes with each `UNIVERSE_CHANNEL_CAPACITY`
    ///                 sized chunk sent to the next universe.
    ///
    /// data:         The data that should be sent, must have a length greater than 0.
    ///
    /// priority:     The E131 priority that the data should be sent with, must be less than `E131_MAX_PRIORITY` (`const.E131_MAX_PRIORITY.packet`),
    ///                 if a value of None is provided then the default of `E131_DEFAULT_PRIORITY` (`const.E131_DEFAULT_PRIORITY.packet`) is used.
    ///
    /// `dst_ip`:       The destination IP, can be Ipv4 or Ipv6, None if should be sent using ip multicast.
    ///
    /// `sync_address`: The address to use for synchronisation, must be a valid universe, None indicates no synchronisation. If synchronisation is required a
    ///                 reasonable default address to use is the first universe that this data is being sent to.
    ///                 It doesn't need to be registered, so a universe used only for synchronisation isn't advertised in
    ///                 universe discovery.
    ///
    /// As per ANSI E1.31-2018 Section 6.6.1 this method shouldn't be called at a higher refresher rate than specified in ANSI E1.11 \[DMX\] unless
    ///     configured by the user to do so in an environment which doesn't contain any E1.31 to DMX512-A converters.
    ///
    /// Note as per ANSI-E1.31-2018 Appendix B.1 it is recommended to have a small delay before sending the follow up sync packet.
    ///
    /// # Errors
    /// `SenderAlreadyTerminated`: Returned if this method is called on an `SacnReceiverInternal` that has already terminated.
    ///
    /// `InvalidInput`: Returned if the data array has length 0 or if an insufficient number of universes for the given data are provided (each universe takes 513 bytes of data).
    ///
    /// `InvalidPriority`: Returned if the priority is greater than the allowed maximum priority of `E131_MAX_PRIORITY`.
    ///
    /// `IllegalUniverse`: Returned if the universe is outwith the allowed range as specified by ANSI E1.31-2018 Section 6.2.7.
    ///
    /// `UniverseNotRegistered`: Returned if the universe is not registered on the given `SacnSourceInternal`.
    ///
    /// `ExceedUniverseCapacity`: Returned if the data has a length greater than the maximum allowed within a universe (`packet::UNIVERSE_CHANNEL_CAPACITY`).
    ///
    /// Io: Returned if the data fails to be sent on the socket, see `send_to(fn.send_to.Socket)`.
    fn send(
        &self,
        universes: &[u16],
        data: &[u8],
        priority: Option<u8>,
        dst_ip: Option<SocketAddr>,
        synchronisation_addr: Option<u16>,
    ) -> Result<()> {
        if !self.running {
            // Indicates that this sender has been terminated.
            return Err(SacnError::SenderAlreadyTerminated(
                "Attempted to send".to_string(),
            ));
        }

        if data.is_empty() {
            return Err(SacnError::DataArrayEmpty());
        }

        // Check all the given universes are valid before doing any action.
        // This prevents leaving the source in an inconsistent state if later a universe is found to be invalid.
        for u in universes {
            self.universe_allowed(u)?;
        }

        // Check that the synchronisation universe is a valid universe. It doesn't need to be registered: a synchronisation
        // universe carries no data, so it isn't advertised in universe discovery.
        if let Some(sync_addr) = synchronisation_addr {
            is_data_universe_in_range(sync_addr)
                .map_err(|_e| SacnError::IllegalSyncUniverse(sync_addr))?;
        }

        // + 1 as there must be at least 1 universe required as the data isn't empty then additional universes for any more.
        let required_universes =
            (data.len() as f64 / UNIVERSE_CHANNEL_CAPACITY as f64).ceil() as usize;

        if universes.len() < required_universes {
            return Err(SacnError::UniverseListEmpty());
        }

        for (i, &universe) in universes.iter().enumerate().take(required_universes) {
            let start_index = i * UNIVERSE_CHANNEL_CAPACITY;
            // Safety check to make sure that the end index doesn't exceed the data length
            let end_index = cmp::min((i + 1) * UNIVERSE_CHANNEL_CAPACITY, data.len());

            self.send_universe(
                universe,
                &data[start_index..end_index],
                priority.unwrap_or(E131_DEFAULT_PRIORITY),
                &dst_ip,
                synchronisation_addr.unwrap_or(NO_SYNC_UNIVERSE),
            )?;
        }

        Ok(())
    }

    /// Sends the given data to the given universe with the given priority, synchronisation address (universe) and destination ip.
    ///
    /// # Arguments
    /// universe:     The sACN universe that the data should be set on.
    ///
    /// data:         The data that should be sent, must be less than or equal in length to `UNIVERSE_CHANNEL_CAPACITY(const.UNIVERSE_CHANNEL_CAPACITY.packet)`.
    ///
    /// priority:     The E131 priority that the data should be sent with, must be less than `E131_MAX_PRIORITY` (`const.E131_MAX_PRIORITY.packet`), default `E131_DEFAULT_PRIORITY`.
    ///
    /// `dst_ip`:       The destination IP, can be Ipv4 or Ipv6, None if should be sent using ip multicast.
    ///
    /// `sync_address`: The address to use for synchronisation, must be a valid universe, 0 indicates no synchronisation.
    ///
    /// # Errors
    /// `InvalidInput`: Returned if the priority is greater than the allowed maximum priority of `E131_MAX_PRIORITY`.
    ///
    /// `ExceedUniverseCapacity`: Returned if the data has a length greater than the maximum allowed within a universe.
    ///
    /// `IllegalUniverse`: Returned if the given universe is outwith the allowed range of universes,
    ///                     see (`universe_to_ipv4_multicast_addr`)[`fn.universe_to_ipv4_multicast_addr.packet`] and (`universe_to_ipv6_multicast_addr`)[`fn.universe_to_ipv6_multicast_addr.packet`].
    ///
    /// Io: Returned if the data fails to be sent on the socket, see `send_to(fn.send_to.Socket)`.
    fn send_universe(
        &self,
        universe: u16,
        data: &[u8],
        priority: u8,
        dst_ip: &Option<SocketAddr>,
        sync_address: u16,
    ) -> Result<()> {
        if priority > E131_MAX_PRIORITY {
            return Err(SacnError::InvalidPriority(priority));
        }

        if data.len() > UNIVERSE_CHANNEL_CAPACITY {
            return Err(SacnError::ExceedUniverseCapacity(data.len()));
        }

        let mut sequence = match self.data_sequences.borrow().get(&universe) {
            Some(s) => *s,
            None => STARTING_SEQUENCE_NUMBER,
        };

        let packet = AcnRootLayerProtocol {
            pdu: E131RootLayer {
                cid: self.cid,
                data: E131RootLayerData::DataPacket(DataPacketFramingLayer {
                    source_name: self.name.as_str().into(),
                    priority,
                    synchronization_address: sync_address,
                    sequence_number: sequence,
                    preview_data: self.preview_data,
                    stream_terminated: false,
                    force_synchronization: self.force_synchronization,
                    universe,
                    data: DataPacketDmpLayer {
                        property_values: {
                            let mut property_values = Vec::with_capacity(data.len());
                            property_values.extend(data);
                            property_values.into()
                        },
                    },
                }),
            },
        };

        log_trace!(
            "Sending sACN data packet: universe {universe}, sequence {sequence}, priority {priority}, sync address {sync_address}, {} bytes, destination {}",
            data.len(),
            dst_ip.map_or_else(|| "multicast".to_string(), |ip| ip.to_string())
        );

        if dst_ip.is_some() {
            self.socket
                .send_to(&packet.pack_alloc().unwrap(), &dst_ip.unwrap().into())
                .map_err(|e| {
                    std::io::Error::new(e.kind(), "Failed to send data unicast on socket")
                })?;
        } else {
            let dst = if self.addr.is_ipv6() {
                universe_to_ipv6_multicast_addr(universe)?
            } else {
                universe_to_ipv4_multicast_addr(universe)?
            };

            self.send_multicast(&packet.pack_alloc().unwrap(), &dst)
                .map_err(|e| {
                    std::io::Error::new(e.kind(), "Failed to send data multicast on socket")
                })?;
        }

        if sequence == 255 {
            sequence = 0;
        } else {
            sequence += 1;
        }
        self.data_sequences.borrow_mut().insert(universe, sequence);
        Ok(())
    }

    /// Sends a synchronisation packet to trigger the sending of packets waiting to be sent together.
    ///
    /// A common pattern would be to use the send method to send data to all the universes that should be synchronised using a
    /// chosen synchronisation universe then wait for a small time as per the recommendation in ANSI-E1.31-2018 Appendix B.1 and
    /// then send a synchronisation packet with the address of the synchronisation universe chosen to trigger the packets.
    ///
    /// # Arguments
    /// universe: The universe of this synchronisation packet.
    /// `dst_ip`:   The destination IP address for this packet or None if it should be sent using multicast.
    ///
    /// The universe doesn't need to be registered: a synchronisation universe carries no data, so it is left out of universe discovery.
    ///
    /// # Errors
    /// `IllegalUniverse`: Returned if the universe is outwith the allowed range of sACN universes as defined in ANSI E1.31-2018 Section 6.2.7.
    ///
    /// Io: Returned if the packet fails to be sent using the underlying network socket.
    ///
    /// `SacnParsePackError`: Returned if the sync packet fails to be packed.
    fn send_sync_packet(&self, universe: u16, dst_ip: Option<SocketAddr>) -> Result<()> {
        is_data_universe_in_range(universe)?;

        let multicast_addr = if self.addr.is_ipv6() {
            universe_to_ipv6_multicast_addr(universe)?
        } else {
            universe_to_ipv4_multicast_addr(universe)?
        };

        let mut sequence = match self.sync_sequences.borrow().get(&universe) {
            Some(s) => *s,
            None => STARTING_SEQUENCE_NUMBER,
        };

        let packet = AcnRootLayerProtocol {
            pdu: E131RootLayer {
                cid: self.cid,
                data: E131RootLayerData::SynchronizationPacket(SynchronizationPacketFramingLayer {
                    sequence_number: sequence,
                    synchronization_address: universe,
                }),
            },
        };
        log_trace!("Sending sACN sync packet: sync address {universe}, sequence {sequence}");
        let buf = packet.pack_alloc()?;
        match dst_ip {
            Some(dst) => self.socket.send_to(&buf, &dst.into()).map(|_| ()),
            None => self.send_multicast(&buf, &multicast_addr),
        }
        .map_err(|e| std::io::Error::new(e.kind(), "Failed to send sync packet on socket"))?;

        if sequence == 255 {
            sequence = 0;
        } else {
            sequence += 1;
        }
        self.sync_sequences.borrow_mut().insert(universe, sequence);
        Ok(())
    }

    /// Sends a stream termination packet for the given universe.
    ///
    /// In normal usage this method would be called three times to send three packets for termination as per
    ///     ANSI E1.31-2018 Section 6.2.6, `Stream_Terminated`: Bit 6.
    ///
    /// # Arguments
    /// universe: The universe of this synchronisation packet.
    /// `dst_ip`:   The destination IP address for this packet or None if it should be sent using multicast.
    ///
    /// # Errors
    /// `IllegalUniverse`: Returned if the universe is outwith the allowed range of sACN universes as defined in ANSI E1.31-2018 Section 6.2.7.
    ///
    /// `UniverseNotRegistered`: Returned if the universe is not registered on the given `SacnSourceInternal`.
    ///
    /// Io: Returned if the termination packets fail to be sent on the underlying socket.
    fn send_terminate_stream_pkt(
        &self,
        universe: u16,
        dst_ip: Option<SocketAddr>,
        start_code: u8,
    ) -> Result<()> {
        self.universe_allowed(&universe)?;

        let multicast_addr = if self.addr.is_ipv6() {
            universe_to_ipv6_multicast_addr(universe)?
        } else {
            universe_to_ipv4_multicast_addr(universe)?
        };

        let mut sequence = match self.data_sequences.borrow_mut().remove(&universe) {
            Some(s) => s,
            None => STARTING_SEQUENCE_NUMBER,
        };

        let packet = AcnRootLayerProtocol {
            pdu: E131RootLayer {
                cid: self.cid,
                data: E131RootLayerData::DataPacket(DataPacketFramingLayer {
                    source_name: self.name.as_str().into(),
                    priority: 100,
                    synchronization_address: 0,
                    sequence_number: sequence,
                    preview_data: self.preview_data,
                    stream_terminated: true,
                    force_synchronization: false,
                    universe,
                    data: DataPacketDmpLayer {
                        property_values: vec![start_code].into(),
                    },
                }),
            },
        };
        let res = &packet.pack_alloc().unwrap();

        log_trace!(
            "Sending sACN stream termination packet: universe {universe}, sequence {sequence}"
        );
        match dst_ip {
            Some(dst) => {
                self.socket.send_to(res, &dst.into())?;
            }
            None => self.send_multicast(res, &multicast_addr)?,
        }

        if sequence == 255 {
            sequence = 0;
        } else {
            sequence += 1;
        }

        self.data_sequences.borrow_mut().insert(universe, sequence);
        Ok(())
    }

    /// Terminates a universe stream.
    ///
    /// Terminates a stream to the specified universe by sending packets with the `Stream_Terminated` flag set to 1.
    /// Number of packets sent as per section 6.2.6 , `Stream_Terminated`: Bit 6 of ANSI E1.31-2018.
    ///
    /// Arguments:
    /// universe: The universe that is being terminated.
    /// `start_code`: used for the first byte of the otherwise empty data payload to indicate the `start_code` of the data.
    ///
    /// # Errors:
    /// `IllegalUniverse`: Returned if the universe is outwith the allowed range of sACN universes as defined in ANSI E1.31-2018 Section 6.2.7.
    ///
    /// `UniverseNotRegistered`: Returned if the universe is not registered on this source.
    ///
    /// Io: Returned if the termination packets fail to be sent on the socket.
    fn terminate_stream(&mut self, universe: u16, start_code: u8) -> Result<()> {
        for _ in 0..E131_TERMINATE_STREAM_PACKET_COUNT {
            self.send_terminate_stream_pkt(universe, None, start_code)?;
        }

        self.deregister_universe(universe)?;
        Ok(())
    }

    /// Terminates a universe stream, sending the termination packets to each of the given unicast destinations.
    ///
    /// See (`SacnSource::terminate_stream_to`)[`SacnSource::terminate_stream_to`].
    fn terminate_stream_to(
        &mut self,
        universe: u16,
        start_code: u8,
        dst_ips: &[SocketAddr],
    ) -> Result<()> {
        if dst_ips.is_empty() {
            return self.terminate_stream(universe, start_code);
        }

        // Check before sending anything so that an unregistered universe sends nothing.
        self.universe_allowed(&universe)?;

        // Keep going after a failed send so one unreachable destination doesn't stop the others being told.
        let mut first_err: Option<SacnError> = None;
        for dst in dst_ips {
            for _ in 0..E131_TERMINATE_STREAM_PACKET_COUNT {
                if let Err(e) = self.send_terminate_stream_pkt(universe, Some(*dst), start_code) {
                    first_err.get_or_insert(e);
                    break;
                }
            }
        }

        self.deregister_universe(universe)?;

        match first_err {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }

    /// Terminates the DMX source.
    ///
    /// This includes terminating each registered universe with the `start_code` given.
    ///
    /// Arguments:
    /// `start_code`: used for the first byte of the otherwise empty data payload to indicate the `start_code` of the data.
    ///
    /// # Errors:
    /// Io: Returned if the termination packets fail to be sent on the underlying socket.
    fn terminate(&mut self, start_code: u8) -> Result<()> {
        self.running = false;
        let universes = self.universes.clone(); // About to start manipulating self.universes as universes are removed so clone original list.

        // Attempt every universe even if one fails, so a single send error doesn't leave the remaining universes
        // unterminated. The first error is returned once all universes have been attempted.
        let mut first_err: Option<SacnError> = None;
        for u in universes {
            if let Err(e) = self.terminate_stream(u, start_code) {
                // terminate_stream only deregisters after its packets were sent; the source is shutting down so
                // deregister regardless.
                let _ = self.deregister_universe(u);
                first_err.get_or_insert(e);
            }
        }

        match first_err {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }

    /// Sends a universe discovery packet advertising the universes that this source is registered to send.
    ///
    /// This packet may be broken down into multiple pages internally resulting in multiple UDP packets.
    ///
    /// # Errors
    /// See (`send_universe_discovery_detailed`)[`fn.send_universe_discovery_detailed.source`].
    fn send_universe_discovery(&self) -> Result<()> {
        for (page, last_page, universes) in discovery_pages(&self.universes) {
            self.send_universe_discovery_detailed(page, last_page, universes)?;
        }
        Ok(())
    }

    /// Sends a page of a universe discovery packet.
    ///
    /// There may be 1 or more pages for each full universe discovery packet with each page sent separately.
    ///
    /// # Arguments
    ///
    /// page: The page number of this universe discovery page.
    ///
    /// `last_page`: The last page that is expected as part of this universe discovery packet.
    ///
    /// universes: The universes to include on the page.
    ///
    /// # Errors
    /// Io: Returned if the discovery packet fails to be sent on the socket.
    ///
    /// `SacnParsePackError`: Returned if the discovery packet cannot be packed to send.
    fn send_universe_discovery_detailed(
        &self,
        page: u8,
        last_page: u8,
        universes: &[u16],
    ) -> Result<()> {
        let packet = AcnRootLayerProtocol {
            pdu: E131RootLayer {
                cid: self.cid,
                data: E131RootLayerData::UniverseDiscoveryPacket(
                    UniverseDiscoveryPacketFramingLayer {
                        source_name: self.name.as_str().into(),
                        data: UniverseDiscoveryPacketUniverseDiscoveryLayer {
                            page,
                            last_page,
                            universes: universes.into(),
                        },
                    },
                ),
            },
        };

        let ip = if self.addr.is_ipv6() {
            universe_to_ipv6_multicast_addr(E131_DISCOVERY_UNIVERSE)?
        } else {
            universe_to_ipv4_multicast_addr(E131_DISCOVERY_UNIVERSE)?
        };

        log_trace!(
            "Sending sACN universe discovery page {page} of {last_page} with {} universes",
            universes.len()
        );
        self.send_multicast(&packet.pack_alloc()?, &ip)?;

        Ok(())
    }

    /// Sends the given packet to the given multicast address once on each of the source's multicast interfaces, or once
    /// on the interface chosen by the operating system if none are set.
    ///
    /// A failure on one interface doesn't stop the packet being sent on the others; the first error is returned.
    ///
    /// # Errors
    /// Io: Returned if the packet fails to be sent on the socket, or the socket's multicast interface can't be set.
    fn send_multicast(&self, buf: &[u8], dst: &SockAddr) -> std::io::Result<()> {
        if self.multicast_interfaces.is_empty() {
            return self.socket.send_to(buf, dst).map(|_| ());
        }

        let mut result = Ok(());
        for iface in &self.multicast_interfaces {
            let sent = match iface {
                MulticastIf::V4(addr) => self.socket.set_multicast_if_v4(addr),
                MulticastIf::V6(index) => self.socket.set_multicast_if_v6(*index),
            }
            .and_then(|()| self.socket.send_to(buf, dst));
            if let Err(e) = sent
                && result.is_ok()
            {
                result = Err(e);
            }
        }
        result
    }

    /// Sets the network interfaces that multicast packets are sent on. See `SacnSource::set_multicast_interfaces`.
    fn set_multicast_interfaces(&mut self, interfaces: &[NetworkInterface]) -> Result<()> {
        let resolved = interface::resolve(interfaces, self.addr.is_ipv6())?;
        if resolved.is_empty() {
            // Hand the choice of interface back to the operating system.
            if self.addr.is_ipv6() {
                self.socket.set_multicast_if_v6(0)?;
            } else {
                self.socket.set_multicast_if_v4(&Ipv4Addr::UNSPECIFIED)?;
            }
        }
        self.multicast_interfaces = resolved;
        Ok(())
    }

    /// Returns the ACN CID device identifier of the `SacnSourceInternal`.
    fn cid(&self) -> &Uuid {
        &self.cid
    }

    /// Sets the ACN CID device identifier.
    ///
    /// # Arguments
    /// cid: The new CID identifier for this source. It is left to the user to ensure that this is always unique within the network the source is in.
    fn set_cid(&mut self, cid: Uuid) {
        self.cid = cid;
    }

    /// Returns the ACN source name.
    fn name(&self) -> &str {
        &self.name
    }

    /// Sets ACN source name.
    ///
    /// # Argument
    /// name: The new name for the source, it is left to the user to ensure this is unique within the sACN network.
    ///
    /// # Errors
    /// `MalformedSourceName`: Returned to indicate that the given source name is longer than the maximum allowed as per `E131_SOURCE_NAME_MAX_LENGTH` (63 bytes, leaving room for the null terminator).
    fn set_name(&mut self, name: &str) -> Result<()> {
        if name.len() > E131_SOURCE_NAME_MAX_LENGTH {
            return Err(SacnError::MalformedSourceName(
                "Source name provided is longer than maximum allowed".to_string(),
            ));
        }
        self.name = name.to_string();

        Ok(())
    }

    /// Returns if `SacnSourceInternal` is in preview mode.
    fn preview_mode(&self) -> bool {
        self.preview_data
    }

    /// Sets the value of the `Preview_Data` flag in packets from this `SacnSourceInternal`.
    ///
    /// # Arguments
    /// `preview_mode`: If true then all data packets from this `SacnSourceInternal` will have the `Preview_Data` flag set to true indicating that the data is not
    ///     for live output. If false then the flag will be set to false.
    fn set_preview_mode(&mut self, preview_mode: bool) {
        self.preview_data = preview_mode;
    }

    /// Sets the multicast time to live.
    ///
    /// # Arguments
    /// `multicast_ttl`: The new time to live value for network packets sent using multicast.
    ///
    /// # Errors
    /// Io: Returned if the multicast TTL fails to be set on the underlying socket.
    fn set_multicast_ttl(&self, multicast_ttl: u32) -> Result<()> {
        if self.addr.is_ipv6() {
            Ok(self.socket.set_multicast_hops_v6(multicast_ttl)?)
        } else {
            Ok(self.socket.set_multicast_ttl_v4(multicast_ttl)?)
        }
    }

    /// Returns the current Time To Live for unicast packets send by this source.
    ///
    /// # Errors
    /// Io: Returned if the TTL cannot be retrieved from the underlying socket.
    fn ttl(&self) -> Result<u32> {
        if self.addr.is_ipv6() {
            Ok(self.socket.unicast_hops_v6()?)
        } else {
            Ok(self.socket.ttl_v4()?)
        }
    }

    /// Sets the Time To Live for unicast packets sent by this source.
    ///
    /// # Arguments
    /// ttl: The new time to live value for network packets sent by the source.
    ///
    /// # Errors
    /// Io: Returned if the TTL fails to be set on the underlying socket.
    fn set_ttl(&mut self, ttl: u32) -> Result<()> {
        if self.addr.is_ipv6() {
            Ok(self.socket.set_unicast_hops_v6(ttl)?)
        } else {
            Ok(self.socket.set_ttl_v4(ttl)?)
        }
    }

    /// Returns the multicast time to live of the socket.
    fn multicast_ttl(&self) -> Result<u32> {
        if self.addr.is_ipv6() {
            Ok(self.socket.multicast_hops_v6()?)
        } else {
            Ok(self.socket.multicast_ttl_v4()?)
        }
    }

    /// Sets if multicast loop is enabled.
    ///
    /// # Arguments:
    /// `multicast_loop`: If true then multicast loop is enabled, if false it is not.
    ///
    /// # Errors
    /// Io: Returned if the `set_multicast_loop` option fails to be set on the socket.
    fn set_multicast_loop_v4(&self, multicast_loop: bool) -> Result<()> {
        Ok(self.socket.set_multicast_loop_v4(multicast_loop)?)
    }

    /// Sets if multicast loop is enabled, using the option for this source's IP version.
    fn set_multicast_loop(&self, multicast_loop: bool) -> Result<()> {
        if self.addr.is_ipv6() {
            Ok(self.socket.set_multicast_loop_v6(multicast_loop)?)
        } else {
            Ok(self.socket.set_multicast_loop_v4(multicast_loop)?)
        }
    }

    /// Returns true if multicast loop is enabled, false if not.
    fn multicast_loop(&self) -> Result<bool> {
        if self.addr.is_ipv6() {
            Ok(self.socket.multicast_loop_v6()?)
        } else {
            Ok(self.socket.multicast_loop_v4()?)
        }
    }

    /// Returns the universes currently registered on this source.
    pub fn universes(&self) -> Vec<u16> {
        self.universes.clone()
    }
}

/// Returns the locked internal `SacnSourceInternal` used within the `SacnSource`.
///
/// This centralises the locking of the source to a single point within the code allowing any changes to the mechanism to be made in one place.
///
/// This differs to (`unlock_internal_mut`) as it takes an immutable reference to internal.
///
/// # Arguments
/// internal: The `SacnSourceInternal` to unlock encapsulated within an Arc and Mutex.
///
/// # Errors
/// `SourceCorrupt`: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
/// a panic while accessing causing the source to be left in a potentially inconsistent state.
fn unlock_internal(
    internal: &Arc<Mutex<SacnSourceInternal>>,
) -> Result<MutexGuard<'_, SacnSourceInternal>> {
    match internal.lock() {
        Err(_) => {
            // The PoisonError returned doesn't contain further information and just allows access to the internal potentially inconsistent sender which
            // shouldn't be exposed to the user (as its internal and would have no use).
            // Cannot directly return the PoisonError due to PoisonError using a different error system to other std modules which doesn't work with
            // error_chain.
            Err(SacnError::SourceCorrupt("Mutex poisoned".to_string()))
        }
        Ok(lock) => Ok(lock),
    }
}

/// Returns the locked internal `SacnSourceInternal` used within the `SacnSource`.
///
/// This centralises the locking of the source to a single point within the code allowing any changes to the mechanism to be made in one place.
///
/// This differs to (`unlock_internal`) as it takes an mutable reference to internal.
///
/// # Arguments
/// internal: The `SacnSourceInternal` to unlock encapsulated within an Arc and Mutex.
///
/// # Errors
/// Returns an `SourceCorrupt` error if the Mutex used to control access to the internal sender is poisoned by a thread encountering
/// a panic while accessing causing the source to be left in a potentially inconsistent state.
fn unlock_internal_mut(
    internal: &mut Arc<Mutex<SacnSourceInternal>>,
) -> Result<MutexGuard<'_, SacnSourceInternal>> {
    match internal.lock() {
        Err(_) => {
            // The PoisonError returned doesn't contain further information and just allows access to the internal potentially inconsistent sender which
            // shouldn't be exposed to the user (as its internal and would have no use).
            // Cannot directly return the PoisonError due to PoisonError using a different error system to other std modules which doesn't work with
            // error_chain.
            Err(SacnError::SourceCorrupt("Mutex poisoned".to_string()))
        }
        Ok(lock) => Ok(lock),
    }
}

/// Called periodically by the source update thread.
///
/// Is responsible for sending the periodic universe discovery packets.
///
/// # Arguments:
/// src: A reference to the `SacnSourceInternal` for which to send the universe discovery packet with/from.
///
/// # Errors
/// Returns a `SourceCorrupt` error if the internal source mutex has been corrupted, see [`unlock_internal`].
///
/// Returns an error if a discovery packet cannot be sent, see (`send_universe_discovery`)[`fn.send_universe_discovery.source`].
fn perform_periodic_update(src: &mut Arc<Mutex<SacnSourceInternal>>) -> Result<()> {
    let mut unwrap_src = unlock_internal_mut(src)?;
    if unwrap_src.is_sending_discovery
        && Instant::now().duration_since(unwrap_src.last_discovery_advert_timestamp)
            > E131_UNIVERSE_DISCOVERY_INTERVAL
    {
        unwrap_src.send_universe_discovery()?;
        unwrap_src.last_discovery_advert_timestamp = Instant::now();
    }
    Ok(())
}

/// Splits the given (sorted) universe list into universe discovery pages.
///
/// Returns one `(page, last_page, universes)` entry per page, each holding at most `DISCOVERY_UNI_PER_PAGE` universes.
/// A list of n universes gives ceil(n / 512) pages with a last page of `(n - 1) / 512`, so a source with no universes
/// sends no discovery packet at all and an exact multiple of 512 does not produce a trailing empty page.
fn discovery_pages(universes: &[u16]) -> Vec<(u8, u8, &[u16])> {
    let chunks: Vec<&[u16]> = universes.chunks(DISCOVERY_UNI_PER_PAGE).collect();
    // A u16 universe field and no duplicates give at most 65536 / 512 = 128 pages, so the page numbers fit in a u8.
    let last_page = chunks.len().saturating_sub(1) as u8;
    chunks
        .into_iter()
        .enumerate()
        .map(|(p, c)| (p as u8, last_page, c))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    /// A send failure on one universe must not stop the remaining universes from being terminated.
    #[cfg(unix)]
    #[test]
    fn test_terminate_attempts_every_universe() {
        let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0);
        let mut src = SacnSourceInternal::with_cid_ip("Test", Uuid::new_v4(), ip).unwrap();
        src.register_universes(&[1, 2, 3]).unwrap();

        // Swap in a Unix domain socket so every IP multicast send fails.
        src.socket = Socket::new(Domain::UNIX, Type::DGRAM, None).unwrap();

        assert!(src.terminate(DEFAULT_TERMINATE_START_CODE).is_err());
        assert!(
            src.universes.is_empty(),
            "universes left registered after terminate: {:?}",
            src.universes
        );
        assert!(!src.running);
    }

    /// A source with no universes has nothing to advertise and sends no discovery page.
    #[test]
    fn test_discovery_pages_empty() {
        assert!(discovery_pages(&[]).is_empty());
    }

    #[test]
    fn test_discovery_pages_single_page() {
        let unis: Vec<u16> = (1..=3).collect();
        let pages = discovery_pages(&unis);
        assert_eq!(pages, vec![(0, 0, &unis[..])]);
    }

    /// Exactly 512 universes fill one page; no trailing empty page is sent.
    #[test]
    fn test_discovery_pages_exact_multiple() {
        let unis: Vec<u16> = (1..=(DISCOVERY_UNI_PER_PAGE as u16)).collect();
        let pages = discovery_pages(&unis);
        assert_eq!(pages, vec![(0, 0, &unis[..])]);

        let unis: Vec<u16> = (1..=(2 * DISCOVERY_UNI_PER_PAGE as u16)).collect();
        let pages = discovery_pages(&unis);
        assert_eq!(pages.len(), 2);
        assert_eq!(pages[0], (0, 1, &unis[..DISCOVERY_UNI_PER_PAGE]));
        assert_eq!(pages[1], (1, 1, &unis[DISCOVERY_UNI_PER_PAGE..]));
    }

    #[test]
    fn test_discovery_pages_partial_last_page() {
        let unis: Vec<u16> = (1..=(DISCOVERY_UNI_PER_PAGE as u16 + 1)).collect();
        let pages = discovery_pages(&unis);
        assert_eq!(pages.len(), 2);
        assert_eq!(pages[0], (0, 1, &unis[..DISCOVERY_UNI_PER_PAGE]));
        assert_eq!(pages[1], (1, 1, &unis[DISCOVERY_UNI_PER_PAGE..]));
    }

    /// The largest possible universe set still fits the 8-bit page fields.
    #[test]
    fn test_discovery_pages_max() {
        let unis: Vec<u16> = (0..=u16::MAX).collect();
        let pages = discovery_pages(&unis);
        assert_eq!(pages.len(), 128);
        assert_eq!(pages.last().unwrap().0, 127);
        assert!(
            pages
                .iter()
                .all(|(_, last, u)| *last == 127 && u.len() == DISCOVERY_UNI_PER_PAGE)
        );
    }
}
