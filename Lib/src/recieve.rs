// Code taken and based on tutorial 
// https://bluejekyll.github.io/blog/rust/2018/03/18/multicasting-in-rust.html September 2019
// https://blog.abby.md/2019/05/16/multicasting-in-rust/ September 2019


// Objective ideas:
// - Ipv4 or Ipv6 Support
// - Simultaneous Ipv4 or Ipv6 support (Ipv6 preferred as newer and going to become more standard?)
// - Support for Windows and Unix

// use net2::{UdpBuilder, UdpSocketExt};

use socket2::{Domain, Protocol, SockAddr, Socket, Type};

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use packet::{AcnRootLayerProtocol, E131RootLayer, E131RootLayerData, E131RootLayerData::DataPacket, 
    E131RootLayerData::SynchronizationPacket, E131RootLayerData::UniverseDiscoveryPacket, UniverseDiscoveryPacketFramingLayer, 
    SynchronizationPacketFramingLayer, DataPacketFramingLayer, UniverseDiscoveryPacketUniverseDiscoveryLayer, ACN_SDT_MULTICAST_PORT,
    universe_to_ipv4_multicast_addr, universe_to_ipv6_multicast_addr, HIGHEST_ALLOWED_UNIVERSE, DISCOVERY_UNIVERSE, UNIVERSE_DISCOVERY_SOURCE_TIMEOUT};

use std::io;
use std::io::{Error, ErrorKind};

use std::cmp::{max, Ordering};
use std::time;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

use std::borrow::Cow;

use std::fmt;

/// The default size of the buffer used to recieve E1.31 packets.
/// 1143 bytes is biggest packet required as per Section 8 of ANSI E1.31-2018, aligned to 64 bit that is 1144 bytes.
pub const RCV_BUF_DEFAULT_SIZE: usize = 1144;

// The synchronisation address used to indicate that there is no synchronisation required for the data packet.
// As defined in ANSI E1.31-2018 Section 6.2.4.1
pub const NO_SYNC_ADDR: u16 = 0;

// DMX payload size in bytes (512 bytes of data + 1 byte start code).
pub const DMX_PAYLOAD_SIZE: usize = 513;

// By default shouldn't check for packets send over the network using unicast.
pub const CHECK_UNICAST_DEFAULT: bool = false;

// By default should check for packets sent over the network using multicast.
pub const CHECK_MUTLICAST_DEFAULT: bool = true;

// By default shouldn't check for packets sent over the network using broadcast.
pub const CHECK_BROADCAST_DEFAULT: bool = false;

// The name of the thread which runs periodically to perform actions on the receiver such as update discovered universes.
pub const RCV_UPDATE_THREAD_NAME: &'static str = "rust_sacn_rcv_update_thread"; 

pub const DEFAULT_RECV_POLL_PERIOD: Duration = time::Duration::from_millis(1000);

// The default value of the process_preview_data flag.
const PROCESS_PREVIEW_DATA_DEFAULT: bool = false;

// The default value for the reading timeout for a DmxReceiver.
pub const DEFAULT_RECV_TIMEOUT: Option<Duration> = Some(time::Duration::from_millis(500));

#[derive(Debug)]
pub struct DMXData{
    pub universe: u16,
    pub values: Vec<u8>,
    pub sync_uni: u16 // The universe the data is waiting for a synchronisation packet from, 0 indicates it isn't waiting for a universe. 
}

impl Clone for DMXData {
    fn clone(&self) -> DMXData {
        let new_vals = self.values.to_vec(); // https://stackoverflow.com/questions/21369876/what-is-the-idiomatic-rust-way-to-copy-clone-a-vector-in-a-parameterized-functio (26/12/2019)
        
        DMXData {
            universe: self.universe,
            values: new_vals,
            sync_uni: self.sync_uni
        }
    }
}

impl Ord for DMXData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.universe.cmp(&other.universe).then(self.sync_uni.cmp(&other.sync_uni)).then(self.values.cmp(&other.values))
    }
}

impl PartialOrd for DMXData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DMXData {
    fn eq(&self, other: &Self) -> bool {
        self.universe == other.universe &&
        self.sync_uni == other.sync_uni &&
        self.values == other.values
    }
}

