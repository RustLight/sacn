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


// TODO
// The internal corruption error handling?
// Keep pulling documentation up
// Spell check for 'receiver' and 'synchronisation'
// Generate documentation, check links
// Fix SSH tests
// Conduct compliance tests
// Generate wireshark packets to use for inspection
// Do wireshark inspection / test
// Pack code.

use error::errors::{*};
use packet::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::cmp;
use std::cmp::min;
use std::time::{Duration, Instant};
use std::thread::{JoinHandle};
use std::thread;
use std::sync::{Arc, MutexGuard, Mutex};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

/// Socket2 used to create the underlying UDP socket that sACN is sent on.
use socket2::{Socket, Domain, Type};

/// UUID library used to handle the UUID's used in the CID fields.
use uuid::Uuid;

/// The name of the thread which runs periodically to perform various actions such as universe discovery adverts for the source.
const SND_UPDATE_THREAD_NAME: &'static str = "rust_sacn_snd_update_thread"; 

/// The default startcode used to send stream termination packets when the SacnSource is closed.
const DEFAULT_TERMINATE_START_CODE: u8 = 0; 

/// The poll rate of the update thread.
/// Discovery updates are sent every E131_UNIVERSE_DISCOVERY_INTERVAL so the poll rate must be lower than or equal to this.
// const DEFAULT_POLL_PERIOD: Duration = E131_UNIVERSE_DISCOVERY_INTERVAL;
const DEFAULT_POLL_PERIOD: Duration = Duration::from_secs(1);

/// A DMX over sACN sender.
///
/// SacnSource is used for sending sACN packets over an IP network.
///
/// # Examples
///
/// ```
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

    /// Update thread which performs actions every DEFAULT_POLL_PERIOD such as checking if a universe 
    /// discovery packet should be sent.
    update_thread: Option<JoinHandle<()>>
}

/// Internal sACN sender, this does most of the work however is encapsulated within SacnSource
/// to allow access by the update_thread which is used to manage sending periodic universe discovery packets.
#[derive(Debug)]
struct SacnSourceInternal {
    /// Underlying UDP socket used for sending sACN packets on the network.
    socket: Socket,

    /// The address of this SacnSourceInternal on the network.
    addr: SocketAddr,

    /// The unique ID of this SacnSourceInternal.
    /// It is the job of the user of the library to ensure that the cid is given on creation of the SacnSourceInternal is unique.
    cid: Uuid,

    /// The human readable name of this source.
    name: String,

    /// Flag which is included in sACN packets to indicate that the data shouldn't be used for live output 
    /// (ie. on actual lighting fixtures). A receiver may or may not be compliant with this so it should not be relied
    /// upon in an untested environment.
    preview_data: bool,

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

    /// Flag that indicates if the SacnSourceInternal is running (the update thread should be triggering periodic discovery packets).
    running: bool,

    /// The time that the last universe discovery advert was send.
    last_discovery_advert_timestamp: Instant,

    /// Flag that is set to True to indicate that the source is sending periodic universe discovery packets.
    is_sending_discovery: bool,
}

impl SacnSource {
    /// Constructs a new SacnSource with the given name, binding to an IPv4 address.
    /// This generates a new CID automatically using random values.
    /// 
    /// # Errors
    /// See (with_cid_ip)[with_cid_ip]
    pub fn new_v4(name: &str) -> Result<SacnSource> {
        let cid = Uuid::new_v4();
        SacnSource::with_cid_v4(name, cid)
    }

