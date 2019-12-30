// Code taken and based on tutorial 
// https://bluejekyll.github.io/blog/rust/2018/03/18/multicasting-in-rust.html September 2019
// https://blog.abby.md/2019/05/16/multicasting-in-rust/ September 2019


// Objective ideas:
// - Ipv4 or Ipv6 Support
// - Simultaneous Ipv4 or Ipv6 support (Ipv6 preferred as newer and going to become more standard?)
// - Support for Windows and Unix

use net2::UdpSocketExt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};

use packet::{AcnRootLayerProtocol, E131RootLayer, E131RootLayerData, E131RootLayerData::DataPacket, 
    E131RootLayerData::SynchronizationPacket, E131RootLayerData::UniverseDiscoveryPacket, UniverseDiscoveryPacketFramingLayer, 
    SynchronizationPacketFramingLayer, DataPacketFramingLayer, UniverseDiscoveryPacketUniverseDiscoveryLayer};

use std::io;
use std::io::{Error, ErrorKind};

use std::borrow::Cow;

use std::cmp::max;

pub const ACN_SDT_MULTICAST_PORT: u16 = 5568; // As defined in ANSI E1.31-2018

/// Value of the highest byte of the IPV4 multicast address as specified in section 9.3.1 of ANSI E1.31-2018.
pub const E131_MULTICAST_IPV4_HIGHEST_BYTE: u8 = 239;

/// Value of the second highest byte of the IPV4 multicast address as specified in section 9.3.1 of ANSI E1.31-2018.
pub const E131_MULTICAST_IPV4_SECOND_BYTE: u8 = 255;

/// The maximum universe number that can be used with the E1.31 protocol as specified in section 9.1.1 of ANSI E1.31-2018.
pub const E131_MAX_MULTICAST_UNIVERSE: u16 = 63999;

/// The lowest / minimum universe number that can be used with the E1.31 protocol as specified in section 9.1.1 of ANSI E1.31-2018.
pub const E131_MIN_MULTICAST_UNIVERSE: u16 = 1;

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

/// Used for receiving dmx or other data on a particular universe using multicast.
struct DmxReciever{
    socket: UdpSocket
}

pub struct DiscoveredSacnSource {

}

/// Allows receiving dmx or other (different startcode) data using sacn.
pub struct SacnReceiver {
    receiver: DmxReciever,
    waiting_data: Vec<DMXData>, // Data that hasn't been passed up yet as it is waiting e.g. due to universe synchronisation.
    universes: Vec<u16>, // Universes that this receiver is currently listening for
    discovered_sources: Vec<DiscoveredSacnSource>, // Sacn sources that have been discovered by this receiver through universe discovery packets.
    merge_func: fn(&DMXData, &DMXData) -> Result<DMXData, Error>,
    partially_discovered_sources: DiscoveredSacnSource // Sacn sources that have been partially discovered by only some of their universes being discovered so far with more pages to go.
}

impl SacnReceiver {
    pub fn listen_universes(&mut self, universes: &[u16]) -> Result<(), Error>{
        self.listen_multicast_universes(universes)?;
        self.universes.extend(universes); // Added all the newly listened to universes.
        Ok(())
    }

    /// Starts listening to the multicast addresses which corresponds to the given universe to allow recieving packets for that universe.
    fn listen_multicast_universes(&self, universes: &[u16]) -> Result<(), Error>{
        for u in universes {
            self.receiver.listen_multicast_universe(*u)?
        }
        Ok(())
    }

    pub fn new (addr: SocketAddr) -> Result<SacnReceiver, Error> {
        Ok (
            SacnReceiver {
                // multicast_universe_receivers: Vec::new(),
                receiver: DmxReciever::new(addr)?,
                waiting_data: Vec::new(),
                universes: Vec::new(),
                merge_func: htp_dmx_merge,
                // next_index: 0,
            }
        )
    }

    pub fn clear_waiting_data(&mut self){
        self.waiting_data.clear();
    }

