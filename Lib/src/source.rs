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
use std::thread::sleep;

use net2::{UdpBuilder, UdpSocketExt};
use uuid::Uuid;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};

use packet::{AcnRootLayerProtocol, DataPacketDmpLayer, DataPacketFramingLayer, SynchronizationPacketFramingLayer, E131RootLayer,
             E131RootLayerData, UNIVERSE_CHANNEL_CAPACITY, NO_SYNC_UNIVERSE, UniverseDiscoveryPacketUniverseDiscoveryLayer, 
             UniverseDiscoveryPacketFramingLayer, ACN_SDT_MULTICAST_PORT}; // As defined in ANSI E1.31-2018};

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

pub fn universe_to_ip(universe: u16) -> Result<String> {
    if universe < LOWEST_ALLOWED_UNIVERSE || universe > HIGHEST_ALLOWED_UNIVERSE {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "universe is limited to the range 1 to 63999",
        ));
    }
    let high_byte = (universe >> 8) & 0xff;
    let low_byte = universe & 0xff;

    // As per ANSI E1.31-2018 Section 9.3.1 Table 9-10.
    Ok(format!("239.255.{}.{}:{}", high_byte, low_byte, ACN_SDT_MULTICAST_PORT))
}

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
#[derive(Debug)]
pub struct DmxSource {
    socket: UdpSocket,
    cid: Uuid,
    name: String,
    preview_data: bool,
    sequences: RefCell<HashMap<u16, u8>>,
    sync_delay: Duration,
    universes: Vec<u16>, // A list of the universes registered to send by this source, used for universe discovery. Always sorted with lowest universe first to allow quicker usage.
}

impl DmxSource {
    /// Constructs a new DmxSource with the given name, binding to an IPv4 address.
    pub fn new_v4(name: &str) -> Result<DmxSource> {
        let cid = Uuid::new_v4();
        DmxSource::with_cid_v4(name, cid)
    }

