extern crate sacn;
extern crate uuid;

#[cfg(test)]
pub mod discovery_parse_tests {

use sacn::packet::*;
use uuid::Uuid;

/// Uses the sACN error-chain errors.
use sacn::error::errors::*;
use sacn::sacn_parse_pack_error::sacn_parse_pack_error;

/// The expected minimum size of a universe discovery packet as per ANSI E1.31-2018 Section 6.1.
const UNIVERSE_DISCOVERY_PACKET_EXPECTED_MIN_SIZE: usize = 120;

/// The expected maximum size of a universe discovery packet as per ANSI E1.31-2018 Section 6.1.
const UNIVERSE_DISCOVERY_PACKET_EXPECTED_MAX_SIZE: usize = 1144;

/// Built up / checked as per:
/// ANSI E1.31-2018:
///     Section 4.3 Table 4-3: E1.31 Universe Discovery Packet Format
///     Section 5 Table 5-4: E1.31 Root Layer
///     Section 6.4 Table 6-7: E1.31 Universe Discovery Packet Framing Layer
///     Section 8 Table 8-9: E1.31 Universe Discovery Packet Universe Discovery Layer
const TEST_UNIVERSE_DISCOVERY_PACKET: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x58, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes, note each universe takes 2 bytes so this represents 3 universes (0x0001, 0x0203, 0x0405) not 6. */
    0x0, 0x1, 0x2, 0x3, 0x4, 0x5,
];

/// Universe discovery packet with the root layer vector set to a vector unknown to ANSI E1.31-2018.
const TEST_UNIVERSE_DISCOVERY_PACKET_ROOT_LAYER_UNKNOWN_VECTOR: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x01, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x58, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes */
    0, 1, 2, 3, 4, 5,
];

/// Universe discovery packet with the root layer vector incorrectly set to the vector for an ANSI E1.31-2018 data packet.
const TEST_UNIVERSE_DISCOVERY_PACKET_ROOT_LAYER_DATA_VECTOR: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x04, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x58, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes */
    0, 1, 2, 3, 4, 5,
];

/// Universe discovery packet which has its E1.31 Framing Layer flags set incorrectly.
const TEST_UNIVERSE_DISCOVERY_PACKET_WRONG_FLAGS: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x60, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x58, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes */
    0, 1, 2, 3, 4, 5,
];

/// Universe discovery packet which has a CID field that is a byte too long.
const TEST_UNIVERSE_DISCOVERY_PACKET_TOO_LONG_CID: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x58, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes */
    0, 1, 2, 3, 4, 5,
];

/// Universe discovery packet which has a CID field that is a byte too short.
const TEST_UNIVERSE_DISCOVERY_PACKET_TOO_SHORT_CID: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x58, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes */
    0, 1, 2, 3, 4, 5,
];

/// Universe discovery packet which has its E1.31 Framing Layer flags set incorrectly.
const TEST_UNIVERSE_DISCOVERY_PACKET_FRAMING_LAYER_WRONG_FLAGS: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x60, 0x58, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes */
    0, 1, 2, 3, 4, 5,
];

/// Universe discovery packet which has its E1.31 Framing Layer length set shorter than it actually is.
const TEST_UNIVERSE_DISCOVERY_PACKET_FRAMING_LAYER_LENGTH_TOO_SHORT: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x57, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes */
    0, 1, 2, 3, 4, 5,
];

/// Universe discovery packet which has its E1.31 Framing Layer length set longer than it actually is.
const TEST_UNIVERSE_DISCOVERY_PACKET_FRAMING_LAYER_LENGTH_TOO_LONG: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x59, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes */
    0, 1, 2, 3, 4, 5,
];

/// Universe discovery packet with the framing vector set incorrectly to the vector for a synchronisation packet.
const TEST_UNIVERSE_DISCOVERY_PACKET_SYNC_FRAMING_VECTOR: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x58, 
    /* Vector, Set to the value for a synchronisation packet */
    0x00, 0x00, 0x00, 0x01, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes */
    0, 1, 2, 3, 4, 5,
];

/// Universe discovery packet with the framing vector set to a completely unrecognised value.
const TEST_UNIVERSE_DISCOVERY_PACKET_UNKNOWN_FRAMING_VECTOR: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x58, 
    /* Vector, Set to an unknown value*/
    0x00, 0x00, 0x00, 0x07, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes */
    0, 1, 2, 3, 4, 5,
];

