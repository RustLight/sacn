// #![warn(missing_docs)]
#![recursion_limit="1024"] // Recursion limit for error-chain.

// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was created as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

//! An example demo sACN source which utilises the sACN library.
//! 
//! Primarily used for testing the library including real-world conformance, compliance, integration and acceptance tests.
//! 
//! Usage instructions are described below.
//! 

#[macro_use]
extern crate error_chain;

/// Import the error-chain handling into the module.
pub mod error;
use error::errors::*;

extern crate sacn;

use sacn::source::SacnSource;
use sacn::packet::ACN_SDT_MULTICAST_PORT;

use std::time::{Duration, Instant};
use std::io;
use std::net::{IpAddr, SocketAddr};
use std::env;
use std::thread::sleep;
use std::str::FromStr;

/// The start code used in termination packets.
const TERMINATE_START_CODE: u8 = 0;

/// The period between updates to the values send during the shape generation command.
/// Default value is approximately 30 updates per second choosen fairly arbitarily to be less than the DMX refresh rate (44 fps).
const SHAPE_DATA_SEND_PERIOD: Duration = Duration::from_millis(33);

/// The string given by the user to perform each of the various options as described in get_usage_str below.
const ACTION_PREVIEW_OPTION:        &str = "p";
const ACTION_DATA_OPTION:           &str = "d";
const ACTION_FULL_DATA_OPTION:      &str = "f";
const ACTION_UNICAST_OPTION:        &str = "u"; 
const ACTION_REGISTER_OPTION:       &str = "r"; 
const ACTION_TERMINATE_OPTION:      &str = "q";
const ACTION_SLEEP_OPTION:          &str = "w"; 
const ACTION_SYNC_OPTION:           &str = "s";
const ACTION_UNICAST_SYNC_OPTION:   &str = "us"; 
const ACTION_DATA_OVER_TIME_OPTION: &str = "t";

/// Describes the various commands / command-line arguments avaliable and what they do.
/// Displayed to the user if they ask for help or enter an unrecognised input.
/// Not a const as const with format! not supported in rust.
fn get_usage_str() -> String{
    format!("Usage ./main <interface_ip> <source_name>\n
    
    Reads data from stdin and sends it using the protocol. \n
    Data must be formatted as, a sync_universe of 0 means no synchronisation, this uses multicast: \n
    {} <universe> <sync_uni> <priority> <data_as_u8_space_seperated> \n
    
    Sends a full universe of data (512 channels + 0 startcode) with the first bytes of the data as specified 
    below (remainder is 0's) \n
    {} <universe> <sync_uni> <priority> <data_as_u8_space_seperate> \n
    
    To send data unicast use: \n
    {} <universe> <sync_uni> <priority> <dst_addr> <data_as_u8_space_seperated> \n
    
    Register a sending universe as: \n
    {} <universe> \n
    
    Terminate a universe, if universe is 0 then will terminate entirely: \n
    {} <universe> \n
    
    Sleep for x milliseconds \n
    {} <milliseconds> \n
    
    Send a synchronisation packet for the given universe \n
    {} <universe> \n
    
    Send a synchronisation packet for the given universe to the given address \n
    {} <universe> <dst_addr> \n
    
    Start a demo shape which continuously sends data to the given universe for the given number of milliseconds \n
    {} <universe> <duration_millis> <priority>\n

    Set the preview data option flag to the given value, this is reflected in packets sent using the other actions\n
    {} <'true'/'false'>\n
    ", ACTION_DATA_OPTION, ACTION_FULL_DATA_OPTION, ACTION_UNICAST_OPTION, ACTION_REGISTER_OPTION, ACTION_TERMINATE_OPTION, 
    ACTION_SLEEP_OPTION, ACTION_SYNC_OPTION, ACTION_UNICAST_SYNC_OPTION, ACTION_DATA_OVER_TIME_OPTION, ACTION_PREVIEW_OPTION)
}

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
    println!("{}", get_usage_str());
}

fn handle_full_data_option(src: &mut SacnSource, split_input: Vec<&str>) -> Result<bool> {
    let universe: u16 = split_input[1].parse().unwrap();

    if split_input.len() < 4 {
        bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts for data line ( < 3 )"));
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

    Ok(true)
}

fn handle_data_option(src: &mut SacnSource, split_input: Vec<&str>) -> Result<bool> {
    let universe: u16 = split_input[1].parse().unwrap();

    if split_input.len() < 4 {
        bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts for data line ( < 3 )"));
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

    Ok(true)
}

fn handle_unicast_option(src: &mut SacnSource, split_input: Vec<&str>) -> Result<bool> {
    let universe: u16 = split_input[1].parse().unwrap();

    if split_input.len() < 5 {
        bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts for data line ( < 5 )"));
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

    Ok(true)
}

fn handle_data_over_time_option(src: &mut SacnSource, split_input: Vec<&str>) -> Result<bool> {
    if split_input.len() < 4 {
        bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts for data line ( < 4 )"));
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

    Ok(true)
}

/// Returns Ok(true) to continue or Ok(false) if no more input.
fn handle_input(src: &mut SacnSource) -> Result <bool>{
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
                bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts ( < 2 )"));
            }

            match split_input[0] {
                ACTION_DATA_OPTION => {
                    handle_data_option(src, split_input)
                }
                ACTION_FULL_DATA_OPTION => {
                    handle_full_data_option(src, split_input)
                }
                ACTION_UNICAST_OPTION => {
                    handle_unicast_option(src, split_input)
                }
                ACTION_DATA_OVER_TIME_OPTION => {
                    handle_data_over_time_option(src, split_input)
                }
                ACTION_SYNC_OPTION => {
                    let universe: u16 = split_input[1].parse().unwrap();
                    src.send_sync_packet(universe, &None)?;
                    Ok(true)
                }
                ACTION_UNICAST_SYNC_OPTION => {
                    if split_input.len() < 3 {
                        bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts for data line ( < 3 )"));
                    }

                    let universe: u16 = split_input[1].parse().unwrap();
                    let dst_ip = split_input[2];
                    src.send_sync_packet(universe, &Some(SocketAddr::from_str(dst_ip).unwrap()))?;
                    Ok(true)
                }
                ACTION_REGISTER_OPTION => {
                    let universe: u16 = split_input[1].parse().unwrap();
                    src.register_universe(universe)?;
                    Ok(true)
                }
                ACTION_PREVIEW_OPTION => {
                    let val = split_input[1].parse();

                    match val {
                        Ok(v) => {
                            src.set_preview_mode(v);
                        },
                        Err(_e) => {
                            bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Preview flag option not 'true'/'false' or otherwise parsable as boolean"));
                        }
                    }
                    Ok(true)
                }
                ACTION_TERMINATE_OPTION => {
                    let universe: u16 = split_input[1].parse().unwrap();
                    if universe == 0 {
                        return Ok(false)
                    } else {
                        src.terminate_stream(universe, TERMINATE_START_CODE)?;
                    }
                    Ok(true)
                }
                ACTION_SLEEP_OPTION => {
                    if split_input.len() < 2 {
                        display_help();
                        bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts ( < 2 )"));
                    }
                    let millis: u64 = split_input[1].parse().unwrap();
                    sleep(Duration::from_millis(millis));
                    Ok(true)
                }
                x => {
                    bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Unknown input type: {}", x)));
                }
            }
        }
        Err(e) => {
            bail!(e);
        }
    }
}