extern crate sacn;
use sacn::DmxSource;
use sacn::recieve::{DmxReciever, ACN_SDT_MULTICAST_PORT, RCV_BUF_DEFAULT_SIZE};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {
    let universe: u16 = 1;
    let reciever = DmxReciever::listen_universe(universe).unwrap();
    
    
}
