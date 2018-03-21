// Copyright 2017 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use core::str;
#[cfg(feature = "std")]
use std::vec::Vec;

use byteorder::{ByteOrder, NetworkEndian};
#[cfg(not(feature = "std"))]
use arrayvec::ArrayVec;
use arrayvec::ArrayString;
use uuid::Uuid;

use error::{PackError, ParseError};

struct PduInfo {
    length: usize,
    vector: u32,
}

fn pdu_info(buf: &[u8], vector_length: usize) -> Result<PduInfo, ParseError> {
    if buf.len() < 2 {
        return Err(ParseError::NotEnoughData);
    }

    // Flags
    let flags = buf[0] & 0xf0;
    if flags != 0x70 {
        return Err(ParseError::PduInvalidFlags(flags));
    }
    // Length
    let length = (NetworkEndian::read_u16(&buf[0..2]) & 0x0fff) as usize;
    if buf.len() < length {
        return Err(ParseError::NotEnoughData);
    }

    // Vector
    let vector = NetworkEndian::read_uint(&buf[2..], vector_length) as u32;

    Ok(PduInfo {
        length: length,
        vector: vector,
    })
}

/// Trait to represent an ACN root layer protocol or nested protocol data units (PDUs).
pub trait Protocol: Sized {
    /// Returns the parsed packet from the given buffer `buf`.
    ///
    /// # Panics
    /// Panics if the packet can not be parsed.
    fn parse(buf: &[u8]) -> Result<Self, ParseError>;

    /// Packs the packet into heap allocated memory.
    #[cfg(feature = "std")]
    fn pack_alloc(&self) -> Result<Vec<u8>, PackError> {
        let mut buf = Vec::with_capacity(self.len());
        self.pack_vec(&mut buf)?;
        Ok(buf)
    }

    /// Packs the packet into the given vector.
    ///
    /// Grows the vector `buf` if necessary.
    #[cfg(feature = "std")]
    fn pack_vec(&self, buf: &mut Vec<u8>) -> Result<(), PackError> {
        buf.clear();
        buf.reserve_exact(self.len());
        unsafe {
            buf.set_len(self.len());
        }
        self.pack(buf)
    }

    /// Packs the packet into the given slice `buf`.
    ///
    /// # Panics
    /// Panics if the given slice is not large enough.
    fn pack(&self, buf: &mut [u8]) -> Result<(), PackError>;

    /// Returns the number of bytes the packet would occupy when packed.
    fn len(&self) -> usize;
}

/// Root layer protocol of the Architecture for Control Networks (ACN) protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcnRootLayerProtocol {
    pub pdu: E131RootLayer,
}

impl<'a> Protocol for AcnRootLayerProtocol {
    fn parse(buf: &[u8]) -> Result<AcnRootLayerProtocol, ParseError> {
        // Preamble Size
        if NetworkEndian::read_u16(&buf[0..2]) != 0x0010 {
            return Err(ParseError::InvalidData("invalid Preamble Size"));
        }
        // Post-amble Size
        if NetworkEndian::read_u16(&buf[2..4]) != 0 {
            return Err(ParseError::InvalidData("invalid Post-amble Size"));
        }
        // ACN Packet Identifier
        if &buf[4..16] != b"ASC-E1.17\x00\x00\x00" {
            return Err(ParseError::InvalidData("invalid ACN packet indentifier"));
        }
        // PDU block
        Ok(AcnRootLayerProtocol {
            pdu: E131RootLayer::parse(&buf[16..])?,
        })
    }

    fn pack(&self, buf: &mut [u8]) -> Result<(), PackError> {
        if buf.len() < self.len() {
            return Err(PackError::BufferNotLargeEnough);
        }

        // Preamble Size
        NetworkEndian::write_u16(&mut buf[0..2], 0x0010);

        // Post-amble Size
        zeros(&mut buf[2..4], 2);

        // ACN Packet Identifier
        buf[4..16].copy_from_slice(b"ASC-E1.17\x00\x00\x00");

        // PDU block
        Ok(self.pdu.pack(&mut buf[16..])?)
    }

    fn len(&self) -> usize {
        // Preamble Size
        2 +
        // Post-amble Size
        2 +
        // ACN Packet Identifier
        12 +
        // PDU block
        self.pdu.len()
    }
}

