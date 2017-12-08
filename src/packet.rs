// Copyright 2017 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[cfg(feature = "std")]
use std::vec::Vec;

use byteorder::{ByteOrder, NetworkEndian};
use uuid::Uuid;
use arrayvec::ArrayString;

/// Trait to represent an ACN root layer protocol or nested protocol data units (PDUs).
pub trait Protocol {
    /// Returns the number of bytes the packet would occupy when packed.
    fn len(&self) -> usize;

    /// Packs the packet into the given vector.
    ///
    /// Grows the vector `buf` if necessary.
    #[cfg(feature = "std")]
    fn pack_vec(&self, buf: &mut Vec<u8>) {
        buf.clear();
        buf.reserve_exact(self.len());
        self.pack(buf)
    }

    /// Packs the packet into the given slice.
    ///
    /// # Panics
    /// Panics if the given slice is not large enough.
    fn pack(&self, &mut [u8]);
}

/// Root layer protocol of the Architecture for Control Networks (ACN) protocol.
#[derive(Debug, Clone)]
pub struct AcnRootLayerProtocol<'a> {
    pub pdu: E131RootLayer<'a>,
}

impl<'a> Protocol for AcnRootLayerProtocol<'a> {
    fn len(&self) -> usize {
        16 + self.pdu.len()
    }

    fn pack(&self, buf: &mut [u8]) {
        assert!(buf.len() >= self.len(), "buffer not large enough");
        // Preamble Size
        NetworkEndian::write_u16(&mut buf[0..2], 0x0010);
        // Post-amble Size
        zeros(&mut buf[2..4], 2);
        // ACN Packet Identifier
        buf[4..16].copy_from_slice(b"ASC-E1.17\x00\x00\x00");
        // PDU block
        self.pdu.pack(&mut buf[16..])
    }
}

/// Root layer protocol data unit (PDU).
#[derive(Debug, Clone)]
pub struct E131RootLayer<'a> {
    pub cid: Uuid,
    pub data: E131RootLayerData<'a>,
}

impl<'a> Protocol for E131RootLayer<'a> {
    fn len(&self) -> usize {
        22 +
            match self.data {
                E131RootLayerData::DataPacket(ref data) => data.len(),
                E131RootLayerData::SynchronizationPacket(ref data) => data.len(),
                E131RootLayerData::UniverseDiscoveryPacket(ref data) => data.len(),
            }
    }

    fn pack(&self, buf: &mut [u8]) {
        assert!(buf.len() >= self.len(), "buffer not large enough");
        // Length
        NetworkEndian::write_u16(&mut buf[0..2], self.len() as u16);
        buf[0] &= 0x0f;
        // Flags
        buf[0] |= 0x70;
        // Vector
        match self.data {
            // VECTOR_ROOT_E131_DATA
            #[cfg_attr(rustfmt, rustfmt_skip)]
            E131RootLayerData::DataPacket(_) => {
                NetworkEndian::write_u32(&mut buf[2..6], 0x00000004)
            }
            // VECTOR_ROOT_E131_EXTENDED
            E131RootLayerData::SynchronizationPacket(_) |
            E131RootLayerData::UniverseDiscoveryPacket(_) => {
                NetworkEndian::write_u32(&mut buf[2..6], 0x00000008)
            }
        }
        // CID
        buf[6..22].copy_from_slice(self.cid.as_bytes());
        // Data
        match self.data {
            E131RootLayerData::DataPacket(ref data) => data.pack(&mut buf[22..]),
            E131RootLayerData::SynchronizationPacket(ref data) => data.pack(&mut buf[22..]),
            E131RootLayerData::UniverseDiscoveryPacket(ref data) => data.pack(&mut buf[22..]),
        }
    }
}

/// Payload of the Root Layer PDU.
#[derive(Debug, Clone)]
pub enum E131RootLayerData<'a> {
    DataPacket(DataPacketFramingLayer<'a>),
    SynchronizationPacket(SynchronizationPacketFramingLayer),
    UniverseDiscoveryPacket(UniverseDiscoveryPacketFramingLayer<'a>),
}

/// Framing layer PDU for sACN data packets.
#[derive(Debug, Clone)]
pub struct DataPacketFramingLayer<'a> {
    pub source_name: ArrayString<[u8; 64]>,
    pub priority: u8,
    pub synchronization_address: u16,
    pub sequence_number: u8,
    pub preview_data: bool,
    pub stream_terminated: bool,
    pub force_synchronization: bool,
    pub universe: u16,
    pub data: DataPacketDmpLayer<'a>,
}

impl<'a> Protocol for DataPacketFramingLayer<'a> {
    fn len(&self) -> usize {
        77 + self.data.len()
    }

