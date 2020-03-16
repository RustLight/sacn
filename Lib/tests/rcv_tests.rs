// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was adapted as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

extern crate sacn;
extern crate uuid;

use sacn::packet::*;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};
use uuid::Uuid;

const TEST_DATA_PACKET: &[u8] = &[
        // Root Layer
        // Preamble Size
        0x00, 0x10,
        // Post-amble Size
        0x00, 0x00,
        // ACN Packet Identifier
        0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00,
        0x00, 0x00,
        // Flags and Length Protocol
        0x72, 0x6e, // 0x7d in E1-31-2016 sample packet (which is wrong)
        // Vector
        0x00, 0x00, 0x00, 0x04,
        // CID
        0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2,
        0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
        // Data Packet Framing Layer
        // Flags and Length
        0x72, 0x58, // 0x57 in E1-31-2016 sample packet (which is wrong)
        // Vector
        0x00, 0x00, 0x00, 0x02,
        // Source Name
        b'S', b'o', b'u', b'r', b'c', b'e', b'_',  b'A', 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
        // Priority
        100,
        // Synchronization Address
        0x1F, 0x1A, // 7962
        // Sequence Number
        154,
        // Options
        0,
        // Universe
        0, 1,
        // DMP Layer
        // Flags and Length
        0x72, 0x0b, // 0x0d in E1-31-2016 sample packet (which is wrong)
        // Vector
        0x02,
        // Address and Data Type
        0xa1,
        // First Property Address
        0x00, 0x00,
        // Address Increment
        0x00, 0x01,
        // Property value count
        0x02, 0x01,
        // Property values
        0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,

        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,

        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,

        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,

        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,

        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const TEST_UNIVERSE_DISCOVERY_PACKET: &[u8] = &[
        // Root Layer
        // Preamble Size
        0x00, 0x10,
        // Post-amble Size
        0x00, 0x00,
        // ACN Packet Identifier
        0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00,
        0x00, 0x00,
        // Flags and Length Protocol
        0x70, 0x6e,
        // Vector
        0x00, 0x00, 0x00, 0x08,
        // CID
        0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2,
        0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
        // E1.31 Framing Layer
        // Flags and Length
        0x70, 0x58,
        // Vector
        0x00, 0x00, 0x00, 0x02,
        // Source Name
        b'S', b'o', b'u', b'r', b'c', b'e', b'_',  b'A', 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
        // Reserved
        0, 0, 0, 0,
        // Universe Discovery Layer
        // Flags and Length
        0x70, 0x0e,
        // Vector
        0x00, 0x00, 0x00, 0x01,
        // Page
        1,
        // Last Page
        2,
        // Universes
        0, 3, 0, 4, 0, 5,
];

const TEST_SYNCHRONIZATION_PACKET: &[u8] = &[
        // Root Layer
        // Preamble Size
        0x00, 0x10,
        // Post-amble Size
        0x00, 0x00,
        // ACN Packet Identifier
        0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00,
        0x00, 0x00,
        // Flags and Length Protocol
        0x70, 0x21, // 0x30 in E1-31-2016 sample packet (which is wrong)
        // Vector
        0x00, 0x00, 0x00, 0x08,
        // CID
        0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2,
        0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e,
        // Synchronization Packet Framing Layer
        // Flags and Length
        0x70, 0x0b, // 0x0a in E1-31-2016 sample packet (which is wrong)
        // Vector
        0x00, 0x00, 0x00, 0x01,
        // Sequence Number
        0x65, // 367 % 0xff
        // Synchronization Address
        0x1F, 0x1A, // 7962
        // Reserved
        0, 0,
];

#[test]
fn data_packet() {
        let packet = AcnRootLayerProtocol {
            pdu: E131RootLayer {
                cid: Uuid::from_bytes(&TEST_DATA_PACKET[22..38]).unwrap(),
                data: E131RootLayerData::DataPacket(DataPacketFramingLayer {
                    source_name: "Source_A".into(),
                    priority: 100,
                    synchronization_address: 7962,
                    sequence_number: 154,
                    preview_data: false,
                    stream_terminated: false,
                    force_synchronization: false,
                    universe: 1,
                    data: DataPacketDmpLayer {
                        #[cfg(feature = "std")]
                        property_values: TEST_DATA_PACKET[125..638].into(),
                        #[cfg(not(feature = "std"))]
                        property_values: {
                            let mut property_values = Vec::new();
                            property_values
                                .extend_from_slice(&TEST_DATA_PACKET[125..638])
                                .unwrap();
                            property_values
                        },
                    },
                }),
            },
        };

        assert_eq!(
            AcnRootLayerProtocol::parse(&TEST_DATA_PACKET).unwrap(),
            packet
        );

        let mut buf = [0; 638];
        packet.pack(&mut buf).unwrap();

        assert_eq!(&buf[..packet.len()], TEST_DATA_PACKET);
    }

#[test]
fn synchronization_packet() {
        let packet = AcnRootLayerProtocol {
            pdu: E131RootLayer {
                cid: Uuid::from_bytes(&TEST_DATA_PACKET[22..38]).unwrap(),
                data: E131RootLayerData::SynchronizationPacket(SynchronizationPacketFramingLayer {
                    sequence_number: 0x65,
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
fn universe_discovery_packet() {
        let packet = AcnRootLayerProtocol {
            pdu: E131RootLayer {
                cid: Uuid::from_bytes(&TEST_DATA_PACKET[22..38]).unwrap(),
                data: E131RootLayerData::UniverseDiscoveryPacket(
                    UniverseDiscoveryPacketFramingLayer {
                        source_name: "Source_A".into(),
                        data: UniverseDiscoveryPacketUniverseDiscoveryLayer {
                            page: 1,
                            last_page: 2,
                            #[cfg(feature = "std")]
                            universes: vec![3, 4, 5].into(),
                            #[cfg(not(feature = "std"))]
                            universes: {
                                let mut universes = Vec::new();
                                universes.extend_from_slice(&[3, 4, 5]).unwrap();
                                universes
                            },
                        },
                    },
                ),
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