const VECTOR_ROOT_E131_DATA: u32 = 0x00000004;
const VECTOR_ROOT_E131_EXTENDED: u32 = 0x00000008;

/// Root layer protocol data unit (PDU).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct E131RootLayer {
    pub cid: Uuid,
    pub data: E131RootLayerData,
}

impl Protocol for E131RootLayer {
    fn parse(buf: &[u8]) -> Result<E131RootLayer, ParseError> {
        // Length and Vector
        let PduInfo { length, vector } = pdu_info(&buf, 4)?;
        if vector != VECTOR_ROOT_E131_DATA && vector != VECTOR_ROOT_E131_EXTENDED {
            return Err(ParseError::PduInvalidVector(vector));
        }

        // CID
        let cid = Uuid::from_bytes(&buf[6..22])?;

        // Data
        let data = match vector {
            VECTOR_ROOT_E131_DATA => {
                E131RootLayerData::DataPacket(DataPacketFramingLayer::parse(&buf[22..length])?)
            }
            VECTOR_ROOT_E131_EXTENDED => {
                let data_buf = &buf[22..length];
                let PduInfo {
                    length: _,
                    vector: data_vector,
                } = pdu_info(&data_buf, 4)?;

                match data_vector {
                    VECTOR_E131_EXTENDED_SYNCHRONIZATION => {
                        E131RootLayerData::SynchronizationPacket(
                            SynchronizationPacketFramingLayer::parse(data_buf)?,
                        )
                    }
                    VECTOR_E131_EXTENDED_DISCOVERY => E131RootLayerData::UniverseDiscoveryPacket(
                        UniverseDiscoveryPacketFramingLayer::parse(data_buf)?,
                    ),

                    vector => return Err(ParseError::PduInvalidVector(vector)),
                }
            }
            vector => return Err(ParseError::PduInvalidVector(vector)),
        };

        Ok(E131RootLayer {
            cid: cid,
            data: data,
        })
    }

    fn pack(&self, buf: &mut [u8]) -> Result<(), PackError> {
        if buf.len() < self.len() {
            return Err(PackError::BufferNotLargeEnough);
        }

        // Flags and Length
        let flags_and_length = 0x7000 | (self.len() as u16) & 0x0fff;
        NetworkEndian::write_u16(&mut buf[0..2], flags_and_length);

        // Vector
        match self.data {
            E131RootLayerData::DataPacket(_) => {
                NetworkEndian::write_u32(&mut buf[2..6], VECTOR_ROOT_E131_DATA)
            }
            E131RootLayerData::SynchronizationPacket(_)
            | E131RootLayerData::UniverseDiscoveryPacket(_) => {
                NetworkEndian::write_u32(&mut buf[2..6], VECTOR_ROOT_E131_EXTENDED)
            }
        }

        // CID
        buf[6..22].copy_from_slice(self.cid.as_bytes());

        // Data
        match self.data {
            E131RootLayerData::DataPacket(ref data) => Ok(data.pack(&mut buf[22..])?),
            E131RootLayerData::SynchronizationPacket(ref data) => Ok(data.pack(&mut buf[22..])?),
            E131RootLayerData::UniverseDiscoveryPacket(ref data) => Ok(data.pack(&mut buf[22..])?),
        }
    }

    fn len(&self) -> usize {
        // Length and Flags
        2 +
        // Vector
        4 +
        // CID
        16 +
        // Data
        match self.data {
            E131RootLayerData::DataPacket(ref data) => data.len(),
            E131RootLayerData::SynchronizationPacket(ref data) => data.len(),
            E131RootLayerData::UniverseDiscoveryPacket(ref data) => data.len(),
        }
    }
}

/// Payload of the Root Layer PDU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum E131RootLayerData {
    DataPacket(DataPacketFramingLayer),
    SynchronizationPacket(SynchronizationPacketFramingLayer),
    UniverseDiscoveryPacket(UniverseDiscoveryPacketFramingLayer),
}

const VECTOR_E131_DATA_PACKET: u32 = 0x00000002;