impl Eq for DMXData {}

/// Used for receiving dmx or other data on a particular universe using multicast.
#[derive(Debug)]
struct DmxReciever{
    socket: Socket,
    addr: SocketAddr
}

#[derive(Clone, Debug)]
pub struct DiscoveredSacnSource {
    pub name: String, // The name of the source, no protocol guarantee this will be unique but if it isn't then universe discovery may not work correctly.
    last_page: u8, // The last page that will be sent by this source.
    pages: Vec<UniversePage>,
    last_updated: Instant
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Debug)]
pub struct UniversePage {
    page: u8, // The most recent page receieved by this source when receiving a universe discovery packet. 
    universes: Vec<u16> // The universes that the source is transmitting.
}

impl DiscoveredSacnSource {
    pub fn has_all_pages(&mut self) -> bool {
        // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html (31/12/2019)
        self.pages.sort_by(|a, b| a.page.cmp(&b.page));
        for i in 0 .. (self.last_page + 1) {
            if self.pages[i as usize].page != i {
                return false;
            }
        }

        return true;
    }

    pub fn get_all_universes(&self) -> Vec<u16> {
        let mut uni: Vec<u16> = Vec::new();
        for p in &self.pages {
            uni.extend_from_slice(&p.universes);
        }
        uni
    }

    pub fn terminate_universe(&mut self, universe: u16) {
        for p in &mut self.pages {
            p.universes.retain(|x| *x != universe)
        }
    }
}

/// Allows receiving dmx or other (different startcode) data using sacn.
pub struct SacnReceiverInternal {
    receiver: DmxReciever,
    waiting_data: Vec<DMXData>, // Data that hasn't been passed up yet as it is waiting e.g. due to universe synchronisation.
    universes: Vec<u16>, // Universes that this receiver is currently listening for
    discovered_sources: Vec<DiscoveredSacnSource>, // Sacn sources that have been discovered by this receiver through universe discovery packets.
    merge_func: fn(&DMXData, &DMXData) -> Result<DMXData, Error>,
    partially_discovered_sources: Vec<DiscoveredSacnSource>, // Sacn sources that have been partially discovered by only some of their universes being discovered so far with more pages to go.
    running: bool,
    process_preview_data: bool
}

// https://doc.rust-lang.org/std/fmt/trait.Debug.html (04/02/2020)
impl fmt::Debug for SacnReceiverInternal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.receiver)?;
        write!(f, "{:?}", self.waiting_data)?;
        write!(f, "{:?}", self.universes)?;
        write!(f, "{:?}", self.discovered_sources)?;
        write!(f, "{:?}", self.partially_discovered_sources)?;
        write!(f, "{:?}", self.running)
    }
}

#[derive(Debug)]
pub struct SacnReceiver {
    internal: Arc<Mutex<SacnReceiverInternal>>
}

impl SacnReceiver {
     /// Constructs a new SacnReceiver binding to an IPv4 address.
     pub fn new_v4() -> Result<SacnReceiver, Error> {
        let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT);
        SacnReceiver::with_ip(ip)
    }

    /// Constructs a new SacnReceiver binding to an IPv6 address.
    /// By default this will only receieve IPv6 data but IPv4 can also be enabled by calling set_ipv6_only(false).
    pub fn new_v6() -> Result<SacnReceiver, Error> {
        let ip = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT);
        SacnReceiver::with_ip(ip)        
    }

    /// By default for an IPv6 address this will only receieve IPv6 data but IPv4 can also be enabled by calling set_ipv6_only(false).
    pub fn with_ip(ip: SocketAddr) -> Result<SacnReceiver, Error> {
        let internal = Arc::new(Mutex::new(SacnReceiverInternal::with_ip(ip)?));
        Ok(SacnReceiver {
            internal: internal.clone()
        })
    }

    // TODO
    pub fn set_merge_fn(&mut self, func: fn(&DMXData, &DMXData) -> Result<DMXData, Error>) -> Result<(), Error> {
        self.internal.lock().unwrap().set_merge_fn(func)
    }

    /// Allow sending on ipv6 
    pub fn set_ipv6_only(&mut self, val: bool) -> Result<(), Error>{
        self.internal.lock().unwrap().set_ipv6_only(val)
    }

    pub fn clear_waiting_data(&mut self){
        self.internal.lock().unwrap().clear_waiting_data()
    }

    /// Starts listening to the multicast addresses which corresponds to the given universe to allow recieving packets for that universe.
    pub fn listen_universes(&mut self, universes: &[u16]) -> Result<(), Error>{
        self.internal.lock().unwrap().listen_universes(universes)
    }

    /// Returns a copy of the Vec of discovered sources by this receiver, note that this isn't kept up to date so these source
    /// may have timed out / changed etc. since this method returned.
    pub fn get_discovered_sources(&self) -> Vec<DiscoveredSacnSource>{
        self.internal.lock().unwrap().get_discovered_sources()
    }

    // Attempt to recieve data from any of the registered universes.
    // This is the main method for receiving data.
    // Any data returned will be ready to act on immediately i.e. waiting e.g. for universe synchronisation
    // is already handled.
    pub fn recv(&mut self, timeout: Option<Duration>) -> Result<Vec<DMXData>, Error> {
        self.internal.lock().unwrap().recv(timeout)
    }
}