/// Universe discovery packet with the reserved bytes set to values, this should be ignored and the packet 
/// parsed normally as per ANSI E1.31-2018 Section 6.4.3.
const TEST_UNIVERSE_DISCOVERY_PACKET_ARBITARY_RESERVED: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x58, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved, Set to arbitary values */
    255, 254, 253, 252, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes (3 universes, 0x0001, 0x0203, 0x0405) */
    0, 1, 2, 3, 4, 5,
];

/// A universe discovery packet with the discovery layer flags set incorrectly meaning the packet should be rejected.
/// As per ANSI E1.31-2018 Section 8.1.
const TEST_UNIVERSE_DISCOVERY_PACKET_DISCOVERY_LAYER_WRONG_FLAGS: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x58, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x60, 0x0e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes */
    0, 1, 2, 3, 4, 5,
];

/// A universe discovery packet with the discovery layer length set too short meaning the packet should be rejected.
/// As per ANSI E1.31-2018 Section 8.1.
const TEST_UNIVERSE_DISCOVERY_PACKET_DISCOVERY_LAYER_LENGTH_TOO_SHORT: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x58, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0d, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes */
    0, 1, 2, 3, 4, 5,
];

/// A universe discovery packet with the discovery layer length set too long meaning the packet should be rejected.
/// As per ANSI E1.31-2018 Section 8.1.
const TEST_UNIVERSE_DISCOVERY_PACKET_DISCOVERY_LAYER_LENGTH_TOO_LONG: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x58, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0f, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes */
    0, 1, 2, 3, 4, 5,
];

/// A universe discovery packet with the discovery layer vector set to an unknown value meaning the packet should be rejected.
/// As per ANSI E1.31-2018 Section 8.2.
const TEST_UNIVERSE_DISCOVERY_PACKET_DISCOVERY_LAYER_VECTOR_UNKNOWN: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x58, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x00, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes */
    0, 1, 2, 3, 4, 5,
];

/// A universe discovery packet with a page number higher than the last page meaning the packet should be rejected.
/// As per ANSI E1.31-2018 Section 8.3, 8.4.
const TEST_UNIVERSE_DISCOVERY_PACKET_PAGE_HIGHER_THAN_LAST_PAGE: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x58, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    3,
    /* Last Page */
    2, 
    /* Universes */
    0, 1, 2, 3, 4, 5,
];

/// A universe discovery packet with the universes in decending order which means it should be rejected as they should
/// be sorted in accending order as per ANSI E1.31-2018 Section 8.5.
const TEST_UNIVERSE_DISCOVERY_PACKET_DECENDING_ORDER: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x58, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes */
    5, 4, 3, 2, 1, 0,
];

/// A universe discovery packet with the universes in a random order which means it should be rejected as they should
/// be sorted in accending order as per ANSI E1.31-2018 Section 8.5.
const TEST_UNIVERSE_DISCOVERY_PACKET_RANDOM_ORDER: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x6e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    0x70, 0x58, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    0x70, 0x0e, 
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2, 
    /* Universes */
    3, 7, 5, 9, 1, 2,
];

#[test]
fn test_discovery_packet_parse_pack() {
    let packet = AcnRootLayerProtocol {
        pdu: E131RootLayer {
            cid: Uuid::from_bytes(&TEST_UNIVERSE_DISCOVERY_PACKET[22..38]).unwrap(),
            data: E131RootLayerData::UniverseDiscoveryPacket(UniverseDiscoveryPacketFramingLayer {
                source_name: "Source_A".into(),
                data: UniverseDiscoveryPacketUniverseDiscoveryLayer {
                    page: 1,
                    last_page: 2,
                    #[cfg(feature = "std")]
                    universes: vec![0x0001, 0x0203, 0x0405].into(),
                    #[cfg(not(feature = "std"))]
                    universes: {
                        let mut universes = Vec::new();
                        universes.extend_from_slice(&[3, 4, 5]).unwrap();
                        universes
                    },
                },
            }),
        },
    };

    assert_eq!(
        AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET).unwrap(),
        packet
    );

    let mut buf = [0; 126];
    packet.pack(&mut buf).unwrap();

    assert_eq!(&buf[..packet.len()], TEST_UNIVERSE_DISCOVERY_PACKET);
}