/// Framing layer PDU for sACN data packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataPacketFramingLayer {
    pub source_name: ArrayString<[u8; 64]>,
    pub priority: u8,
    pub synchronization_address: u16,
    pub sequence_number: u8,
    pub preview_data: bool,
    pub stream_terminated: bool,
    pub force_synchronization: bool,
    pub universe: u16,
    pub data: DataPacketDmpLayer,
}

impl Protocol for DataPacketFramingLayer {
    fn parse(buf: &[u8]) -> Result<DataPacketFramingLayer, ParseError> {
        // Length and Vector
        let PduInfo { length, vector } = pdu_info(&buf, 4)?;
        if vector != VECTOR_E131_DATA_PACKET {
            return Err(ParseError::PduInvalidVector(vector));
        }

        // Source Name
        let source_name = parse_source_name(&buf[6..70])?;

        // Priority
        let priority = buf[70];

        // Synchronization Address
        let synchronization_address = NetworkEndian::read_u16(&buf[71..73]);

        // Sequence Number
        let sequence_number = buf[73];

        // Options
        let preview_data = buf[74] & 0b01000000 != 0;
        let stream_terminated = buf[74] & 0b00100000 != 0;
        let force_synchronization = buf[74] & 0b00010000 != 0;

        // Universe
        let universe = NetworkEndian::read_u16(&buf[75..77]);

        // Data
        let data = DataPacketDmpLayer::parse(&buf[77..length])?;

        Ok(DataPacketFramingLayer {
            source_name: source_name,
            priority: priority,
            synchronization_address: synchronization_address,
            sequence_number: sequence_number,
            preview_data: preview_data,
            stream_terminated: stream_terminated,
            force_synchronization: force_synchronization,
            universe: universe,
            data: data,
        })
    }

    fn pack(&self, buf: &mut [u8]) -> Result<(), PackError> {
        if buf.len() < self.len() {
            return Err(PackError::BufferNotLargeEnough);
        }

        // Flags and Length
        let flags_and_length = 0x7000 | (self.len() as u16) & 0x0fff;
        NetworkEndian::write_u16(&mut buf[0..2], flags_and_length);

        // Vector
        NetworkEndian::write_u32(&mut buf[2..6], VECTOR_E131_DATA_PACKET);

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
        Ok(self.data.pack(&mut buf[77..])?)
    }

    fn len(&self) -> usize {
        // Length and Flags
        2 +
        // Vector
        4 +
        // Source Name
        64 +
        // Priority
        1 +
        // Synchronization Address
        2 +
        // Sequence Number
        1 +
        // Options
        1 +
        // Universe
        2 +
        // Data
        self.data.len()
    }
}

const VECTOR_DMP_SET_PROPERTY: u8 = 0x02;

/// Device Management Protocol PDU with SET PROPERTY vector.
///
/// Used for sACN data packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataPacketDmpLayer {
    pub property_values: DataPacketDmpLayerPropertyValues,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataPacketDmpLayerPropertyValues {
    pub start_code: u8,
    #[cfg(feature = "std")] pub dmx_data: Vec<u8>,
    #[cfg(not(feature = "std"))] pub dmx_data: ArrayVec<[u8; 512]>,
}