    /// Constructs a new SacnSource with the given name and specified CID binding to an IPv4 address.
    /// 
    /// # Errors
    /// See (with_cid_ip)[with_cid_ip]
    pub fn with_cid_v4(name: &str, cid: Uuid) -> Result<SacnSource> {
        let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT);
        SacnSource::with_cid_ip(name, cid, ip)
    }

    /// Constructs a new SacnSource with the given name, binding to an IPv6 address.
    /// By default this will only receive IPv6 data but IPv4 can also be enabled by calling set_ipv6_only(false).
    /// 
    /// # Errors
    /// See (with_cid_ip)[with_cid_ip]
    pub fn new_v6(name: &str) -> Result<SacnSource> {
        let cid = Uuid::new_v4();
        SacnSource::with_cid_v6(name, cid)
    }

    /// Constructs a new SacnSource with the given name and specified CID binding to an IPv6 address.
    /// 
    /// # Errors
    /// See (with_cid_ip)[with_cid_ip]
    pub fn with_cid_v6(name: &str, cid: Uuid) -> Result<SacnSource> {
        let ip = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT);
        SacnSource::with_cid_ip(name, cid, ip)
    }

    /// Constructs a new SacnSource with the given name and binding to the supplied ip.
    /// 
    /// # Errors
    /// See (with_cid_ip)[with_cid_ip]
    pub fn with_ip(name: &str, ip: SocketAddr) -> Result<SacnSource> {
        SacnSource::with_cid_ip(name, Uuid::new_v4(), ip)
    }

    /// Constructs a new SacnSource with the given name, cid and binding to the supplied ip.
    /// 
    /// # Errors
    /// Io: Returned if the underlying UDP socket cannot be created and bound or if the thread used for sending periodic 
    ///     discovery adverts fails to be created. Causes can be distinguished by looking at the error chain.
    /// 
    /// UnsupportedIpVersion: Returned if the SocketAddr is not IPv4 or IPv6.
    /// 
    pub fn with_cid_ip(name: &str, cid: Uuid, ip: SocketAddr) -> Result<SacnSource> {
        let trd_builder = thread::Builder::new().name(SND_UPDATE_THREAD_NAME.into());

        let internal_src = Arc::new(Mutex::new(SacnSourceInternal::with_cid_ip(name, cid, ip)?));

        let mut trd_src = internal_src.clone();

        let src = SacnSource { 
            internal: internal_src,
            update_thread: Some(trd_builder.spawn(move || {
                while trd_src.lock().unwrap().running {
                    thread::sleep(DEFAULT_POLL_PERIOD);
                    match perform_periodic_update(&mut trd_src) {
                        Err(e) => {
                            println!("Periodic error: {:?}", e);
                        }
                        
                        _ => {
                            // In-case of an error on the discovery thread the source continues to operate and tries again.
                            // As no unsafe code blocks are used the rust compiler guarantees this is memory safe.
                            
                        }
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
    /// set_is_sending_discovery flag) periodically. 
    /// 
    /// This is more efficient than repeated calls to register_universe as it means only 1 mutex unlock is required.
    /// 
    /// # Arguments
    /// universes: The sACN universes to register for usage as data universes and/or synchronisation addresses. Note that sACN
    ///     universes start at 1 not 0.
    /// 
    /// # Errors
    /// IllegalUniverse: Returned if a universe is outwith the range permitted by ANSI E1.31-2018. 
    /// 
    /// SourceCorrupt: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state. 
    /// 
    pub fn register_universes(&mut self, universes: &[u16]) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.register_universes(universes)
    }

    /// Registers a single universe on this source in addition to already registered universes.
    /// 
    /// This allows sending data to those universes or using them as synchronisation addresses as well as adding them to 
    /// the list of universes that appear in universe discovery packets that are sent (depending on the 
    /// set_is_sending_discovery flag) periodically. 
    /// 
    /// If registering multiple universes see (register_universes)[register_universes].
    /// 
    /// # Arguments
    /// universe: The sACN universe to register for usage as a data universe and/or synchronisation address. Note that sACN
    ///     universes start at 1 not 0.
    /// 
    /// # Errors
    /// IllegalUniverse: Returned if the universe is outwith the range permitted by ANSI E1.31-2018. 
    /// 
    /// SourceCorrupt: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state. 
    /// 
    pub fn register_universe(&mut self, universe: u16) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.register_universe(universe)
    }

    /// Sends the given data to the given universes with the given priority, synchronisation address (universe) and destination ip.
    /// 
    /// # Arguments
    /// 
    /// universe:     The sACN universes that the data should be set on, the data will be split over these universes with each UNIVERSE_CHANNEL_CAPACITY
    ///                 sized chunk sent to the next universe.
    /// 
    /// data:         The data that should be sent, must have a length greater than 0.
    /// 
    /// priority:     The E131 priority that the data should be sent with, must be less than E131_MAX_PRIORITY (const.E131_MAX_PRIORITY.packet), 
    ///                 if a value of None is provided then the default of E131_DEFAULT_PRIORITY (const.E131_DEFAULT_PRIORITY.packet) is used.
    /// 
    /// dst_ip:       The destination IP, can be Ipv4 or Ipv6, None if should be sent using ip multicast.
    /// 
    /// sync_address: The address to use for synchronisation, must be a valid universe, None indicates no synchronisation. If synchronisation is required a
    ///                 reasonable default address to use is the first universe that this data is being sent to.
    /// 
    /// As per ANSI E1.31-2018 Section 6.6.1 this method shouldn't be called at a higher refresher rate than specified in ANSI E1.11 [DMX] unless 
    ///     configured by the user to do so in an environment which doesn't contain any E1.31 to DMX512-A converters.
    /// 
    /// Note as per ANSI-E1.31-2018 Appendix B.1 it is recommended to have a small delay before sending the follow up sync packet.
    /// 
    /// # Errors
    /// SenderAlreadyTerminated: Returned if this method is called on an SacnReceiverInternal that has already terminated.
    /// 
    /// InvalidInput: Returned if the data array has length 0 or if an insufficient number of universes for the given data are provided (each universe takes 513 bytes of data).
    /// 
    /// InvalidPriority: Returned if the priority is greater than the allowed maximum priority of E131_MAX_PRIORITY.
    /// 
    /// IllegalUniverse: Returned if the universe is outwith the allowed range as specified by ANSI E1.31-2018 Section 6.2.7.
    /// 
    /// UniverseNotRegistered: Returned if the universe is not registered on the given SacnSourceInternal.
    /// 
    /// ExceedUniverseCapacity: Returned if the data has a length greater than the maximum allowed within a universe (packet::UNIVERSE_CHANNEL_CAPACITY).
    /// 
    /// Io: Returned if the data fails to be sent on the socket, see send_to(fn.send_to.Socket).
    /// 
    /// SourceCorrupt: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state. 
    /// 
    pub fn send(&mut self, universes: &[u16], data: &[u8], priority: Option<u8>, dst_ip: Option<SocketAddr>, synchronisation_addr: Option<u16>) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.send(universes, data, priority, dst_ip, synchronisation_addr)
    }

    /// Sends a synchronisation packet to trigger the sending of packets waiting to be sent together.
    /// 
    /// A common pattern would be to use the send method to send data to all the universes that should be synchronised using a 
    /// chosen synchronisation universe then wait for a small time as per the recommendation in ANSI-E1.31-2018 Appendix B.1 and 
    /// then send a synchronisation packet with the address of the synchronisation universe chosen to trigger the packets.
    /// 
    /// # Arguments
    /// universe: The universe of this synchronisation packet.
    /// dst_ip:   The destination IP address for this packet or None if it should be sent using multicast.
    /// 
    /// # Errors
    /// IllegalUniverse: Returned if the universe is outwith the allowed range of sACN universes as defined in ANSI E1.31-2018 Section 6.2.7.
    /// 
    /// UniverseNotRegistered: Returned if the universe is not registered on the given SacnSourceInternal.
    /// 
    /// Io: Returned if the packet fails to be sent using the underlying network socket.
    /// 
    /// SacnParsePackError: Returned if the sync packet fails to be packed.
    /// 
    /// SourceCorrupt: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state. 
    /// 
    pub fn send_sync_packet(&mut self, universe: u16, dst_ip: Option<SocketAddr>) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.send_sync_packet(universe, dst_ip)
    }

    /// Terminates sending on the given universe.
    /// 
    /// # Errors:
    /// IllegalUniverse: Returned if the universe is outwith the allowed range of sACN universes as defined in ANSI E1.31-2018 Section 6.2.7.
    /// 
    /// UniverseNotRegistered: Returned if the universe is not registered on this source.
    /// 
    /// Io: Returned if the termination packets fail to be sent on the socket.
    /// 
    /// SourceCorrupt: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state. 
    /// 
    pub fn terminate_stream(&mut self, universe: u16, start_code: u8) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.terminate_stream(universe, start_code)
    }

    /// Returns the ACN CID device identifier of the SacnSourceInternal.
    /// 
    /// # Errors
    /// SourceCorrupt: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state. 
    /// 
    pub fn cid(&self) -> Result<Uuid> {
        Ok(*unlock_internal(&self.internal)?.cid())
    }

    /// Sets the ACN CID device identifier.
    /// 
    /// # Arguments
    /// cid: The new CID identifier for this source. It is left to the user to ensure that this is always unique within the network the source is in.
    /// 
    /// # Errors
    /// SourceCorrupt: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state. 
    /// 
    pub fn set_cid(&mut self, cid: Uuid) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.set_cid(cid);
        Ok(())
    }

    /// Returns the ACN source name.
    /// 
    /// # Errors
    /// SourceCorrupt: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state. 
    /// 
    pub fn name(&self) -> Result<String> {
        Ok(unlock_internal(&self.internal)?.name().into())
    }

    /// Sets ACN source name.
    /// 
    /// # Argument
    /// name: The new name for the source, it is left to the user to ensure this is unique within the sACN network.
    /// 
    /// # Errors
    /// SourceCorrupt: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state. 
    /// 
    pub fn set_name(&mut self, name: &str) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.set_name(name);
        Ok(())
    }

    /// Returns true if SacnSourceInternal is in preview mode, false if not.
    /// 
    /// For details of preview_mode see (set_preview_mode)[set_preview_mode].
    /// 
    /// # Errors
    /// SourceCorrupt: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state. 
    /// 
    pub fn preview_mode(&self) -> Result<bool> {
        Ok(unlock_internal(&self.internal)?.preview_mode())
    }

    /// Sets the value of the Preview_Data flag in packets from this SacnSource.
    ///
    /// # Arguments
    /// preview_mode: If true then all data packets from this SacnSource will have the Preview_Data flag set to true indicating that the data is not
    ///     for live output. If false then the flag will be set to false.
    /// 
    /// # Errors
    /// SourceCorrupt: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state. 
    /// 
    pub fn set_preview_mode(&mut self, preview_mode: bool) -> Result<()> {
       unlock_internal_mut(&mut self.internal)?.set_preview_mode(preview_mode);
       Ok(())
    }

    /// Sets the is_sending_discovery flag to the given value.
    /// 
    /// # Arguments
    /// val: The new value for the is_sending_discovery flag, if true then source will send periodic universe discovery packets
    /// and if false it won't.
    /// 
    pub fn set_is_sending_discovery(&mut self, val: bool) {
        self.internal.lock().unwrap().set_is_sending_discovery(val);
    }

    /// Returns the multicast time to live of the socket.
    /// 
    pub fn multicast_ttl(&self) -> Result<u32> {
        unlock_internal(&self.internal)?.multicast_ttl()
    }

    /// Sets the multicast time to live.
    /// 
    /// # Arguments
    /// multicast_ttl: The new time to live value for network packets sent using multicast.
    /// 
    /// # Errors
    /// Io: Returned if the multicast TTL fails to be set on the underlying socket.
    /// 
    /// SourceCorrupt: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state. 
    /// 
    pub fn set_multicast_ttl(&mut self, multicast_ttl: u32) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.set_multicast_ttl(multicast_ttl)
    }

    /// Sets the Time To Live for packets sent by this source.
    /// 
    /// # Arguments
    /// ttl: The new time to live value for new packets.
    /// 
    /// # Errors
    /// Io: Returned if the TTL value cannot be changed.
    /// 
    /// SourceCorrupt: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state. 
    /// 
    pub fn set_ttl(&mut self, ttl: u32) -> Result<()>{
        unlock_internal_mut(&mut self.internal)?.set_ttl(ttl)
    }

    /// Sets if multicast loop is enabled.
    /// 
    /// # Arguments:
    /// multicast_loop: If true then multicast loop is enabled, if false it is not.
    /// 
    /// # Errors
    /// Io: Returned if the set_multicast_loop option fails to be set on the socket.
    /// 
    /// SourceCorrupt: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state. 
    /// 
    pub fn set_multicast_loop_v4(&mut self, multicast_loop: bool) -> Result<()> {
        unlock_internal_mut(&mut self.internal)?.set_multicast_loop_v4(multicast_loop)
    }

    /// Returns true if multicast loop is enabled, false if not.
    /// 
    /// # Errors
    /// SourceCorrupt: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
    /// a panic while accessing causing the source to be left in a potentially inconsistent state. 
    /// 
    pub fn multicast_loop(&self) -> Result<bool> {
        unlock_internal(&self.internal)?.multicast_loop()
    }
}