impl SacnReceiverInternal {
    /// By default for an IPv6 address this will only receieve IPv6 data but IPv4 can also be enabled by calling set_ipv6_only(false).
    pub fn with_ip(ip: SocketAddr) -> Result<SacnReceiverInternal, Error> {
        let mut sri = SacnReceiverInternal {
                receiver: DmxReciever::new(ip)?,
                waiting_data: Vec::new(),
                universes: Vec::new(),
                discovered_sources: Vec::new(),
                merge_func: htp_dmx_merge,
                partially_discovered_sources: Vec::new(),
                running: true,
                process_preview_data: PROCESS_PREVIEW_DATA_DEFAULT
        };

        sri.listen_universes(&[DISCOVERY_UNIVERSE])?;

        Ok(sri)
    }

    
    pub fn set_merge_fn(&mut self, func: fn(&DMXData, &DMXData) -> Result<DMXData, Error>) -> Result<(), Error> {
        self.merge_func = func;
        Ok(())
    }

    /// Allow sending on ipv6 
    pub fn set_ipv6_only(&mut self, val: bool) -> Result<(), Error>{
        self.receiver.set_only_v6(val)
    }

    pub fn clear_waiting_data(&mut self){
        self.waiting_data.clear();
    }

    /// Starts listening to the multicast addresses which corresponds to the given universe to allow recieving packets for that universe.
    pub fn listen_universes(&mut self, universes: &[u16]) -> Result<(), Error>{
        for u in universes {
            if (*u != DISCOVERY_UNIVERSE) && (*u == 0 || *u > HIGHEST_ALLOWED_UNIVERSE) {
                return Err(Error::new(ErrorKind::InvalidInput, format!("Attempted to listen on a universe outwith the allowed range: {}", u)));
            }
        }

        for u in universes {
            match self.universes.binary_search(u) { 
                Err(i) => { // Value not found, i is the position it should be inserted
                    self.universes.insert(i, *u);
                    self.receiver.listen_multicast_universe(*u)?
                }
                Ok(_) => {
                    // If value found then don't insert to avoid duplicates.
                }
            }
        }

        Ok(())
    }

    fn set_process_preview_data(&mut self, val: bool) {
        self.process_preview_data = val;
    }