     // Handles the given data packet for this DMX reciever.
    // Returns the universe data if successful.
    // If the returned Vec is empty it indicates that the data was received successfully but isn't ready to act on.
    // Synchronised data packets handled as per ANSI E1.31-2018 Section 6.2.4.1.
    fn handle_data_packet(&mut self, data_pkt: DataPacketFramingLayer) -> Result<Vec<DMXData>, Error>{
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

            return Ok(vec![dmx_data]);
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
            
            Ok(Vec::new())
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
    fn handle_sync_packet(&mut self, sync_pkt: SynchronizationPacketFramingLayer) -> Result<Vec<DMXData>, Error>{
        self.rtrv_waiting_data(sync_pkt.synchronization_address)
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

    // Report point: There is no guarantees made by the protocol that different sources will have different names.
    // As names are used to match universe discovery packets this means that if 2 sources have the same name it won't
    // be clear which one is sending what universes as they will appear as one source. 

    // Report point: partially discovered sources are only marked as discovered when a full set of discovery packets has been
    // receieved, if a discovery packet is receieved but there are more pages the source won't be discovered until all the pages are receieved.
    // If a page is lost this therefore means the source update / discovery in its entirety will be lost - implementation detail.

    fn handle_universe_discovery_packet(&mut self, discovery_pkt: UniverseDiscoveryPacketFramingLayer) -> Result<Vec<DMXData>, Error>{
        let src_name = discovery_pkt.source_name;
        let data: UniverseDiscoveryPacketUniverseDiscoveryLayer = discovery_pkt.data;

        let page: u8 = data.page;
        let last_page: u8 = data.last_page;

        #[cfg(feature = "std")]
        let universes: Cow<'a, [u16]> = data.universes;

        #[cfg(not(feature = "std"))]
        let universes: Vec<u16, [u16; 512]> = data.universes;

        if (page == last_page){ // Indicates that all discovery pages from this source have been receieved.
            
        }

        for i in 0 .. self.discovered_sources.len() {
            if self.discovered_sources[i].src_name == src_name { // Already know about this source so update it.
                self.discovered_sources[i];
                break;
            }
        }

        // TODO, this is a forced type pattern, perhaps the returned type should be different for each handler and an enum/option 
        // used to switch between them.
        Ok(Vec::new())
    }

    pub fn set_nonblocking(&mut self, is_nonblocking: bool) -> Result<(), Error> {
        self.receiver.set_nonblocking(is_nonblocking)
    }

    // Attempt to recieve data from any of the registered universes.
    // This is the main method for receiving data.
    // Any data returned will be ready to act on immediately i.e. waiting e.g. for universe synchronisation
    // is already handled.
    // This method will return a WouldBlock error if there is no data available on any of the enabled receive modes (uni-, multi- or broad- cast).
    pub fn recv(&mut self) -> Result<Vec<DMXData>, Error> {
        let mut buf: [u8; RCV_BUF_DEFAULT_SIZE] = [0; RCV_BUF_DEFAULT_SIZE];
        match self.receiver.recv(&mut buf){
            Ok(pkt) => {
                let pdu: E131RootLayer = pkt.pdu;
                let data: E131RootLayerData = pdu.data;
                match data {
                    DataPacket(d) => { 
                        let r = self.handle_data_packet(d)?;
                        if r.len() <= 0 { // Indicates that there is no data ready to pass up yet so don't return.
                            return self.recv();
                        }
                        Ok(r)
                    },
                    SynchronizationPacket(s) => self.handle_sync_packet(s),
                    UniverseDiscoveryPacket(u) => self.handle_universe_discovery_packet(u)
                }
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}

#[test]
fn test_store_retrieve_waiting_data(){
    let mut dmx_rcv = SacnReceiver::new(SocketAddr::new(Ipv4Addr::new(127,0,0,1).into(), ACN_SDT_MULTICAST_PORT)).unwrap();

    let sync_uni: u16 = 1;
    let universe: u16 = 0;
    let vals: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let dmx_data = DMXData {
        universe: universe,
        values: vals.clone(),
        sync_uni: sync_uni 
    };

    dmx_rcv.store_waiting_data(dmx_data).unwrap();

    let res: Vec<DMXData> = dmx_rcv.rtrv_waiting_data(sync_uni).unwrap();

    assert_eq!(res.len(), 1);
    assert_eq!(res[0].universe, universe);
    assert_eq!(res[0].sync_uni, sync_uni);
    assert_eq!(res[0].values, vals);
}

#[test]
fn test_store_2_retrieve_1_waiting_data(){
    let mut dmx_rcv = SacnReceiver::new(SocketAddr::new(Ipv4Addr::new(127,0,0,1).into(), ACN_SDT_MULTICAST_PORT)).unwrap();

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

    dmx_rcv.store_waiting_data(dmx_data).unwrap();
    dmx_rcv.store_waiting_data(dmx_data2).unwrap();

    let res: Vec<DMXData> = dmx_rcv.rtrv_waiting_data(sync_uni).unwrap();

    assert_eq!(res.len(), 1);
    assert_eq!(res[0].universe, universe);
    assert_eq!(res[0].sync_uni, sync_uni);
    assert_eq!(res[0].values, vals);
}

#[test]
fn test_store_2_retrieve_2_waiting_data(){
    let mut dmx_rcv = SacnReceiver::new(SocketAddr::new(Ipv4Addr::new(127,0,0,1).into(), ACN_SDT_MULTICAST_PORT)).unwrap();

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

    dmx_rcv.store_waiting_data(dmx_data).unwrap();
    dmx_rcv.store_waiting_data(dmx_data2).unwrap();

    let res: Vec<DMXData> = dmx_rcv.rtrv_waiting_data(sync_uni).unwrap();

    assert_eq!(res.len(), 1);
    assert_eq!(res[0].universe, universe);
    assert_eq!(res[0].sync_uni, sync_uni);
    assert_eq!(res[0].values, vals);

    let res2: Vec<DMXData> = dmx_rcv.rtrv_waiting_data(sync_uni + 1).unwrap();

    assert_eq!(res2.len(), 1);
    assert_eq!(res2[0].universe, universe + 1);
    assert_eq!(res2[0].sync_uni, sync_uni + 1);
    assert_eq!(res2[0].values, vals2);
}

impl DmxReciever {
    // Creates a new DMX receiver on the interface specified by the given address.
    // TODO, look at ways to refer to interfaces without using IP's.
    pub fn new (addr: SocketAddr) -> Result<DmxReciever, Error> {
        Ok(
            DmxReciever {
                socket: bind_socket(addr)?
            }
        )
    }

    /// Connects a socket to the multicast address which corresponds to the given universe to allow recieving packets for that universe.
    /// Returns as a Result containing a DmxReciever if Ok which recieves multicast packets for the given universe.
    pub fn listen_multicast_universe(&self, universe: u16) -> Result<(), Error> {
        let ipv4_addr_segments = universe_to_ipv4_arr(universe)?;
        let multicast_addr: IpAddr = Ipv4Addr::new(ipv4_addr_segments[0], ipv4_addr_segments[1], ipv4_addr_segments[2], ipv4_addr_segments[3]).into();

        join_multicast(&self.socket, SocketAddr::new(multicast_addr, ACN_SDT_MULTICAST_PORT))?;

        Ok(())
    }

    // Returns a packet if there is one available. The packet may not be ready to transmit if it is awaiting synchronisation.
    // Doesn't block so may return a WouldBlock error to indicate that there was no data ready.
    fn recv<'a>(&self, buf: &'a mut [u8; RCV_BUF_DEFAULT_SIZE]) -> Result<AcnRootLayerProtocol<'a>, Error>{
        let (_len, _remote_addr) = self.socket.recv_from(&mut buf[0..])?;

        match AcnRootLayerProtocol::parse(buf) {
            Ok(pkt) => {
                Ok(pkt)
            }
            Err(err) => {
                Err(Error::new(ErrorKind::Other, err))
            }
        }
    }

    pub fn set_nonblocking(&mut self, is_nonblocking: bool) -> Result<(), Error> {
        self.socket.set_nonblocking(is_nonblocking)
    }
}

// Performs a HTP DMX merge of data.
// The first argument (i) is the existing data, n is the new data.
// This function is only valid if both inputs have the same universe, sync addr, start_code and the data contains at least the first value (the start code).
// If this doesn't hold an error will be returned.
// Other merge functions may allow merging different start codes or not check for them.
fn htp_dmx_merge(i: &DMXData, n: &DMXData) -> Result<DMXData, Error>{
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

/// Converts given universe number in range 1 - 63999 inclusive into an u8 array of length 4 with the first byte being
/// the highest byte in the multicast IP for that universe, the second byte being the second highest and so on.
/// 
/// Converstion done as specified in section 9.3.1 of ANSI E1.31-2018
///
/// Returns as a Result with the OK value being the array and the Err value being an Error.
fn universe_to_ipv4_arr(universe: u16) -> Result<[u8;4], Error>{
    if universe == 0 || universe > E131_MAX_MULTICAST_UNIVERSE {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "universe is limited to the range 1 to 63999",
        ));
    }
    let high_byte: u8 = ((universe >> 8) & 0xff) as u8;
    let low_byte: u8 = (universe & 0xff) as u8;

    Ok([E131_MULTICAST_IPV4_HIGHEST_BYTE, E131_MULTICAST_IPV4_SECOND_BYTE, high_byte, low_byte])
}

// fn new_socket(addr: &SocketAddr) -> io::Result<UdpSocket> {
//     let domain = if addr.is_ipv4(){
//         Domain::ipv4()
//     } else {
//         Domain::ipv6()
//     };

//     let socket = Socket::new(domain, Type::dgram(), Some(Protocol::udp()))?;
//     socket.set_nonblocking(true)?;

//     Ok(socket.into_udp_socket())
// }

fn join_multicast(socket: &UdpSocket, addr: SocketAddr) -> io::Result<()> {
    let ip_addr = addr.ip();

    match ip_addr {
        IpAddr::V4(ref mdns_v4) => {
            socket.join_multicast_v4(mdns_v4, &Ipv4Addr::new(0,0,0,0))?; // Needs to be set to the IP of the interface/network which the multicast packets are sent on (unless only 1 network)
        }
        IpAddr::V6(ref mdns_v6) => {
            socket.join_multicast_v6(mdns_v6, 0)?;
            socket.set_only_v6(true)?; // TODO, check if this is as expected - why only v6?
        }
    };

    Ok(())
}

#[cfg(windows)]
fn bind_socket(addr: SocketAddr) -> io::Result<UdpSocket>{
    let addr = match addr {
        SocketAddr::V4(addr) => {
            SocketAddr::new(Ipv4Addr::new(0,0,0,0).into(), addr.port())
        }
        SocketAddr::V6(addr) => {
            SocketAddr::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0).into(), addr.port())
        }
    };