    fn pack(&self, buf: &mut [u8]) {
        assert!(buf.len() >= self.len(), "buffer not large enough");
        // Length
        NetworkEndian::write_u16(&mut buf[0..2], self.len() as u16);
        buf[0] &= 0x0f;
        // Flags
        buf[0] |= 0x70;
        // Vector VECTOR_E131_DATA_PACKET
        NetworkEndian::write_u32(&mut buf[2..6], 0x00000002);
        // Source Name
        zeros(&mut buf[6..70], 64);
        buf[6..6 + self.source_name.len()].copy_from_slice(self.source_name.as_bytes());
        // Priority
        buf[70] = self.priority;
        // Synchronization Address
        NetworkEndian::write_u16(&mut buf[71..73], self.synchronization_address);
        // Sequence Number
        buf[73] = self.sequence_number;
        // Options
        buf[74] = 0;
        // Preview Data
        if self.preview_data {
            buf[74] = 0b01000000
        }
        // Stream Terminated
        if self.stream_terminated {
            buf[74] |= 0b00100000
        }
        // Force Synchronization
        if self.force_synchronization {
            buf[74] |= 0b00010000
        }
        // Universe
        NetworkEndian::write_u16(&mut buf[75..77], self.universe);
        // Data
        self.data.pack(&mut buf[77..])
    }
}

/// Device Management Protocol PDU with SET PROPERTY vector.
///
/// Used for sACN data packets.
#[derive(Debug, Clone)]
pub struct DataPacketDmpLayer<'a> {
    pub property_values: &'a [u8],
}

impl<'a> Protocol for DataPacketDmpLayer<'a> {
    fn len(&self) -> usize {
        assert!(self.property_values.len() > 0, "DMX START Code required");
        assert!(
            self.property_values.len() <= 513,
            "max 512 data slots + DMX START Code allowed"
        );
        10 + self.property_values.len()
    }

    fn pack(&self, buf: &mut [u8]) {
        assert!(buf.len() >= self.len(), "buffer not large enough");
        // Length
        NetworkEndian::write_u16(&mut buf[0..2], self.len() as u16);
        buf[0] &= 0x0f;
        // Flags
        buf[0] |= 0x70;
        // Vector VECTOR_DMP_SET_PROPERTY
        buf[2] = 0x02;
        // Address and Data Type
        buf[3] = 0xa1;
        // First Property Address
        zeros(&mut buf[4..6], 2);
        // Address Increment
        NetworkEndian::write_u16(&mut buf[6..8], 0x0001);
        // Property value count
        NetworkEndian::write_u16(&mut buf[8..10], self.property_values.len() as u16);
        // Property values
        buf[10..10 + self.property_values.len()].copy_from_slice(self.property_values);
    }
}

/// sACN synchronization packet PDU.
#[derive(Debug, Clone)]
pub struct SynchronizationPacketFramingLayer {
    pub sequence_number: u8,
    pub synchronization_address: u16,
}

impl Protocol for SynchronizationPacketFramingLayer {
    fn len(&self) -> usize {
        11
    }

    fn pack(&self, buf: &mut [u8]) {
        assert!(buf.len() >= self.len(), "buffer not large enough");
        // Length
        NetworkEndian::write_u16(&mut buf[0..2], self.len() as u16);
        buf[0] &= 0x0f;
        // Flags
        buf[0] |= 0x70;
        // Vector VECTOR_E131_EXTENDED_SYNCHRONIZATION
        NetworkEndian::write_u32(&mut buf[2..6], 0x00000001);
        // Sequence Number
        buf[6] = self.sequence_number;
        // Synchronization Address
        NetworkEndian::write_u16(&mut buf[7..9], self.synchronization_address);
        // Reserved
        zeros(&mut buf[9..11], 2);
    }
}

// Framing layer PDU for sACN universe discovery packets.
#[derive(Debug, Clone)]
pub struct UniverseDiscoveryPacketFramingLayer<'a> {
    pub source_name: ArrayString<[u8; 64]>,
    data: UniverseDiscoveryPacketUniverseDiscoveryLayer<'a>,
}

impl<'a> Protocol for UniverseDiscoveryPacketFramingLayer<'a> {
    fn len(&self) -> usize {
        74 + self.data.len()
    }

    fn pack(&self, buf: &mut [u8]) {
        assert!(buf.len() >= self.len(), "buffer not large enough");
        // Length
        NetworkEndian::write_u16(&mut buf[0..2], self.len() as u16);
        buf[0] &= 0x0f;
        // Flags
        buf[0] |= 0x70;
        // Vector VECTOR_E131_EXTENDED_DISCOVERY
        NetworkEndian::write_u32(&mut buf[2..6], 0x00000002);
        // Source Name
        zeros(&mut buf[6..70], 64);
        buf[6..6 + self.source_name.len()].copy_from_slice(self.source_name.as_bytes());
        // Reserved
        zeros(&mut buf[70..74], 4);
        // Data
        self.data.pack(&mut buf[74..])
    }
}

