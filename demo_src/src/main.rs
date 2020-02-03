#![allow(dead_code)]
#![allow(unused_imports)]

extern crate lazy_static;
extern crate sacn;
use sacn::SacnSource;
use std::{time}; // https://doc.rust-lang.org/std/thread/fn.sleep.html (20/09/2019)
use std::io;
use std::io::{Error, ErrorKind};
use sacn::packet::ACN_SDT_MULTICAST_PORT;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use std::env;

/// Usage ./main <interface_ip> <source_name>
/// Reads data from stdin and sends it using the protocol.
/// Data must be formatted as: 
/// d <universe> <sync_uni> <priority> <data as bytes>
/// Register a sending universe as:
/// r <universe>
fn main(){
    let cmd_args: Vec<String> = env::args().collect();

    let interface_ip = &cmd_args[1];

    let source_name = &cmd_args[2];

    // let priority: u8 = cmd_args[3].parse().unwrap();

    let mut src = SacnSource::with_ip(source_name, SocketAddr::new(IpAddr::V4(interface_ip.parse().unwrap()), ACN_SDT_MULTICAST_PORT)).unwrap();

    loop {
        // https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line (03/02/2020)
        match handle_input(&mut src) {
            Ok(_) => {}
            Err(e) => {
                println!("Error: Input data line unusable: {}", e);
            }
        }
    } 
}

fn handle_input(src: &mut SacnSource) -> Result <(), Error>{
    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // https://www.tutorialspoint.com/rust/rust_string.htm (03/02/2020)
            let split_input: Vec<&str> = input.split_whitespace().collect();
            if split_input.len() < 2 {
                return Err(Error::new(ErrorKind::InvalidInput, "Insufficient parts ( < 2 )"));
            }

            let universe: u16 = split_input[1].parse().unwrap();

            match split_input[0] {
                "d" => {
                    if split_input.len() < 3 {
                        return Err(Error::new(ErrorKind::InvalidInput, "Insufficient parts for data line ( < 3 )"));
                    }
                    // src.send(&[universe], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), None, None).unwrap();
                }
                "r" => {
                    src.register_universe(universe);
                }
                x => {
                    return Err(Error::new(ErrorKind::InvalidInput, format!("Unknown input type: {}", x)));
                }
            }
            Ok(())
        }
        Err(e) => {
            return Err(e);
        }
    }
}