    UdpSocket::bind(addr)
}


#[cfg(unix)]
fn bind_socket(socket: &Socket, addr: &SocketAddr) -> io::Result<()> {
    socket.bind(&SockAddr::from(addr))?;
}

#[test]
fn test_universe_to_ip_array_lowest_byte_normal(){
    let val: u16 = 119;
    let res = universe_to_ipv4_arr(val).unwrap();
    assert!(res[0] == E131_MULTICAST_IPV4_HIGHEST_BYTE);
    assert!(res[1] == E131_MULTICAST_IPV4_SECOND_BYTE);
    assert!(res[2] == ((val / 256) as u8)); // val / 256 = value in highest byte. 256 = 2^8 (number of values within one 8 bit byte inc. 0).
    assert!(res[3] == ((val % 256) as u8)); // val % 256 = value in lowest byte.  
}

#[test]
fn test_universe_to_ip_array_both_bytes_normal(){
    let val: u16 = 300;
    let res = universe_to_ipv4_arr(val).unwrap();
    assert!(res[0] == E131_MULTICAST_IPV4_HIGHEST_BYTE);
    assert!(res[1] == E131_MULTICAST_IPV4_SECOND_BYTE);
    assert!(res[2] == ((val / 256) as u8)); // val / 256 = value in highest byte. 256 = 2^8 (number of values within one 8 bit byte inc. 0).
    assert!(res[3] == ((val % 256) as u8)); // val % 256 = value in lowest byte.  
}