/// By implementing the Drop trait for SacnSource it means that the user doesn't have to explicitly clean up the source
/// and if it goes out of reference it will clean itself up and send the required termination packets etc.
/// 
impl Drop for SacnSource {
    fn drop(&mut self){
        match unlock_internal_mut(&mut self.internal) {
            Ok(mut i) => { i.running = false; }
            Err(_) => { return; } // As drop isn't always explicitly called and cannot return an error the error is ignored. Memory safety is maintain and this prevents causing a panic!.
        };

        if let Some(thread) = self.update_thread.take() {
            {
                match unlock_internal_mut(&mut self.internal) { // Internal is accessed twice separately, this allows the discovery thread to interleave between running being set to false speeding up termination.
                    Ok(mut i) => { 
                        match i.terminate(DEFAULT_TERMINATE_START_CODE) {
                            _ => {} // For same reasons as above a potential error is ignored and a 'best attempt' is used to clean up.
                        }
                    }
                    Err(_) => {} // As drop isn't always explicitly called and cannot return an error the error is ignored. Memory safety is maintain and this prevents causing a panic!.
                };
            }
            thread.join().unwrap();
        }
    }
}

impl SacnSourceInternal {
    /// Constructs a new SacnSourceInternal with DMX START code set to 0 with specified CID and binding IP address.
    /// 
    /// By default for an IPv6 address this will only receive IPv6 data but IPv4 can also be enabled by calling set_ipv6_only(false).
    /// By default the TTL for ipv4 packets is 1 to keep them within the local network.
    /// 
    /// # Arguments:
    /// name: The human readable name for this sacn source.
    /// cid:  The UUID for this source.
    /// ip:   The address that this source should bind to.
    /// 
    /// # Errors
    /// Io: Returned if the underlying socket cannot be created or the IP cannot be bound to the underlying socket. See (UdpBuilder::new_v4)[fn.new_v4.UdpBuilder], (UdpBuilder::new_v6)[fn.new_v6.UdpBuilder] and (Socket::bind)[fn.bind.Socket2].
    /// 
    /// UnsupportedIpVersion: Returned if the SockAddr is not IPv4 or IPv6.
    /// 
    fn with_cid_ip(name: &str, cid: Uuid, ip: SocketAddr) -> Result<SacnSourceInternal> {
        let socket = if ip.is_ipv4() {
            Socket::new(Domain::ipv4(), Type::dgram(), None).unwrap()
        } else if ip.is_ipv6() {
            Socket::new(Domain::ipv6(), Type::dgram(), None).unwrap()
        } else {
            bail!(ErrorKind::UnsupportedIpVersion("Address to create SacnSource is not IPv4 or IPv6".to_string()));
        };
        
        // Multiple different processes might want to send to the sACN stream so therefore need to allow re-using the ACN port.
        // Set reuse port is only supported on linux.
        #[cfg(target_os = "linux")]
        socket.set_reuse_port(true)?;

        // Set reuse address supported on linux and windows.
        socket.set_reuse_address(true)?;
        socket.bind(&ip.into())?;

        let ds = SacnSourceInternal {
            socket: socket,
            addr: ip,
            cid: cid,
            name: name.to_string(),
            preview_data: false,
            data_sequences: RefCell::new(HashMap::new()),
            sync_sequences: RefCell::new(HashMap::new()),
            universes: Vec::new(),
            running: true,
            last_discovery_advert_timestamp: Instant::now(),
            is_sending_discovery: true
        };

        Ok(ds)
    }

