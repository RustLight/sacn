// Copyright 2018 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Errors used through the sacn crate.

#![warn(missing_docs)]

use core::fmt;
use core::str::Utf8Error;

#[cfg(feature = "std")]
use std::error;

use uuid;

// https://github.com/rust-lang-nursery/error-chain/issues/112 (08/03/2020)
/// The errors used within the SacnLibrary.
/// 
/// Uses the error-chain crate to allow errors to allow more informative backtraces through error chaining.
/// https://docs.rs/error-chain/0.12.2/error_chain/
pub mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error); // Allow IO errors to be used with the error-chain system.
            Parse(::error::ParseError); // Allow the existing ParseError's to be used with the error-chain system.
        }

        errors {
            IllegalUniverse(msg: String) {
                description("illegal universe used, outwith allowed range of [E131_MIN_MULTICAST_UNIVERSE 
                - E131_MAX_MULTICAST_UNIVERSE] + E131_DISCOVERY_UNIVERSE inclusive"),
                display("illegal universe used, outwith allowed range, msg: {}", msg)
            }

            IpVersionError(msg: String) {
                description("Ip version (ipv4 or ipv6) used when the other is expected"),
                display("Ip version (ipv4 or ipv6) used when the other is expected, msg: {}", msg)
            }

            DmxMergeError(msg: String) {
                description("Error when merging DMX data"),
                display("Error when merging DMX data, msg: {}", msg)
            }

            /// When parsing packet invalid data encountered.
            ParseInvalidData(msg: String) {
                description("Data provided to parse into a packet is invalid"),
                display("Error when parsing data into packet, msg: {}", msg)
            }

            /// When packing a packet into a buffer invalid data encountered.
            PackInvalidData(msg: String) {
                description("When packing a packet into a buffer invalid data encountered"),
                display("When packing a packet into a buffer invalid data encountered, msg: {}", msg)
            }

            /// Supplied buffer is not large enough to pack packet into.
            PackBufferInsufficient(msg: String) {
                description("Supplied buffer is not large enough to pack packet into"),
                display("Supplied buffer is not large enough to pack packet into, msg: {}", msg)
            }

            /// Supplied buffer does not contain enough data.
            ParseInsufficientData(msg: String) {
                description("Supplied buffer does not contain enough data"),
                display("Supplied buffer does not contain enough data, msg: {}", msg)
            }

            /// Received PDU flags are invalid for parsing.
            ParsePduInvalidFlags(flags: u8) {
                description("Received PDU flags are invalid"),
                display("PDU Flags {:#b} are invalid for parsing", flags)
            }

            /// Received PDU length is invalid.
            PduInvalidLength(length: usize) {
                description("Received PDU length is invalid"),
                display("PDU Length {} is invalid", len)
            }

            /// Received PDU vector is invalid/unsupported by this library.
            PduInvalidVector(vec: u32) {
                description("Received PDU vector is invalid/unsupported by this library"),
                display("Vector {:#x} not supported", vec)
            }

            /// Error parsing the received UUID.
            UuidError() {
                description("Error parsing the received UUID"),
                display("Error parsing the received UUID")
            }

            /// Error parsing received UTF8 string.
            Utf8Error() {
                description("Error parsing received UTF8 string"),
                display("Error parsing received UTF8 string")
            }

        }
    }
}
