#![warn(missing_docs)]

// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was modified as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.


// Report note:
// One of the problems with the existing SACN sending is how it didn't treat the payload transparently because it 
// would add on the start code. As this is part of the DMX protocol and not the SACN protocol this was removed as it 
// violated the extends of SACN.
// Report: The first universe for the data to be synchronised across multiple universes is 
// used as the syncronisation universe by default. This is done as it means that the receiever should
// be listening for this universe. 

use error::errors::{*};
use packet::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::cmp;
use std::cmp::min;
use std::time;
use std::time::{Duration, Instant};
use std::thread::{JoinHandle};
use std::thread;
use std::sync::{Arc, Mutex};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

/// Socket2 used to create the underlying UDP socket that sACN is sent on.
use socket2::{Socket, Domain, Type, SockAddr};

/// UUID library used to handle the UUID's used in the CID fields.
use uuid::Uuid;

/// The name of the thread which runs periodically to perform various actions such as universe discovery adverts for the source.
pub const SND_UPDATE_THREAD_NAME: &'static str = "rust_sacn_snd_update_thread"; 

/// The default startcode used to send stream termination packets when the SacnSource is closed.
pub const DEFAULT_TERMINATE_START_CODE: u8 = 0; 

/// The poll rate of the update thread.
pub const DEFAULT_POLL_PERIOD: Duration = time::Duration::from_millis(1000);

