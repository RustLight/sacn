#![allow(dead_code)]
#![allow(unused_imports)]

extern crate sacn;
use sacn::recieve::DMXData;
use sacn::recieve::SacnReceiver;
use sacn::packet::ACN_SDT_MULTICAST_PORT;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use std::io::{Error};
use std::env;

/// Demo receiver, this is used as part of the intergration tests across the network.
/// This receiver will receive on the universes given as command line arguments.
/// The receiver will print any received data to act on to std out. 

const USAGE_STR: &'static str = "Usage: ./main <interface_ip> <timeout_secs> <recv_attempts> <universe_1> <universe_2> ...";

fn main() {
    let cmd_args: Vec<String> = env::args().collect();

    if cmd_args.len() < 4 {
        return display_help();
    }

    let interface_ip = &cmd_args[1];

    // https://stackoverflow.com/questions/27043268/convert-a-string-to-int-in-rust (03/02/2020)
    let timeout_secs: u64 = cmd_args[2].parse().unwrap();

    let recv_attempts: usize = cmd_args[3].parse().unwrap();

    let mut universes: Vec<u16> = Vec::new();
    for i in 4 .. cmd_args.len() {
        universes.push(cmd_args[i].parse().unwrap()); // All remaining arguments are universes
    }

    let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(interface_ip.parse().unwrap()), ACN_SDT_MULTICAST_PORT)).unwrap();

    if timeout_secs == 0 {
        match dmx_recv.set_timeout(None) {
            Err(e) => {
                println!("Failed to set timeout: {:?}", e);
                return;
            },
            Ok(_) => {}
        }
        
    } else {
        match dmx_recv.set_timeout(Some(Duration::from_secs(timeout_secs))) {
            Err(e) => {
                println!("Failed to set timeout: {:?}", e);
                return;
            },
            Ok(_) => {}
        }
    }

    if universes.len() > 0 {
        dmx_recv.listen_universes(&universes).unwrap();
    }

    for _ in 0 .. recv_attempts { 
        match dmx_recv.recv(){
            Err(e) => {
                println!("Error Encountered: {:?}", e);
            },
            Ok(d) => {
                println!("{:?}", d);
            }
        }
    }
}

fn display_help(){
    println!("{}", USAGE_STR);
}
