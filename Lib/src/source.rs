// Copyright 2018 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.


// Report note:
// One of the problems with the existing SACN sending is how it didn't treat the payload transparently because it 
// would add on the start code. As this is part of the DMX protocol and not the SACN protocol this was removed as it 
// violated the extends of SACN.

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};
use std::cmp;
use std::time;
use std::time::Duration;
use std::thread::{sleep, JoinHandle};
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

// use math::round;

use net2::{UdpBuilder, UdpSocketExt};
use uuid::Uuid;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};

use packet::{AcnRootLayerProtocol, DataPacketDmpLayer, DataPacketFramingLayer, SynchronizationPacketFramingLayer, E131RootLayer,
             E131RootLayerData, UNIVERSE_CHANNEL_CAPACITY, NO_SYNC_UNIVERSE, UniverseDiscoveryPacketUniverseDiscoveryLayer, 
             UniverseDiscoveryPacketFramingLayer, ACN_SDT_MULTICAST_PORT,
             universe_to_ipv4_multicast_addr, universe_to_ipv6_multicast_addr}; // As defined in ANSI E1.31-2018};

/// The default delay between sending data packets and sending a synchronisation packet, used as advised by ANSI-E1.31-2018 Appendix B.1
pub const DEFAULT_SYNC_DELAY: Duration = time::Duration::from_millis(10);

/// The maximum number of universes per page in a universe discovery packet.
pub const DISCOVERY_UNI_PER_PAGE: usize = 512;

/// The universe used for universe discovery as defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative)
pub const DISCOVERY_UNIVERSE: u16 = 64214;

/// The default priority used for the E1.31 packet priority field, as per ANSI E1.31 Section 4.1 Table 4-1
pub const DEFAULT_PRIORITY: u8 = 100;

pub const LOWEST_ALLOWED_UNIVERSE: u16 = 1; // The lowest valued universe allowed as per ANSI E1.31-2018 Section 6.2.7

pub const HIGHEST_ALLOWED_UNIVERSE: u16 = 63999; // The highest valued universe allowed as per ANSI E1.31-2018 Section 6.2.7

pub const UPDATE_THREAD_NAME: &'static str = "rust_sacn_update_thread"; // The name of the thread which runs periodically to perform various actions such as universe discovery adverts.

pub const DEFAULT_TERMINATE_START_CODE: u8 = 0; // The default startcode used to send stream termination packets when the SacnSource is dropped.

// The poll rate of the update thread.
pub const DEFAULT_POLL_PERIOD: Duration = time::Duration::from_millis(1000);

// Report: The first universe for the data to be synchronised across multiple universes is 
// used as the syncronisation universe by default. This is done as it means that the receiever should
// be listening for this universe. 

// TODO, Write live code examples like the one below:

// Note the test below is depreciated so won't work without the sections being commented out.

/// A DMX over sACN sender.
///
/// DmxSource is used for sending sACN packets over ethernet.
///
/// Each universe will be sent to a dedicated multicast address
/// "239.255.{universe_high_byte}.{universe_low_byte}".
///
/// # Examples
///
/// ```
/// use sacn::DmxSource;
///
/// // let mut dmx_source = DmxSource::new("Controller").unwrap();
///
/// // dmx_source.send(1, &[0, 100, 100, 100, 100, 100, 100]);
/// // dmx_source.terminate_stream(1, 0);
/// ```

// General info / concept of running flag.
// https://www.reddit.com/r/rust/comments/b4ys9j/is_there_a_way_to_force_thread_to_join/ (12/01/2020)

#[derive(Debug)]
pub struct DmxSource {
    socket: UdpSocket,
    addr: SocketAddr,
    cid: Uuid,
    name: String,
    preview_data: bool,
    sequences: RefCell<HashMap<u16, u8>>,
    sync_delay: Duration,
    universes: Vec<u16>, // A list of the universes registered to send by this source, used for universe discovery. Always sorted with lowest universe first to allow quicker usage.
    running: bool
    // update_thread: JoinHandle<()> // The thread which runs every poll_period to perform various periodic action such as send universe discovery adverts. 
}

#[derive(Debug)]
pub struct SacnSource {
    // internal: Arc<Mutex<DmxSource>>
    internal: Arc<Mutex<DmxSource>>,
    update_thread: Option<JoinHandle<()>>
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

    pub fn with_cid_ip(name: &str, cid: Uuid, ip: SocketAddr) -> Result<SacnSource> {
        let trd_builder = thread::Builder::new().name(UPDATE_THREAD_NAME.into());

        let mut internal_src = Arc::new(Mutex::new(DmxSource::with_cid_ip(name, cid, ip)?));

        let mut trd_src = internal_src.clone();

        let src = SacnSource { 
            internal: internal_src,
            update_thread: Some(trd_builder.spawn(move || {
                while (trd_src.lock().unwrap().running) {
                    thread::sleep(DEFAULT_POLL_PERIOD);
                    perform_periodic_update(&mut trd_src);
                }
            }).unwrap()),
        };

        Ok(src)
    }