/// A DMX over sACN sender.
///
/// SacnSourceInternal is used for sending sACN packets over ethernet.
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
/// let mut data: Vec<u8> = vec![0, 0, 0, 0, 255, 255, 128, 128]; // Some arbitary data, must have length <= 513 (including start-code).
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
    /// upon.
    preview_data: bool,

    /// The sequence numbers used for data packets, keeps a reference of the next sequence number to use for each universe.
    /// Sequence numbers are always in the range [0, 255].
    data_sequences: RefCell<HashMap<u16, u8>>,

    /// The sequence numbers used for sync packets, keeps a reference of the next sequence number to use for each universe.
    /// Sequence numbers are always in the range [0, 255].
    sync_sequences: RefCell<HashMap<u16, u8>>,

    /// A list of the universes registered to send by this source, used for universe discovery. 
    /// Always sorted with lowest universe first to allow quicker usage.
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
    pub fn new_v4(name: &str) -> Result<SacnSource> {
        let cid = Uuid::new_v4();
        SacnSource::with_cid_v4(name, cid)
    }

    /// Constructs a new SacnSource with the given name and specified CID binding to an IPv4 address.
    pub fn with_cid_v4(name: &str, cid: Uuid) -> Result<SacnSource> {
        let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT);
        SacnSource::with_cid_ip(name, cid, ip)
    }

    /// Constructs a new SacnSource with the given name, binding to an IPv6 address.
    /// By default this will only receieve IPv6 data but IPv4 can also be enabled by calling set_ipv6_only(false).
    pub fn new_v6(name: &str) -> Result<SacnSource> {
        let cid = Uuid::new_v4();
        SacnSource::with_cid_v6(name, cid)
    }

    /// Constructs a new SacnSource with the given name and specified CID binding to an IPv6 address.
    pub fn with_cid_v6(name: &str, cid: Uuid) -> Result<SacnSource> {
        let ip = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT);
        SacnSource::with_cid_ip(name, cid, ip)
    }

    /// Consturcts a new SacnSource with the given name and binding to the supplied ip.
    pub fn with_ip(name: &str, ip: SocketAddr) -> Result<SacnSource> {
        SacnSource::with_cid_ip(name, Uuid::new_v4(), ip)
    }

    /// Constructs a new SacnSource with the given name, cid and binding to the supplied ip.
    /// 
    /// # Errors
    /// Will return an Error if the internal SacnSourceInternal fails to be created with the reasoning error-chained.
    /// See source::SacnSourceInternal::with_cid_ip() for more details.
    pub fn with_cid_ip(name: &str, cid: Uuid, ip: SocketAddr) -> Result<SacnSource> {
        let trd_builder = thread::Builder::new().name(SND_UPDATE_THREAD_NAME.into());

        let internal_src = Arc::new(Mutex::new(SacnSourceInternal::with_cid_ip(name, cid, ip).chain_err(|| "Failed to create SacnSourceInternal")?));

        let mut trd_src = internal_src.clone();

        let src = SacnSource { 
            internal: internal_src,
            update_thread: Some(trd_builder.spawn(move || {
                while trd_src.lock().unwrap().running {
                    thread::sleep(DEFAULT_POLL_PERIOD);
                    match perform_periodic_update(&mut trd_src) {
                        Err(_e) => {// TODO HANDLE
                        },
                        Ok(_) => {}
                    }
                }
            }).unwrap()),
        };

        Ok(src)
    }

    /// Allow sending on ipv6 
    pub fn set_ipv6_only(&mut self, val: bool) -> Result<()>{
        Ok(self.internal.lock().unwrap().socket.set_only_v6(val)?)
    }
    
    /// Sets the Time To Live for packets sent by this source.
    pub fn set_ttl(&mut self, ttl: u32) -> Result<()>{
        self.internal.lock().unwrap().set_ttl(ttl)
    }

    /// Sets the is_sending_discovery flag to the given value.
    /// 
    /// If set to true then the source will send universe discovery packets periodically.
    /// If set to false then it won't.
    pub fn set_is_sending_discovery(&mut self, val: bool) {
        self.internal.lock().unwrap().set_is_sending_discovery(val);
    }

    /// Registers the given universes on this source in addition to already registered universes.
    /// 
    /// This allows sending data to those universes aswell as adding them to the list of universes
    /// that appear in universe discovery packets that are sent (depending on set_is_sending_discovery flag) periodically. 
    /// 
    /// # Errors
    /// See (register_universes)[fn.register_universes.SacnSourceInternal]
    pub fn register_universes(&mut self, universes: &[u16]) -> Result<()> {
        self.internal.lock().unwrap().register_universes(universes)
    }

    /// Registers a single universe on this source in addition to already registered universes.
    /// 
    /// This allows sending data to those universes aswell as adding them to the list of universes
    /// that appear in universe discovery packets that are sent (depending on set_is_sending_discovery flag) periodically. 
    /// 
    /// # Errors
    /// See (register_universes)[fn.register_universe.SacnSourceInternal]
    pub fn register_universe(&mut self, universe: u16) -> Result<()> {
        self.internal.lock().unwrap().register_universe(universe)
    }

    /// See [send](fn.send.SacnSourceInternal) for more details
    pub fn send(&self, universes: &[u16], data: &[u8], priority: Option<u8>, dst_ip: Option<SocketAddr>, syncronisation_addr: Option<u16>) -> Result<()> {
        self.internal.lock().unwrap().send(universes, data, priority, dst_ip, syncronisation_addr)
    }

    /// Sends a synchronisation packet.
    /// 
    /// See [send_sync_packet](fn.send_sync_packet.SacnSourceInternal) for more details
    pub fn send_sync_packet(&self, universe: u16, dst_ip: Option<SocketAddr>) -> Result<()> {
        self.internal.lock().unwrap().send_sync_packet(universe, dst_ip)
    }

    /// Terminates sending on the given universe.
    /// 
    /// See [terminate_stream](fn.terminate_stream.SacnSourceInternal) for more details
    pub fn terminate_stream(&mut self, universe: u16, start_code: u8) -> Result<()> {
        self.internal.lock().unwrap().terminate_stream(universe, start_code)
    }

    /// Send a universe discovery packet.
    /// 
    /// See [send_universe_discovery](fn.send_universe_discovery.SacnSourceInternal) for more details
    pub fn send_universe_discovery(&self) -> Result<()> {
        self.internal.lock().unwrap().send_universe_discovery()
    }

     /// Returns the ACN CID device identifier of the SacnSourceInternal.
    pub fn cid(&self) -> Uuid {
        *self.internal.lock().unwrap().cid()
    }

    /// Sets the ACN CID device identifier.
    pub fn set_cid(&mut self, cid: Uuid) {
        self.internal.lock().unwrap().set_cid(cid);
    }

    /// Returns the ACN source name.
    pub fn name(&self) -> String {
        self.internal.lock().unwrap().name().into()
    }

    /// Sets ACN source name.
    pub fn set_name(&mut self, name: &str) {
        self.internal.lock().unwrap().set_name(name);
    }

    /// Returns if SacnSourceInternal is in preview mode.
    pub fn preview_mode(&self) -> bool {
        self.internal.lock().unwrap().preview_mode()
    }

    /// Sets the SacnSourceInternal to preview mode.
    ///
    /// All packets will be sent with Preview_Data flag set to 1.
    pub fn set_preview_mode(&mut self, preview_mode: bool) {
        self.internal.lock().unwrap().set_preview_mode(preview_mode);
    }

    /// Sets the multicast time to live.
    pub fn set_multicast_ttl(&self, multicast_ttl: u32) -> Result<()> {
        self.internal.lock().unwrap().set_multicast_ttl(multicast_ttl)
    }

    /// Returns the multicast time to live of the socket.
    pub fn multicast_ttl(&self) -> Result<u32> {
        self.internal.lock().unwrap().multicast_ttl()
    }

    /// Sets if multicast loop is enabled.
    pub fn set_multicast_loop(&self, multicast_loop: bool) -> Result<()> {
        self.internal.lock().unwrap().set_multicast_loop(multicast_loop)
    }

    /// Returns if multicast loop of the socket is enabled.
    pub fn multicast_loop(&self) -> Result<bool> {
        self.internal.lock().unwrap().multicast_loop()
    }
}