#[test]
fn test_discovery_packet_root_layer_unknown_vector_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_ROOT_LAYER_UNKNOWN_VECTOR) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PduInvalidVector(_)) => {
                    assert!(true, "Expected error returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
            
        }
        Ok(_) => {
            assert!(
                false,
                "Malformed packet was parsed when should have been rejected"
            );
        }
    }
}

#[test]
fn test_discovery_packet_root_layer_data_vector_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_ROOT_LAYER_DATA_VECTOR) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(_) => {
                    // As the packet will be treated as a data packet it is unclear where the parse will fail so only assert that it must fail
                    // with a parse type error rather than a specific error.
                    assert!(true, "Expected error family returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
            
        }
        Ok(_) => {
            assert!(
                false,
                "Malformed packet was parsed when should have been rejected"
            );
        }
    }
}

#[test]
fn test_discovery_packet_too_short_cid_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_TOO_SHORT_CID) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInsufficientData(_)) => {
                    assert!(true, "Expected error returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
            
        }
        Ok(_) => {
            assert!(
                false,
                "Malformed packet was parsed when should have been rejected"
            );
        }
    }
}

#[test]
fn test_discovery_packet_too_long_cid_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_TOO_LONG_CID) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(_) => {
                    // Difficult to predict / assert what error should be caused by a field being too long as all 
                    // other fields will be shifted and no clear way to know the true end of the CID field.
                    // Therefore just assert that the packet was detected as malformed rather than a specific error.
                    assert!(true, "Expected error family returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
            
        }
        Ok(_) => {
            assert!(
                false,
                "Malformed packet was parsed when should have been rejected"
            );
        }
    }
}

#[test]
fn test_discovery_packet_wrong_flags_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_WRONG_FLAGS) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParsePduInvalidFlags(_)) => {
                    assert!(true, "Expected error returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
            
        }
        Ok(_) => {
            assert!(
                false,
                "Malformed packet was parsed when should have been rejected"
            );
        }
    }
}

#[test]
fn test_discovery_packet_framing_layer_length_too_long_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_FRAMING_LAYER_LENGTH_TOO_LONG) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInsufficientData(_)) => {
                    assert!(true, "Expected error returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
            
        }
        Ok(_) => {
            assert!(
                false,
                "Malformed packet was parsed when should have been rejected"
            );
        }
    }
}

#[test]
fn test_discovery_packet_framing_layer_length_too_short_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_FRAMING_LAYER_LENGTH_TOO_SHORT) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInsufficientData(_)) => {
                    assert!(true, "Expected error returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
            
        }
        Ok(_) => {
            assert!(
                false,
                "Malformed packet was parsed when should have been rejected"
            );
        }
    }
}

#[test]
fn test_discovery_packet_framing_layer_wrong_flags_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_FRAMING_LAYER_WRONG_FLAGS) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParsePduInvalidFlags(_)) => {
                    assert!(true, "Expected error returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
            
        }
        Ok(_) => {
            assert!(
                false,
                "Malformed packet was parsed when should have been rejected"
            );
        }
    }
}

#[test]
fn test_discovery_packet_sync_framing_vector_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_SYNC_FRAMING_VECTOR) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(_) => {
                    // Difficult to assert the exact error caused by using the sync vector as the packet will then
                    // be parsed as a sync packet and could be rejected for multiple reasons.
                    // The key part is that it is rejected successfully for a parse error.
                    assert!(true, "Expected error family returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
            
        }
        Ok(_) => {
            assert!(
                false,
                "Malformed packet was parsed when should have been rejected"
            );
        }
    }
}

#[test]
fn test_discovery_packet_unknown_framing_vector_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_UNKNOWN_FRAMING_VECTOR) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PduInvalidVector(_)) => {
                    assert!(true, "Expected error returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
            
        }
        Ok(_) => {
            assert!(
                false,
                "Malformed packet was parsed when should have been rejected"
            );
        }
    }
}

