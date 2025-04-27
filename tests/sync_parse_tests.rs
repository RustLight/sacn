#![cfg_attr(rustfmt, rustfmt_skip)]
extern crate sacn;
extern crate uuid;

#[cfg(test)]
pub mod sync_parse_tests {

use sacn::packet::*;
use uuid::Uuid;

/// Uses the sACN error-chain errors.
use sacn::error::errors::*;
use sacn::sacn_parse_pack_error::sacn_parse_pack_error;

/// A test synchronisation packet as specified as an example in
/// ANSI E1.31-2018 Appendix B Table B-14: Universe Synchronization Example E1.31 Synchronization Packet.
const TEST_SYNCHRONIZATION_PACKET: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10,
    /* Post-amble Size */
    0x00, 0x00,
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x21,
    /* Vector */
    0x00, 0x00, 0x00, 0x08,
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* Synchronization Packet Framing Layer */
    /* Flags and Length */
    0x70, 0x0b,
    /* Vector */
    0x00, 0x00, 0x00, 0x01,
    /* Sequence Number - Specifies a value of 367 which doesn't fit within a unsigned 8-bit byte, therefore used 367 % 0xff = 0x70 */
    0x70,
    /* Synchronization Address = 7962 */
    0x1F, 0x1A,
    /* Reserved */
    0, 0,
];

/// Synchronisation packet with the root layer vector set to a vector for an unknown (not ANSI E1.31-2018) packet.
const TEST_SYNCHRONIZATION_PACKET_ROOT_LAYER_UNKNOWN_VECTOR: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10,
    /* Post-amble Size */
    0x00, 0x00,
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x21,
    /* Vector */
    0x00, 0x00, 0x00, 0x09,
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* Synchronization Packet Framing Layer */
    /* Flags and Length */
    0x70, 0x0b,
    /* Vector */
    0x00, 0x00, 0x00, 0x01,
    /* Sequence Number - Specifies a value of 367 which doesn't fit within a unsigned 8-bit byte, therefore used 367 % 0xff = 0x70 */
    0x70,
    /* Synchronization Address = 7962 */
    0x1F, 0x1A,
    /* Reserved */
    0, 0,
];

/// Synchronisation packet with the root layer vector set to a vector for a data packet.
const TEST_SYNCHRONIZATION_PACKET_ROOT_LAYER_DATA_VECTOR: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10,
    /* Post-amble Size */
    0x00, 0x00,
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x21,
    /* Vector */
    0x00, 0x00, 0x00, 0x04,
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* Synchronization Packet Framing Layer */
    /* Flags and Length */
    0x70, 0x0b,
    /* Vector */
    0x00, 0x00, 0x00, 0x01,
    /* Sequence Number - Specifies a value of 367 which doesn't fit within a unsigned 8-bit byte, therefore used 367 % 0xff = 0x70 */
    0x70,
    /* Synchronization Address = 7962 */
    0x1F, 0x1A,
    /* Reserved */
    0, 0,
];

/// Synchronisation packet with the CID set a byte (17 bytes) longer than expected (16 bytes).
/// As per ANSI E1.31-2018 Section 4.2 Table 4-2.
const TEST_SYNCHRONIZATION_PACKET_TOO_LONG_CID: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10,
    /* Post-amble Size */
    0x00, 0x00,
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x21,
    /* Vector */
    0x00, 0x00, 0x00, 0x08,
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e, 0x3e,
    /* Synchronization Packet Framing Layer */
    /* Flags and Length */
    0x70, 0x0b,
    /* Vector */
    0x00, 0x00, 0x00, 0x01,
    /* Sequence Number - Specifies a value of 367 which doesn't fit within a unsigned 8-bit byte, therefore used 367 % 0xff = 0x70 */
    0x70,
    /* Synchronization Address = 7962 */
    0x1F, 0x1A,
    /* Reserved */
    0, 0,
];

