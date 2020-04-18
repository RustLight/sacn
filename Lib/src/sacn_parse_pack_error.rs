// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was created as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

/// The errors used within the SacnLibrary specifically those related to parsing and packeting packets received/sent on the network.
/// 
pub mod sacn_parse_pack_error {
    error_chain! {
        errors {   
            /// When parsing packet invalid data encountered.
            /// 
            /// # Arguments
            /// msg: A message providing further details (if any) as to what data was invalid.
            /// 
            ParseInvalidData(msg: String) {
                description("Data provided to parse into a packet is invalid"),
                display("Error when parsing data into packet, msg: {}", msg)
            }

            /// Attempted to parse a priority value that is outwith the allowed range of [0, E131_MAX_PRIORITY].
            /// As per ANSI E1.31-2018 Section 6.2.3
            /// 
            /// # Arguments
            /// msg: A message providing further details (if any) as to why the priority valid was invalid.
            /// 
            ParseInvalidPriority(msg: String) {
                description("Attempted to parse a priority value that is outwith the allowed range of [0, 200]"),
                display("Attempted to parse a priority value that is outwith the allowed range of [0, 200], msg: {}", msg)
            }

            /// Attempted to parse a page value that is invalid - e.g. the page value is higher than the last_page value.
            /// 
            /// # Arguments
            /// msg: A message providing further details (if any) as to why the page was invalid.
            /// 
            ParseInvalidPage(msg: String) {
                description("Attempted to parse a page value that is invalid"),
                display("Error when parsing page value, msg: {}", msg)
            }

            /// Attempted to parse a sync address value that is outwith the allowed range of [0, E131_MAX_MULTICAST_UNIVERSE].
            /// As per ANSI E1.31-2018 Section 9.1.1.
            /// 
            /// # Arguments
            /// msg: A message providing further details (if any) as to why the synchronisation address was invalid.
            /// 
            ParseInvalidSyncAddr(msg: String) {
                description("Attempted to parse a sync_addr value that is outwith the allowed range of [0, 63999]"),
                display("Attempted to parse a sync_addr value that is outwith the allowed range of [0, 63999], msg: {}", msg)
            }

            /// Attempted to parse a universe value that is outwith the allowed range of [1, E131_MAX_MULTICAST_UNIVERSE].
            /// As per ANSI E1.31-2018 Section 9.1.1.
            /// 
            /// # Arguments
            /// msg: A message providing further details (if any) as to why the universe field was invalid.
            /// 
            ParseInvalidUniverse(msg: String) {
                description("Attempted to parse a universe value that is outwith the allowed range of [1, 63999]"),
                display("Attempted to parse a universe value that is outwith the allowed range of [1, 63999], msg: {}", msg)
            }

            /// Attempted to parse a packet with an invalid ordering of universes.
            /// For example a discovery packet where the universes aren't correctly ordered in assending order.
            /// 
            /// # Arguments
            /// msg: A message providing further details (if any) as to why the universe ordering was invalid.
            /// 
            ParseInvalidUniverseOrder(msg: String) {
                description("Attempted to parse a packet with an invalid ordering of universes"),
                display("Attempted to parse a packet with an invalid ordering of universes, msg: {}", msg)
            }

            /// When packing a packet into a buffer invalid data encountered.
            /// 
            /// # Arguments
            /// msg: A message providing further details (if any) as to why the data couldn't be packed.
            /// 
            PackInvalidData(msg: String) {
                description("When packing a packet into a buffer invalid data encountered"),
                display("When packing a packet into a buffer invalid data encountered, msg: {}", msg)
            }

            /// Supplied buffer is not large enough to pack packet into.
            /// 
            /// # Arguments
            /// msg: A message providing further details (if any) as to why the pack buffer is insufficient.
            /// 
            PackBufferInsufficient(msg: String) {
                description("Supplied buffer is not large enough to pack packet into"),
                display("Supplied buffer is not large enough to pack packet into, msg: {}", msg)
            }

            /// Supplied buffer does not contain enough data.
            /// 
            /// # Arguments
            /// msg: A message providing further details (if any) as to why there was insufficient data for parsing.
            /// 
            ParseInsufficientData(msg: String) {
                description("Supplied buffer does not contain enough data"),
                display("Supplied buffer does not contain enough data, msg: {}", msg)
            }

            /// Received PDU flags are invalid for parsing.
            /// 
            /// # Arguments
            /// flags: The flags that were found which are invalid.
            /// 
            ParsePduInvalidFlags(flags: u8) {
                description("Received PDU flags are invalid"),
                display("PDU Flags {:#b} are invalid for parsing", flags)
            }

            /// Received PDU length is invalid.
            /// 
            /// # Arguments
            /// len: The length provided in the Pdu which is invalid.
            /// 
            PduInvalidLength(len: usize) {
                description("Received PDU length is invalid"),
                display("PDU Length {} is invalid", len)
            }

            /// Received PDU vector is invalid/unsupported by this library.
            /// 
            /// # Arguments
            /// vec: The vector parsed which is invalid / cannot be used.
            /// 
            PduInvalidVector(vec: u32) {
                description("Received PDU vector is invalid/unsupported by this library"),
                display("Vector {:#x} not supported", vec)
            }

            /// Error parsing the received UUID.
            /// 
            /// # Arguments
            /// msg: A message providing further details (if any) as to why the uuid (used for CID) couldn't be parsed.
            /// 
            UuidError(msg: String) {
                description("Error parsing the received UUID"),
                display("Error parsing the received UUID, msg: {}", msg)
            }

            /// Error parsing received UTF8 string.
            /// 
            /// # Arguments
            /// msg: A message providing further details (if any) as to why the string couldn't be parsed.
            /// 
            Utf8Error(msg: String) {
                description("Error parsing received UTF8 string"),
                display("Error parsing received UTF8 string, msg: {}", msg)
            }

            /// Source name in packet was invalid, for example due to not being null terminated.
            /// 
            /// # Arguments
            /// msg: A message providing further details (if any) as to why the source name was invalid.
            /// 
            SourceNameInvalid(msg: String) {
                description("Attempted to parse invalid source name"),
                display("Attempted to parse invalid source name, msg: {}", msg)
            }
        }
    }
}