#[test]
fn test_universe_to_ip_array_limit_high(){
    let res = universe_to_ipv4_arr(E131_MAX_MULTICAST_UNIVERSE).unwrap();
    assert!(res[0] == E131_MULTICAST_IPV4_HIGHEST_BYTE);
    assert!(res[1] == E131_MULTICAST_IPV4_SECOND_BYTE);
    assert!(res[2] == ((E131_MAX_MULTICAST_UNIVERSE / 256) as u8)); // val / 256 = value in highest byte. 256 = 2^8 (number of values within one 8 bit byte inc. 0).
    assert!(res[3] == ((E131_MAX_MULTICAST_UNIVERSE % 256) as u8)); // val % 256 = value in lowest byte. 
}

#[test]
fn test_universe_to_ip_array_limit_low(){
    let res = universe_to_ipv4_arr(E131_MIN_MULTICAST_UNIVERSE).unwrap();
    assert!(res[0] == E131_MULTICAST_IPV4_HIGHEST_BYTE);
    assert!(res[1] == E131_MULTICAST_IPV4_SECOND_BYTE);
    assert!(res[2] == ((E131_MIN_MULTICAST_UNIVERSE / 256) as u8)); // val / 256 = value in highest byte. 256 = 2^8 (number of values within one 8 bit byte inc. 0).
    assert!(res[3] == ((E131_MIN_MULTICAST_UNIVERSE % 256) as u8)); // val % 256 = value in lowest byte. 
}

#[test]
#[should_panic]
fn test_universe_to_ip_array_out_range_low(){
    universe_to_ipv4_arr(0).unwrap();
}

#[test]
#[should_panic]
fn test_universe_to_ip_array_out_range_high(){
    universe_to_ipv4_arr(E131_MAX_MULTICAST_UNIVERSE + 1).unwrap();
}
