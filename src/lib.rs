#![doc(html_root_url = "http://lschmierer.github.io/sacn/")]
//! Implementation of sACN network protocol.
//!
//! This crate implements the Streaming ACN (sACN) network protocol
//! as specified in ANSI E1.31.
//! sACN is a subset of the ACN protocol (ANSI E1.17).

extern crate net2;
extern crate uuid;

pub use self::source::DmxSource;

mod source;
mod packet;