impl<'a> Protocol for DataPacketDmpLayer {
    fn parse(buf: &[u8]) -> Result<DataPacketDmpLayer, ParseError> {
        // Length and Vector
        let PduInfo { length, vector } = pdu_info(&buf, 1)?;
        if vector != VECTOR_DMP_SET_PROPERTY as u32 {
            return Err(ParseError::PduInvalidVector(vector));
        }

        // Address and Data Type
        if buf[3] != 0xa1 {
            return Err(ParseError::InvalidData("invalid Address and Data Type"));
        }

        // First Property Address
        if NetworkEndian::read_u16(&buf[4..6]) != 0 {
            return Err(ParseError::InvalidData("invalid First Property Address"));
        }

        // Address Increment
        if NetworkEndian::read_u16(&buf[6..8]) != 0x0001 {
            return Err(ParseError::InvalidData("invalid Address Increment"));
        }

        // Property value count
        if NetworkEndian::read_u16(&buf[8..10]) as usize + 10 != length {
            return Err(ParseError::InvalidData("invalid Property value count"));
        }

        // Property values
        let property_values_dmx_data_len = length - 11;
        let mut property_values = DataPacketDmpLayerPropertyValues {
            start_code: buf[10],
            #[cfg(feature = "std")]
            dmx_data: Vec::with_capacity(property_values_dmx_data_len),
            #[cfg(not(feature = "std"))]
            dmx_data: ArrayVec::new(),
        };
        unsafe {
            property_values
                .dmx_data
                .set_len(property_values_dmx_data_len);
        }
        property_values.dmx_data[..property_values_dmx_data_len].copy_from_slice(&buf[11..length]);

        Ok(DataPacketDmpLayer {
            property_values: property_values,
        })
    }

    fn pack(&self, buf: &mut [u8]) -> Result<(), PackError> {
        if buf.len() < self.len() {
            return Err(PackError::BufferNotLargeEnough);
        }

        // Flags and Length
        let flags_and_length = 0x7000 | (self.len() as u16) & 0x0fff;
        NetworkEndian::write_u16(&mut buf[0..2], flags_and_length);

        // Vector
        buf[2] = VECTOR_DMP_SET_PROPERTY;

        // Address and Data Type
        buf[3] = 0xa1;

        // First Property Address
        zeros(&mut buf[4..6], 2);

        // Address Increment
        NetworkEndian::write_u16(&mut buf[6..8], 0x0001);

        // Property value count
        NetworkEndian::write_u16(
            &mut buf[8..10],
            self.property_values.dmx_data.len() as u16 + 1,
        );
        // Property values
        buf[10] = self.property_values.start_code;
        buf[11..11 + self.property_values.dmx_data.len()]
            .copy_from_slice(&self.property_values.dmx_data);

        Ok(())
    }

    fn len(&self) -> usize {
        assert!(
            self.property_values.dmx_data.len() <= 512,
            "max 512 DMX data slots"
        );

        // Length and Flags
        2 +
        // Vector
        1 +
        // Address and Data Type
        1 +
        // First Property Address
        2 +
        // Address Increment
        2 +
        // Property value count
        2 +
        // Property values
        1 + self.property_values.dmx_data.len()
    }
}

const VECTOR_E131_EXTENDED_SYNCHRONIZATION: u32 = 0x00000001;

/// sACN synchronization packet PDU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynchronizationPacketFramingLayer {
    pub sequence_number: u8,
    pub synchronization_address: u16,
}

impl<'a> Protocol for SynchronizationPacketFramingLayer {
    fn parse(buf: &[u8]) -> Result<SynchronizationPacketFramingLayer, ParseError> {
        // Length and Vector
        let PduInfo { length: _, vector } = pdu_info(&buf, 4)?;
        if vector != VECTOR_E131_EXTENDED_SYNCHRONIZATION {
            return Err(ParseError::PduInvalidVector(vector));
        }

        // Sequence Number
        let sequence_number = buf[6];

        // Synchronization Address
        let synchronization_address = NetworkEndian::read_u16(&buf[7..9]);

        // Reserved
        if buf[9..11] != [0, 0] {
            return Err(ParseError::InvalidData("invalid Reserved"));
        }

        Ok(SynchronizationPacketFramingLayer {
            sequence_number: sequence_number,
            synchronization_address: synchronization_address,
        })
    }

    fn pack(&self, buf: &mut [u8]) -> Result<(), PackError> {
        if buf.len() < self.len() {
            return Err(PackError::BufferNotLargeEnough);
        }

        // Flags and Length
        let flags_and_length = 0x7000 | (self.len() as u16) & 0x0fff;
        NetworkEndian::write_u16(&mut buf[0..2], flags_and_length);

        // Vector
        NetworkEndian::write_u32(&mut buf[2..6], VECTOR_E131_EXTENDED_SYNCHRONIZATION);

        // Sequence Number
        buf[6] = self.sequence_number;

        // Synchronization Address
        NetworkEndian::write_u16(&mut buf[7..9], self.synchronization_address);

        // Reserved
        zeros(&mut buf[9..11], 2);

        Ok(())
    }

    fn len(&self) -> usize {
        // Length and Flags
        2 +
        // Vector
        4 +
        // Sequence Number
        1 +
        // Synchronization Address
        2 +
        // Reserved
        2
    }
}

