// Copyright 2018 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Implementation of sACN network protocol.
//!
//! This crate implements the Streaming ACN (sACN) network protocol
//! as specified in ANSI E1.31-2016.
//! Streaming ACN is built on top of and is compatible with the ACN
//! protocol suite (ANSI E1.17-2015).

#![cfg_attr(not(feature = "std"), no_std)]
#![doc(html_root_url = "https://docs.rs/sacn/")]

#[cfg(feature = "std")]
extern crate core;

extern crate byteorder;
#[cfg(not(feature = "std"))]
extern crate heapless;
#[cfg(feature = "std")]
extern crate net2;
extern crate uuid;

#[cfg(feature = "std")]
pub use self::source::SacnSource;

pub mod error;
pub mod packet;
#[cfg(feature = "std")]
mod source;

extern crate lazy_static;
extern crate socket2;
pub mod recieve;
