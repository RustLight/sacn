// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was created as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

#![warn(missing_docs)]

/// The errors used within the SacnLibrary.
/// 
/// Uses the error-chain crate to allow errors to allow more informative backtraces through error chaining.
/// https://docs.rs/error-chain/0.12.2/error_chain/
pub mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);       // Allow IO errors to be used with the error-chain system.
            Str(::std::str::Utf8Error); // Allow standard string library errors to be used with the error-chain system.
            Uuid(uuid::ParseError);     // Allow UUID library to be used with error-chain system.
        }

        errors {
            
            /// Used to indicate that the limit for the number of supported sources has been reached. 
            /// This is based on unique CID values.
            /// as per ANSI E1.31-2018 Section 6.2.3.3.
            SourcesExceededError(msg: String) {
                description("Limit for the number of supported sources has been reached"),
                display("Limit for the number of supported sources has been reached, msg: {}", msg)
            }

            /// A source was discovered by a receiver with the announce_discovery_flag set to true.
            SourceDiscovered(msg: String) {
                description("A source was discovered by a receiver with the announce_discovery_flag set to true"),
                display("A source was discovered by a receiver with the announce_discovery_flag set to true, msg: {}", msg)
            }

            /// Attempted to exceed the capacity of a single universe (packet::UNIVERSE_CHANNEL_CAPACITY).
            ExceedUniverseCapacity(msg: String) {
                description("Attempted to exceed the capacity of a single universe"),
                display("Attempted to exceed the capacity of a single universe, msg: {}", msg)
            }

            /// Attempted to use illegal universe, outwith allowed range of [E131_MIN_MULTICAST_UNIVERSE, E131_MAX_MULTICAST_UNIVERSE] 
            /// + E131_DISCOVERY_UNIVERSE inclusive
            IllegalUniverse(msg: String) {
                description("Attempted to use illegal universe, outwith allowed range of [E131_MIN_MULTICAST_UNIVERSE 
                - E131_MAX_MULTICAST_UNIVERSE] + E131_DISCOVERY_UNIVERSE inclusive"),
                display("illegal universe used, outwith allowed range, msg: {}", msg)
            }

            /// Attempted to use a universe that wasn't first registered for use.
            /// To send from a universe with a sender it must first be registered. This allows universe discovery adverts to include the universe.
            UniverseNotRegistered(msg: String) {
                description("Attempted to use a universe that wasn't first registered for use"),
                display("Attempted to use a universe that wasn't first registered for use, msg: {}", msg)
            }

            /// Ip version (ipv4 or ipv6) used when the other is expected
            IpVersionError(msg: String) {
                description("Ip version (ipv4 or ipv6) used when the other is expected"),
                display("Ip version (ipv4 or ipv6) used when the other is expected, msg: {}", msg)
            }

            /// Attempted to use an unsupported (not Ipv4 or Ipv6) IP version.
            UnsupportedIpVersion(msg: String) {
                description("Attempted to use an unsupported (not Ipv4 or Ipv6) IP version"),
                display("Attempted to use an unsupported (not Ipv4 or Ipv6) IP version, msg: {}", msg)
            }

            /// Attempted to use a sender which has already been terminated.
            SenderAlreadyTerminated(msg: String) {
                description("Attempted to use a sender which has already been terminated"),
                display("Attempted to use a sender which has already been terminated, msg: {}", msg)
            }

            /// An error was encountered when attempting to merge DMX data together.
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
            PduInvalidLength(len: usize) {
                description("Received PDU length is invalid"),
                display("PDU Length {} is invalid", len)
            }

            /// Received PDU vector is invalid/unsupported by this library.
            PduInvalidVector(vec: u32) {
                description("Received PDU vector is invalid/unsupported by this library"),
                display("Vector {:#x} not supported", vec)
            }

            /// Error parsing the received UUID.
            UuidError(msg: String) {
                description("Error parsing the received UUID"),
                display("Error parsing the received UUID, msg: {}", msg)
            }

            /// Error parsing received UTF8 string.
            Utf8Error(msg: String) {
                description("Error parsing received UTF8 string"),
                display("Error parsing received UTF8 string, msg: {}", msg)
            }

            /// Packet was received out of sequence and so should be discarded.
            OutOfSequence(msg: String) {
                description("Packet was received out of sequence and so should be discarded"),
                display("Packet was received out of sequence and so should be discarded, msg: {}", msg)
            }

            /// A source terminated a universe and this was detected when trying to receive data.
            UniverseTerminated(msg: String) {
                description("A source terminated a universe and this was detected when trying to receive data"),
                display("A source terminated a universe and this was detected when trying to receive data, msg: {}", msg)
            }

            /// When looking for a specific universe it wasn't found. This might happen for example if trying to mute a universe on a receiver that
            /// wasn't being listened.
            UniverseNotFound(msg: String) {
                description("When looking for a specific universe it wasn't found"),
                display("When looking for a specific universe it wasn't found, msg: {}", msg)
            }
        }
    }
}