/// Synchronisation packet with the CID set a byte (15 bytes) shorter than expected (16 bytes).
/// As per ANSI E1.31-2018 Section 4.2 Table 4-2.
const TEST_SYNCHRONIZATION_PACKET_TOO_SHORT_CID: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10,
    /* Post-amble Size */
    0x00, 0x00,
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x21,
    /* Vector */
    0x00, 0x00, 0x00, 0x08,
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14,
    /* Synchronization Packet Framing Layer */
    /* Flags and Length */
    0x70, 0x0b,
    /* Vector */
    0x00, 0x00, 0x00, 0x01,
    /* Sequence Number - Specifies a value of 367 which doesn't fit within a unsigned 8-bit byte, therefore used 367 % 0xff = 0x70 */
    0x70,
    /* Synchronization Address = 7962 */
    0x1F, 0x1A,
    /* Reserved */
    0, 0,
];

/// Synchronisation packet with the framing layer length set shorter than the actual packet is.
const TEST_SYNCHRONIZATION_PACKET_FRAMING_LAYER_WRONG_FLAGS: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10,
    /* Post-amble Size */
    0x00, 0x00,
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x21,
    /* Vector */
    0x00, 0x00, 0x00, 0x08,
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* Synchronization Packet Framing Layer */
    /* Flags and Length */
    0x60, 0x0b,
    /* Vector */
    0x00, 0x00, 0x00, 0x01,
    /* Sequence Number - Specifies a value of 367 which doesn't fit within a unsigned 8-bit byte, therefore used 367 % 0xff = 0x70 */
    0x70,
    /* Synchronization Address = 7962 */
    0x1F, 0x1A,
    /* Reserved */
    0, 0,
];

/// Synchronisation packet with the framing layer length set longer than the actual packet is.
const TEST_SYNCHRONIZATION_PACKET_FRAMING_LAYER_LENGTH_TOO_LONG: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10,
    /* Post-amble Size */
    0x00, 0x00,
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x21,
    /* Vector */
    0x00, 0x00, 0x00, 0x08,
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* Synchronization Packet Framing Layer */
    /* Flags and Length */
    0x70, 0x0c,
    /* Vector */
    0x00, 0x00, 0x00, 0x01,
    /* Sequence Number - Specifies a value of 367 which doesn't fit within a unsigned 8-bit byte, therefore used 367 % 0xff = 0x70 */
    0x70,
    /* Synchronization Address = 7962 */
    0x1F, 0x1A,
    /* Reserved */
    0, 0,
];

/// Synchronisation packet with the framing layer length set shorter than the actual packet is.
const TEST_SYNCHRONIZATION_PACKET_FRAMING_LAYER_LENGTH_TOO_SHORT: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10,
    /* Post-amble Size */
    0x00, 0x00,
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x21,
    /* Vector */
    0x00, 0x00, 0x00, 0x08,
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* Synchronization Packet Framing Layer */
    /* Flags and Length */
    0x70, 0x0a,
    /* Vector */
    0x00, 0x00, 0x00, 0x01,
    /* Sequence Number - Specifies a value of 367 which doesn't fit within a unsigned 8-bit byte, therefore used 367 % 0xff = 0x70 */
    0x70,
    /* Synchronization Address = 7962 */
    0x1F, 0x1A,
    /* Reserved */
    0, 0,
];

/// Synchronisation packet with the framing layer vector set to the vector for a discovery packet.
const TEST_SYNCHRONIZATION_PACKET_FRAMING_LAYER_DISCOVERY_VECTOR: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10,
    /* Post-amble Size */
    0x00, 0x00,
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x21,
    /* Vector */
    0x00, 0x00, 0x00, 0x08,
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* Synchronization Packet Framing Layer */
    /* Flags and Length */
    0x70, 0x0b,
    /* Vector */
    0x00, 0x00, 0x00, 0x02,
    /* Sequence Number - Specifies a value of 367 which doesn't fit within a unsigned 8-bit byte, therefore used 367 % 0xff = 0x70 */
    0x70,
    /* Synchronization Address = 7962 */
    0x1F, 0x1A,
    /* Reserved */
    0, 0,
];