#[test]
fn test_discovery_packet_arbitary_reserved_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_ARBITARY_RESERVED) {
        Err(e) => {
                assert!(false, format!("Unexpected error returned: {}", e));
            }
        Ok(p) => {
            match p.pdu.data {
                E131RootLayerData::UniverseDiscoveryPacket(udpfl) => {
                    assert_eq!(udpfl.source_name, "Source_A");
                    assert_eq!(udpfl.data.page, 1);
                    assert_eq!(udpfl.data.last_page, 2);
                    assert_eq!(udpfl.data.universes, vec!(0x01, 0x0203, 0x0405));
                }
                _ => {
                    assert!(false, "Packet not parsed as discovery-packet as expected");
                }
            }
        }
    }
}

#[test]
fn test_discovery_packet_discovery_layer_wrong_flags_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_DISCOVERY_LAYER_WRONG_FLAGS) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParsePduInvalidFlags(_)) => {
                    assert!(true, "Expected error returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
            
        }
        Ok(_) => {
            assert!(
                false,
                "Malformed packet was parsed when should have been rejected"
            );
        }
    }
}

#[test]
fn test_discovery_packet_discovery_layer_length_too_short_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_DISCOVERY_LAYER_LENGTH_TOO_SHORT) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInsufficientData(_)) => {
                    assert!(true, "Expected error returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
            
        }
        Ok(p) => {
            assert!(
                false,
                format!("Malformed packet was parsed when should have been rejected: {:?}", p)
            );
        }
    }
}

#[test]
fn test_discovery_packet_discovery_layer_length_too_long_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_DISCOVERY_LAYER_LENGTH_TOO_LONG) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInsufficientData(_)) => {
                    assert!(true, "Expected error returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
            
        }
        Ok(_) => {
            assert!(
                false,
                "Malformed packet was parsed when should have been rejected"
            );
        }
    }
}

#[test]
fn test_discovery_packet_discovery_layer_vector_unknown_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_DISCOVERY_LAYER_VECTOR_UNKNOWN) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PduInvalidVector(_)) => {
                    assert!(true, "Expected error returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
            
        }
        Ok(_) => {
            assert!(
                false,
                "Malformed packet was parsed when should have been rejected"
            );
        }
    }
}

#[test]
fn test_discovery_packet_page_higher_than_last_page_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_PAGE_HIGHER_THAN_LAST_PAGE) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInvalidPage(_)) => {
                    assert!(true, "Expected error returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
            
        }
        Ok(_) => {
            assert!(
                false,
                "Malformed packet was parsed when should have been rejected"
            );
        }
    }
}

#[test]
fn test_discovery_packet_decending_order_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_DECENDING_ORDER) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInvalidUniverseOrder(_)) => {
                    assert!(true, "Expected error returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
            
        }
        Ok(_) => {
            assert!(
                false,
                "Malformed packet was parsed when should have been rejected"
            );
        }
    }
}

#[test]
fn test_discovery_packet_random_order_parse() {
    match AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET_RANDOM_ORDER) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInvalidUniverseOrder(_)) => {
                    assert!(true, "Expected error returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
            
        }
        Ok(_) => {
            assert!(
                false,
                "Malformed packet was parsed when should have been rejected"
            );
        }
    }
}

