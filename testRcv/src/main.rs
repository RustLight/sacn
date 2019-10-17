extern crate sacn;
use sacn::DmxSource;
use sacn::recieve::DmxReciever;
use sacn::recieve::ACN_SDT_MULTICAST_PORT;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// The default size of the buffer used to recieve E1.31 packets.
/// 1143 bytes is biggest packet required as per Section 8 of ANSI E1.31-2018, aligned to 64 bit that is 1144 bytes.
pub const RCV_BUF_DEFAULT_SIZE: usize = 1144;

fn main() {
    let universe: u16 = 1;
    let reciever = DmxReciever::listen_universe(universe).unwrap();

    let mut buf = [0u8; RCV_BUF_DEFAULT_SIZE];

    match reciever.recv_blocking(buf) {
        Ok (pkt) => {
            println!("Packet recieved: {:?}", pkt);
        }
        Err (err) => {
            println!("Error recieving packet: {:?}", err);
        }
    }
    
}