    fn terminate_stream<'a>(&mut self, source_name: Cow<'a, str>, universe: u16){
        match find_discovered_src(&self.discovered_sources, &source_name.to_string()){
            Some(index) => {
                self.discovered_sources[index].terminate_universe(universe);
            },
            None => {}
        }
    }

    // Handles the given data packet for this DMX reciever.
    // Returns the universe data if successful.
    // If the returned value is None it indicates that the data was received successfully but isn't ready to act on.
    // Synchronised data packets handled as per ANSI E1.31-2018 Section 6.2.4.1.
    fn handle_data_packet(&mut self, data_pkt: DataPacketFramingLayer) -> Result<Option<Vec<DMXData>>, Error>{
        // TODO - Sequence numbering is not supported by this receiver, it has been left for if there is sufficient time later.

        if data_pkt.preview_data && !self.process_preview_data {
            // Don't process preview data unless receiver has process_preview_data flag set.
            return Ok(None);
        }

        if data_pkt.stream_terminated {
            self.terminate_stream(data_pkt.source_name, data_pkt.universe);
            return Ok(None); // TODO, do we want to return an error here to indicate a stream was terminated?
        }

        if data_pkt.synchronization_address == NO_SYNC_ADDR {
            self.clear_waiting_data();

            #[cfg(feature = "std")]
            let vals: Vec<u8> = data_pkt.data.property_values.into_owned();
            let dmx_data: DMXData = DMXData {
                universe: data_pkt.universe, 
                values: vals.to_vec(),
                sync_uni: data_pkt.synchronization_address
            };

            #[cfg(not(feature = "std"))]
            let dmx_data: DMXData = DMXData {
                universe: data_pkt.universe,
                values: data_pkt.data.property_values,
                sync_uni: data_pkt.synchronization_address
            };

            return Ok(Some(vec![dmx_data]));
        } else {
            #[cfg(feature = "std")]
            let vals: Vec<u8> = data_pkt.data.property_values.into_owned();
            let dmx_data: DMXData = DMXData {
                universe: data_pkt.universe,
                values: vals.to_vec(),
                sync_uni: data_pkt.synchronization_address
            };

            #[cfg(not(feature = "std"))]
            let dmx_data: DMXData = DMXData {
                universe: data_pkt.universe,
                values: data_pkt.data.property_values,
                sync_uni: data_pkt.synchronization_address
            };

            self.store_waiting_data(dmx_data)?;
            
            Ok(None)
        }
    }

    /// Store given data in this receive by adding it to the waiting buffer.
    fn store_waiting_data(&mut self, data: DMXData) -> Result<(), Error>{
        for i in 0 .. self.waiting_data.len() {
            if self.waiting_data[i].universe == data.universe && self.waiting_data[i].sync_uni == data.sync_uni { 
                // Implementation detail: Multiple bits of data for the same universe can 
                // be buffered at one time as long as the data is waiting for different synchronisation universes.
                // Only if the data is for the same universe and is waiting for the same synchronisation universe is it merged.
                self.waiting_data[i] = ((self.merge_func)(&self.waiting_data[i], &data)).unwrap();
                return Ok(())
            }
        }

        self.waiting_data.push(data);
        Ok(())
    }
        
    // Handles the given synchronisation packet for this DMX receiver. 
    // Returns the released / previously blocked data if successful.
    // If the returned Vec is empty it indicates that no data was waiting.
    // Synchronisation packets handled as described by ANSI E1.31-2018 Section 6.2.4.1
    /* E1.31 Synchronization Packets occur on specific universes. Upon receipt, they indicate that any data advertising that universe as its Synchronization Address must be acted upon.
        In an E1.31 Data Packet, a value of 0 in the Synchronization Address indicates that the universe data is not synchronized. If a receiver is presented with an E1.31 Data Packet 
        containing a Synchronization Address of 0, it shall discard any data waiting to be processed and immediately act on that Data Packet. 
        
        If the Synchronization Address field is not 0, and the receiver is receiving an active synchronization stream for that Synchronization Address, 
        it shall hold that E1.31 Data Packet until the arrival of the appropriate E1.31 Synchronization Packet before acting on it.

*/
    fn handle_sync_packet(&mut self, sync_pkt: SynchronizationPacketFramingLayer) -> Result<Option<Vec<DMXData>>, Error>{
        let res = self.rtrv_waiting_data(sync_pkt.synchronization_address)?;
        if res.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(res))
        }
    }

    // Retrieves and removes the DMX data of all waiting data with a synchronisation address matching the one provided.
    fn rtrv_waiting_data(&mut self, sync_uni: u16) -> Result<Vec<DMXData>, Error>{
        let mut res: Vec<DMXData> = Vec::new();

        let mut i: usize = 0;
        let mut len: usize = self.waiting_data.len();

        while i < len {
            if self.waiting_data[i].sync_uni == sync_uni { 
                res.push(self.waiting_data.remove(i));
                len = len - 1;
            } else {
                i = i + 1;
            }
        }
        Ok(res)
    }

    fn update_discovered_srcs(&mut self, src: DiscoveredSacnSource) {
        match find_discovered_src(&self.discovered_sources, &src.name){
            Some(index) => {
                self.discovered_sources.remove(index);
            },
            None => {}
        }
        self.discovered_sources.push(src);
    }

    // Report point: There is no guarantees made by the protocol that different sources will have different names.
    // As names are used to match universe discovery packets this means that if 2 sources have the same name it won't
    // be clear which one is sending what universes as they will appear as one source. 

    // Report point: partially discovered sources are only marked as discovered when a full set of discovery packets has been
    // receieved, if a discovery packet is receieved but there are more pages the source won't be discovered until all the pages are receieved.
    // If a page is lost this therefore means the source update / discovery in its entirety will be lost - implementation detail.

    fn handle_universe_discovery_packet(&mut self, discovery_pkt: UniverseDiscoveryPacketFramingLayer) -> Result<Option<Vec<DMXData>>, Error>{
        let data: UniverseDiscoveryPacketUniverseDiscoveryLayer = discovery_pkt.data;

        let page: u8 = data.page;
        let last_page: u8 = data.last_page;

        #[cfg(feature = "std")]
        let universes = data.universes;

        let uni_page: UniversePage = UniversePage {
                page: page,
                universes: universes.into()
            };

        match find_discovered_src(&self.partially_discovered_sources, &discovery_pkt.source_name.to_string()) {
            Some(index) => {
                self.partially_discovered_sources[index].pages.push(uni_page);
                self.partially_discovered_sources[index].last_updated = Instant::now();
                if self.partially_discovered_sources[index].has_all_pages() {
                    let discovered_src: DiscoveredSacnSource = self.partially_discovered_sources.remove(index);
                    self.update_discovered_srcs(discovered_src);
                }
            }
            None => {
                let discovered_src: DiscoveredSacnSource = DiscoveredSacnSource {
                    name: discovery_pkt.source_name.to_string(),
                    last_page: last_page,
                    pages: vec![uni_page],
                    last_updated: Instant::now()
                };

                if page == 0 && page == last_page { // Indicates that this is a single page universe discovery packet.
                    self.update_discovered_srcs(discovered_src);
                } else { // Indicates that this is a page in a set of pages as part of a sources universe discovery.
                    self.partially_discovered_sources.push(discovered_src);
                }
            }
        }

        Ok(None)
    }

    // Attempt to recieve data from any of the registered universes.
    // This is the main method for receiving data.
    // Any data returned will be ready to act on immediately i.e. waiting e.g. for universe synchronisation
    // is already handled.
    // This method will return a WouldBlock error if there is no data ready within the given timeout.
    // Due to the underlying socket a timeout of duration 0 will instantly return a WouldBlock error without
    // checking for data.
    pub fn recv(&mut self, timeout: Option<Duration>) -> Result<Vec<DMXData>, Error> {
        let mut buf: [u8; RCV_BUF_DEFAULT_SIZE ] = [0; RCV_BUF_DEFAULT_SIZE];

        if timeout == Some(Duration::from_secs(0)) {
            return Err(Error::new(ErrorKind::WouldBlock, "No data avaliable in given timeout"));
        }

        self.receiver.set_timeout(timeout)?;
            let start_time = Instant::now();

            match self.receiver.recv(&mut buf){
                Ok(pkt) => {
                    let pdu: E131RootLayer = pkt.pdu;
                    let data: E131RootLayerData = pdu.data;
                    let res = match data {
                        DataPacket(d) => self.handle_data_packet(d)?,
                        SynchronizationPacket(s) => self.handle_sync_packet(s)?,
                        UniverseDiscoveryPacket(u) => self.handle_universe_discovery_packet(u)?
                    };
                    match res {
                        Some(r) => {
                            Ok(r)
                        },
                        None => { // Indicates that there is no data ready to pass up yet even if a packet was received.
                            // To stop recv blocking forever with a non-None timeout due to packets being received consistently (that reset the timeout)
                            // within the receive timeout (e.g. universe discovery packets if the discovery interval < timeout) the timeout needs to be 
                            // adjusted to account for the time already taken.
                            if !timeout.is_none() {
                                let elapsed = start_time.elapsed();
                                match timeout.unwrap().checked_sub(elapsed) {
                                    None => { // Indicates that elapsed is bigger than timeout so its time to return.
                                        return Err(Error::new(ErrorKind::WouldBlock, "No data avaliable in given timeout"));
                                    }
                                    Some(new_timeout) => {
                                        return self.recv(Some(new_timeout))
                                    }
                                }
                            } else {
                                // If the timeout was none then would keep looping till data is returned as the method should keep blocking till then.
                                self.recv(timeout)
                            }
                        } 
                    }
                }
                Err(err) => {
                    Err(err)
                }
            }
    }
    
    /// Terminates the SacnReceiver including cleaning up all used resources as necessary.
    pub fn terminate(&mut self){
        self.running = false;
    }

    pub fn get_discovered_sources(&mut self) -> Vec<DiscoveredSacnSource>{
        self.remove_expired_sources();
        self.discovered_sources.clone()
    }

    /// Goes through all discovered sources and removes any that have timed out
    fn remove_expired_sources(&mut self) {
        self.partially_discovered_sources.retain(|s| s.last_updated.elapsed() < UNIVERSE_DISCOVERY_SOURCE_TIMEOUT);
        self.discovered_sources.retain(|s| s.last_updated.elapsed() < UNIVERSE_DISCOVERY_SOURCE_TIMEOUT);
    }
}

