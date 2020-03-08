#![allow(dead_code)]
#![allow(unused_imports)]
#![warn(missing_docs)]
#![recursion_limit="1024"] // Recursion limit for error-chain.

#[macro_use]
extern crate error_chain;
pub mod error;

extern crate sacn;

use sacn::recieve::{DMXData, SacnReceiver, DiscoveredSacnSource};
use sacn::packet::ACN_SDT_MULTICAST_PORT;

use error::errors::*;
use error::errors::ErrorKind::*;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use std::io;
use std::env;
use std::thread::sleep;

/// Demo receiver, this is used as part of the intergration tests across the network.
/// This receiver will receive on the universes given as command line arguments.
/// The receiver will print any received data to act on to std out. 

const USAGE_STR: &'static str = "Usage: ./main <interface_ip>\n
Receive data: \n
r <timeout in secs, 0 means no timeout>\n

Attempt to receive data with the given timeout for each receive for the given number of times: \n
a <timeout in secs> <count> \n

Print discovered sources: \n
s \n

Print discovered sources without checking if they are timed out: \n
x \n

Quit \n
q \n

Help \n
h \n

Listen universe \n
l <universe> \n

Stop Listening Universe \n
t <universe> \n

Sleep for x seconds \n
w <secs>\n
";

fn main() {
    let cmd_args: Vec<String> = env::args().collect();

    if cmd_args.len() < 2 {
        return display_help();
    }

    let interface_ip = &cmd_args[1];

    let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(interface_ip.parse().unwrap()), ACN_SDT_MULTICAST_PORT)).unwrap();

    println!("Started");

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

/// Handle a line of input on stdin to the program.
/// Returns true if there is more input expected and false if not.
fn handle_input(dmx_recv: &mut SacnReceiver) -> Result<bool> {
    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            if n == 0 {
                // Means EOF is reached so terminate
                return Ok(false);
            }

            // https://www.tutorialspoint.com/rust/rust_string.htm (03/02/2020)
            let split_input: Vec<&str> = input.split_whitespace().collect();

            if split_input.len() < 1 {
                display_help();
                return Ok(true);
            }

            match split_input[0] {
                "h" => { // Display help
                    display_help();
                }
                "r" => { // Receive data
                    if split_input.len() < 2 {
                        display_help();
                        bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts ( < 2 )"));
                    }

                    // https://stackoverflow.com/questions/27043268/convert-a-string-to-int-in-rust (03/02/2020)
                    let timeout_secs: u64 = split_input[1].parse().unwrap();

                    let timeout = if timeout_secs == 0 { // A timeout value of 0 means no timeout.
                        None
                    } else {
                        Some(Duration::from_secs(timeout_secs))
                    };

                    // https://docs.rs/error-chain/0.12.2/error_chain/ (08/03/2020)
                    let res = dmx_recv.recv(timeout).map_err(|e| e.into());
                    print_recv(res);
                }
                "a" => { // Receive data continously.
                    if split_input.len() < 3 {
                        display_help();
                        bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts ( < 3 )"));
                    }

                     // https://stackoverflow.com/questions/27043268/convert-a-string-to-int-in-rust (03/02/2020)
                     let timeout_secs: u64 = split_input[1].parse().unwrap();

                     let count: u64 = split_input[2].parse().unwrap();

                     let timeout = if timeout_secs == 0 { // A timeout value of 0 means no timeout.
                         None
                     } else {
                         Some(Duration::from_secs(timeout_secs))
                     };

                    for _ in 0 .. count {
                        let res = dmx_recv.recv(timeout).map_err(|e| e.into());
                        print_recv(res);
                    }
                }
                "s" => { // Print discovered sources, note that no sources will be discovered unless you try and recv first.
                    print_discovered_sources(&dmx_recv.get_discovered_sources());
                }
                "x" => { // Print discovered sources without checking if they are timed out already.
                    print_discovered_sources(&dmx_recv.get_discovered_sources_no_check());
                }
                "q" => { // Quit
                    return Ok(false)
                }
                "w" => {
                    if split_input.len() < 2 {
                        display_help();
                        bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts ( < 2 )"));
                    }
                    let secs: u64 = split_input[1].parse().unwrap();
                    sleep(Duration::from_secs(secs));

                }
                "l" => { // Listen universe
                    if split_input.len() < 2 {
                        display_help();
                        bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts ( < 2 )"));
                    }
                    let universe: u16 = split_input[1].parse().unwrap();
                    dmx_recv.listen_universes(&[universe])?;
                }
                "t" => { // Stop listening to universe
                    if split_input.len() < 2 {
                        display_help();
                        bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts ( < 2 )"));
                    }
                    // TODO
                    bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not Impl"));
                }
                x => {
                    bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Unknown input type: {}", x)));
                }
            }
            Ok(true)
        }
        Err(e) => {
            bail!(e);
        }
    }
}

fn print_recv(res: Result<Vec<DMXData>>) {
    match res {
        Err(e) => {
            println!("Error Encountered: {:?}", e);
        },
        Ok(d) => {
            print_data(d);
        }
    }
}

fn print_data(data: Vec<DMXData>) {
    print!("[");
    for d in data {
        print!("{{ {}, {}, {:?} }}, ", d.universe, d.sync_uni, d.values);
    }
    println!("]");
}

fn print_discovered_sources(srcs: &Vec<DiscoveredSacnSource>) {
    for s in srcs {
        println!("Name: {}, Universes: {:?}", s.name, s.get_all_universes());
    }
}

fn display_help(){
    println!("{}", USAGE_STR);
}