/// By implementing the Drop trait for SacnSource it means that the user doesn't have to explicitly cleanup the source
/// and if it goes out of reference it will clean itself up and send the required termination packets etc.
impl Drop for SacnSource {
    fn drop(&mut self){
        self.internal.lock().unwrap().running = false;
        if let Some(thread) = self.update_thread.take() {
            {
                match self.internal.lock().unwrap().terminate(DEFAULT_TERMINATE_START_CODE) {
                    Err(_e) => {},
                    Ok(_) => {}
                }
            }
            thread.join().unwrap();
        }
    }
}

impl SacnSourceInternal {
    /// Constructs a new SacnSourceInternal with DMX START code set to 0 with specified CID and binding IP address.
    /// 
    /// By default for an IPv6 address this will only receieve IPv6 data but IPv4 can also be enabled by calling set_ipv6_only(false).
    /// By default the TTL for ipv4 packets is 1 to keep them within the local network.
    /// 
    /// Arguments:
    /// name: The human readable name for this sacn source.
    /// cid:  The UUID for this source.
    /// ip:   The address that this source should bind to.
    /// 
    /// # Errors
    /// Will return an error if the UDP socket builder cannot be created. 
    /// See (UdpBuilder::new_v4)[fn.new_v4.UdpBuilder] and (UdpBuilder::new_v6)[fn.new_v6.UdpBuilder] for details.
    /// 
    /// Will return an error if the IP cannot be bound to the underlying socket. See (Socket::bind)[fn.bind.Socket2].
    /// 
    /// Will return UnsupportedIpVersion if the SockAddr is not IPv4 or IPv6.
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
        socket.set_reuse_port(true);
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
    /// Arguments:
    /// val: The new value of the is_sending_discovery flag.
    pub fn set_is_sending_discovery(&mut self, val: bool) {
        self.is_sending_discovery = val;
    }