/// Universe discovery layer PDU.
#[derive(Debug, Clone)]
pub struct UniverseDiscoveryPacketUniverseDiscoveryLayer<'a> {
    pub page: u8,
    pub last_page: u8,
    pub universes: &'a [u16],
}

impl<'a> Protocol for UniverseDiscoveryPacketUniverseDiscoveryLayer<'a> {
    fn len(&self) -> usize {
        assert!(
            self.universes.len() <= 512,
            "max 512 universes per page allowed"
        );
        8 + self.universes.len() * 2
    }

    fn pack(&self, buf: &mut [u8]) {
        assert!(buf.len() >= self.len(), "buffer not large enough");
        // Length
        NetworkEndian::write_u16(&mut buf[0..2], self.len() as u16);
        buf[0] &= 0x0f;
        // Flags
        buf[0] |= 0x70;
        // Vector VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST
        NetworkEndian::write_u32(&mut buf[2..6], 0x00000001);
        // Page
        buf[6] = self.page;
        // Last Page
        buf[7] = self.last_page;
        // Universes
        for i in 1..self.universes.len() {
            assert!(
                self.universes[i] != self.universes[i - 1],
                "univerese at position {} is not unique",
                i
            );
            assert!(
                self.universes[i] >= self.universes[i - 1],
                "univereses not sorted"
            );
        }
        NetworkEndian::write_u16_into(
            &self.universes[..self.universes.len()],
            &mut buf[8..8 + self.universes.len() * 2],
        )
    }
}

#[inline]
fn zeros(buf: &mut [u8], n: usize) {
    for i in 0..n {
        buf[i] = 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg_attr(rustfmt, rustfmt_skip)]
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

    #[cfg_attr(rustfmt, rustfmt_skip)]
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

    #[cfg_attr(rustfmt, rustfmt_skip)]
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

    #[test]
    fn test_pack_data_packet() {
        let packet = AcnRootLayerProtocol {
            pdu: E131RootLayer {
                cid: Uuid::from_bytes(&TEST_DATA_PACKET[22..38]).unwrap(),
                data: E131RootLayerData::DataPacket(DataPacketFramingLayer {
                    source_name: ArrayString::from("Source_A").unwrap(),
                    priority: 100,
                    synchronization_address: 7962,
                    sequence_number: 154,
                    preview_data: false,
                    stream_terminated: false,
                    force_synchronization: false,
                    universe: 1,
                    data: DataPacketDmpLayer { property_values: &TEST_DATA_PACKET[125..638] },
                }),
            },
        };

        assert_eq!(packet.len(), TEST_DATA_PACKET.len());

        let mut buf = [0; 638];
        packet.pack(&mut buf);

        assert_eq!(&buf[..], TEST_DATA_PACKET);
    }

    #[test]
    fn test_pack_synchronization_packet() {
        let packet = AcnRootLayerProtocol {
            pdu: E131RootLayer {
                cid: Uuid::from_bytes(&TEST_DATA_PACKET[22..38]).unwrap(),
                data: E131RootLayerData::SynchronizationPacket(SynchronizationPacketFramingLayer {
                    sequence_number: 0x65,
                    synchronization_address: 7962,
                }),
            },
        };

        assert_eq!(packet.len(), TEST_SYNCHRONIZATION_PACKET.len());

        let mut buf = [0; 49];
        packet.pack(&mut buf);

        assert_eq!(&buf[..], TEST_SYNCHRONIZATION_PACKET);
    }

    #[test]
    fn test_pack_universe_discovery_packet() {
        let packet = AcnRootLayerProtocol {
            pdu: E131RootLayer {
                cid: Uuid::from_bytes(&TEST_DATA_PACKET[22..38]).unwrap(),
                data: E131RootLayerData::UniverseDiscoveryPacket(
                    UniverseDiscoveryPacketFramingLayer {
                        source_name: ArrayString::from("Source_A").unwrap(),
                        data: UniverseDiscoveryPacketUniverseDiscoveryLayer {
                            page: 1,
                            last_page: 2,
                            universes: &[3, 4, 5],
                        },
                    },
                ),
            },
        };

        assert_eq!(packet.len(), TEST_UNIVERSE_DISCOVERY_PACKET.len());

        let mut buf = [0; 126];
        packet.pack(&mut buf);

        assert_eq!(&buf[..], TEST_UNIVERSE_DISCOVERY_PACKET);
    }
}