// Searches for the discovered source with the given name in the given vector of discovered sources and returns the index of the src or None if not found.
fn find_discovered_src(srcs: &Vec<DiscoveredSacnSource>, name: &String) -> Option<usize> {
    for i in 0 .. srcs.len() {
        if srcs[i].name == *name {
            return Some(i);
        }
    }
    None
}

/// In general all lower level transport layer and below stuff is handled by DmxReciever . 
impl DmxReciever {
    // Creates a new DMX receiver on the interface specified by the given address.
    /// If the given address is an IPv4 address then communication will only work between IPv4 devices, if the given address is IPv6 then communication
    /// will only work between IPv6 devices by default but IPv4 receiving can be enabled using set_ipv6_only(false).
    pub fn new (ip: SocketAddr) -> Result<DmxReciever, Error> {
        Ok(
            DmxReciever {
                socket: create_socket(ip)?,
                addr: ip
            }
        )
    }

    /// Connects a socket to the multicast address which corresponds to the given universe to allow recieving packets for that universe.
    /// Returns as a Result containing a DmxReciever if Ok which recieves multicast packets for the given universe.
    pub fn listen_multicast_universe(&self, universe: u16) -> Result<(), Error> {
        let multicast_addr;

        if self.addr.is_ipv4() {
            multicast_addr = universe_to_ipv4_multicast_addr(universe)?;
        } else {
            multicast_addr = universe_to_ipv6_multicast_addr(universe)?;
        }

        join_multicast(&self.socket, multicast_addr)
    }

