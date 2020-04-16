// #![warn(missing_docs)]      // Used during development to warn about a lack of documentation.
#![recursion_limit="1024"]  // Recursion limit for error-chain, value used as recommended by the crates documentation.

// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was created as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

//! An example demo sACN receiver which utilises the sACN library.
//! 
//! Primarily used for testing the library including real-world conformance, compliance, integration and acceptance tests.
//! As a test program the error handling is limited for simplicity.
//! 
//! Usage instructions are described below.
//! 

#[macro_use]
extern crate error_chain;

/// Import the error-chain handling into the module.
pub mod error;
use error::errors::*;

extern crate sacn;

use sacn::receive::{DMXData, SacnReceiver, DiscoveredSacnSource};
use sacn::packet::ACN_SDT_MULTICAST_PORT;

use std::net::{SocketAddr};
use std::time::Duration;
use std::io;
use std::env;
use std::thread::sleep;
use std::fs::File;
use std::io::prelude::*;

/// The string given by the user to perform each of the various options as described in get_usage_str below.
const ACTION_RECV:                                  &str = "r";
const ACTION_RECV_CONTINUOUS:                       &str = "c";
const ACTION_PRINT_DISCOVERED_SOURCES:              &str = "s";
const ACTION_PRINT_DISCOVERED_SOURCES_NO_TIMEOUT:   &str = "x";
const ACTION_QUIT:                                  &str = "q";
const ACTION_HELP:                                  &str = "h";
const ACTION_LISTEN_UNIVERSE:                       &str = "l";
const ACTION_STOP_LISTEN_UNIVERSE:                  &str = "t";
const ACTION_SLEEP:                                 &str = "w";
const ACTION_PREVIEW:                               &str = "p";
const ACTION_ANNOUNCE_DISCOVERED:                   &str = "a";
const ACTION_IGNORE:                                &str = "#";
const ACTION_FILE_OUT:                              &str = "f";
const ACTION_ANNOUNCE_TERMINATION:                  &str = "e"; // e for end.

/// The headers used for the top of the file when the FILE_OUT action is used.
const WRITE_TO_FILE_HEADERS: &str = "Data_ID, Universe, Sync_Addr, Priority, Preview_data?, Payload";

/// Describes the various commands / command-line arguments available and what they do.
/// Displayed to the user if they ask for help or enter an unrecognised input.
/// Not a const as const with format! not supported in rust.
fn get_usage_str() -> String {
    format!("Usage: ./main <interface_ip>\n
    Receive data: \n
    {} <timeout in secs, 0 means no timeout>\n

    Attempt to receive data with the given timeout for each receive for the given number of times: \n
    {} <timeout in secs> <count> \n

    Print discovered sources: \n
    {} \n

    Print discovered sources without checking if they are timed out: \n
    {} \n

    Quit \n
    {} \n

    Help \n
    {} \n

    Listen universe \n
    {} <universe> \n

    Stop Listening Universe \n
    {} <universe> \n

    Sleep for x milliseconds \n
    {} <milliseconds>\n

    Enter preview mode, true means preview data will be received, false means preview data is ignored, default is false\n
    {} <'true'/'false'>\n

    Enter announce discovery mode, true means that universe discovery packets will be announced as soon as received, false means they are handled silently, default is false\n
    {} <'true'/'false'>\n

    Enter announce termination mode, true means that termination packets will be announced during a recv() attempt. False means they are handled silently, default is false\n
    {} <'true'/'false'>\n

    Output received data to a file
    {} <file-path> <recv-count> <timeout in sec>\n

    All input is ignored on lines starting with '{} '.
    ", ACTION_RECV, ACTION_RECV_CONTINUOUS, ACTION_PRINT_DISCOVERED_SOURCES, ACTION_PRINT_DISCOVERED_SOURCES_NO_TIMEOUT, 
    ACTION_QUIT, ACTION_HELP, ACTION_LISTEN_UNIVERSE, ACTION_STOP_LISTEN_UNIVERSE, ACTION_SLEEP, ACTION_PREVIEW, ACTION_ANNOUNCE_DISCOVERED,
    ACTION_ANNOUNCE_TERMINATION, ACTION_FILE_OUT, ACTION_IGNORE)
}