    /// Sets the is_sending_discovery flag to the given value.
    /// 
    /// If is_sending_discovery is set to false then no discovery adverts for this source
    /// will be sent otherwise (and by default) they will be sent every UNIVERSE_DISCOVERY_INTERVAL.
    /// 
    /// # Arguments:
    /// val: The new value of the is_sending_discovery flag.
    /// 
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
    /// See register_universe(fn.register_universe.source) for more details.
    /// 
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
    /// IllegalUniverse: Returned if the universe is outwith the allowed range, see (is_universe_in_range)[fn.is_universe_in_range.packet].
    /// 
    fn register_universe(&mut self, universe: u16) -> Result<()> {
        is_universe_in_range(universe)?;

        if self.universes.len() == 0 {
            self.universes.push(universe);
        } else {
            match self.universes.binary_search(&universe) {
                Err(i) => { // Value not found, i is the position it should be inserted
                    self.universes.insert(i, universe);
                }
                Ok(_) => {
                    // If value found then don't insert to avoid duplicates.
                }
            }
        }
        Ok(())
    }

    /// De-registers the given universe for sending with this source.
    /// 
    /// # Errors
    /// IllegalUniverse: Returned if the universe is outwith the allowed range, see (is_universe_in_range)[fn.is_universe_in_range.packet].
    /// 
    /// UniverseNotFound: Returned if the given universe was never registered originally.
    /// 
    fn deregister_universe(&mut self, universe: u16) -> Result<()> {
        is_universe_in_range(universe)?;

        if self.universes.len() == 0 {
            self.universes.push(universe);
            Ok(())
        } else {
            match self.universes.binary_search(&universe) {
                Err(_i) => { // Value not found
                    bail!(ErrorKind::UniverseNotFound("Attempted to de-register a universe that was never registered".to_string()))
                }
                Ok(i) => { // Value found, i is index.
                    self.universes.remove(i);
                    Ok(())
                }
            }
        }
    }

