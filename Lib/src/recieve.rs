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
use std::io::{Error, ErrorKind, Result};

pub const ACN_SDT_MULTICAST_PORT: u16 = 5568; // As defined in ANSI E1.31-2018

/// Value of the highest byte of the IPV4 multicast address as specified in section 9.3.1 of ANSI E1.31-2018.
pub const E131_MULTICAST_IPV4_HIGHEST_BYTE: u8 = 239;

/// Value of the second highest byte of the IPV4 multicast address as specified in section 9.3.1 of ANSI E1.31-2018.
pub const E131_MULTICAST_IPV4_SECOND_BYTE: u8 = 255;

/// The maximum universe number that can be used with the E1.31 protocol as specified in section 9.1.1 of ANSI E1.31-2018.
pub const E131_MAX_MULTICAST_UNIVERSE: u16 = 63999;

/// The lowest / minimum universe number that can be used with the E1.31 protocol as specified in section 9.1.1 of ANSI E1.31-2018.
pub const E131_MIN_MULTICAST_UNIVERSE: u16 = 1;

pub fn listen_universe(universe: u16) -> Result<()> {
    Err(Error::new(ErrorKind::Other ,"Not implemented yet"))
}

/// Converts given universe number in range 1 - 63999 inclusive into an u8 array of length 4 with the first byte being
/// the highest byte in the multicast IP for that universe, the second byte being the second highest and so on.
/// 
/// Converstion done as specified in section 9.3.1 of ANSI E1.31-2018
///
/// Returns as a Result with the OK value being the array and the Err value being an Error.
fn universe_to_ipv4_arr(universe: u16) -> Result<[u8;4]>{
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
    let res = universe_to_ipv4_arr(0).unwrap();
}

#[test]
#[should_panic]
fn test_universe_to_ip_array_out_range_high(){
    let res = universe_to_ipv4_arr(E131_MAX_MULTICAST_UNIVERSE + 1).unwrap();
}

fn new_socket(addr: &SocketAddr) -> io::Result<Socket> {
    let domain = if addr.is_ipv4(){
        Domain::ipv4()
    } else {
        Domain::ipv6()
    };

    let socket = Socket::new(domain, Type::dgram(), Some(Protocol::udp()))?;

    Ok(socket)
}

fn join_multicast(addr: SocketAddr) -> io::Result<Socket> {
    let ip_addr = addr.ip();
    let socket = new_socket(&addr)?;
    println!("RCV socket: {:#?}", socket);

    match ip_addr {
        IpAddr::V4(ref mdns_v4) => {
            socket.join_multicast_v4(mdns_v4, &Ipv4Addr::new(0,0,0,0))?;
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