    /// Allow sending on ipv6 
    pub fn set_ipv6_only(&mut self, val: bool) -> Result<()>{
        // self.socket.set_only_v6(val)
        Err(Error::new(ErrorKind::Other, "Not impl"))
    }

    pub fn register_universes(&mut self, universes: &[u16]){
        self.internal.lock().unwrap().register_universes(universes);
    }

    pub fn register_universe(&mut self, universe: u16) {
        self.internal.lock().unwrap().register_universe(universe);
    }

    pub fn send(&self, universes: &[u16], data: &[u8], priority: Option<u8>, dst_ip: Option<SocketAddr>, syncronisation_addr: Option<u16>) -> Result<()> {
        self.internal.lock().unwrap().send(universes, data, priority, dst_ip, syncronisation_addr)
    }

    pub fn send_sync_packet(&self, universe: u16, dst_ip: &Option<SocketAddr>) -> Result<()> {
        self.internal.lock().unwrap().send_sync_packet(universe, dst_ip)
    }

    pub fn terminate_stream(&mut self, universe: u16, start_code: u8) -> Result<()> {
        self.internal.lock().unwrap().terminate_stream(universe, start_code)
    }

    pub fn send_universe_discovery(&self) -> Result<()> {
        self.internal.lock().unwrap().send_universe_discovery()
    }

     /// Returns the ACN CID device identifier of the DmxSource.
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

    /// Returns if DmxSource is in preview mode.
    pub fn preview_mode(&self) -> bool {
        self.internal.lock().unwrap().preview_mode()
    }

    /// Sets the DmxSource to preview mode.
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

impl Drop for SacnSource {
    fn drop(&mut self){
        // https://doc.rust-lang.org/1.22.1/book/second-edition/ch20-06-graceful-shutdown-and-cleanup.html (12/01/2020)
        if let Some(thread) = self.update_thread.take() {
            {
                self.internal.lock().unwrap().terminate(DEFAULT_TERMINATE_START_CODE);
            }
            thread.join().unwrap();
        }
    }
}

impl DmxSource {
    /// Constructs a new DmxSource with DMX START code set to 0 with specified CID and IP address.
    /// By default for an IPv6 address this will only receieve IPv6 data but IPv4 can also be enabled by calling set_ipv6_only(false).
    /// By default the TTL for ipv4 packets is 1 to keep them within the local network.
    fn with_cid_ip(name: &str, cid: Uuid, ip: SocketAddr) -> Result<DmxSource> {
        let socket_builder;

        if ip.is_ipv4() {
            socket_builder = UdpBuilder::new_v4()?;
            // socket.set_multicast_ttl_v4(1).expect("Failed to set ipv4 multicast TTL"); // Keep packets within the local network by default.
        } else if ip.is_ipv6() {
            socket_builder = UdpBuilder::new_v6()?;
            socket_builder.only_v6(true)?;
        } else {
            return Err(Error::new(ErrorKind::InvalidInput, "Unrecognised socket address type! Not IPv4 or IPv6"));
        }

        let socket: UdpSocket = socket_builder.bind(ip)?;

        Ok(DmxSource {
            socket: socket,
            addr: ip,
            cid: cid,
            name: name.to_string(),
            preview_data: false,
            sequences: RefCell::new(HashMap::new()),
            sync_delay: DEFAULT_SYNC_DELAY,
            universes: Vec::new(),
            running: true
        })
    }

    pub fn register_universes(&mut self, universes: &[u16]){
        for u in universes {
            self.register_universe(*u);
        }
    }

    fn register_universe(&mut self, universe: u16){
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
    }

    fn universe_allowed(&self, u: &u16) -> Result<()>{
        if *u < LOWEST_ALLOWED_UNIVERSE || *u > HIGHEST_ALLOWED_UNIVERSE{
            return Err(Error::new(ErrorKind::InvalidInput, format!("Universes must be in the range [{} - {}]", LOWEST_ALLOWED_UNIVERSE, HIGHEST_ALLOWED_UNIVERSE)));
        }

        if !self.universes.contains(u) {
            return Err(Error::new(ErrorKind::Other, "Must register universes to send before sending on them"));
        }

        Ok(())
    }

