extern crate sacn;
use sacn::DmxSource;
use sacn::recieve::DmxReciever;
use sacn::recieve::ACN_SDT_MULTICAST_PORT;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {
    let universe: u16 = 1;
    let reciever = DmxReciever::listen_universe(universe).unwrap();
    match reciever.recv_blocking() {
        Ok (len) {
            println!("Packet recieved: {:?}", protoPacket);
        }
        Err (err) {
            println!("Error recieving packet: {:?}", err);
        }
    }
    
}