    /// If set to true then only receieve over IPv6. If false then receiving will be over both IPv4 and IPv6. 
    /// This will return an error if the SacnReceiver wasn't created using an IPv6 address to bind to.
    pub fn set_only_v6(&mut self, val: bool) -> Result<(), Error>{
        if self.addr.is_ipv4() {
            Err(Error::new(ErrorKind::Other, "Attempted to set IPv6 only when set to use an IPv4 address"))
        } else {
            self.socket.set_only_v6(val)
        }
    }

    // Returns a packet if there is one available. The packet may not be ready to transmit if it is awaiting synchronisation.
    // Will only block if set_timeout was called with a timeout of None so otherwise (and by default) it won't 
    // block so may return a WouldBlock error to indicate that there was no data ready.
    fn recv<'a>(&self, buf: &'a mut [u8; RCV_BUF_DEFAULT_SIZE]) -> Result<AcnRootLayerProtocol<'a>, Error> {
        self.socket.recv(&mut buf[0..])?;

        match AcnRootLayerProtocol::parse(buf) {
            Ok(pkt) => {
                Ok(pkt)
            }
            Err(err) => {
                Err(Error::new(ErrorKind::Other, err))
            }
        }
    }

    // Set the timeout for the recv operation, if this is called with a value of None then the recv operation will become blocking.
    // A timeout with Duration 0 will cause an error.
    pub fn set_timeout(&mut self, timeout: Option<Duration>) -> Result<(), Error> {
        self.socket.set_read_timeout(timeout)
    }

    /// Returns the current read timeout for the receiver.
    /// A timeout of None indicates infinite blocking behaviour.
    pub fn read_timeout(&self) -> Result<Option<Duration>, Error> {
        self.socket.read_timeout()
    }
}

