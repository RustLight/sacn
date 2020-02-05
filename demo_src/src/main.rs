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

/// The start code used in terminate packets.
const TERMINATE_START_CODE: u8 = 0;

/// Usage ./main <interface_ip> <source_name>
/// Reads data from stdin and sends it using the protocol.
/// Data must be formatted as, a sync_universe of 0 means no synchronisation, this uses multicast: 
/// d <universe> <sync_uni> <priority> <data_as_u8_space_seperated>
/// To send data unicast use:
/// u <universe> <sync_uni> <priority> <dst_addr> <data_as_u8_space_seperated>
/// Register a sending universe as:
/// r <universe>
/// Terminate a universe using, if universe is 0 then will terminate entirely:
/// q <universe>
fn main(){
    let cmd_args: Vec<String> = env::args().collect();

    if cmd_args.len() < 3 {
        return display_help();
    }

    let interface_ip = &cmd_args[1];

    let source_name = &cmd_args[2];

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

fn display_help(){
    println!("
Usage ./main <interface_ip> <source_name>\n
Reads data from stdin and sends it using the protocol. \n
Data must be formatted as, a sync_universe of 0 means no synchronisation, this uses multicast: \n
d <universe> <sync_uni> <priority> <data_as_u8_space_seperated> \n
To send data unicast use: \n
u <universe> <sync_uni> <priority> <dst_addr> <data_as_u8_space_seperated> \n
Register a sending universe as: \n
r <universe> \n
Terminate a universe using, if universe is 0 then will terminate entirely: \n
q <universe> \n
    ");
}

fn handle_input(src: &mut SacnSource) -> Result <(), Error>{
    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // https://www.tutorialspoint.com/rust/rust_string.htm (03/02/2020)
            let split_input: Vec<&str> = input.split_whitespace().collect();
            if split_input.len() < 2 {
                display_help();
                return Err(Error::new(ErrorKind::InvalidInput, "Insufficient parts ( < 2 )"));
            }

            let universe: u16 = split_input[1].parse().unwrap();

            match split_input[0] {
                "d" => {
                    if split_input.len() < 4 {
                        return Err(Error::new(ErrorKind::InvalidInput, "Insufficient parts for data line ( < 3 )"));
                    }

                    let sync_uni: u16 = split_input[2].parse().unwrap();

                    let priority: u8 = split_input[3].parse().unwrap();

                    let mut data: Vec<u8> = Vec::new();

                    for i in 4 .. split_input.len() {
                        data.push(split_input[i].parse().unwrap());
                    }

                    if sync_uni == 0 {
                        src.send(&[universe], &data, Some(priority), None, None)?;
                    } else {
                        src.send(&[universe], &data, Some(priority), None, Some(sync_uni))?;
                    }
                }
                "u" => {
                    if split_input.len() < 5 {
                        return Err(Error::new(ErrorKind::InvalidInput, "Insufficient parts for data line ( < 3 )"));
                    }

                    let sync_uni: u16 = split_input[2].parse().unwrap();

                    let priority: u8 = split_input[3].parse().unwrap();

                    let dst_ip = split_input[4];

                    let mut data: Vec<u8> = Vec::new();

                    for i in 5 .. split_input.len() {
                        data.push(split_input[i].parse().unwrap());
                    }

                    if sync_uni == 0 {
                        src.send(&[universe], &data, Some(priority), Some(SocketAddr::new(IpAddr::V4(dst_ip.parse().unwrap()), ACN_SDT_MULTICAST_PORT)), None)?;
                    } else {
                        src.send(&[universe], &data, Some(priority), Some(SocketAddr::new(IpAddr::V4(dst_ip.parse().unwrap()), ACN_SDT_MULTICAST_PORT)), Some(sync_uni))?;
                    }
                }
                "r" => {
                    src.register_universe(universe);
                }
                "q" => {
                    if universe == 0 {
                        return Ok(())
                    } else {
                        src.terminate_stream(universe, TERMINATE_START_CODE)?;
                    }
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