/// Synchronisation packet with the framing layer vector set to an unknown vector.
const TEST_SYNCHRONIZATION_PACKET_FRAMING_LAYER_UNKNOWN_VECTOR: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10,
    /* Post-amble Size */
    0x00, 0x00,
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x21,
    /* Vector */
    0x00, 0x00, 0x00, 0x08,
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* Synchronization Packet Framing Layer */
    /* Flags and Length */
    0x70, 0x0b,
    /* Vector */
    0x00, 0x00, 0x00, 0x07,
    /* Sequence Number - Specifies a value of 367 which doesn't fit within a unsigned 8-bit byte, therefore used 367 % 0xff = 0x70 */
    0x70,
    /* Synchronization Address = 7962 */
    0x1F, 0x1A,
    /* Reserved */
    0, 0,
];

/// Synchronisation packet with the synchronisation address set higher than the maximum allowed universe/address.
/// As per ANSI E1.31-2018 Section 6.2.7
const TEST_SYNCHRONIZATION_PACKET_TOO_HIGH_SYNC_ADDRESS: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10,
    /* Post-amble Size */
    0x00, 0x00,
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x21,
    /* Vector */
    0x00, 0x00, 0x00, 0x08,
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* Synchronization Packet Framing Layer */
    /* Flags and Length */
    0x70, 0x0b,
    /* Vector */
    0x00, 0x00, 0x00, 0x01,
    /* Sequence Number - Specifies a value of 367 which doesn't fit within a unsigned 8-bit byte, therefore used 367 % 0xff = 0x70 */
    0x70,
    /* Synchronization Address = 64000 = E131_MAX_MULTICAST_UNIVERSE + 1 */
    0xFA, 0x00,
    /* Reserved */
    0, 0,
];

/// Synchronisation packet with the synchronisation address set lower than the maximum allowed universe/address (0).
/// As per ANSI E1.31-2018 Section 6.2.7
const TEST_SYNCHRONIZATION_PACKET_TOO_LOW_SYNC_ADDRESS: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10,
    /* Post-amble Size */
    0x00, 0x00,
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x21,
    /* Vector */
    0x00, 0x00, 0x00, 0x08,
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* Synchronization Packet Framing Layer */
    /* Flags and Length */
    0x70, 0x0b,
    /* Vector */
    0x00, 0x00, 0x00, 0x01,
    /* Sequence Number - Specifies a value of 367 which doesn't fit within a unsigned 8-bit byte, therefore used 367 % 0xff = 0x70 */
    0x70,
    /* Synchronization Address = 0 which is below the expected range */
    0x00, 0x00,
    /* Reserved */
    0, 0,
];

/// Synchronisation packet with the reserved bytes set to arbitary values.
/// As per ANSI E1.31-2018 Section 6.3.4 these should be ignored and the packet parsed as normal.
const TEST_SYNCHRONIZATION_PACKET_ARBITARY_RESERVED: &[u8] = &[
    /* Root Layer */
    /* Preamble Size */
    0x00, 0x10,
    /* Post-amble Size */
    0x00, 0x00,
    /* ACN Packet Identifier */
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
    /* Flags and Length Protocol */
    0x70, 0x21,
    /* Vector */
    0x00, 0x00, 0x00, 0x08,
    /* CID */
    0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
    /* Synchronization Packet Framing Layer */
    /* Flags and Length */
    0x70, 0x0b,
    /* Vector */
    0x00, 0x00, 0x00, 0x01,
    /* Sequence Number - Specifies a value of 367 which doesn't fit within a unsigned 8-bit byte, therefore used 367 % 0xff = 0x70 */
    0x70,
    /* Synchronization Address = 7962 */
    0x1F, 0x1A,
    /* Reserved */
    255, 254,
];