/// Generates a test universe discovery packet with the given number of universes.
/// This function has no usage outside the parse tests - it is just used as an auxillary function.
fn generate_test_universe_discovery_packet(universes_to_generate: u16) -> Vec<u8> {
    let flags_val: u8 = 0x70;

    // Note that .to_be_bytes() returns in network byte order which is required to be used for this.

    // 8 because this is the number of bytes always required in the discovery layer
    // + the universes to generate x 2 because each universe is 2 bytes.
    // As per ANSI E1.31-2018 Table 4-3: E1.31 Universe Discovery Packet Format.
    let discovery_layer_length: u16 = 8 + (universes_to_generate * 2);

    let discovery_layer_parts = discovery_layer_length.to_be_bytes();

    let discovery_layer_flags_length_upper: u8 = flags_val | discovery_layer_parts[0];
    let discovery_layer_flags_length_lower: u8 = discovery_layer_parts[1];

    // 74 because this is the number of bytes always required in the framing layer
    // + the discovery layer as the framing layer encapsulates the discovery layer.
    // As per ANSI E1.31-2018 Table 4-3: E1.31 Universe Discovery Packet Format.
    let framing_layer_length: u16 = 74 + discovery_layer_length;

    let framing_layer_parts = framing_layer_length.to_be_bytes();

    let framing_layer_flags_length_upper: u8 = flags_val | framing_layer_parts[0];
    let framing_layer_flags_length_lower: u8 = framing_layer_parts[1];
    
    // 22 as this is the number of bytes always in the root layer.
    // + the framing layer as the root layer encapsulates it.
    // As per ANSI E1.31-2018 Table 4-3: E1.31 Universe Discovery Packet Format.
    let root_layer_length: u16 = 22 + framing_layer_length;

    let root_layer_parts = root_layer_length.to_be_bytes();

    let root_layer_flags_length_upper: u8 = flags_val | root_layer_parts[0];
    let root_layer_flags_length_lower: u8 = root_layer_parts[1];

    let mut test_universe_discovery_packet = vec!{
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10, 
    /* Post-amble Size */
    0x00, 0x00, 
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    root_layer_flags_length_upper, root_layer_flags_length_lower, 
    /* Vector */
    0x00, 0x00, 0x00, 0x08, 
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* E1.31 Framing Layer */
    /* Flags and Length */
    framing_layer_flags_length_upper, framing_layer_flags_length_lower, 
    /* Vector */
    0x00, 0x00, 0x00, 0x02, 
    /* Source Name */
    b'S', b'o', b'u', b'r', b'c', b'e', b'_', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 
    /* Reserved */
    0, 0, 0, 0, 
    /* Universe Discovery Layer */
    /* Flags and Length */
    discovery_layer_flags_length_upper, discovery_layer_flags_length_lower,
    /* Vector */
    0x00, 0x00, 0x00, 0x01, 
    /* Page */
    1,
    /* Last Page */
    2
    };

    for i in 0 .. universes_to_generate {
        let vals = i.to_be_bytes();
        test_universe_discovery_packet.push(vals[0]);
        test_universe_discovery_packet.push(vals[1]);
    }

    return test_universe_discovery_packet;
}

#[test]
fn test_discovery_packet_no_universes() {
    let generated_packet = generate_test_universe_discovery_packet(0);

    assert_eq!(generated_packet.len(), UNIVERSE_DISCOVERY_PACKET_EXPECTED_MIN_SIZE);

    match AcnRootLayerProtocol::parse(&generated_packet) {
        Err(e) => {
            assert!(false, format!("Unexpected error returned: {}", e));
        }
        Ok(p) => {
            match p.pdu.data {
                E131RootLayerData::UniverseDiscoveryPacket(udpfl) => {
                    assert_eq!(udpfl.source_name, "Source_A");
                    assert_eq!(udpfl.data.page, 1);
                    assert_eq!(udpfl.data.last_page, 2);
                    assert_eq!(udpfl.data.universes, Vec::new());
                }
                _ => {
                    assert!(false, "Packet not parsed as discovery-packet as expected");
                }
            }
        }
    }
}

#[test]
fn test_discovery_packet_max_universe_capacity() {
    let generated_packet = generate_test_universe_discovery_packet(DISCOVERY_UNI_PER_PAGE as u16);

    assert_eq!(generated_packet.len(), UNIVERSE_DISCOVERY_PACKET_EXPECTED_MAX_SIZE);

    match AcnRootLayerProtocol::parse(&generated_packet) {
        Err(e) => {
            assert!(false, format!("Unexpected error returned: {}", e));
        }
        Ok(p) => {
            match p.pdu.data {
                E131RootLayerData::UniverseDiscoveryPacket(udpfl) => {
                    assert_eq!(udpfl.source_name, "Source_A");
                    assert_eq!(udpfl.data.page, 1);
                    assert_eq!(udpfl.data.last_page, 2);
                    assert_eq!(udpfl.data.universes.into_owned().len(), DISCOVERY_UNI_PER_PAGE);
                }
                _ => {
                    assert!(false, "Packet not parsed as discovery-packet as expected");
                }
            }
        }
    }
}

#[test]
fn test_discovery_packet_above_max_universe_capacity() {
    match AcnRootLayerProtocol::parse(&generate_test_universe_discovery_packet((DISCOVERY_UNI_PER_PAGE as u16) + 1)) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PduInvalidLength(_)) => {
                    assert!(true, "Expected error returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned: {}", x));
                }
            }
        }
        Ok(_) => {
            assert!(
                false,
                "Malformed packet was parsed when should have been rejected"
            );
        }
    }
}

}