const VECTOR_E131_EXTENDED_DISCOVERY: u32 = 0x00000002;

// Framing layer PDU for sACN universe discovery packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniverseDiscoveryPacketFramingLayer {
    pub source_name: ArrayString<[u8; 64]>,
    pub data: UniverseDiscoveryPacketUniverseDiscoveryLayer,
}

impl Protocol for UniverseDiscoveryPacketFramingLayer {
    fn parse(buf: &[u8]) -> Result<UniverseDiscoveryPacketFramingLayer, ParseError> {
        // Length and Vector
        let PduInfo { length, vector } = pdu_info(&buf, 4)?;
        if vector != VECTOR_E131_EXTENDED_DISCOVERY {
            return Err(ParseError::PduInvalidVector(vector));
        }

        // Source Name
        let source_name = parse_source_name(&buf[6..70])?;

        // Reserved
        if buf[70..74] != [0, 0, 0, 0] {
            return Err(ParseError::InvalidData("invalid Reserved"));
        }

        // Data
        let data = UniverseDiscoveryPacketUniverseDiscoveryLayer::parse(&buf[74..length])?;

        Ok(UniverseDiscoveryPacketFramingLayer {
            source_name: source_name,
            data: data,
        })
    }

    fn pack(&self, buf: &mut [u8]) -> Result<(), PackError> {
        if buf.len() < self.len() {
            return Err(PackError::BufferNotLargeEnough);
        }

        // Flags and Length
        let flags_and_length = 0x7000 | (self.len() as u16) & 0x0fff;
        NetworkEndian::write_u16(&mut buf[0..2], flags_and_length);

        // Vector
        NetworkEndian::write_u32(&mut buf[2..6], VECTOR_E131_EXTENDED_DISCOVERY);

        // Source Name
        zeros(&mut buf[6..70], 64);
        buf[6..6 + self.source_name.len()].copy_from_slice(self.source_name.as_bytes());

        // Reserved
        zeros(&mut buf[70..74], 4);

        // Data
        Ok(self.data.pack(&mut buf[74..])?)
    }

    fn len(&self) -> usize {
        // Length and Flags
        2 +
        // Vector
        4 +
        // Source Name
        64 +
        // Reserved
        4 +
        // Data
        self.data.len()
    }
}

const VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST: u32 = 0x00000001;

/// Universe discovery layer PDU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniverseDiscoveryPacketUniverseDiscoveryLayer {
    pub page: u8,
    pub last_page: u8,
    #[cfg(feature = "std")] pub universes: Vec<u16>,
    #[cfg(not(feature = "std"))] pub universes: ArrayVec<[u16; 512]>,
}

impl Protocol for UniverseDiscoveryPacketUniverseDiscoveryLayer {
    fn parse(buf: &[u8]) -> Result<UniverseDiscoveryPacketUniverseDiscoveryLayer, ParseError> {
        // Length and Vector
        let PduInfo { length, vector } = pdu_info(&buf, 4)?;
        if vector != VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST {
            return Err(ParseError::PduInvalidVector(vector));
        }

        // Page
        let page = buf[6];

        // Last Page
        let last_page = buf[7];

        // Universes
        let universes_len = (length - 8) / 2;
        #[cfg(feature = "std")]
        let mut universes = Vec::with_capacity(universes_len);
        #[cfg(not(feature = "std"))]
        let mut universes = ArrayVec::new();
        unsafe {
            universes.set_len(universes_len);
        }
        NetworkEndian::read_u16_into(&buf[8..length], &mut universes[..universes_len]);

        Ok(UniverseDiscoveryPacketUniverseDiscoveryLayer {
            page: page,
            last_page: last_page,
            universes: universes,
        })
    }

    fn pack(&self, buf: &mut [u8]) -> Result<(), PackError> {
        if buf.len() < self.len() {
            return Err(PackError::BufferNotLargeEnough);
        }

        // Flags and Length
        let flags_and_length = 0x7000 | (self.len() as u16) & 0x0fff;
        NetworkEndian::write_u16(&mut buf[0..2], flags_and_length);

        // Vector
        NetworkEndian::write_u32(&mut buf[2..6], VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST);

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
        );