#[test]
fn test_sync_packet_length() {
    const EXPECTED_SYNC_PACKET_LEN: usize = 49; // As per ANSI E1.31-2018 Section 5.4.
    assert_eq!(TEST_SYNCHRONIZATION_PACKET.len(), EXPECTED_SYNC_PACKET_LEN);
}

#[test]
fn test_synchronization_packet_parse_pack() {
    let packet = AcnRootLayerProtocol {
        pdu: E131RootLayer {
            cid: Uuid::from_bytes(TEST_SYNCHRONIZATION_PACKET[22..38].try_into().unwrap()),
            data: E131RootLayerData::SynchronizationPacket(SynchronizationPacketFramingLayer {
                sequence_number: 0x70,
                synchronization_address: 7962,
            }),
        },
    };

    assert_eq!(
        AcnRootLayerProtocol::parse(&TEST_SYNCHRONIZATION_PACKET).unwrap(),
        packet
    );

    let mut buf = [0; 49];
    packet.pack(&mut buf).unwrap();

    assert_eq!(&buf[..packet.len()], TEST_SYNCHRONIZATION_PACKET);
}

#[test]
fn test_sync_packet_root_layer_data_vector_parse() {
    match AcnRootLayerProtocol::parse(&TEST_SYNCHRONIZATION_PACKET_ROOT_LAYER_DATA_VECTOR) {
        Err(e) => {
            match e {
                SacnError::SacnParsePackError(_) => {
                    // As the packet will be treated as a data packet it is unclear where the parse will fail so only assert that it must fail
                    // with a parse type error rather than a specific error.
                    assert!(true, "Expected error family returned");
                }
                _ => {
                    assert!(false, "Unexpected error type returned");
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
fn test_sync_packet_root_layer_unknown_vector_parse() {
    match AcnRootLayerProtocol::parse(&TEST_SYNCHRONIZATION_PACKET_ROOT_LAYER_UNKNOWN_VECTOR) {
        Err(e) => {
            match e {
                SacnError::SacnParsePackError(sacn_parse_pack_error::SacnParsePackError::PduInvalidVector(_)) => {
                    assert!(true, "Expected error returned");
                }
                _ => {
                    assert!(false, "Unexpected error type returned");
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
fn test_sync_packet_too_short_cid_parse() {
    match AcnRootLayerProtocol::parse(&TEST_SYNCHRONIZATION_PACKET_TOO_SHORT_CID) {
        Err(e) => {
            match e {
                SacnError::SacnParsePackError(_) => {
                    // As packet is too short it is unclear exactly what error will occur, just need to assert
                    // that the packet is successfully rejected as malformed.
                    assert!(true, "Expected error family returned");
                }
                _ => {
                    assert!(false, "Unexpected error type returned");
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
fn test_sync_packet_too_long_cid_parse() {
    match AcnRootLayerProtocol::parse(&TEST_SYNCHRONIZATION_PACKET_TOO_LONG_CID) {
        Err(e) => {
            match e {
                SacnError::SacnParsePackError(_) => {
                    // As packet is too long it is unclear exactly what error will occur, just need to assert
                    // that the packet is successfully rejected as malformed.
                    assert!(true, "Expected error family returned");
                }
                _ => {
                    assert!(false, "Unexpected error type returned");
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
fn test_sync_packet_framing_layer_wrong_flags_parse() {
    match AcnRootLayerProtocol::parse(&TEST_SYNCHRONIZATION_PACKET_FRAMING_LAYER_WRONG_FLAGS) {
        Err(e) => {
            match e {
                SacnError::SacnParsePackError(sacn_parse_pack_error::SacnParsePackError::ParsePduInvalidFlags(_)) => {
                    assert!(true, "Expected error returned");
                }
                _ => {
                    assert!(false, "Unexpected error type returned");
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
fn test_sync_packet_framing_layer_length_too_long_parse() {
    match AcnRootLayerProtocol::parse(&TEST_SYNCHRONIZATION_PACKET_FRAMING_LAYER_LENGTH_TOO_LONG) {
        Err(e) => {
            match e {
                SacnError::SacnParsePackError(sacn_parse_pack_error::SacnParsePackError::ParseInsufficientData(_)) => {
                    assert!(true, "Expected error returned");
                }
                _ => {
                    assert!(false, "Unexpected error type returned");
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
fn test_sync_packet_framing_layer_length_too_short_parse() {
    match AcnRootLayerProtocol::parse(&TEST_SYNCHRONIZATION_PACKET_FRAMING_LAYER_LENGTH_TOO_SHORT) {
        Err(e) => {
            match e {
                SacnError::SacnParsePackError(sacn_parse_pack_error::SacnParsePackError::PduInvalidLength(_)) => {
                    assert!(true, "Expected error returned");
                }
                _ => {
                    assert!(false, "Unexpected error type returned");
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
fn test_sync_packet_framing_layer_discovery_vector() {
    match AcnRootLayerProtocol::parse(&TEST_SYNCHRONIZATION_PACKET_FRAMING_LAYER_DISCOVERY_VECTOR) {
        Err(e) => {
            match e {
                SacnError::SacnParsePackError(_) => {
                    // The packet will be parsed as if it was a discovery packet which means that the parsing might fail
                    // for a number of reasons with it being hard to assert which one ahead of time.
                    // Therefore just assert that the parsing fails / the packet is rejected.
                    assert!(true, "Expected error family returned");
                }
                _ => {
                    assert!(false, "Unexpected error type returned");
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
fn test_sync_packet_framing_layer_unknown_vector() {
    match AcnRootLayerProtocol::parse(&TEST_SYNCHRONIZATION_PACKET_FRAMING_LAYER_UNKNOWN_VECTOR) {
        Err(e) => {
            match e {
                SacnError::SacnParsePackError(sacn_parse_pack_error::SacnParsePackError::PduInvalidVector(_)) => {
                    assert!(true, "Expected error family returned");
                }
                _ => {
                    assert!(false, "Unexpected error type returned");
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
fn test_sync_packet_too_high_sync_addr() {
    match AcnRootLayerProtocol::parse(&TEST_SYNCHRONIZATION_PACKET_TOO_HIGH_SYNC_ADDRESS) {
        Err(e) => {
            match e {
                SacnError::SacnParsePackError(sacn_parse_pack_error::SacnParsePackError::ParseInvalidUniverse(_)) => {
                    assert!(true, "Expected error family returned");
                }
                _ => {
                    assert!(false, "Unexpected error type returned");
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
fn test_sync_packet_too_low_sync_addr() {
    match AcnRootLayerProtocol::parse(&TEST_SYNCHRONIZATION_PACKET_TOO_LOW_SYNC_ADDRESS) {
        Err(e) => {
            match e {
                SacnError::SacnParsePackError(sacn_parse_pack_error::SacnParsePackError::ParseInvalidUniverse(_)) => {
                    assert!(true, "Expected error family returned");
                }
                _ => {
                    assert!(false, "Unexpected error type returned");
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
fn test_sync_packet_arbitary_reserved() {
    match AcnRootLayerProtocol::parse(&TEST_SYNCHRONIZATION_PACKET_ARBITARY_RESERVED) {
        Err(_) => {
            assert!(false, "Unexpected error returned");
        }
        Ok(p) => {
            match p.pdu.data {
                E131RootLayerData::SynchronizationPacket(spfl) => {
                    assert_eq!(spfl.sequence_number, 0x70);
                    assert_eq!(spfl.synchronization_address, 7962);
                }
                _ => {
                    assert!(false, "Packet not parsed as sync-packet as expected");
                }
            }
        }
    }
}
}