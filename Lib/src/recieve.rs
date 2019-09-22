// Based on tutorial 
// https://bluejekyll.github.io/blog/rust/2018/03/18/multicasting-in-rust.html September 2019

// Objective ideas:
// - Ipv4 or Ipv6 Support
// - Simultaneous Ipv4 or Ipv6 support (Ipv6 preferred as newer and going to become more standard?)

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use socket2::{Socket, Domain, Type};

pub const ACN_SDT_MULTICAST_PORT: u16 = 5568; // As defined in ANSI E1.31-2018

// TODO create a universe to IP version to return the values as an len 4 array with each one being a byte to use below to replace magic constant.
lazy_static! {
    pub static ref IPV4: IpAddr = Ipv4Addr::new(239, 255, 0, 1).into();
}

#[test]
fn test_ipv4_multicast_range(){
    assert!(IPV4.is_multicast());
}
