#![allow(dead_code)]
#![allow(unused_imports)]

extern crate sacn;
use sacn::recieve::DMXData;
use sacn::recieve::SacnReceiver;
use sacn::packet::ACN_SDT_MULTICAST_PORT;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use std::io;
use std::io::{Error, ErrorKind};
use std::env;

/// Demo receiver, this is used as part of the intergration tests across the network.
/// This receiver will receive on the universes given as command line arguments.
/// The receiver will print any received data to act on to std out. 

const USAGE_STR: &'static str = "Usage: ./main <interface_ip>\n
Receive data: \n
r <timeout, 0 means no timeout>\n

Print discovered sources: \n
s \n

Quit \n
q \n

Help \n
h \n

Listen universe \n
l <universe> \n

Stop Listening Universe \n
t <universe> \n
";

fn main() {
    let cmd_args: Vec<String> = env::args().collect();

    if cmd_args.len() < 2 {
        return display_help();
    }

    let interface_ip = &cmd_args[1];

    let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(interface_ip.parse().unwrap()), ACN_SDT_MULTICAST_PORT)).unwrap();

    loop {
        // https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line (03/02/2020)
        match handle_input(&mut dmx_recv) {
            Ok(should_continue) => {
                if !should_continue {
                    break;
                }
            }
            Err(e) => {
                println!("Error: Input data line unusable: {}", e);
            }
        }
    } 
}

fn handle_input(dmx_recv: &mut SacnReceiver) -> Result<bool, Error> {
    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            if n == 0 {
                // Means EOF is reached so terminate
                return Ok(false)
            }

            // https://www.tutorialspoint.com/rust/rust_string.htm (03/02/2020)
            let split_input: Vec<&str> = input.split_whitespace().collect();

            if split_input.len() < 1 {
                display_help();
                return Ok(());
            }

            match split_input[0] {
                "h" => { // Display help
                    display_help();
                }
                "r" => { // Receive data
                    if split_input.len() < 2 {
                        display_help();
                        return Err(Error::new(ErrorKind::InvalidInput, "Insufficient parts ( < 2 )"));
                    }

                    // https://stackoverflow.com/questions/27043268/convert-a-string-to-int-in-rust (03/02/2020)
                    let timeout_secs: u64 = split_input[1].parse().unwrap();

                    let timeout = if timeout_secs == 0 { // A timeout value of 0 means no timeout.
                        None
                    } else {
                        Some(Duration::from_secs(timeout_secs))
                    };

                    dmx_recv.set_timeout(timeout).expect("Failed to set timeout");

                    match dmx_recv.recv(){
                        Err(e) => {
                            println!("Error Encountered: {:?}", e);
                        },
                        Ok(d) => {
                            println!("{:?}", d);
                        }
                    }
                }
                "s" => { // Print discovered sources, note that no sources will be discovered unless you try and recv first.
                    print_discovered_sources(dmx_recv);
                }
                "q" => { // Quit
                    // TODO
                    return Err(Error::new(ErrorKind::InvalidInput, "Not Impl"));
                }
                "l" => { // Listen universe
                    if split_input.len() < 2 {
                        display_help();
                        return Err(Error::new(ErrorKind::InvalidInput, "Insufficient parts ( < 2 )"));
                    }
                    let universe: u16 = split_input[1].parse().unwrap();
                    dmx_recv.listen_universes(&[universe]);
                }
                "t" => { // Stop listening to universe
                    if split_input.len() < 2 {
                        display_help();
                        return Err(Error::new(ErrorKind::InvalidInput, "Insufficient parts ( < 2 )"));
                    }
                    // TODO
                    return Err(Error::new(ErrorKind::InvalidInput, "Not Impl"));
                }
                x => {
                    return Err(Error::new(ErrorKind::InvalidInput, format!("Unknown input type: {}", x)));
                }
            }
            Ok(true
        }
        Err(e) => {
            return Err(e);
        }
    }
}

fn print_discovered_sources(dmx_recv: &mut SacnReceiver) {
    println!("{:#?}", dmx_recv.get_discovered_sources());
}

fn display_help(){
    println!("{}", USAGE_STR);
}