    /// Registers the given array of universes with this source.
    /// 
    /// Any universes already registered won't be re-registered and will have no effect.
    /// 
    /// See register_universe(fn.register_universe.source) for more details.
    pub fn register_universes(&mut self, universes: &[u16]) -> Result<()> {
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
    /// Returns an IllegalUniverse error if the universe is outwith the allowed range, see (is_universe_in_range)[fn.is_universe_in_range.packet].
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
    /// Returns an IllegalUniverse error if the universe is outwith the allowed range, see (is_universe_in_range)[fn.is_universe_in_range.packet].
    /// 
    /// Returns a UniverseNotFound error if the given universe was never registered originally.
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
    /// Returns an IllegalUniverse error if the universe is outwith the allowed range, see (is_universe_in_range)[fn.is_universe_in_range.packet]
    /// 
    /// Returns an UniverseNotRegistered error if the universe is not registered on the given SacnSourceInternal.
    fn universe_allowed(&self, u: &u16) -> Result<()>{
        is_universe_in_range(*u)?;

        if !self.universes.contains(u) {
            bail!(ErrorKind::UniverseNotRegistered(format!("Attempted to send on unregistered universe : {}", u).to_string()));
        }

        Ok(())
    }

    /// Sends the given data to the given universes with the given priority, syncronisation address (universe) and destination ip.
    /// 
    /// Arguments
    /// universe:     The sACN universes that the data should be set on, the data will be split over these universes with each UNIVERSE_CHANNEL_CAPACITY
    ///                 sized chunk sent to the next universe.
    /// data:         The data that should be sent, must have a length greater than 0.
    /// priority:     The E131 priority that the data should be sent with, must be less than E131_MAX_PRIORITY (const.E131_MAX_PRIORITY.packet), 
    ///                 if a value of None is provided then the default of E131_DEFAULT_PRIORITY (const.E131_DEFAULT_PRIORITY.packet) is used.
    /// dst_ip:       The destination IP, can be Ipv4 or Ipv6, None if should be sent using ip multicast.
    /// sync_address: The address to use for synchronisation, must be a valid universe, None indicates no synchronisation. If synchronisation is required a
    ///                 reasonable default address to use is the first universe that this data is being sent to.
    /// 
    /// As per ANSI E1.31-2018 Section 6.6.1 this method shouldn't be called at a higher refresher rate than specified in ANSI E1.11 [DMX] unless 
    ///     configured by the user to do so in an environment which doesn't contain any E1.31 to DMX512-A converters.
    /// 
    /// Note as per ANSI-E1.31-2018 Appendix B.1 it is recommended to have a small delay before sending the followup sync packet.
    /// 
    /// # Errors
    /// Will return a SenderAlreadyTerminated error if this method is called on an SacnReceiverInternal that has already terminated.
    /// 
    /// Will return an InvalidInput error if the data array has length 0 or if an insufficient number of universes for the given data are provided. The sufficient number
    ///     of universes = ceiling(data.len() / UNIVERSE_CHANNEL_CAPACITY). 
    /// 
    /// Will return an error if any of the universes including the synchronisation universe are outwith the allowed range for data packets, 
    ///     see (universe_allowed)[fn.universe_allowed.source] or if a universe is not already registered with this source.
    /// 
    /// Will return an error if the data fails to send, see (send_universe)[fn.send_universe.source]
    /// 
    fn send(&self, universes: &[u16], data: &[u8], priority: Option<u8>, dst_ip: Option<SocketAddr>, syncronisation_addr: Option<u16>) -> Result<()> {
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
        if syncronisation_addr.is_some() {
            self.universe_allowed(&syncronisation_addr.unwrap()).chain_err(|| "Synchronisation universe not allowed")?;
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
                priority.unwrap_or(E131_DEFAULT_PRIORITY), &dst_ip, syncronisation_addr.unwrap_or(NO_SYNC_UNIVERSE))
                .chain_err(|| "Failed to send universe of data")?;
        }

        Ok(())
    }

