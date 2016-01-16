// Copyright 2016 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Implementation of sACN network protocol.
//!
//! This crate implements the Streaming ACN (sACN) network protocol
//! as specified in ANSI E1.31.
//! sACN is a subset of the ACN protocol (ANSI E1.17).

#![doc(html_root_url = "http://lschmierer.github.io/sacn/")]

extern crate net2;
extern crate uuid;

pub use self::source::DmxSource;

mod source;
mod packet;
