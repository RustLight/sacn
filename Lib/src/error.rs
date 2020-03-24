// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was created as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

/// The errors used within the SacnLibrary.
/// 
/// Uses the error-chain crate to allow errors to allow more informative backtraces through error chaining.
/// https://docs.rs/error-chain/0.12.2/error_chain/
pub mod errors {
    use sacn_parse_pack_error::sacn_parse_pack_error;

    /// UUID library used to handle the UUID's used in the CID fields, used here so that error can include the cid in messages.
    use uuid::Uuid;

    error_chain! {
        foreign_links {
            Io(::std::io::Error);       // Allow IO errors to be used with the error-chain system.
            Str(::std::str::Utf8Error); // Allow standard string library errors to be used with the error-chain system.
            Uuid(uuid::ParseError);     // Allow UUID library to be used with error-chain system. 
        }

        links {
            SacnParsePackError(sacn_parse_pack_error::Error, sacn_parse_pack_error::ErrorKind);
        }

        errors {
            /// Attempted to perform an action using a priority value that is invalid. For example sending with a priority > 200.
            /// This is distinct from the SacnParsePackError(ParseInvalidPriority) as it is for a local use of an invalid priority
            /// rather than receiving an invalid priority from another source.
            InvalidPriority(msg: String) {
                description("Attempted to perform an action using a priority value that is invalid"),
                display("Attempted to perform an action using a priority value that is invalid, msg: {}", msg)
            }
            
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

            /// A source universe timed out as no data was received on that universe within E131_NETWORK_DATA_LOSS_TIMEOUT as per ANSI E1.31-2018 Section 6.7.1.
            UniverseTimeout(src_cid: Uuid, uni: u16) {
                description("A source universe timed out as no data was received within E131_NETWORK_DATA_LOSS_TIMEOUT as per ANSI E1.31-2018 Section 6.7.1"),
                display("(Source,Universe) timed out: ({},{})", src_cid, uni)
            }

            /// When looking for a specific universe it wasn't found. This might happen for example if trying to mute a universe on a receiver that
            /// wasn't being listened.
            UniverseNotFound(msg: String) {
                description("When looking for a specific universe it wasn't found"),
                display("When looking for a specific universe it wasn't found, msg: {}", msg)
            }

            /// Attempted to find a source and failed. This might happen on a receiver for example if trying to remove a source which was never 
            /// registered or discovered.
            SourceNotFound(msg: String) {
                description("When looking for a specific source it wasn't found"),
                display("Source not found, msg: {}", msg)
            }

            /// Thrown to indicate that the operation attempted is unsupported on the current OS
            /// For example this is used to indicate that multicast-IPv6 isn't supported current on Windows.
            OsOperationUnsupported(msg: String) {
                description("Thrown to indicate that the operation attempted is unsupported on the current OS"),
                display("Operation attempted is unsupported on the current OS, msg: {}", msg)
            }
        }
    }
}