    /// Constructs a new DmxSource with the given name and specified CID binding to an IPv4 address.
    pub fn with_cid_v4(name: &str, cid: Uuid) -> Result<DmxSource> {
        let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT);
        DmxSource::with_cid_ip(name, cid, ip)
    }

    /// Constructs a new DmxSource with the given name, binding to an IPv6 address.
    /// By default this will only receieve IPv6 data but IPv4 can also be enabled by calling set_ipv6_only(false).
    pub fn new_v6(name: &str) -> Result<DmxSource> {
        let cid = Uuid::new_v4();
        DmxSource::with_cid_v6(name, cid)
    }

    /// Constructs a new DmxSource with the given name and specified CID binding to an IPv6 address.
    pub fn with_cid_v6(name: &str, cid: Uuid) -> Result<DmxSource> {
        let ip = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT);
        DmxSource::with_cid_ip(name, cid, ip)
    }

    /// Consturcts a new DmxSource with the given name and binding to the supplied ip.
    pub fn with_ip(name: &str, ip: SocketAddr) -> Result<DmxSource> {
         DmxSource::with_cid_ip(name, Uuid::new_v4(), ip)
    }
    
    /// Constructs a new DmxSource with DMX START code set to 0 with specified CID and IP address.
    /// By default for an IPv6 address this will only receieve IPv6 data but IPv4 can also be enabled by calling set_ipv6_only(false).
    pub fn with_cid_ip(name: &str, cid: Uuid, ip: SocketAddr) -> Result<DmxSource> {
        let socket_builder;
        let socket;

        if ip.is_ipv4() {
            socket_builder = UdpBuilder::new_v4()?;
            socket = socket_builder.bind(ip)?;
            socket.set_multicast_ttl_v4(42).expect("Failed to set multicast TTL"); // TODO, is this needed? Why is this here?
        } else if ip.is_ipv6() {
            socket_builder = UdpBuilder::new_v6()?;
            socket_builder.only_v6(true)?;
            socket = socket_builder.bind(ip)?;
        } else {
            return Err(Error::new(ErrorKind::InvalidInput, "Unrecognised socket address type! Not IPv4 or IPv6"));
        }

        Ok(DmxSource {
            socket,
            cid,
            name: name.to_string(),
            preview_data: false,
            sequences: RefCell::new(HashMap::new()),
            sync_delay: DEFAULT_SYNC_DELAY,
            universes: Vec::new() 
        })
    }

    /// Allow sending on ipv6 
    pub fn set_ipv6_only(&mut self, val: bool) -> Result<()>{
        self.socket.set_only_v6(val)
    }

    pub fn register_universe(&mut self, universe: u16){
        if self.universes.len() == 0 {
            self.universes.push(universe);
        } else {
            // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.binary_search (30/12/2019)
            let i = self.universes.binary_search(&universe).unwrap_or_else(|x| x);
            self.universes.insert(i, universe);
        }
    }

    pub fn register_universes(&mut self, universes: &[u16]){
        for u in universes {
            self.register_universe(*u);
        }
    }

    /// Sends DMX data that spans multiple universes using universe synchronization.
    /// Will fail if the universes haven't been previously registered with this SacnSource.
    /// As per ANSI E1.31-2018 Section 6.6.1 this method shouldn't be called at a higher refresher rate
    /// than specified in ANSI E1.11 [DMX] unless configured by the user to do so in an environment 
    /// which doesn't contain any E1.31 to DMX512-A converters.
    /// If dst_ip is None then multicast is used otherwise unicast is used to the given ip.
    /// If priority is None then the default priority is used (100).
    /// If syncronisation_addr is None then the first universe is used for synchronisation if the data spans across multiple universes.
    ///     Note that if the data all fits within one universe it won't be synchronised.
    ///     To send data that must wait for later synchronisation use the send_sync method.
    pub fn send(&self, universes: &[u16], data: &[u8], priority: Option<u8>, dst_ip: Option<SocketAddr>, syncronisation_addr: Option<u16>) -> Result<()> {
        if data.len() == 0 {
           return Err(Error::new(ErrorKind::InvalidInput, "Must provide data to send, data.len() == 0"));
        }

        for u in universes {
            if *u < LOWEST_ALLOWED_UNIVERSE || *u > HIGHEST_ALLOWED_UNIVERSE{
                return Err(Error::new(ErrorKind::InvalidInput, format!("Universes must be in the range [{} - {}]", LOWEST_ALLOWED_UNIVERSE, HIGHEST_ALLOWED_UNIVERSE)));
            }

            if !self.universes.contains(u) {
                return Err(Error::new(ErrorKind::Other, "Must register universes to send before sending on them"));
            }
        }

        // + 1 as there must be at least 1 universe required as the data isn't empty then additional universes for any more.
        let required_universes = (data.len() / UNIVERSE_CHANNEL_CAPACITY) + 1;

        if universes.len() < required_universes {
            return Err(Error::new(ErrorKind::InvalidInput, "Must provide enough universes to send on"));
        }

        let sync_addr = if required_universes <= 1 {
            NO_SYNC_UNIVERSE
        } else {
            syncronisation_addr.unwrap_or(universes[0])
        };
        
        for i in 0 .. required_universes {
            let start_index = i * UNIVERSE_CHANNEL_CAPACITY;
            // Safety check to make sure that the end index doesn't exceed the data length
            let end_index = cmp::min((i + 1) * UNIVERSE_CHANNEL_CAPACITY, data.len());

            self.send_detailed(universes[i], &data[start_index .. end_index], priority.unwrap_or(DEFAULT_PRIORITY), sync_addr, &dst_ip)?;
        }

        if required_universes > 1 {
            // Small delay before sending sync packet as per ANSI-E1.31-2018 Appendix B.1
            sleep(self.sync_delay);
            
            self.send_sync_packet(sync_addr)?; // A sync packet must be sent so that the receiver will act on the sent data.
        }

        Ok(())
    }

    pub fn send_detailed(&self, universe: u16, data: &[u8], priority: u8, sync_address: u16, dst_ip: &Option<SocketAddr>) -> Result<()> {
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
            // self.socket.send_to(&packet.pack_alloc().unwrap(), &*(universe_to_ip(universe)?))?;
            let dst = universe_to_ip(universe)?;
            self.socket.send_to(&packet.pack_alloc().unwrap(), dst).expect("Send to failed!");
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
    pub fn send_sync_packet(&self, universe: u16) -> Result<()> {
        let ip = universe_to_ip(universe)?;
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
        self.socket.send_to(&packet.pack_alloc().unwrap(), &*ip)?;

        if sequence == 255 {
            sequence = 0;
        } else {
            sequence += 1;
        }
        self.sequences.borrow_mut().insert(universe, sequence);
        Ok(())
    }

    /// Terminates a universe stream.
    ///
    /// Terminates a stream to a specified universe by sending three packages with
    /// Stream_Terminated flag set to 1.
    /// The start code passed in is used for the first byte of the otherwise empty data payload to indicate the 
    /// start_code of the data.
    pub fn terminate_stream(&self, universe: u16, start_code: u8) -> Result<()> {
        let ip = universe_to_ip(universe)?;
        let mut sequence = match self.sequences.borrow_mut().remove(&universe) {
            Some(s) => s,
            None => 0,
        };

        for _ in 0..3 {
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
            self.socket.send_to(&packet.pack_alloc().unwrap(), &*ip)?;

            if sequence == 255 {
                sequence = 0;
            } else {
                sequence += 1;
            }
        }
        Ok(())
    }

    /// Sends a universe discovery packet advertising the universes that this source is registered to send.
    pub fn send_universe_discovery(&self) -> Result<()>{
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

        let ip = universe_to_ip(DISCOVERY_UNIVERSE)?;
        self.socket.send_to(&packet.pack_alloc().unwrap(), &*ip)?;

        Ok(())
    }

    /// Returns the ACN CID device identifier of the DmxSource.
    pub fn cid(&self) -> &Uuid {
        &self.cid
    }

    /// Sets the ACN CID device identifier.
    pub fn set_cid(&mut self, cid: Uuid) {
        self.cid = cid;
    }

    /// Returns the ACN source name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets ACN source name.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Returns if DmxSource is in preview mode.
    pub fn preview_mode(&self) -> bool {
        self.preview_data
    }

    /// Sets the DmxSource to preview mode.
    ///
    /// All packets will be sent with Preview_Data flag set to 1.
    pub fn set_preview_mode(&mut self, preview_mode: bool) {
        self.preview_data = preview_mode;
    }

    /// Sets the multicast time to live.
    pub fn set_multicast_ttl(&self, multicast_ttl: u32) -> Result<()> {
        self.socket.set_multicast_ttl_v4(multicast_ttl)
    }

    /// Returns the multicast time to live of the socket.
    pub fn multicast_ttl(&self) -> Result<u32> {
        self.socket.multicast_ttl_v4()
    }

    /// Sets if multicast loop is enabled.
    pub fn set_multicast_loop(&self, multicast_loop: bool) -> Result<()> {
        self.socket.set_multicast_loop_v4(multicast_loop)
    }

    /// Returns if multicast loop of the socket is enabled.
    pub fn multicast_loop(&self) -> Result<bool> {
        self.socket.multicast_loop_v4()
    }
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
        let ip: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT + 1);
        let source = DmxSource::with_ip("Source", ip).unwrap();

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
