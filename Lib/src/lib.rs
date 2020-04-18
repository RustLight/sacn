// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was modified as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

//! Implementation of sACN network protocol.
//!
//! This crate implements the Streaming ACN (sACN) network protocol
//! as specified in ANSI E1.31-2018.
//! Streaming ACN is built on top of and is compatible with the ACN
//! protocol suite (ANSI E1.17-2015).
//! 
//! This file was modified as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.
//! 
//! # Examples
//! 
//! Creating an sACN receiver and receiving data.
//! ```
//! use sacn::receive::SacnReceiver;
//! use sacn::packet::ACN_SDT_MULTICAST_PORT;
//! 
//! use std::net::{IpAddr, Ipv4Addr, SocketAddr};
//! use std::time::Duration;
//! 
//! const UNIVERSE1: u16 = 1;
//! const TIMEOUT: Option<Duration> = Some(Duration::from_secs(1)); // A timeout of None means blocking behaviour, some indicates the actual timeout.
//! 
//! let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT);
//!
//! let mut dmx_rcv = SacnReceiver::with_ip(addr, None).unwrap();
//!
//! dmx_rcv.listen_universes(&[UNIVERSE1]).unwrap();
//! 
//! match dmx_rcv.recv(TIMEOUT) {
//!     Err(e) => {
//!         // Print out the error.
//!         println!("{:?}", e);
//!     }
//!     Ok(p) => {
//!         // Print out the packet.
//!         println!("{:?}", p);
//!     }
//! }
//! ```
//! 
//! Creating a sACN sender and sending some unsychronised data.
//! 
//! '''
//! 
//! use sacn::source::SacnSource;
//! use sacn::packet::ACN_SDT_MULTICAST_PORT;
//! use std::net::{IpAddr, SocketAddr};
//!
//! let local_addr: SocketAddr = SocketAddr::new(IpAddr::V4("0.0.0.0".parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);
//!
//! let mut src = SacnSource::with_ip("Source", local_addr).unwrap();
//!
//! let universe: u16 = 1;                        // Universe the data is to be sent on.
//! let sync_uni: Option<u16> = None;             // Don't want the packet to be delayed on the receiver awaiting synchronisation.
//! let priority: u8 = 100;                       // The priority for the sending data, must be 1-200 inclusive,  None means use default.
//! let dst_ip: Option<SocketAddr> = None;        // Sending the data using IP multicast so don't have a destination IP.
//!
//! src.register_universe(universe).unwrap(); // Register with the source that will be sending on the given universe.
//!
//! let mut data: Vec<u8> = vec![0, 0, 0, 0, 255, 255, 128, 128]; // Some arbitrary data, must have length <= 513 (including start-code).
//!
//! src.send(&[universe], &data, Some(priority), dst_ip, sync_uni).unwrap(); // Actually send the data
//! 
//! ```
//! 

#![doc(html_root_url = "https://docs.rs/sacn/")]
// #![warn(missing_docs)]
// Recursion limit for error_chain.
#![recursion_limit="1024"]

#[macro_use]
extern crate error_chain;

/// The errors within the sACN crate related to parse/pack errors.
/// Error-chain is used for errors within the library to allow chaining errors together to provide more informative backtraces.
/// This completely replaces the old error system (sACN crate version 0.4.4) which relied on a simple Enum model without proper backtraces.
pub mod sacn_parse_pack_error;

/// The errors used within the sACN crate, parse/pack errors are seperated out into sacn_parse_pack_error.
pub mod error;

/// The library is built on top of socket2 to provide the underlying UDP networking interface.
extern crate socket2;
extern crate libc;

/// The core crate is used for string processing during packet parsing/packing as well as to provide access to the Hash trait.
extern crate core;

/// The byteorder crate is used for marshalling data on/off the network in Network Byte Order.
extern crate byteorder;

/// The uuid crate is used for working with/generating UUIDs which sACN uses as part of the cid field in the protocol.
extern crate uuid;

/// The packet module handles the sACN packets including parsing/packing and sACN related constants.
pub mod packet;

/// The source module handles generation of sACN on the network.
pub mod source;

/// The receive module handles the receiving of sACN on the network.
pub mod receive;