    /// Checks if the given universe is a valid universe to send on (within allowed range) and that it is registered with this SacnSourceInternal.
    /// 
    /// # Errors
    /// IllegalUniverse: Returned if the universe is outwith the allowed range, see (is_universe_in_range)[fn.is_universe_in_range.packet].
    /// 
    /// UniverseNotRegistered: Returned if the universe is not registered on the given SacnSourceInternal.
    /// 
    fn universe_allowed(&self, u: &u16) -> Result<()>{
        is_universe_in_range(*u)?;

        if !self.universes.contains(u) {
            bail!(ErrorKind::UniverseNotRegistered(format!("Attempted to send on unregistered universe : {}", u).to_string()));
        }

        Ok(())
    }

    /// Sends the given data to the given universes with the given priority, synchronisation address (universe) and destination ip.
    /// 
    /// # Arguments
    /// 
    /// universe:     The sACN universes that the data should be set on, the data will be split over these universes with each UNIVERSE_CHANNEL_CAPACITY
    ///                 sized chunk sent to the next universe.
    /// 
    /// data:         The data that should be sent, must have a length greater than 0.
    /// 
    /// priority:     The E131 priority that the data should be sent with, must be less than E131_MAX_PRIORITY (const.E131_MAX_PRIORITY.packet), 
    ///                 if a value of None is provided then the default of E131_DEFAULT_PRIORITY (const.E131_DEFAULT_PRIORITY.packet) is used.
    /// 
    /// dst_ip:       The destination IP, can be Ipv4 or Ipv6, None if should be sent using ip multicast.
    /// 
    /// sync_address: The address to use for synchronisation, must be a valid universe, None indicates no synchronisation. If synchronisation is required a
    ///                 reasonable default address to use is the first universe that this data is being sent to.
    /// 
    /// As per ANSI E1.31-2018 Section 6.6.1 this method shouldn't be called at a higher refresher rate than specified in ANSI E1.11 [DMX] unless 
    ///     configured by the user to do so in an environment which doesn't contain any E1.31 to DMX512-A converters.
    /// 
    /// Note as per ANSI-E1.31-2018 Appendix B.1 it is recommended to have a small delay before sending the follow up sync packet.
    /// 
    /// # Errors
    /// SenderAlreadyTerminated: Returned if this method is called on an SacnReceiverInternal that has already terminated.
    /// 
    /// InvalidInput: Returned if the data array has length 0 or if an insufficient number of universes for the given data are provided (each universe takes 513 bytes of data).
    /// 
    /// InvalidPriority: Returned if the priority is greater than the allowed maximum priority of E131_MAX_PRIORITY.
    /// 
    /// IllegalUniverse: Returned if the universe is outwith the allowed range as specified by ANSI E1.31-2018 Section 6.2.7.
    /// 
    /// UniverseNotRegistered: Returned if the universe is not registered on the given SacnSourceInternal.
    /// 
    /// ExceedUniverseCapacity: Returned if the data has a length greater than the maximum allowed within a universe (packet::UNIVERSE_CHANNEL_CAPACITY).
    /// 
    /// Io: Returned if the data fails to be sent on the socket, see send_to(fn.send_to.Socket).
    /// 
    fn send(&self, universes: &[u16], data: &[u8], priority: Option<u8>, dst_ip: Option<SocketAddr>, synchronisation_addr: Option<u16>) -> Result<()> {
        if self.running == false { // Indicates that this sender has been terminated.
            bail!(ErrorKind::SenderAlreadyTerminated("Attempted to send".to_string())); 
        }

        if data.len() == 0 {
           bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Must provide data to send, data.len() == 0"));
        }

        // Check all the given universes are valid before doing any action.
        // This prevents leaving the source in an inconsistent state if later a universe is found to be invalid.
        for u in universes {
            self.universe_allowed(u)?;
        }

        // Check that the synchronisation universe is also valid.
        if synchronisation_addr.is_some() {
            self.universe_allowed(&synchronisation_addr.unwrap()).chain_err(|| "Synchronisation universe not allowed")?;
        }

        // + 1 as there must be at least 1 universe required as the data isn't empty then additional universes for any more.
        let required_universes = (data.len() as f64 / UNIVERSE_CHANNEL_CAPACITY as f64).ceil() as usize;

        if universes.len() < required_universes {
            bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Must provide enough universes to send on, universes provided: {}", universes.len())));
        }
        
        for i in 0 .. required_universes {
            let start_index = i * UNIVERSE_CHANNEL_CAPACITY;
            // Safety check to make sure that the end index doesn't exceed the data length
            let end_index = cmp::min((i + 1) * UNIVERSE_CHANNEL_CAPACITY, data.len());

            self.send_universe(universes[i], &data[start_index .. end_index], 
                priority.unwrap_or(E131_DEFAULT_PRIORITY), &dst_ip, synchronisation_addr.unwrap_or(NO_SYNC_UNIVERSE))?;
        }

        Ok(())
    }

