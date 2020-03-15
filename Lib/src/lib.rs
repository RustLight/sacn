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

#![doc(html_root_url = "https://docs.rs/sacn/")]
// #![warn(missing_docs)]
// Recursion limit for error_chain.
#![recursion_limit="1024"]

#[macro_use]
extern crate error_chain;
/// Error-chain is used for errors within the library to allow chaining errors together to provide more informative backtraces.
/// This completely replaces the old error system (sACN crate version 0.4.4) which relied on a simple Enum model without proper backtraces.
pub mod error;

/// The library is built on top of socket2 to provide the underlying UDP networking interface.
extern crate socket2;
extern crate libc;
extern crate net2;

/// The core crate is used for string processing during packet parsing/packing aswell as to provide access to the Hash trait.
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
pub mod recieve;