    /// Sends DMX data that spans multiple universes using universe synchronization.
    /// Will fail if the universes haven't been previously registered with this SacnSource.
    /// As per ANSI E1.31-2018 Section 6.6.1 this method shouldn't be called at a higher refresher rate
    /// than specified in ANSI E1.11 [DMX] unless configured by the user to do so in an environment 
    /// which doesn't contain any E1.31 to DMX512-A converters.
    /// If dst_ip is None then multicast is used otherwise unicast is used to the given ip. Broadcast is achieved by giving the broadcast IP.
    /// If priority is None then the default priority is used (100).
    /// If syncronisation_addr is Some() then the data will be sent with the given synchronisation address meaning it won't be acted on by receivers until a corresponding
    ///     synchronisation packet is sent using send_sync_packet(). If it is None then the data won't be synchronised and so will be acted upon by recievers immediately. This means
    ///     if sending more than 1 universe of data it may be acted upon at different times for each universe. A reasonable default for this is to use the first of the universes if 
    ///     synchronisation is required. Note as per ANSI-E1.31-2018 Appendix B.1 it is recommended to have a small delay before sending the sync packet.
    fn send(&self, universes: &[u16], data: &[u8], priority: Option<u8>, dst_ip: Option<SocketAddr>, syncronisation_addr: Option<u16>) -> Result<()> {
        println!("Sending universes: {:?} data: {:?} priority: {:?} dst_ip: {:?} sync_addr: {:?}", universes, data, priority, dst_ip, syncronisation_addr);

        if self.running == false { // Indicates that this sender has been terminated.
            return Err(Error::new(ErrorKind::NotConnected, "Sender has been terminated / isn't live")); 
        }

        if data.len() == 0 {
           return Err(Error::new(ErrorKind::InvalidInput, "Must provide data to send, data.len() == 0"));
        }

        for u in universes {
            self.universe_allowed(u)?;
        }

        if syncronisation_addr.is_some() {
            self.universe_allowed(&syncronisation_addr.unwrap())?;
        }

        // + 1 as there must be at least 1 universe required as the data isn't empty then additional universes for any more.
        let required_universes = (data.len() as f64 / UNIVERSE_CHANNEL_CAPACITY as f64).ceil() as usize;

        if universes.len() < required_universes {
            return Err(Error::new(ErrorKind::InvalidInput, "Must provide enough universes to send on"));
        }
        
        for i in 0 .. required_universes {
            let start_index = i * UNIVERSE_CHANNEL_CAPACITY;
            // Safety check to make sure that the end index doesn't exceed the data length
            let end_index = cmp::min((i + 1) * UNIVERSE_CHANNEL_CAPACITY, data.len());

            self.send_universe(universes[i], &data[start_index .. end_index], priority.unwrap_or(DEFAULT_PRIORITY), syncronisation_addr.unwrap_or(NO_SYNC_UNIVERSE), &dst_ip)?;
        }

        Ok(())
    }

