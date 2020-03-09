extern crate socket2;

use std::io;
use std::time::Duration;

use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use std::thread::sleep;

use std::env;

fn main() {
    let cmd_args: Vec<String> = env::args().collect();

    println!("Started");

    // let addr = SocketAddr::new(Ipv6Addr::new(0xff18, 0, 0, 0, 0, 0, 0x8300, 1).into(), 5568);
    let addr = SocketAddr::new(Ipv4Addr::new(239,255,0,1).into(), 5568);
    let addr2 = SocketAddr::new(Ipv4Addr::new(239,255,0,2).into(), 5568);
    let interface_addr = SocketAddr::new(Ipv4Addr::new(192, 168, 1, 8).into(), 5568);
    let unix_interface_addr = SocketAddr::new(Ipv4Addr::new(0, 0 , 0, 0).into(), 5568);

    ///https://stackoverflow.com/questions/43292357/detect-platform-in-rust
    let socket;

    if cfg!(windows) {
        socket = new_win_socket(&interface_addr).unwrap();
    } else { // if cfg!(unix)
        socket = new_unix_socket(&unix_interface_addr, &addr2).unwrap();
    }


    // let socket = new_socket(&addr).unwrap();

    // // socket.join_multicast_v6(&Ipv6Addr::new(0xff18, 0, 0, 0, 0, 0, 0x8300, 1), 0).unwrap();

    socket.join_multicast_v4(&Ipv4Addr::new(239,255,0,1), &Ipv4Addr::new(0, 0, 0, 0)).unwrap();

    socket.join_multicast_v4(&Ipv4Addr::new(239,255,0,2), &Ipv4Addr::new(0, 0, 0, 0)).unwrap();

    
    // socket.bind(&SockAddr::from(SocketAddr::new(Ipv4Addr::new(239,255,0,1).into(), 5568))).unwrap();

    socket.set_multicast_loop_v4(false).unwrap();

    // https://stackoverflow.com/questions/31289588/converting-a-str-to-a-u8 (05/02/2020)
    let message = &cmd_args[1];
    let message2 = &cmd_args[2];

    // Background:
    // https://stackoverflow.com/questions/2741611/receiving-multiple-multicast-feeds-on-the-same-port-c-linux/2741989#2741989 (05/02/2020)
    // https://www.reddit.com/r/networking/comments/7nketv/proper_use_of_bind_for_multicast_receive_on_linux/ (05/02/2020)

    
    loop {
        socket.send_to(message.as_bytes(), &SockAddr::from(addr)).unwrap();
        socket.send_to(message2.as_bytes(), &SockAddr::from(addr2)).unwrap();
        sleep(Duration::from_secs(1));

        let mut buf = [0u8; 64];

        let res = socket.recv(&mut buf);
        match res{
            Err(e) => {
                println!("Err: {}", e);
            }
            Ok(_) => {
                print!("Res: ");
                print!("{}", std::str::from_utf8(&buf).unwrap());
                println!("");
            }
        }

    }
}

// Flat stole bits of this code from https://bluejekyll.github.io/blog/rust/2018/03/18/multicasting-in-rust.html (05/02/2020) just as a test to see if multicast works at-all.
// THIS CODE SHOULD NOT BE INCLUDED IN SUBMITTED MATERIAL
// I CLAIM NO OWNERSHIP

// this will be common for all our sockets

/// addr: Bind address.
fn new_win_socket(addr: &SocketAddr) -> io::Result<Socket> {
    let domain = if addr.is_ipv4() {
        Domain::ipv4()
    } else {
        Domain::ipv6()
    };

    let socket = Socket::new(domain, Type::dgram(), Some(Protocol::udp()))?;

    let winAddr = match addr {
        SocketAddr::V4(addr) => SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), addr.port()),
        SocketAddr::V6(addr) => {
            SocketAddr::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0).into(), addr.port())
        }
    };
    
    socket.bind(&socket2::SockAddr::from(winAddr));

    // we're going to use read timeouts so that we don't hang waiting for packets
    socket.set_read_timeout(Some(Duration::from_millis(1000)))?;

    Ok(socket)
}

/// addr: The multicast address to bind to.
fn new_unix_socket(addr: &SocketAddr, addr2: &SocketAddr) -> io::Result<Socket> {

// fn new_unix_socket(addr: &SocketAddr) -> io::Result<Socket> {  
    let domain = if addr.is_ipv4() {
        Domain::ipv4()
    } else {
        Domain::ipv6()
    };

    let socket = Socket::new(domain, Type::dgram(), Some(Protocol::udp()))?;

    socket.bind(&SockAddr::from(*addr)).unwrap();
    // socket.bind(&SockAddr::from(*addr2)).unwrap();

    // we're going to use read timeouts so that we don't hang waiting for packets
    socket.set_read_timeout(Some(Duration::from_millis(1000)))?;

    Ok(socket)
}

// fn _join_multicast(addr: SocketAddr) -> io::Result<Socket> {
//     let ip_addr = addr.ip();

//     let socket = new_socket(&addr)?;

//     // depending on the IP protocol we have slightly different work
//     match ip_addr {
//         IpAddr::V4(ref mdns_v4) => {
//             // join to the multicast address, with all interfaces
//             socket.join_multicast_v4(mdns_v4, &Ipv4Addr::new(0, 0, 0, 0))?;
//         }
//         IpAddr::V6(ref mdns_v6) => {
//             // join to the multicast address, with all interfaces (ipv6 uses indexes not addresses)
//             socket.join_multicast_v6(mdns_v6, 0)?;
//             socket.set_only_v6(true)?;
//         }
//     };

//     // bind us to the socket address.
//     socket.bind(&SockAddr::from(addr))?;
//     Ok(socket)
// }
