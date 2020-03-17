pub mod sacn_parse_pack_error {
    error_chain! {
        errors {   
            /// When parsing packet invalid data encountered.
            ParseInvalidData(msg: String) {
                description("Data provided to parse into a packet is invalid"),
                display("Error when parsing data into packet, msg: {}", msg)
            }

            /// Attempted to parse a priority value that is outwith the allowed range of [0, E131_MAX_PRIORITY].
            /// As per ANSI E1.31-2018 Section 6.2.3
            ParseInvalidPriority(msg: String) {
                description("Attempted to parse a priority value that is outwith the allowed range of [0, 200]"),
                display("Attempted to parse a priority value that is outwith the allowed range of [0, 200], msg: {}", msg)
            }

            /// Attempted to parse a sync address value that is outwith the allowed range of [0, E131_MAX_MULTICAST_UNIVERSE].
            /// As per ANSI E1.31-2018 Section 9.1.1
            ParseInvalidSyncAddr(msg: String) {
                description("Attempted to parse a sync_addr value that is outwith the allowed range of [0, 63999]"),
                display("Attempted to parse a sync_addr value that is outwith the allowed range of [0, 63999], msg: {}", msg)
            }

            /// Attempted to parse a universe value that is outwith the allowed range of [1, E131_MAX_MULTICAST_UNIVERSE].
            /// As per ANSI E1.31-2018 Section 9.1.1
            ParseInvalidUniverse(msg: String) {
                description("Attempted to parse a universe value that is outwith the allowed range of [1, 63999]"),
                display("Attempted to parse a universe value that is outwith the allowed range of [1, 63999], msg: {}", msg)
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
        }
    }
}