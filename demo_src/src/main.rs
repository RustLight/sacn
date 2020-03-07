#![allow(dead_code)]
#![allow(unused_imports)]
#![warn(missing_docs)]

extern crate lazy_static;
extern crate sacn;
use sacn::SacnSource;
use std::{time}; // https://doc.rust-lang.org/std/thread/fn.sleep.html (20/09/2019)
use std::time::{Duration, Instant};
use std::io;
use std::io::{Error, ErrorKind};
use sacn::packet::ACN_SDT_MULTICAST_PORT;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::env;
use std::thread::sleep;
use std::str::FromStr;

/// The start code used in terminate packets.
const TERMINATE_START_CODE: u8 = 0;

// Approximately 30 updates per second
const SHAPE_DATA_SEND_PERIOD: Duration = Duration::from_millis(33);

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

const USAGE_STR: &'static str = "Usage ./main <interface_ip> <source_name>\n
    Reads data from stdin and sends it using the protocol. \n
    Data must be formatted as, a sync_universe of 0 means no synchronisation, this uses multicast: \n
    d <universe> <sync_uni> <priority> <data_as_u8_space_seperated> \n
    Sends a full universe of data (512 channels + 0 startcode) with the first bytes of the data as specified 
    below (remainder is 0's) \n
    f <universe> <sync_uni> <priority> <data_as_u8_space_seperate> \n
    To send data unicast use: \n
    u <universe> <sync_uni> <priority> <dst_addr> <data_as_u8_space_seperated> \n
    Register a sending universe as: \n
    r <universe> \n
    Terminate a universe, if universe is 0 then will terminate entirely: \n
    q <universe> \n
    Sleep for x milliseconds \n
    w <milliseconds> \n
    Send a synchronisation packet for the given universe \n
    s <universe> \n
    Send a synchronisation packet for the given universe to the given address \n
    us <universe> <dst_addr> \n
    Start a demo shape which continuously sends data to the given universe for the given number of milliseconds \n
    t <universe> <duration_millis> <priority>\n
    ";

fn main(){
    let cmd_args: Vec<String> = env::args().collect();

    if cmd_args.len() < 3 {
        return display_help();
    }

    let interface_ip = &cmd_args[1];

    let source_name = &cmd_args[2];

    let mut src = SacnSource::with_ip(source_name, SocketAddr::new(IpAddr::V4(interface_ip.parse().unwrap()), ACN_SDT_MULTICAST_PORT)).unwrap();

    println!("Started");

    loop {
        // https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line (03/02/2020)
        match handle_input(&mut src) {
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

fn display_help(){
    println!("{}", USAGE_STR);
}

/// Returns Ok(true) to continue or Ok(false) if no more input.
fn handle_input(src: &mut SacnSource) -> Result <bool, Error>{
    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            if n == 0 {
                // Means EOF is reached so terminate
                return Ok(false)
            }

            // https://www.tutorialspoint.com/rust/rust_string.htm (03/02/2020)
            let split_input: Vec<&str> = input.split_whitespace().collect();
            if split_input.len() < 2 {
                display_help();
                return Err(Error::new(ErrorKind::InvalidInput, "Insufficient parts ( < 2 )"));
            }

            match split_input[0] {
                "d" => {
                    let universe: u16 = split_input[1].parse().unwrap();

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
                "f" => {
                    let universe: u16 = split_input[1].parse().unwrap();

                    if split_input.len() < 4 {
                        return Err(Error::new(ErrorKind::InvalidInput, "Insufficient parts for data line ( < 3 )"));
                    }

                    let sync_uni: u16 = split_input[2].parse().unwrap();

                    let priority: u8 = split_input[3].parse().unwrap();

                    let mut data: [u8; 513] = [0; 513];

                    for i in 4 .. split_input.len() {
                        data[i - 4] = split_input[i].parse().unwrap();
                    }

                    if sync_uni == 0 {
                        src.send(&[universe], &data, Some(priority), None, None)?;
                    } else {
                        src.send(&[universe], &data, Some(priority), None, Some(sync_uni))?;
                    }
                }
                "u" => {
                    let universe: u16 = split_input[1].parse().unwrap();

                    if split_input.len() < 5 {
                        return Err(Error::new(ErrorKind::InvalidInput, "Insufficient parts for data line ( < 5 )"));
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
                "t" => {
                    if split_input.len() < 4 {
                        return Err(Error::new(ErrorKind::InvalidInput, "Insufficient parts for data line ( < 4 )"));
                    }

                    let universe: u16 = split_input[1].parse().unwrap();
                    let duration_millis: u64 = split_input[2].parse().unwrap();
                    let priority: u8 = split_input[3].parse().unwrap();

                    let duration: Duration = Duration::from_millis(duration_millis);

                    let start_time = Instant::now();

                    while start_time.elapsed() < duration {
                        let x: f64 = (start_time.elapsed().as_millis() as f64) / (1000 as f64);
                        let d: u8 = (255.0 * x.sin()) as u8;

                        let mut data: [u8; 513] = [d; 513];
                        data[0] = 0; // Use 0 startcode

                        src.send(&[universe], &data, Some(priority), None, None)?;

                        sleep(SHAPE_DATA_SEND_PERIOD);
                    }
                }
                "s" => {
                    let universe: u16 = split_input[1].parse().unwrap();
                    src.send_sync_packet(universe, &None)?;
                }
                "us" => {
                    if split_input.len() < 3 {
                        return Err(Error::new(ErrorKind::InvalidInput, "Insufficient parts for data line ( < 3 )"));
                    }

                    let universe: u16 = split_input[1].parse().unwrap();
                    let dst_ip = split_input[2];
                    src.send_sync_packet(universe, &Some(SocketAddr::from_str(dst_ip).unwrap()))?;
                }
                "r" => {
                    let universe: u16 = split_input[1].parse().unwrap();
                    src.register_universe(universe);
                }
                "q" => {
                    let universe: u16 = split_input[1].parse().unwrap();
                    if universe == 0 {
                        return Ok(false)
                    } else {
                        src.terminate_stream(universe, TERMINATE_START_CODE)?;
                    }
                }
                "w" => {
                    if split_input.len() < 2 {
                        display_help();
                        return Err(Error::new(ErrorKind::InvalidInput, "Insufficient parts ( < 2 )"));
                    }
                    let millis: u64 = split_input[1].parse().unwrap();
                    sleep(Duration::from_millis(millis));

                }
                x => {
                    return Err(Error::new(ErrorKind::InvalidInput, format!("Unknown input type: {}", x)));
                }
            }
            Ok(true)
        }
        Err(e) => {
            return Err(e);
        }
    }
}