// For create_socket and join_multicast methods.
// Code adapted from:
// https://bluejekyll.github.io/blog/rust/2018/03/18/multicasting-in-rust.html (05/02/2020)
// https://github.com/bluejekyll/multicast-example (05/02/2020)
// 
// With background reading from:
// https://stackoverflow.com/questions/2741611/receiving-multiple-multicast-feeds-on-the-same-port-c-linux/2741989#2741989 (05/02/2020)
// https://www.reddit.com/r/networking/comments/7nketv/proper_use_of_bind_for_multicast_receive_on_linux/ (05/02/2020)

pub fn create_socket(ip: SocketAddr) -> Result<Socket, Error> {
    if ip.is_ipv4() {
        let socket = Socket::new(Domain::ipv4(), Type::dgram(), Some(Protocol::udp()))?;
        // Bind to the unspecified address - allows receiving from any multicast address joined with the right port - as described in background.
        socket.bind(&SockAddr::from(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), ACN_SDT_MULTICAST_PORT))).unwrap();
        Ok(socket)

    } else {
        let socket = Socket::new(Domain::ipv6(), Type::dgram(), Some(Protocol::udp()))?;
        socket.bind(&SockAddr::from(SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), ACN_SDT_MULTICAST_PORT))).unwrap();
        Ok(socket)
    }
}

fn join_multicast(socket: &Socket, addr: SocketAddr) -> io::Result<()> {
    match addr.ip() {
        IpAddr::V4(ref mdns_v4) => {
            socket.join_multicast_v4(mdns_v4, &Ipv4Addr::new(0,0,0,0))?; // Needs to be set to the IP of the interface/network which the multicast packets are sent on (unless only 1 network)
        }
        IpAddr::V6(ref mdns_v6) => {
            socket.join_multicast_v6(mdns_v6, 0)?;
            socket.set_only_v6(true)?;
        }
    };

    Ok(())
}

// Performs a HTP DMX merge of data.
// The first argument (i) is the existing data, n is the new data.
// This function is only valid if both inputs have the same universe, sync addr, start_code and the data contains at least the first value (the start code).
// If this doesn't hold an error will be returned.
// Other merge functions may allow merging different start codes or not check for them.
pub fn htp_dmx_merge(i: &DMXData, n: &DMXData) -> Result<DMXData, Error>{
    if i.values.len() < 1 || n.values.len() < 1 || i.universe != n.universe || i.values[0] != n.values[0] || i.sync_uni != n.sync_uni {
        return Err(Error::new(ErrorKind::InvalidInput, "Attempted DMX merge on dmx data with different universes, syncronisation universes or data with no values"))
    }

    let mut r: DMXData = DMXData{
        universe: i.universe,
        values: Vec::new(),
        sync_uni: i.sync_uni
    };

    let mut i_iter = i.values.iter();
    let mut n_iter = n.values.iter();

    let mut i_val = i_iter.next();
    let mut n_val = n_iter.next();

    while (i_val.is_some()) || (n_val.is_some()){
        if i_val == None {
            r.values.push(*n_val.unwrap());
        } else if n_val == None {
            r.values.push(*i_val.unwrap());
        } else {
            r.values.push(max(*n_val.unwrap(), *i_val.unwrap()));
        }

        i_val = i_iter.next();
        n_val = n_iter.next();
    }

    Ok(r)
}