    /// Sends the given data to the given universe with the given priority, synchronisation address (universe) and destination ip.
    /// 
    /// # Arguments
    /// universe:     The sACN universe that the data should be set on.
    /// 
    /// data:         The data that should be sent, must be less than or equal in length to UNIVERSE_CHANNEL_CAPACITY(const.UNIVERSE_CHANNEL_CAPACITY.packet).
    /// 
    /// priority:     The E131 priority that the data should be sent with, must be less than E131_MAX_PRIORITY (const.E131_MAX_PRIORITY.packet), default E131_DEFAULT_PRIORITY.
    /// 
    /// dst_ip:       The destination IP, can be Ipv4 or Ipv6, None if should be sent using ip multicast.
    /// 
    /// sync_address: The address to use for synchronisation, must be a valid universe, 0 indicates no synchronisation.
    /// 
    /// # Errors
    /// InvalidInput: Returned if the priority is greater than the allowed maximum priority of E131_MAX_PRIORITY.
    /// 
    /// ExceedUniverseCapacity: Returned if the data has a length greater than the maximum allowed within a universe.
    /// 
    /// IllegalUniverse: Returned if the given universe is outwith the allowed range of universes,
    ///                     see (universe_to_ipv4_multicast_addr)[fn.universe_to_ipv4_multicast_addr.packet] and (universe_to_ipv6_multicast_addr)[fn.universe_to_ipv6_multicast_addr.packet].
    /// 
    /// Io: Returned if the data fails to be sent on the socket, see send_to(fn.send_to.Socket).
    /// 
    fn send_universe(&self, universe: u16, data: &[u8], priority: u8, dst_ip: &Option<SocketAddr>, sync_address: u16) -> Result<()> {
        if priority > E131_MAX_PRIORITY {
            bail!(ErrorKind::InvalidPriority(format!("Priority must be within allowed range of [0-E131_MAX_PRIORITY], priority provided: {}", priority)));
        }

        if data.len() > UNIVERSE_CHANNEL_CAPACITY {
            bail!(ErrorKind::ExceedUniverseCapacity(format!("Data provided must fit in a single universe, data len: {}", data.len())));
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
                    force_synchronization: false,
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

        if dst_ip.is_some() {
            self.socket.send_to(&packet.pack_alloc().unwrap(), &dst_ip.unwrap().into()).chain_err(|| "Failed to send data unicast on socket")?;
        } else {
            let dst;

            if self.addr.is_ipv6(){
                dst = universe_to_ipv6_multicast_addr(universe).chain_err(|| "Failed to convert universe to Ipv6 multicast address")?;
            } else {
                dst = universe_to_ipv4_multicast_addr(universe).chain_err(|| "Failed to convert universe to Ipv4 multicast address")?;
            }

            self.socket.send_to(&packet.pack_alloc().unwrap(), &dst).chain_err(|| "Failed to send data multicast on socket")?;
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
    /// dst_ip:   The destination IP address for this packet or None if it should be sent using multicast.
    /// 
    /// # Errors
    /// IllegalUniverse: Returned if the universe is outwith the allowed range of sACN universes as defined in ANSI E1.31-2018 Section 6.2.7.
    /// 
    /// UniverseNotRegistered: Returned if the universe is not registered on the given SacnSourceInternal.
    /// 
    /// Io: Returned if the packet fails to be sent using the underlying network socket.
    /// 
    /// SacnParsePackError: Returned if the sync packet fails to be packed.
    /// 
    fn send_sync_packet(&self, universe: u16, dst_ip: Option<SocketAddr>) -> Result<()> {
        self.universe_allowed(&universe)?;

        let ip;

        if dst_ip.is_none() {
            if self.addr.is_ipv6(){
                ip = universe_to_ipv6_multicast_addr(universe)?;
            } else {
                ip = universe_to_ipv4_multicast_addr(universe)?;
            }
        } else {
            ip = dst_ip.unwrap().into();
        }

        let mut sequence = match self.sync_sequences.borrow().get(&universe) {
            Some(s) => *s,
            None => STARTING_SEQUENCE_NUMBER,
        };

        let packet = AcnRootLayerProtocol {
            pdu: E131RootLayer {
                cid: self.cid,
                data: E131RootLayerData::SynchronizationPacket(SynchronizationPacketFramingLayer {
                    sequence_number: sequence,
                    synchronization_address: universe
                })
            }
        };
        self.socket.send_to(&packet.pack_alloc()?, &ip).chain_err(|| "Failed to send sync packet on socket")?;

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
    ///     ANSI E1.31-2018 Section 6.2.6, Stream_Terminated: Bit 6.
    /// 
    /// # Arguments
    /// universe: The universe of this synchronisation packet.
    /// dst_ip:   The destination IP address for this packet or None if it should be sent using multicast.
    /// 
    /// # Errors
    /// IllegalUniverse: Returned if the universe is outwith the allowed range of sACN universes as defined in ANSI E1.31-2018 Section 6.2.7.
    /// 
    /// UniverseNotRegistered: Returned if the universe is not registered on the given SacnSourceInternal.
    /// 
    /// Io: Returned if the termination packets fail to be sent on the underlying socket.
    /// 
    fn send_terminate_stream_pkt(&self, universe: u16, dst_ip: Option<SocketAddr>, start_code: u8) -> Result<()> {
        self.universe_allowed(&universe)?;

        let ip = match dst_ip{
            Some(x) => x.into(),
            None => {
                if self.addr.is_ipv6(){
                    universe_to_ipv6_multicast_addr(universe)?
                } else {
                    universe_to_ipv4_multicast_addr(universe)?
                }
            }
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

        self.socket.send_to(res, &ip)?;

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
    /// Terminates a stream to the specified universe by sending packets with the Stream_Terminated flag set to 1.
    /// Number of packets sent as per section 6.2.6 , Stream_Terminated: Bit 6 of ANSI E1.31-2018.
    /// 
    /// Arguments:
    /// universe: The universe that is being terminated.
    /// start_code: used for the first byte of the otherwise empty data payload to indicate the start_code of the data.
    /// 
    /// # Errors:
    /// IllegalUniverse: Returned if the universe is outwith the allowed range of sACN universes as defined in ANSI E1.31-2018 Section 6.2.7.
    /// 
    /// UniverseNotRegistered: Returned if the universe is not registered on this source.
    /// 
    /// Io: Returned if the termination packets fail to be sent on the socket.
    /// 
    fn terminate_stream(&mut self, universe: u16, start_code: u8) -> Result<()> {
        for _ in 0 .. E131_TERMINATE_STREAM_PACKET_COUNT {
            self.send_terminate_stream_pkt(universe, None, start_code)?;
        }

        self.deregister_universe(universe)?;
        Ok(())
    }

    /// Terminates the DMX source.
    /// 
    /// This includes terminating each registered universe with the start_code given.
    /// 
    /// Arguments:
    /// start_code: used for the first byte of the otherwise empty data payload to indicate the start_code of the data.
    /// 
    /// # Errors:
    /// Io: Returned if the termination packets fail to be sent on the underlying socket.
    /// 
    fn terminate(&mut self, start_code: u8) -> Result<()>{
        self.running = false;
        let universes = self.universes.clone(); // About to start manipulating self.universes as universes are removed so clone original list.
        for u in universes {
            self.terminate_stream(u, start_code)?;
        }
        Ok(())
    }

    /// Sends a universe discovery packet advertising the universes that this source is registered to send.
    /// 
    /// This packet may be broken down into multiple pages internally resulting in multiple UDP packets.
    /// 
    /// # Errors
    /// See (send_universe_discovery_detailed)[fn.send_universe_discovery_detailed.source].
    /// 
    fn send_universe_discovery(&self) -> Result<()>{
        // Given a u16 universe field and self.universes containing no duplicates it means that the maximum total number of universes (65536, ignoring sACN restrictions) 
        // divided by the number of universes per page (512) is 128 which therefore fits into the discovery universe 8 bit page field making this cast safe.
        let pages_req: u8 = ((self.universes.len() / DISCOVERY_UNI_PER_PAGE) + 1) as u8;

        for p in 0 .. pages_req {
            let start_index = (p as usize) * DISCOVERY_UNI_PER_PAGE;
            let end_index = min( ((p as usize) + 1) * DISCOVERY_UNI_PER_PAGE , self.universes.len());
            self.send_universe_discovery_detailed(p, pages_req - 1, &self.universes[start_index .. end_index])?;
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
    /// last_page: The last page that is expected as part of this universe discovery packet.
    /// 
    /// universes: The universes to include on the page.
    /// 
    /// # Errors
    /// Io: Returned if the discovery packet fails to be sent on the socket.
    /// 
    /// SacnParsePackError: Returned if the discovery packet cannot be packed to send.
    /// 
    fn send_universe_discovery_detailed(&self, page: u8, last_page: u8, universes: &[u16]) -> Result<()>{
        let packet = AcnRootLayerProtocol {
            pdu: E131RootLayer {
                cid: self.cid,
                data: E131RootLayerData::UniverseDiscoveryPacket(
                    UniverseDiscoveryPacketFramingLayer {
                        source_name: self.name.as_str().into(),
                        data: UniverseDiscoveryPacketUniverseDiscoveryLayer {
                            page: page,
                            last_page: last_page,
                            universes: universes.into(),
                        },
                    },
                ),
            },
        };

        let ip;
        if self.addr.is_ipv6(){
            ip = universe_to_ipv6_multicast_addr(E131_DISCOVERY_UNIVERSE)?;
        } else {
            ip = universe_to_ipv4_multicast_addr(E131_DISCOVERY_UNIVERSE)?;
        }

        self.socket.send_to(&packet.pack_alloc()?, &ip)?;

        Ok(())
    }

    /// Returns the ACN CID device identifier of the SacnSourceInternal.
    fn cid(&self) -> &Uuid {
        &self.cid
    }

    /// Sets the ACN CID device identifier.
    /// 
    /// # Arguments
    /// cid: The new CID identifier for this source. It is left to the user to ensure that this is always unique within the network the source is in.
    /// 
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
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Returns if SacnSourceInternal is in preview mode.
    fn preview_mode(&self) -> bool {
        self.preview_data
    }

    /// Sets the value of the Preview_Data flag in packets from this SacnSourceInternal.
    ///
    /// # Arguments
    /// preview_mode: If true then all data packets from this SacnSourceInternal will have the Preview_Data flag set to true indicating that the data is not
    ///     for live output. If false then the flag will be set to false.
    /// 
    fn set_preview_mode(&mut self, preview_mode: bool) {
        self.preview_data = preview_mode;
    }

    /// Sets the multicast time to live.
    /// 
    /// # Arguments
    /// multicast_ttl: The new time to live value for network packets sent using multicast.
    /// 
    /// # Errors
    /// Io: Returned if the multicast TTL fails to be set on the underlying socket.
    /// 
    fn set_multicast_ttl(&self, multicast_ttl: u32) -> Result<()> {
        Ok(self.socket.set_multicast_ttl_v4(multicast_ttl)?)
    }

    /// Sets the Time To Live for unicast packets sent by this source.
    /// 
    /// # Arguments
    /// ttl: The new time to live value for network packets sent by the source.
    /// 
    /// # Errors
    /// Io: Returned if the TTL fails to be set on the underlying socket.
    /// 
    fn set_ttl(&mut self, ttl: u32) -> Result<()> {
        Ok(self.socket.set_ttl(ttl)?)
    }

    /// Returns the multicast time to live of the socket.
    /// 
    fn multicast_ttl(&self) -> Result<u32> {
        Ok(self.socket.multicast_ttl_v4()?)
    }

    /// Sets if multicast loop is enabled.
    /// 
    /// # Arguments:
    /// multicast_loop: If true then multicast loop is enabled, if false it is not.
    /// 
    /// # Errors
    /// Io: Returned if the set_multicast_loop option fails to be set on the socket.
    /// 
    fn set_multicast_loop_v4(&self, multicast_loop: bool) -> Result<()> {
        Ok(self.socket.set_multicast_loop_v4(multicast_loop)?)
    }

    /// Returns true if multicast loop is enabled, false if not.
    fn multicast_loop(&self) -> Result<bool> {
        Ok(self.socket.multicast_loop_v4()?)
    }
}

/// Returns the locked internal SacnSourceInternal used within the SacnSource.
/// 
/// This centralises the locking of the source to a single point within the code allowing any changes to the mechanism to be made in one place.
/// 
/// This differs to (unlock_internal_mut) as it takes an immutable reference to internal.
/// 
/// # Arguments
/// internal: The SacnSourceInternal to unlock encapsulated within an Arc and Mutex.
/// 
/// # Errors
/// SourceCorrupt: Returned if the Mutex used to control access to the internal sender is poisoned by a thread encountering
/// a panic while accessing causing the source to be left in a potentially inconsistent state. 
/// 
fn unlock_internal(internal: &Arc<Mutex<SacnSourceInternal>>) -> Result<MutexGuard<SacnSourceInternal>> {
    match internal.lock() {
        Err(_) => {
            // The PoisonError returned doesn't contain further information and just allows access to the internal potentially inconsistent sender which 
            // shouldn't be exposed to the user (as its internal and would have no use).
            // Cannot directly return the PoisonError due to PoisonError using a different error system to other std modules which doesn't work with
            // error_chain.
            bail!(ErrorKind::SourceCorrupt("Mutex poisoned".to_string()));
        }
        Ok(lock) => {
            Ok(lock)
        }
    }
}

/// Returns the locked internal SacnSourceInternal used within the SacnSource.
/// 
/// This centralises the locking of the source to a single point within the code allowing any changes to the mechanism to be made in one place.
/// 
/// This differs to (unlock_internal) as it takes an mutable reference to internal.
/// 
/// # Arguments
/// internal: The SacnSourceInternal to unlock encapsulated within an Arc and Mutex.
/// 
/// # Errors
/// Returns an SourceCorrupt error if the Mutex used to control access to the internal sender is poisoned by a thread encountering
/// a panic while accessing causing the source to be left in a potentially inconsistent state. 
/// 
fn unlock_internal_mut(internal: &mut Arc<Mutex<SacnSourceInternal>>) -> Result<MutexGuard<SacnSourceInternal>> {
    match internal.lock() {
        Err(_) => {
            // The PoisonError returned doesn't contain further information and just allows access to the internal potentially inconsistent sender which 
            // shouldn't be exposed to the user (as its internal and would have no use).
            // Cannot directly return the PoisonError due to PoisonError using a different error system to other std modules which doesn't work with
            // error_chain.
            bail!(ErrorKind::SourceCorrupt("Mutex poisoned".to_string()));
        }
        Ok(lock) => {
            Ok(lock)
        }
    }
}

/// Called periodically by the source update thread.
/// 
/// Is responsible for sending the periodic universe discovery packets.
/// 
/// # Arguments:
/// src: A reference to the SacnSourceInternal for which to send the universe discovery packet with/from.
/// 
/// # Errors
/// Returns a SourceCorrupt error if the internal source mutex has been corrupted, see (unlock_internal)[unlock_internal].
/// 
/// Returns an error if a discovery packet cannot be sent, see (send_universe_discovery)[fn.send_universe_discovery.source].
/// 
fn perform_periodic_update(src: &mut Arc<Mutex<SacnSourceInternal>>) -> Result<()>{
    let mut unwrap_src = unlock_internal_mut(src)?;
    if unwrap_src.is_sending_discovery && Instant::now().duration_since(unwrap_src.last_discovery_advert_timestamp) > E131_UNIVERSE_DISCOVERY_INTERVAL {
        unwrap_src.send_universe_discovery()?;
        unwrap_src.last_discovery_advert_timestamp = Instant::now();
    }
    Ok(())
}