    /// Sends the given data to the given universe with the given priority, syncronisation address (universe) and destination ip.
    /// 
    /// Arguments
    /// universe:     The sACN universe that the data should be set on.
    /// data:         The data that should be sent, must be less than or equal in length to UNIVERSE_CHANNEL_CAPACITY(const.UNIVERSE_CHANNEL_CAPACITY.packet).
    /// priority:     The E131 priority that the data should be sent with, must be less than E131_MAX_PRIORITY (const.E131_MAX_PRIORITY.packet), default E131_DEFAULT_PRIORITY.
    /// dst_ip:       The destination IP, can be Ipv4 or Ipv6, None if should be sent using ip multicast.
    /// sync_address: The address to use for synchronisation, must be a valid universe, 0 indicates no synchronisation.
    /// 
    /// # Errors
    /// Will return an InvalidInput error if the priority is greater than the allowed maximum priority of E131_MAX_PRIORITY.
    /// 
    /// Will return an ExceedUniverseCapacity error if the data has a length greater than the maximum allowed within a universe (packet::UNIVERSE_CHANNEL_CAPACITY).
    /// 
    /// Will return an error if sending using multicast and the universe cannot be converted to a multicast address, 
    /// see universe_to_ipv4_multicast_addr(fn.universe_to_ipv4_multicast_addr.packet) and universe_to_ipv6_multicast_addr(fn.universe_to_ipv6_multicast_addr.packet).
    /// 
    /// Will return an error if the data fails to be sent on the socket. See send_to(fn.send_to.Socket).
    /// 
    fn send_universe(&self, universe: u16, data: &[u8], priority: u8, dst_ip: &Option<SocketAddr>, sync_address: u16) -> Result<()> {
        if priority > E131_MAX_PRIORITY {
            bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Priority must be within allowed range of [0-E131_MAX_PRIORITY], priority provided: {}", priority)));
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
    /// then send a synchronisation packet with the address of the syhcnronisation universe chosen to trigger the packets.
    /// 
    /// Arguments
    /// universe: The universe of this synchronisation packet.
    /// dst_ip:   The destination IP address for this packet or None if it should be sent using multicast.
    /// 
    /// # Errors
    /// Will return an error if sending using multicast and the universe cannot be converted to a multicast address, 
    /// see universe_to_ipv4_multicast_addr(fn.universe_to_ipv4_multicast_addr.packet) and 
    /// universe_to_ipv6_multicast_addr(fn.universe_to_ipv6_multicast_addr.packet).
    /// 
    /// Will return an error if the universe is outwith the allowed range for data packets, see (universe_allowed)[fn.universe_allowed.source] 
    ///     or if a universe is not already registered with this source.
    /// 
    /// Will return an error if the data fails to be sent on the socket. See send_to(fn.send_to.Socket).
    /// 
    fn send_sync_packet(&self, universe: u16, dst_ip: Option<SocketAddr>) -> Result<()> {
        self.universe_allowed(&universe).chain_err(|| format!("Universe {} not allowed", universe))?;

        let ip;

        if dst_ip.is_none() {
            if self.addr.is_ipv6(){
                ip = universe_to_ipv6_multicast_addr(universe).chain_err(|| "Failed to convert universe to ipv6 multicast addr")?;
            } else {
                ip = universe_to_ipv4_multicast_addr(universe).chain_err(|| "Failed to convert universe to ipv4 multicast addr")?;
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
        self.socket.send_to(&packet.pack_alloc().unwrap(), &ip).chain_err(|| "Failed to send sync packet on socket")?;

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
    ///     section 6.2.6 , Stream_Teminated: Bit 6, of ANSI E1.31-2018.
    /// 
    /// Arguments
    /// universe: The universe of this synchronisation packet.
    /// dst_ip:   The destination IP address for this packet or None if it should be sent using multicast.
    /// 
    /// # Errors
    /// Will return an error if sending using multicast and the universe cannot be converted to a multicast address, 
    /// see universe_to_ipv4_multicast_addr(fn.universe_to_ipv4_multicast_addr.packet) and 
    /// universe_to_ipv6_multicast_addr(fn.universe_to_ipv6_multicast_addr.packet). This is not expected if the universe
    /// is a valid universe.
    /// 
    /// Will return an error if the universe is outwith the allowed range for data packets, see (universe_allowed)[fn.universe_allowed.source] 
    ///     or if a universe is not already registered with this source.
    /// 
    /// Will return an error if the data fails to be sent on the socket. See send_to(fn.send_to.Socket).
    /// 
    fn send_terminate_stream_pkt(&self, universe: u16, dst_ip: Option<SocketAddr>, start_code: u8) -> Result<()> {
        self.universe_allowed(&universe).chain_err(|| format!("Universe {} not allowed", universe))?;

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
    /// Terminates a stream to the specified universe by sending three packets with the Stream_Terminated flag set to 1.
    /// Three packets sent as per section 6.2.6 , Stream_Teminated: Bit 6 of ANSI E1.31-2018.
    /// 
    /// Arguments:
    /// universe: The universe that is being terminated.
    /// start_code: used for the first byte of the otherwise empty data payload to indicate the start_code of the data.
    /// 
    /// # Errors:
    /// See (send_terminate_stream_pkt)[fn.send_terminate_stream_pkt.source].
    /// 
    fn terminate_stream(&mut self, universe: u16, start_code: u8) -> Result<()> {
        for _ in 0..3 {
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
    /// See (send_terminate_stream_pkt)[fn.send_terminate_stream_pkt.source].
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
        let pages_req: u8 = ((self.universes.len() / DISCOVERY_UNI_PER_PAGE) + 1) as u8;

        for p in 0 .. pages_req {
            let start_index = (p as usize) * DISCOVERY_UNI_PER_PAGE;
            let end_index = min( ((p as usize) + 1) * DISCOVERY_UNI_PER_PAGE , self.universes.len());
            self.send_universe_discovery_detailed(p, pages_req - 1, &self.universes[start_index .. end_index])
                .chain_err(|| "Failed to send universe discovery packet")?;
        }
        Ok(())
    }

    /// Sends a page of a universe discovery packet.
    /// 
    /// There may be 1 or more pages for each full universe discovery packet with each page sent seperately.
    /// 
    /// Arguments
    /// page: The page number of this universe discovery page.
    /// last_page: The last page that is expected as part of this universe discovery packet.
    /// universes: The universes to include on the page.
    /// 
    /// # Errors
    /// Will return an error if sending using multicast and the universe cannot be converted to a multicast address, 
    ///     see universe_to_ipv4_multicast_addr(fn.universe_to_ipv4_multicast_addr.packet) and 
    ///     universe_to_ipv6_multicast_addr(fn.universe_to_ipv6_multicast_addr.packet). This is not expected as the discovery
    ///     universe is used which should always be valid.
    /// 
    /// Will return an error if the data fails to be sent on the socket. See send_to(fn.send_to.Socket).
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

        self.socket.send_to(&packet.pack_alloc().unwrap(), &ip).chain_err(|| "Failed to send discovery on socket")?;

        Ok(())
    }

    /// Returns the ACN CID device identifier of the SacnSourceInternal.
    fn cid(&self) -> &Uuid {
        &self.cid
    }

    /// Sets the ACN CID device identifier.
    fn set_cid(&mut self, cid: Uuid) {
        self.cid = cid;
    }

    /// Returns the ACN source name.
    fn name(&self) -> &str {
        &self.name
    }

    /// Sets ACN source name.
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Returns if SacnSourceInternal is in preview mode.
    fn preview_mode(&self) -> bool {
        self.preview_data
    }

    /// Sets the SacnSourceInternal to preview mode.
    ///
    /// All packets will be sent with Preview_Data flag set to 1.
    fn set_preview_mode(&mut self, preview_mode: bool) {
        self.preview_data = preview_mode;
    }

    /// Sets the multicast time to live.
    fn set_multicast_ttl(&self, multicast_ttl: u32) -> Result<()> {
        Ok(self.socket.set_multicast_ttl_v4(multicast_ttl)?)
    }

    /// Sets the Time To Live for packets sent by this source.
    pub fn set_ttl(&mut self, ttl: u32) -> Result<()> {
        Ok(self.socket.set_ttl(ttl)?)
    }

    /// Returns the multicast time to live of the socket.
    fn multicast_ttl(&self) -> Result<u32> {
        Ok(self.socket.multicast_ttl_v4()?)
    }

    /// Sets if multicast loop is enabled.
    fn set_multicast_loop(&self, multicast_loop: bool) -> Result<()> {
        Ok(self.socket.set_multicast_loop_v4(multicast_loop)?)
    }

    /// Returns if multicast loop of the socket is enabled.
    fn multicast_loop(&self) -> Result<bool> {
        Ok(self.socket.multicast_loop_v4()?)
    }
}

/// Called periodically by the source update thread.
/// 
/// Is responsible for sending the periodic universe discovery packets.
/// 
/// Arguments:
/// src: A reference to the SacnSourceInternal for which to send the universe discovery packet with/from.
/// 
/// # Errors
/// An error might be returned by (send_universe_discovery)[fn.send_universe_discovery.source].
/// 
fn perform_periodic_update(src: &mut Arc<Mutex<SacnSourceInternal>>) -> Result<()>{
    let mut unwrap_src = src.lock().unwrap();
    if unwrap_src.is_sending_discovery && Instant::now().duration_since(unwrap_src.last_discovery_advert_timestamp) > E131_UNIVERSE_DISCOVERY_INTERVAL {
        unwrap_src.send_universe_discovery().chain_err(|| "Failed to send universe discovery packet")?;
        unwrap_src.last_discovery_advert_timestamp = Instant::now();
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter;
    use std::net::Ipv4Addr;

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_dmx_source() {
        let cid = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let universe = 1;
        let source_name = "SourceName";
        let priority = 150;
        let sequence = 0;
        let preview_data = false;
        let mut dmx_data: Vec<u8> = Vec::new();
        dmx_data.push(0); // Start code
        dmx_data.extend(iter::repeat(100).take(255));

        // Root Layer
        let mut packet = Vec::new();
        // Preamble Size
        packet.extend("\x00\x10".bytes());
        // Post-amble Size
        packet.extend("\x00\x00".bytes());
        // ACN Packet Identifier
        packet.extend("\x41\x53\x43\x2d\x45\x31\x2e\x31\x37\x00\x00\x00".bytes());
        // Flags and Length (22 + 343)
        packet.push(0b01110001);
        packet.push(0b01101101);
        // Vector
        packet.extend("\x00\x00\x00\x04".bytes());
        // CID
        packet.extend(&cid);

        // E1.31 Framing Layer
        // Flags and Length (77 + 266)
        packet.push(0b01110001);
        packet.push(0b01010111);
        // Vector
        packet.extend("\x00\x00\x00\x02".bytes());
        // Source Name
        let source_name = source_name.to_string() +
                          "\0\0\0\0\0\0\0\0\0\0" +
                          "\0\0\0\0\0\0\0\0\0\0" +
                          "\0\0\0\0\0\0\0\0\0\0" +
                          "\0\0\0\0\0\0\0\0\0\0" +
                          "\0\0\0\0\0\0\0\0\0\0" +
                          "\0\0\0\0";
        assert_eq!(source_name.len(), 64);
        packet.extend(source_name.bytes());
        // Priority
        packet.push(priority);
        // Reserved
        packet.extend("\x00\x00".bytes());
        // Sequence Number
        packet.push(sequence);
        // Options
        packet.push(0);
        // Universe
        packet.push(0);
        packet.push(1);

        // DMP Layer
        // Flags and Length (266)
        packet.push(0b01110001);
        packet.push(0b00001010);
        // Vector
        packet.push(0x02);
        // Address Type & Data Type
        packet.push(0xa1);
        // First Property Address
        packet.extend("\x00\x00".bytes());
        // Address Increment
        packet.extend("\x00\x01".bytes());
        // Property value count
        packet.push(0b1);
        packet.push(0b00000000);
        // Property values
        packet.extend(&dmx_data);

        let ip: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT + 1);
        let mut source = SacnSourceInternal::with_cid_ip(&source_name, Uuid::from_bytes(&cid).unwrap(), ip).unwrap();

        source.set_preview_mode(preview_data);
        source.set_multicast_loop(true).unwrap();

        let recv_socket = Socket::new(Domain::ipv4(), Type::dgram(), None).unwrap();
        
        let addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT);

        recv_socket.bind(&addr.into()).unwrap();

        recv_socket.join_multicast_v4(&Ipv4Addr::new(239, 255, 0, 1), &Ipv4Addr::new(0, 0, 0, 0))
                   .unwrap();

        let mut recv_buf = [0; 1024];

        source.register_universes(&[universe]).unwrap();

        source.send(&[universe], &dmx_data, Some(priority), None, None).unwrap();
        let (amt, _) = recv_socket.recv_from(&mut recv_buf).unwrap();

        assert_eq!(&packet[..], &recv_buf[0..amt]);
    }

    #[test]
    fn test_terminate_stream() {
        let cid = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

        let ip: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT + 1);
        let mut source = SacnSourceInternal::with_cid_ip(&"Source", Uuid::from_bytes(&cid).unwrap(), ip).unwrap();

        source.set_multicast_loop(true).unwrap();

        let recv_socket = Socket::new(Domain::ipv4(), Type::dgram(), None).unwrap();
        
        let addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT);

        recv_socket.bind(&addr.into()).unwrap();

        recv_socket
            .join_multicast_v4(&Ipv4Addr::new(239, 255, 0, 1), &Ipv4Addr::new(0, 0, 0, 0))
            .unwrap();

        let mut recv_buf = [0; 1024];

        let start_code: u8 = 0;

        source.register_universes(&[1]).unwrap();

        source.terminate_stream(1, start_code).unwrap();
        for _ in 0..2 {
            recv_socket.recv_from(&mut recv_buf).unwrap();
            assert_eq!(
                match AcnRootLayerProtocol::parse(&recv_buf).unwrap().pdu.data {
                    E131RootLayerData::DataPacket(data) => data.stream_terminated,
                    _ => panic!(),
                },
                true
            )
        }
    }
}