#[test]
fn test_handle_single_page_discovery_packet() {
    let mut dmx_rcv = SacnReceiver::new_v4().unwrap();

    let name = "Test Src 1";
    let page: u8 = 0;
    let last_page: u8 = 0;
    let universes: Vec<u16> = vec![0, 1, 2, 3, 4, 5];

    let discovery_pkt: UniverseDiscoveryPacketFramingLayer = UniverseDiscoveryPacketFramingLayer {
        source_name: name.into(),

        /// Universe dicovery layer.
        data: UniverseDiscoveryPacketUniverseDiscoveryLayer {
            page: page,

            /// The number of the final page.
            last_page: last_page,

            /// List of universes.
            universes: universes.clone().into(),
        },
    };

    let mut internal = dmx_rcv.internal.lock().unwrap();
    
    let res: Option<Vec<DMXData>> = internal.handle_universe_discovery_packet(discovery_pkt).unwrap();

    assert!(res.is_none());

    assert_eq!(internal.discovered_sources.len(), 1);

    assert_eq!(internal.discovered_sources[0].name, name);
    assert_eq!(internal.discovered_sources[0].last_page, last_page);
    assert_eq!(internal.discovered_sources[0].pages.len(), 1);
    assert_eq!(internal.discovered_sources[0].pages[0].page, page);
    assert_eq!(internal.discovered_sources[0].pages[0].universes, universes);
}

#[test]
fn test_store_retrieve_waiting_data(){
    let mut dmx_rcv = SacnReceiver::new_v4().unwrap();

    let sync_uni: u16 = 1;
    let universe: u16 = 0;
    let vals: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let dmx_data = DMXData {
        universe: universe,
        values: vals.clone(),
        sync_uni: sync_uni 
    };

    let mut internal = dmx_rcv.internal.lock().unwrap();

    internal.store_waiting_data(dmx_data).unwrap();

    let res: Vec<DMXData> = internal.rtrv_waiting_data(sync_uni).unwrap();

    assert_eq!(res.len(), 1);
    assert_eq!(res[0].universe, universe);
    assert_eq!(res[0].sync_uni, sync_uni);
    assert_eq!(res[0].values, vals);
}

#[test]
fn test_store_2_retrieve_1_waiting_data(){
    let mut dmx_rcv = SacnReceiver::new_v4().unwrap();

    let sync_uni: u16 = 1;
    let universe: u16 = 0;
    let vals: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let dmx_data = DMXData {
        universe: universe,
        values: vals.clone(),
        sync_uni: sync_uni 
    };

    let dmx_data2 = DMXData {
        universe: universe + 1,
        values: vals.clone(),
        sync_uni: sync_uni + 1 
    };

    let mut internal = dmx_rcv.internal.lock().unwrap();

    internal.store_waiting_data(dmx_data).unwrap();
    internal.store_waiting_data(dmx_data2).unwrap();

    let res: Vec<DMXData> = internal.rtrv_waiting_data(sync_uni).unwrap();

    assert_eq!(res.len(), 1);
    assert_eq!(res[0].universe, universe);
    assert_eq!(res[0].sync_uni, sync_uni);
    assert_eq!(res[0].values, vals);
}

#[test]
fn test_store_2_retrieve_2_waiting_data(){
    let mut dmx_rcv = SacnReceiver::new_v4().unwrap();

    let sync_uni: u16 = 1;
    let universe: u16 = 0;
    let vals: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let dmx_data = DMXData {
        universe: universe,
        values: vals.clone(),
        sync_uni: sync_uni 
    };

    let vals2: Vec<u8> = vec![0, 9, 7, 3, 2, 4, 5, 6, 5, 1, 2, 3];

    let dmx_data2 = DMXData {
        universe: universe + 1,
        values: vals2.clone(),
        sync_uni: sync_uni + 1 
    };

    let mut internal = dmx_rcv.internal.lock().unwrap();

    internal.store_waiting_data(dmx_data).unwrap();
    internal.store_waiting_data(dmx_data2).unwrap();

    let res: Vec<DMXData> = internal.rtrv_waiting_data(sync_uni).unwrap();

    assert_eq!(res.len(), 1);
    assert_eq!(res[0].universe, universe);
    assert_eq!(res[0].sync_uni, sync_uni);
    assert_eq!(res[0].values, vals);

    let res2: Vec<DMXData> = internal.rtrv_waiting_data(sync_uni + 1).unwrap();

    assert_eq!(res2.len(), 1);
    assert_eq!(res2[0].universe, universe + 1);
    assert_eq!(res2[0].sync_uni, sync_uni + 1);
    assert_eq!(res2[0].values, vals2);
}
