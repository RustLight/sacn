// Code taken and based on tutorial 
// https://bluejekyll.github.io/blog/rust/2018/03/18/multicasting-in-rust.html September 2019
// https://blog.abby.md/2019/05/16/multicasting-in-rust/ September 2019


// Objective ideas:
// - Ipv4 or Ipv6 Support
// - Simultaneous Ipv4 or Ipv6 support (Ipv6 preferred as newer and going to become more standard?)
// - Support for Windows and Unix

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use socket2::{Socket, Domain, Protocol, Type, SockAddr};

use std::io;
use std::time::Duration;

pub const ACN_SDT_MULTICAST_PORT: u16 = 5568; // As defined in ANSI E1.31-2018

// TODO create a universe to IP version to return the values as an len 4 array with each one being a byte to use below to replace magic constant.

lazy_static! {
    pub static ref IPV4: IpAddr = Ipv4Addr::new(239, 255, 0, 1).into();
}

fn new_socket(addr: &SocketAddr) -> io::Result<Socket> {
    let domain = if addr.is_ipv4(){
        Domain::ipv4()
    } else {
        Domain::ipv6()
    };

    let socket = Socket::new(domain, Type::dgram(), Some(Protocol::udp()))?;

    // socket.set_read_timeout(Some(Duration::from_millis(1000)))?; 

    Ok(socket)
}

pub fn join_multicast(addr: SocketAddr) -> io::Result<Socket> {
    let ip_addr = addr.ip();
    let socket = new_socket(&addr)?;
    println!("RCV socket: {:#?}", socket);

    match ip_addr {
        IpAddr::V4(ref mdns_v4) => {
            socket.join_multicast_v4(mdns_v4, &Ipv4Addr::new(0,0,0,0))?; // Needs to be set to the IP of the interface/network which the multicast packets are sent on (unless only 1 network)
        }
        IpAddr::V6(ref mdns_v6) => {
            socket.join_multicast_v6(mdns_v6, 0)?;
            socket.set_only_v6(true)?;
        }
    };

    bind_multicast(&socket, &addr)?;
    
    Ok(socket)
}

#[cfg(windows)]
fn bind_multicast(socket: &Socket, addr: &SocketAddr) -> io::Result<()>{
    println!("Windows binding multicast... ADDR: {}", addr);
    let addr = match *addr {
        SocketAddr::V4(addr) => {
            SocketAddr::new(Ipv4Addr::new(0,0,0,0).into(), addr.port())
        }
        SocketAddr::V6(addr) => {
            SocketAddr::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0).into(), addr.port())
        }
    };
    socket.bind(&socket2::SockAddr::from(addr))
}


#[cfg(unix)]
fn bind_multicast(socket: &Socket, addr: &SocketAddr) -> io::Result<()> {
    socket.bind(&SockAddr::from(addr))?;
}

// In code tests

#[test]
fn test_ipv4_multicast_range(){
    assert!(IPV4.is_multicast());
}