        Ok(())
    }

    fn len(&self) -> usize {
        assert!(
            self.universes.len() <= 512,
            "max 512 universes per page allowed"
        );

        // Length and Flags
        2 +
        // Vector
        4 +
        // Page
        1 +
        // Last Page
        1 +
        // Universes
        self.universes.len() * 2
    }
}

#[inline]
fn parse_source_name(buf: &[u8]) -> Result<ArrayString<[u8; 64]>, ParseError> {
    let mut source_name_len = buf.len();
    for i in 0..buf.len() {
        if buf[i] == 0 {
            source_name_len = i;
            break;
        }
    }
    Ok(ArrayString::from(str::from_utf8(&buf[..source_name_len])?).unwrap())
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
    fn test_data_packet() {
        #[cfg(feature = "std")]
        let dmx_data = {
            let mut dmx_data = vec![0; 512];
            dmx_data.copy_from_slice(&TEST_DATA_PACKET[126..638]);
            dmx_data
        };
        #[cfg(not(feature = "std"))]
        let dmx_data = {
            let mut dmx_data = [0; 512];
            dmx_data.copy_from_slice(&TEST_DATA_PACKET[126..638]);
            ArrayVec::from(dmx_data)
        };

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
                    data: DataPacketDmpLayer {
                        property_values: DataPacketDmpLayerPropertyValues {
                            start_code: 0,
                            dmx_data: dmx_data,
                        },
                    },
                }),
            },
        };

        assert_eq!(packet.len(), TEST_DATA_PACKET.len());

        let mut buf = [0; 638];
        packet.pack(&mut buf).unwrap();

        assert_eq!(&buf[..], TEST_DATA_PACKET);
        assert_eq!(
            AcnRootLayerProtocol::parse(&TEST_DATA_PACKET).unwrap(),
            packet
        );
    }

    #[test]
    fn test_synchronization_packet() {
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
        packet.pack(&mut buf).unwrap();

        assert_eq!(&buf[..], TEST_SYNCHRONIZATION_PACKET);
        assert_eq!(
            AcnRootLayerProtocol::parse(&TEST_SYNCHRONIZATION_PACKET).unwrap(),
            packet
        );
    }

    #[test]
    fn test_universe_discovery_packet() {
        let packet = AcnRootLayerProtocol {
            pdu: E131RootLayer {
                cid: Uuid::from_bytes(&TEST_DATA_PACKET[22..38]).unwrap(),
                data: E131RootLayerData::UniverseDiscoveryPacket(
                    UniverseDiscoveryPacketFramingLayer {
                        source_name: ArrayString::from("Source_A").unwrap(),
                        data: UniverseDiscoveryPacketUniverseDiscoveryLayer {
                            page: 1,
                            last_page: 2,
                            universes: {
                                #[cfg(feature = "std")]
                                let mut universes = Vec::new();
                                #[cfg(not(feature = "std"))]
                                let mut universes = ArrayVec::new();
                                universes.push(3);
                                universes.push(4);
                                universes.push(5);
                                universes
                            },
                        },
                    },
                ),
            },
        };

        assert_eq!(packet.len(), TEST_UNIVERSE_DISCOVERY_PACKET.len());

        let mut buf = [0; 126];
        packet.pack(&mut buf).unwrap();

        assert_eq!(
            UniverseDiscoveryPacketFramingLayer::parse(&TEST_UNIVERSE_DISCOVERY_PACKET[38..])
                .unwrap(),
            UniverseDiscoveryPacketFramingLayer {
                source_name: ArrayString::from("Source_A").unwrap(),
                data: UniverseDiscoveryPacketUniverseDiscoveryLayer {
                    page: 1,
                    last_page: 2,
                    universes: {
                        #[cfg(feature = "std")]
                        let mut universes = Vec::new();
                        #[cfg(not(feature = "std"))]
                        let mut universes = ArrayVec::new();
                        universes.push(3);
                        universes.push(4);
                        universes.push(5);
                        universes
                    },
                },
            },
        );

        assert_eq!(&buf[..], TEST_UNIVERSE_DISCOVERY_PACKET);
        assert_eq!(
            AcnRootLayerProtocol::parse(&TEST_UNIVERSE_DISCOVERY_PACKET).unwrap(),
            packet
        );
    }
}