    fn send_universe(&self, universe: u16, data: &[u8], priority: u8, sync_address: u16, dst_ip: &Option<SocketAddr>) -> Result<()> {
        if priority > 200 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "priority must be <= 200",
            ));
        }

        let mut sequence = match self.sequences.borrow().get(&universe) {
            Some(s) => *s,
            None => 0,
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
            self.socket.send_to(&packet.pack_alloc().unwrap(), dst_ip.unwrap())?;
        } else {
            let dst;

            if self.addr.is_ipv6(){
                dst = universe_to_ipv6_multicast_addr(universe)?;
            } else {
                dst = universe_to_ipv4_multicast_addr(universe)?;
            }

            println!("{}", dst);

            self.socket.send_to(&packet.pack_alloc().unwrap(), dst)?;
        }

        if sequence == 255 {
            sequence = 0;
        } else {
            sequence += 1;
        }
        self.sequences.borrow_mut().insert(universe, sequence);
        Ok(())
    }

    // Sends a synchronisation packet to trigger the sending of packets waiting to be sent together.
    fn send_sync_packet(&self, universe: u16, dst_ip: &Option<SocketAddr>) -> Result<()> {
        let ip;

        if dst_ip.is_none() {
            if self.addr.is_ipv6(){
                ip = universe_to_ipv6_multicast_addr(universe)?;
            } else {
                ip = universe_to_ipv4_multicast_addr(universe)?;
            }
        } else {
            ip = dst_ip.unwrap();
        }

        let mut sequence = match self.sequences.borrow().get(&universe) {
            Some(s) => *s,
            None => 0,
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
        self.socket.send_to(&packet.pack_alloc().unwrap(), ip)?;

        if sequence == 255 {
            sequence = 0;
        } else {
            sequence += 1;
        }
        self.sequences.borrow_mut().insert(universe, sequence);
        Ok(())
    }

    fn send_terminate_stream_pkt(&self, universe: u16, dst_ip: &Option<SocketAddr>, start_code: u8) -> Result<()> {
        let ip = match dst_ip{
            Some(x) => *x,
            None => {
                if self.addr.is_ipv6(){
                    universe_to_ipv6_multicast_addr(universe)?
                } else {
                    universe_to_ipv4_multicast_addr(universe)?
                }
            }
        };

        let mut sequence = match self.sequences.borrow_mut().remove(&universe) {
            Some(s) => s,
            None => 0,
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

        // println!("Terminate stream pkt: {:?}", res);

        self.socket.send_to(res, ip)?;

        if sequence == 255 {
            sequence = 0;
        } else {
            sequence += 1;
        }

        Ok(())
    }

    /// Terminates a universe stream.
    ///
    /// Terminates a stream to a specified universe by sending three packages with
    /// Stream_Terminated flag set to 1.
    /// The start code passed in is used for the first byte of the otherwise empty data payload to indicate the 
    /// start_code of the data.
    fn terminate_stream(&self, universe: u16, start_code: u8) -> Result<()> {
        for _ in 0..3 {
            self.send_terminate_stream_pkt(universe, &None, start_code)?;
        }
        Ok(())
    }

    /// Terminates the DMX source.
    /// This includes terminating each registered universe with the start_code given.
    fn terminate(&mut self, start_code: u8) {
        self.running = false;
        for u in &self.universes {
            self.terminate_stream(*u, start_code);
        }
    }

    /// Sends a universe discovery packet advertising the universes that this source is registered to send.
    fn send_universe_discovery(&self) -> Result<()>{
        let pages_req: u8 = ((self.universes.len() / DISCOVERY_UNI_PER_PAGE) + 1) as u8;

        for p in 0 .. pages_req {
            self.send_universe_discovery_detailed(p, pages_req - 1, &self.universes[(p as usize) .. (((p as usize) + 1) * DISCOVERY_UNI_PER_PAGE)])?;
        }
        Ok(())
    }

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
            ip = universe_to_ipv6_multicast_addr(DISCOVERY_UNIVERSE)?;
        } else {
            ip = universe_to_ipv4_multicast_addr(DISCOVERY_UNIVERSE)?;
        }

        self.socket.send_to(&packet.pack_alloc().unwrap(), ip)?;

        Ok(())
    }

    /// Returns the ACN CID device identifier of the DmxSource.
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

    /// Returns if DmxSource is in preview mode.
    fn preview_mode(&self) -> bool {
        self.preview_data
    }

    /// Sets the DmxSource to preview mode.
    ///
    /// All packets will be sent with Preview_Data flag set to 1.
    fn set_preview_mode(&mut self, preview_mode: bool) {
        self.preview_data = preview_mode;
    }

    /// Sets the multicast time to live.
    fn set_multicast_ttl(&self, multicast_ttl: u32) -> Result<()> {
        self.socket.set_multicast_ttl_v4(multicast_ttl)
    }

    /// Returns the multicast time to live of the socket.
    fn multicast_ttl(&self) -> Result<u32> {
        self.socket.multicast_ttl_v4()
    }

    /// Sets if multicast loop is enabled.
    fn set_multicast_loop(&self, multicast_loop: bool) -> Result<()> {
        self.socket.set_multicast_loop_v4(multicast_loop)
    }

    /// Returns if multicast loop of the socket is enabled.
    fn multicast_loop(&self) -> Result<bool> {
        self.socket.multicast_loop_v4()
    }
}

fn perform_periodic_update(src: &mut Arc<Mutex<DmxSource>>){
    
}

#[cfg(test)]
mod test {
    use super::*;
    use net2::UdpBuilder;
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
        let mut source = DmxSource::with_cid_ip(&source_name, Uuid::from_bytes(&cid).unwrap(), ip).unwrap();

        source.set_preview_mode(preview_data);
        source.set_multicast_loop(true).unwrap();

        let recv_socket = UdpSocket::bind("0.0.0.0:5568").unwrap();

        recv_socket.join_multicast_v4(&Ipv4Addr::new(239, 255, 0, 1), &Ipv4Addr::new(0, 0, 0, 0))
                   .unwrap();

        let mut recv_buf = [0; 1024];

        source.register_universes(&[universe]);

        source.send(&[universe], &dmx_data, Some(priority), None, None).unwrap();
        let (amt, _) = recv_socket.recv_from(&mut recv_buf).unwrap();

        assert_eq!(&packet[..], &recv_buf[0..amt]);
    }

    #[test]
    fn test_terminate_stream() {
        let cid = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

        let ip: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT + 1);
        let source = DmxSource::with_cid_ip(&"Source", Uuid::from_bytes(&cid).unwrap(), ip).unwrap();

        source.set_multicast_loop(true).unwrap();

        let recv_socket = UdpBuilder::new_v4().unwrap().bind("0.0.0.0:5568").unwrap();
        recv_socket
            .join_multicast_v4(&Ipv4Addr::new(239, 255, 0, 1), &Ipv4Addr::new(0, 0, 0, 0))
            .unwrap();

        let mut recv_buf = [0; 1024];

        let start_code: u8 = 0;

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
