#[macro_use]
extern crate lazy_static;
extern crate sacn;
use sacn::DmxSource;
use sacn::recieve::join_multicast;
use sacn::recieve::ACN_SDT_MULTICAST_PORT;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

lazy_static! {
    pub static ref IPV4: IpAddr = Ipv4Addr::new(239, 255, 0, 1).into();
    pub static ref LOCAL_IPV4: IpAddr = Ipv4Addr::new(192, 168, 1, 6).into(); // REMOVE ME EVENTUALLY, TESTING ONLY - SPECIFYS INTERFACE TO USE
}

fn main() {
    // let addr = SocketAddr::new(*IPV4, ACN_SDT_MULTICAST_PORT);
    let addr = SocketAddr::new(*IPV4, ACN_SDT_MULTICAST_PORT);

    match join_multicast(addr) {
        Ok(listener) => {
            println!("Listening");

            let mut buf = [0u8; 256]; // RCV Buffer

            match listener.recv_from(&mut buf) {
                Ok((len, remote_addr)) => {
                    let data = &buf[..len];

                    println!("Data recieved");
                }

                Err(err) => {
                    println!("Error recieving data");
                }
            }
        }

        Err (err) => {
            println!("Failed to join multicast, error: {:?}", err);
        }
    }

    
}
