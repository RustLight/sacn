// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was created as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

/// The errors used within the SacnLibrary. The ErrorKind subsection of this within the documentation contains details of all the errors.
/// 
/// Errors from external sources are wrapped within this error-chain.
/// 
/// Io errors from std::io::Error are wrapped within Io(::std::io::Error)
/// 
/// String errors from std::str::Utf8Error are wrapped within Str(::std::str::Utf8Error)
/// 
/// Uuid errors from uuid::ParseError are wrapped within Uuid(uuid::ParseError)
/// 
/// 
/// ParsePack related errors come within their own family wrapped inside this error to allow easy matching (can just match for SacnParsePackError rather than a specific).
/// 
/// SacnParsePackError(sacn_parse_pack_error::Error, sacn_parse_pack_error::ErrorKind)
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
            // All parse/pack errors live within the same chain ('family') of errors as described in sacn_parse_packet_error.
            SacnParsePackError(sacn_parse_pack_error::Error, sacn_parse_pack_error::ErrorKind); 
        }

        errors {
            /// Returned to indicate that an invalid or malformed source name was used.
            /// 
            /// # Arguments
            /// msg: A string describing why the source name is malformed.
            /// 
            MalformedSourceName(msg: String) {
                description("The given source name was malformed and couldn't be used"),
                display("The given source name was malformed and couldn't be used, msg: {}", msg)
            }

            /// Attempted to perform an action using a priority value that is invalid. For example sending with a priority > 200.
            /// This is distinct from the SacnParsePackError(ParseInvalidPriority) as it is for a local use of an invalid priority
            /// rather than receiving an invalid priority from another source.
            /// 
            /// # Arguments
            /// msg: A string describing why the priority is invalid.
            /// 
            InvalidPriority(msg: String) {
                description("Attempted to perform an action using a priority value that is invalid"),
                display("Attempted to perform an action using a priority value that is invalid, msg: {}", msg)
            }
            
            /// Used to indicate that the limit for the number of supported sources has been reached. 
            /// This is based on unique CID values.
            /// as per ANSI E1.31-2018 Section 6.2.3.3.
            /// 
            /// # Arguments
            /// msg: A string describing why the sources exceeded error was returned.
            /// 
            SourcesExceededError(msg: String) {
                description("Limit for the number of supported sources has been reached"),
                display("Limit for the number of supported sources has been reached, msg: {}", msg)
            }

            /// A source was discovered by a receiver with the announce_discovery_flag set to true.
            /// 
            /// # Arguments
            /// source_name: The name of the source discovered.
            /// 
            SourceDiscovered(source_name: String) {
                description("A source was discovered by a receiver with the announce_discovery_flag set to true"),
                display("A source was discovered by a receiver with the announce_discovery_flag set to true, source name: {}", source_name)
            }

            /// Attempted to exceed the capacity of a single universe (packet::UNIVERSE_CHANNEL_CAPACITY).
            /// 
            /// # Arguments
            /// msg: A string describing why/how the universe capacity was exceeded.
            /// 
            ExceedUniverseCapacity(msg: String) {
                description("Attempted to exceed the capacity of a single universe"),
                display("Attempted to exceed the capacity of a single universe, msg: {}", msg)
            }

            /// Attempted to use illegal universe, outwith allowed range of [E131_MIN_MULTICAST_UNIVERSE, E131_MAX_MULTICAST_UNIVERSE] 
            /// + E131_DISCOVERY_UNIVERSE inclusive
            /// 
            /// # Arguments
            /// msg: A string describing why/how the universe is an illegal universe.
            /// 
            IllegalUniverse(msg: String) {
                description("Attempted to use illegal universe, outwith allowed range of [E131_MIN_MULTICAST_UNIVERSE 
                - E131_MAX_MULTICAST_UNIVERSE] + E131_DISCOVERY_UNIVERSE inclusive"),
                display("illegal universe used, outwith allowed range, msg: {}", msg)
            }

            /// Attempted to use a universe that wasn't first registered for use.
            /// To send from a universe with a sender it must first be registered. This allows universe discovery adverts to include the universe.
            /// 
            /// # Arguments
            /// msg: A string describing why the error was returned.
            /// 
            UniverseNotRegistered(msg: String) {
                description("Attempted to use a universe that wasn't first registered for use"),
                display("Attempted to use a universe that wasn't first registered for use, msg: {}", msg)
            }

            /// Ip version (ipv4 or ipv6) used when the other is expected.
            /// 
            /// # Arguments
            /// msg: A string describing the situation where the wrong IpVersion was encountered.
            /// 
            IpVersionError(msg: String) {
                description("Ip version (ipv4 or ipv6) used when the other is expected"),
                display("Ip version (ipv4 or ipv6) used when the other is expected, msg: {}", msg)
            }

            /// Attempted to use an unsupported (not Ipv4 or Ipv6) IP version.
            /// 
            /// # Arguments
            /// msg: A string describing the situation where an unsupported IP version is used.
            /// 
            UnsupportedIpVersion(msg: String) {
                description("Attempted to use an unsupported (not Ipv4 or Ipv6) IP version"),
                display("Attempted to use an unsupported (not Ipv4 or Ipv6) IP version, msg: {}", msg)
            }

            /// Attempted to use a sender which has already been terminated.
            /// 
            /// # Arguments
            /// msg: A string describing why the error was returned.
            /// 
            SenderAlreadyTerminated(msg: String) {
                description("Attempted to use a sender which has already been terminated"),
                display("Attempted to use a sender which has already been terminated, msg: {}", msg)
            }

            /// An error was encountered when attempting to merge DMX data together.
            /// 
            /// # Arguments
            /// msg: A string describing why the error was returned.
            /// 
            DmxMergeError(msg: String) {
                description("Error when merging DMX data"),
                display("Error when merging DMX data, msg: {}", msg)
            }

            /// Packet was received out of sequence and so should be discarded.
            /// 
            /// # Arguments
            /// msg: A string describing why the error was returned.
            /// 
            OutOfSequence(msg: String) {
                description("Packet was received out of sequence and so should be discarded"),
                display("Packet was received out of sequence and so should be discarded, msg: {}", msg)
            }

            /// A source terminated a universe and this was detected when trying to receive data.
            /// This is only returned if the announce_stream_termination flag is set to true (default false).
            /// 
            /// # Arguments
            /// 
            /// src_cid: The CID of the source which sent the termination packet.
            /// 
            /// uni: The universe that the termination packet is for.
            /// 
            UniverseTerminated(src_cid: Uuid, uni: u16) {
                description("A source terminated a universe and this was detected when trying to receive data"),
                display("Source cid: {:?} terminated universe: {}", src_cid, uni)
            }

            /// A source universe timed out as no data was received on that universe within E131_NETWORK_DATA_LOSS_TIMEOUT as per ANSI E1.31-2018 Section 6.7.1.
            /// 
            /// # Arguments
            /// 
            /// src_cid: The CID of the source which timed out.
            /// 
            /// uni: The universe that timed out.
            /// 
            UniverseTimeout(src_cid: Uuid, uni: u16) {
                description("A source universe timed out as no data was received within E131_NETWORK_DATA_LOSS_TIMEOUT as per ANSI E1.31-2018 Section 6.7.1"),
                display("(Source,Universe) timed out: ({},{})", src_cid, uni)
            }

            /// When looking for a specific universe it wasn't found. This might happen for example if trying to mute a universe on a receiver that
            /// wasn't being listened to.
            /// 
            /// # Arguments
            /// msg: A message describing why this error was returned.
            /// 
            UniverseNotFound(msg: String) {
                description("When looking for a specific universe it wasn't found"),
                display("When looking for a specific universe it wasn't found, msg: {}", msg)
            }

            /// Attempted to find a source and failed. This might happen on a receiver for example if trying to remove a source which was never 
            /// registered or discovered.
            /// 
            /// # Arguments
            /// msg: A message describing why this error was returned / when the source was not found.
            /// 
            SourceNotFound(msg: String) {
                description("When looking for a specific source it wasn't found"),
                display("Source not found, msg: {}", msg)
            }

            /// Thrown to indicate that the operation attempted is unsupported on the current OS
            /// For example this is used to indicate that multicast-IPv6 isn't supported current on Windows.
            /// 
            /// # Arguments
            /// msg: A message describing why this error was returned / the operation that was not supported.
            /// 
            OsOperationUnsupported(msg: String) {
                description("Thrown to indicate that the operation attempted is unsupported on the current OS"),
                display("Operation attempted is unsupported on the current OS, msg: {}", msg)
            }

            /// Thrown to indicate that the source has corrupted for the reason specified by the error chain.
            /// This is currently only thrown if the source mutex is poisoned by a thread with access panic-ing.
            /// This prevents the panic propagating to the user of this library and allows them to handle it appropriately
            /// such as by creating a new source.
            /// 
            /// # Arguments
            /// msg: A message providing further details (if any) as to why the SourceCorrupt error was returned.
            /// 
            SourceCorrupt(msg: String) {
                description("The sACN source has corrupted due to an internal panic! and should no longer be used"),
                display("The sACN source has corrupted due to an internal panic! and should no longer be used, {}", msg)
            }
        }
    }
}