fn main() {
    let cmd_args: Vec<String> = env::args().collect();

    if cmd_args.len() < 2 {
        return display_help();
    }

    let interface_ip = &cmd_args[1];

    let source_limit = None;

    let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(interface_ip.parse().unwrap(), ACN_SDT_MULTICAST_PORT), source_limit).unwrap();

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
                ACTION_IGNORE => {
                    // Ignore the input
                }
                ACTION_HELP => { // Display help
                    display_help();
                }
                ACTION_RECV => { // Receive data
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
                ACTION_RECV_CONTINUOUS => { // Receive data continuously.
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
                ACTION_PRINT_DISCOVERED_SOURCES => { // Print discovered sources, note that no sources will be discovered unless you try and recv first.
                    print_discovered_sources(&dmx_recv.get_discovered_sources());
                }
                ACTION_PRINT_DISCOVERED_SOURCES_NO_TIMEOUT => { // Print discovered sources without checking if they are timed out already.
                    print_discovered_sources(&dmx_recv.get_discovered_sources_no_check());
                }
                ACTION_QUIT => {
                    return Ok(false)
                }
                ACTION_SLEEP => {
                    if split_input.len() < 2 {
                        display_help();
                        bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts ( < 2 )"));
                    }
                    let millisecs: u64 = split_input[1].parse().unwrap();
                    sleep(Duration::from_millis(millisecs));
                }
                ACTION_LISTEN_UNIVERSE => {
                    if split_input.len() < 2 {
                        display_help();
                        bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts ( < 2 )"));
                    }
                    let universe: u16 = split_input[1].parse().unwrap();
                    dmx_recv.listen_universes(&[universe])?;
                }
                ACTION_STOP_LISTEN_UNIVERSE => {
                    if split_input.len() < 2 {
                        display_help();
                        bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts ( < 2 )"));
                    }
                    let universe: u16 = split_input[1].parse().unwrap();

                    dmx_recv.mute_universe(universe)?;
                }
                ACTION_PREVIEW => {
                    let val = split_input[1].parse();
                    match val {
                        Ok(v) => {
                            dmx_recv.set_process_preview_data(v);
                        },
                        Err(_e) => {
                            bail!(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput, "Preview flag option not 'true'/'false' or otherwise parsable as boolean"));
                        }
                    }
                }
                ACTION_ANNOUNCE_DISCOVERED => {
                    let val = split_input[1].parse();
                    match val {
                        Ok(v) => {
                            dmx_recv.set_announce_source_discovery(v);
                        },
                        Err(_e) => {
                            bail!(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput, "Announce discovery option not 'true'/'false' or otherwise parsable as boolean"));
                        }
                    }
                }
                ACTION_ANNOUNCE_TERMINATION => {
                    let val = split_input[1].parse();
                    match val {
                        Ok(v) => {
                            dmx_recv.set_announce_stream_termination(v);
                        },
                        Err(_e) => {
                            bail!(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput, "Announce stream termination option not 'true'/'false' or otherwise parsable as boolean"));
                        }
                    }
                }
                ACTION_FILE_OUT => {
                    if split_input.len() < 4 {
                        display_help();
                        bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts ( < 3 )"));
                    }

                    let file_path = split_input[1];

                    let count: u64 = split_input[2].parse().unwrap();

                    let timeout_secs: u64 = split_input[3].parse().unwrap();

                    let timeout = if timeout_secs == 0 { // A timeout value of 0 means no timeout.
                        None
                    } else {
                        Some(Duration::from_secs(timeout_secs))
                    };

                    let out_file = File::create(file_path)?;

                    let mut boxed_file = Box::new(out_file);

                    write!(boxed_file, "{}\n", WRITE_TO_FILE_HEADERS)?;

                    for i in 0 .. count {
                        let res: Vec<DMXData> = dmx_recv.recv(timeout).unwrap();
                        write_to_file(&mut boxed_file, res, i)?;
                    }
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

/// Writes the given data to the given file (uses the given data_id as first column).
/// Uses comma separated values.
fn write_to_file(file: &mut Box<File>, data: Vec<DMXData>, data_id: u64) -> Result<()> {
    for d in data {
        let values_str = create_values_str(d.values)?;

        // Note that the formatting string literal must be here and cannot be subsituted using const.
        write!(*file, "{},{},{},{},{},{}\n", data_id, d.universe, d.sync_uni, d.priority, d.preview, values_str)?;
    }

    Ok(())
}

/// Converts the given array of u8 values into a comma separated string.
/// https://users.rust-lang.org/t/what-is-right-ways-to-concat-strings/3780/4 (09/04/2020)
fn create_values_str(values: Vec<u8>) -> Result<String> {
    let mut res: String = "".to_string();

    if values.len() < 1 {
        return Ok(res);
    }

    let mut iter = values.iter();

    res.push_str(&format!("{}", iter.next().unwrap()));
    
    for v in iter {
        res.push_str(&format!(",{}", v));
    }

    Ok(res)
}

fn print_recv(res: Result<Vec<DMXData>>) {
    match res {
        Err(e) => {
            match e.kind() {
                ErrorKind::Sacn(x) => {
                    match x.kind() {
                        sacn::error::errors::ErrorKind::UniverseTerminated(_src_cid, uni) => {
                            println!("Universe {} Terminated", uni);
                        }
                        z => {
                            println!("Error Encountered: {:?}", z);
                        }
                    }
                },
                x => {
                    println!("Error Encountered: {:?}", x);
                }
            }
        },
        Ok(d) => {
            print_data(d);
        }
    }
}

fn print_data(mut data: Vec<DMXData>) {
    print!("[");
    // Sort the data with lower universes first, this means that even though the data returned from the waiting data can be in any order this means 
    // that the ordering will be known which makes checking the output using a test script easier.
    data.sort(); 
    for d in data {
        print!("{{ Universe(s): {}, Sync_Universe: {}, Values: {:?} }}, ", d.universe, d.sync_uni, d.values);
    }
    println!("]");
}

fn print_discovered_sources(srcs: &Vec<DiscoveredSacnSource>) {
    for s in srcs {
        println!("Name: {}, Universes: {:?}", s.name, s.get_all_universes());
    }
}

fn display_help(){
    println!("{}", get_usage_str());
}
