// Copyright 2018 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Parsing of sacn network packets.
//!
//! The packets live within the scope of the ACN protocol suite.
//!
//! # Examples
//!
//! ```
//! # extern crate uuid;
//! # extern crate sacn;
//! # use uuid::Uuid;
//! # use sacn::packet::{AcnRootLayerProtocol, E131RootLayer, E131RootLayerData, DataPacketFramingLayer, DataPacketDmpLayer};
//! # fn main() {
//! #[cfg(feature = "std")]
//! # {
//! let packet = AcnRootLayerProtocol {
//!     pdu: E131RootLayer {
//!         cid: Uuid::new_v4(),
//!         data: E131RootLayerData::DataPacket(DataPacketFramingLayer {
//!             source_name: "Source_A".into(),
//!             priority: 100,
//!             synchronization_address: 7962,
//!             sequence_number: 154,
//!             preview_data: false,
//!             stream_terminated: false,
//!             force_synchronization: false,
//!             universe: 1,
//!             data: DataPacketDmpLayer {
//!                 property_values: vec![0, 1, 2, 3].into(),
//!             },
//!         }),
//!     },
//! };
//!
//! let mut buf = [0; 638];
//! packet.pack(&mut buf).unwrap();
//!
//! assert_eq!(
//!     AcnRootLayerProtocol::parse(&buf).unwrap(),
//!     packet
//! );
//! # }}
//! ```

use core::hash::{self, Hash};
use core::str;
#[cfg(feature = "std")]
use std::borrow::Cow;
#[cfg(feature = "std")]
use std::vec::Vec;

use byteorder::{ByteOrder, NetworkEndian};
#[cfg(not(feature = "std"))]
use heapless::{String, Vec};
use uuid::Uuid;

use error::{PackError, ParseError};

// The payload capacity for a sacn packet, for DMX data this would translate to 512 frames + a startcode byte.
pub const UNIVERSE_CHANNEL_CAPACITY: usize = 513;

// The synchronisation universe/address of packets which do not require synchronisation as specified in section 6.2.4.1 of ANSI E1.31-2018.
pub const NO_SYNC_UNIVERSE: u16 = 0;

// Could be anything, implementation dependent, default universe used as the syncronisation universe.
pub const DEFAULT_SYNC_UNIVERSE: u16 = 1;

#[inline]
fn zeros(buf: &mut [u8], n: usize) {
    for b in buf.iter_mut().take(n) {
        *b = 0;
    }
}

#[inline]
fn parse_c_str(buf: &[u8]) -> Result<&str, ParseError> {
    let mut source_name_length = buf.len();
    for (i, b) in buf.iter().enumerate() {
        if *b == 0 {
            source_name_length = i;
            break;
        }
    }
    Ok(str::from_utf8(&buf[..source_name_length])?)
}

macro_rules! impl_acn_root_layer_protocol {
    ( $( $lt:tt )* ) => {
        /// Root layer protocol of the Architecture for Control Networks (ACN) protocol.
        #[derive(Clone, Eq, PartialEq, Hash, Debug)]
        pub struct AcnRootLayerProtocol$( $lt )* {
            /// The PDU this packet carries.
            pub pdu: E131RootLayer$( $lt )*,
        }

        impl$( $lt )* AcnRootLayerProtocol$( $lt )* {
            /// Parse the packet from the given buffer.
            pub fn parse(buf: &[u8]) -> Result<AcnRootLayerProtocol, ParseError> {
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

            /// Packs the packet into heap allocated memory.
            #[cfg(feature = "std")]
            pub fn pack_alloc(&self) -> Result<Vec<u8>, PackError> {
                let mut buf = Vec::with_capacity(self.len());
                self.pack_vec(&mut buf)?;
                Ok(buf)
            }

            /// Packs the packet into the given vector.
            ///
            /// Grows the vector `buf` if necessary.
            #[cfg(feature = "std")]
            pub fn pack_vec(&self, buf: &mut Vec<u8>) -> Result<(), PackError> {
                buf.clear();
                buf.reserve_exact(self.len());
                unsafe {
                    buf.set_len(self.len());
                }
                self.pack(buf)
            }

            /// Packs the packet into the givven buffer.
            pub fn pack(&self, buf: &mut [u8]) -> Result<(), PackError> {
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

            /// The length of the packet when packed.
            pub fn len(&self) -> usize {
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
    };
}

#[cfg(feature = "std")]
impl_acn_root_layer_protocol!(<'a>);

#[cfg(not(feature = "std"))]
impl_acn_root_layer_protocol!();

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

    Ok(PduInfo { length, vector })
}

trait Pdu: Sized {
    fn parse(buf: &[u8]) -> Result<Self, ParseError>;

    fn pack(&self, buf: &mut [u8]) -> Result<(), PackError>;

    fn len(&self) -> usize;
}

const VECTOR_ROOT_E131_DATA: u32 = 0x0000_0004;
const VECTOR_ROOT_E131_EXTENDED: u32 = 0x0000_0008;

macro_rules! impl_e131_root_layer {
    ( $( $lt:tt )* ) => {
        /// Payload of the Root Layer PDU.
        #[derive(Clone, Eq, PartialEq, Hash, Debug)]
        pub enum E131RootLayerData$( $lt )* {
            /// DMX data packet.
            DataPacket(DataPacketFramingLayer$( $lt )*),

            /// Synchronization packet.
            SynchronizationPacket(SynchronizationPacketFramingLayer),

            /// Universe discovery packet.
            UniverseDiscoveryPacket(UniverseDiscoveryPacketFramingLayer$( $lt )*),
        }

        /// Root layer protocol data unit (PDU).
        #[derive(Clone, Eq, PartialEq, Hash, Debug)]
        pub struct E131RootLayer$( $lt )* {
            /// Sender UUID.
            pub cid: Uuid,
            /// Data carried by the Root Layer PDU.
            pub data: E131RootLayerData$( $lt )*,
        }

        impl$( $lt )* Pdu for E131RootLayer$( $lt )* {
            fn parse(buf: &[u8]) -> Result<E131RootLayer$( $lt )*, ParseError> {
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
                        let PduInfo { vector, .. } = pdu_info(&data_buf, 4)?;

                        match vector {
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
                    cid,
                    data,
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
    };
}

#[cfg(feature = "std")]
impl_e131_root_layer!(<'a>);

#[cfg(not(feature = "std"))]
impl_e131_root_layer!();

const VECTOR_E131_DATA_PACKET: u32 = 0x0000_0002;

macro_rules! impl_data_packet_framing_layer {
    ( $( $lt:tt )* ) => {
        /// Framing layer PDU for sACN data packets.
        #[derive(Eq, PartialEq, Debug)]
        pub struct DataPacketFramingLayer$( $lt )* {
            /// The name of the source.
            #[cfg(feature = "std")]
            pub source_name: Cow<'a, str>,
            #[cfg(not(feature = "std"))]
            pub source_name: String<[u8; 64]>,

            /// Priority of this data packet.
            pub priority: u8,

            /// Synchronization adress.
            pub synchronization_address: u16,

            /// The sequence number of this packet.
            pub sequence_number: u8,

            /// If this packets data is preview data.
            pub preview_data: bool,

            /// If transmission on this universe is terminated.
            pub stream_terminated: bool,

            /// Force synchronization if no synchronization packets are received.
            pub force_synchronization: bool,

            /// The universe DMX data is transmitted for.
            pub universe: u16,

            /// DMP layer containing the DMX data.
            pub data: DataPacketDmpLayer$( $lt )*,
        }

        impl$( $lt )* Pdu for DataPacketFramingLayer$( $lt )* {
            fn parse(buf: &[u8]) -> Result<DataPacketFramingLayer$( $lt )*, ParseError> {
                // Length and Vector
                let PduInfo { length, vector } = pdu_info(&buf, 4)?;
                if vector != VECTOR_E131_DATA_PACKET {
                    return Err(ParseError::PduInvalidVector(vector));
                }

                // Source Name
                let source_name = String::from(parse_c_str(&buf[6..70])?);

                // Priority
                let priority = buf[70];

                // Synchronization Address
                let synchronization_address = NetworkEndian::read_u16(&buf[71..73]);

                // Sequence Number
                let sequence_number = buf[73];

                // Options
                let preview_data = buf[74] & 0b0100_0000 != 0;
                let stream_terminated = buf[74] & 0b0010_0000 != 0;
                let force_synchronization = buf[74] & 0b0001_0000 != 0;

                // Universe
                let universe = NetworkEndian::read_u16(&buf[75..77]);

                // Data
                let data = DataPacketDmpLayer::parse(&buf[77..length])?;

                Ok(DataPacketFramingLayer {
                    #[cfg(feature = "std")]
                    source_name: source_name.into(),
                    #[cfg(not(feature = "std"))]
                    source_name,
                    priority,
                    synchronization_address,
                    sequence_number,
                    preview_data,
                    stream_terminated,
                    force_synchronization,
                    universe,
                    data,
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
                    buf[74] = 0b0100_0000
                }

                // Stream Terminated
                if self.stream_terminated {
                    buf[74] |= 0b0010_0000
                }

                // Force Synchronization
                if self.force_synchronization {
                    buf[74] |= 0b0001_0000
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

        impl$( $lt )* Clone for DataPacketFramingLayer$( $lt )* {
            fn clone(&self) -> Self {
                DataPacketFramingLayer {
                    #[cfg(feature = "std")]
                    source_name: self.source_name.clone(),
                    #[cfg(not(feature = "std"))]
                    source_name: self.source_name.as_str().into(),
                    priority: self.priority,
                    synchronization_address: self.synchronization_address,
                    sequence_number: self.sequence_number,
                    preview_data: self.preview_data,
                    stream_terminated: self.stream_terminated,
                    force_synchronization: self.force_synchronization,
                    universe: self.universe,
                    data: self.data.clone(),
                }
            }
        }

        impl$( $lt )* Hash for DataPacketFramingLayer$( $lt )* {
            #[inline]
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                (&*self.source_name).hash(state);
                self.priority.hash(state);
                self.synchronization_address.hash(state);
                self.sequence_number.hash(state);
                self.preview_data.hash(state);
                self.stream_terminated.hash(state);
                self.force_synchronization.hash(state);
                self.universe.hash(state);
                self.data.hash(state);
            }
        }
    };
}

#[cfg(feature = "std")]
impl_data_packet_framing_layer!(<'a>);

#[cfg(not(feature = "std"))]
impl_data_packet_framing_layer!();

const VECTOR_DMP_SET_PROPERTY: u8 = 0x02;

macro_rules! impl_data_packet_dmp_layer {
    ( $( $lt:tt )* ) => {
        /// Device Management Protocol PDU with SET PROPERTY vector.
        ///
        /// Used for sACN data packets.
        #[derive(Eq, PartialEq, Debug)]
        pub struct DataPacketDmpLayer$( $lt )* {
            /// DMX data property values (DMX start coder + 512 slots).
            #[cfg(feature = "std")]
            pub property_values: Cow<'a, [u8]>,
            #[cfg(not(feature = "std"))]
            pub property_values: Vec<u8, [u8; 513]>,
        }

        impl$( $lt )* Pdu for DataPacketDmpLayer$( $lt )* {
            fn parse(buf: &[u8]) -> Result<DataPacketDmpLayer$( $lt )*, ParseError> {
                // Length and Vector
                let PduInfo { length, vector } = pdu_info(&buf, 1)?;
                if vector != u32::from(VECTOR_DMP_SET_PROPERTY) {
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
                let property_values_length = length - 10;
                if property_values_length > 513 {
                    return Err(ParseError::InvalidData("only 512 DMX slots allowed"));
                }

                #[cfg(feature = "std")]
                let mut property_values = Vec::with_capacity(property_values_length);
                #[cfg(not(feature = "std"))]
                let mut property_values = Vec::new();

                #[cfg(feature = "std")]
                property_values.extend_from_slice(&buf[10..length]);
                #[cfg(not(feature = "std"))]
                property_values.extend_from_slice(&buf[10..length]).unwrap();

                Ok(DataPacketDmpLayer {
                    #[cfg(feature = "std")]
                    property_values: property_values.into(),
                    #[cfg(not(feature = "std"))]
                    property_values,
                })
            }

            fn pack(&self, buf: &mut [u8]) -> Result<(), PackError> {
                if self.property_values.len() > 513 {
                    return Err(PackError::InvalidData("only 512 DMX values allowed"));
                }

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
                NetworkEndian::write_u16(&mut buf[8..10], self.property_values.len() as u16);

                // Property values
                buf[10..10 + self.property_values.len()].copy_from_slice(&self.property_values);

                Ok(())
            }

            fn len(&self) -> usize {
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
                self.property_values.len()
            }
        }

        impl$( $lt )* Clone for DataPacketDmpLayer$( $lt )* {
            fn clone(&self) -> Self {
                DataPacketDmpLayer {
                    #[cfg(feature = "std")]
                    property_values: self.property_values.clone(),
                    #[cfg(not(feature = "std"))]
                    property_values: {
                        let mut property_values = Vec::new();
                        property_values
                            .extend_from_slice(&self.property_values)
                            .unwrap();
                        property_values
                    },
                }
            }
        }

        impl$( $lt )* Hash for DataPacketDmpLayer$( $lt )* {
            #[inline]
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                (&*self.property_values).hash(state);
            }
        }
    };
}

#[cfg(feature = "std")]
impl_data_packet_dmp_layer!(<'a>);

#[cfg(not(feature = "std"))]
impl_data_packet_dmp_layer!();

const VECTOR_E131_EXTENDED_SYNCHRONIZATION: u32 = 0x0000_0001;

/// sACN synchronization packet PDU.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Copy)]
pub struct SynchronizationPacketFramingLayer {
    /// The sequence number of the packet.
    pub sequence_number: u8,

    /// The address to synchronize.
    pub synchronization_address: u16,
}

impl Pdu for SynchronizationPacketFramingLayer {
    fn parse(buf: &[u8]) -> Result<SynchronizationPacketFramingLayer, ParseError> {
        // Length and Vector
        let PduInfo { vector, .. } = pdu_info(&buf, 4)?;
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
            sequence_number,
            synchronization_address,
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

const VECTOR_E131_EXTENDED_DISCOVERY: u32 = 0x0000_0002;

macro_rules! impl_universe_discovery_packet_framing_layer {
    ( $( $lt:tt )* ) => {
        /// Framing layer PDU for sACN universe discovery packets.
        #[derive(Eq, PartialEq, Debug)]
        pub struct UniverseDiscoveryPacketFramingLayer$( $lt )* {
            /// Name of the source.
            #[cfg(feature = "std")]
            pub source_name: Cow<'a, str>,
            #[cfg(not(feature = "std"))]
            pub source_name: String<[u8; 64]>,

            /// Universe dicovery layer.
            pub data: UniverseDiscoveryPacketUniverseDiscoveryLayer$( $lt )*,
        }

        impl$( $lt )* Pdu for UniverseDiscoveryPacketFramingLayer$( $lt )* {
            fn parse(buf: &[u8]) -> Result<UniverseDiscoveryPacketFramingLayer$( $lt )*, ParseError> {
                // Length and Vector
                let PduInfo { length, vector } = pdu_info(&buf, 4)?;
                if vector != VECTOR_E131_EXTENDED_DISCOVERY {
                    return Err(ParseError::PduInvalidVector(vector));
                }

                // Source Name
                let source_name = String::from(parse_c_str(&buf[6..70])?);

                // Reserved
                if buf[70..74] != [0, 0, 0, 0] {
                    return Err(ParseError::InvalidData("invalid Reserved"));
                }

                // Data
                let data = UniverseDiscoveryPacketUniverseDiscoveryLayer::parse(&buf[74..length])?;

                Ok(UniverseDiscoveryPacketFramingLayer {
                    #[cfg(feature = "std")]
                    source_name: source_name.into(),
                    #[cfg(not(feature = "std"))]
                    source_name,
                    data,
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

        impl$( $lt )* Clone for UniverseDiscoveryPacketFramingLayer$( $lt )* {
            fn clone(&self) -> Self {
                UniverseDiscoveryPacketFramingLayer {
                    #[cfg(feature = "std")]
                    source_name: self.source_name.clone(),
                    #[cfg(not(feature = "std"))]
                    source_name: self.source_name.as_str().into(),
                    data: self.data.clone(),
                }
            }
        }

        impl$( $lt )* Hash for UniverseDiscoveryPacketFramingLayer$( $lt )* {
            #[inline]
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                (&*self.source_name).hash(state);
                self.data.hash(state);
            }
        }
    };
}

#[cfg(feature = "std")]
impl_universe_discovery_packet_framing_layer!(<'a>);

#[cfg(not(feature = "std"))]
impl_universe_discovery_packet_framing_layer!();

const VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST: u32 = 0x0000_0001;

macro_rules! impl_universe_discovery_packet_universe_discovery_layer {
    ( $( $lt:tt )* ) => {
        /// Universe discovery layer PDU.
        #[derive(Eq, PartialEq, Debug)]
        pub struct UniverseDiscoveryPacketUniverseDiscoveryLayer$( $lt )* {
            /// Current page of the dicovery packet.
            pub page: u8,

            /// The number of the final page.
            pub last_page: u8,

            /// List of universes.
            #[cfg(feature = "std")]
            pub universes: Cow<'a, [u16]>,
            #[cfg(not(feature = "std"))]
            pub universes: Vec<u16, [u16; 512]>,
        }

        impl$( $lt )* Pdu for UniverseDiscoveryPacketUniverseDiscoveryLayer$( $lt )* {
            fn parse(buf: &[u8]) -> Result<UniverseDiscoveryPacketUniverseDiscoveryLayer$( $lt )*, ParseError> {
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
                let universes_length = (length - 8) / 2;
                #[cfg(feature = "std")]
                let mut universes = Vec::with_capacity(universes_length);
                #[cfg(not(feature = "std"))]
                let mut universes = Vec::new();

                #[cfg(feature = "std")]
                universes.resize(universes_length, 0);
                #[cfg(not(feature = "std"))]
                universes.resize(universes_length, 0).unwrap();

                NetworkEndian::read_u16_into(&buf[8..length], &mut universes[..universes_length]);

                Ok(UniverseDiscoveryPacketUniverseDiscoveryLayer {
                    page,
                    last_page,
                    #[cfg(feature = "std")]
                    universes: universes.into(),
                    #[cfg(not(feature = "std"))]
                    universes,
                })
            }

            fn pack(&self, buf: &mut [u8]) -> Result<(), PackError> {
                if self.universes.len() > 512 {
                    return Err(PackError::InvalidData("only 512 universes allowed"));
                }

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
                    if self.universes[i] == self.universes[i - 1] {
                        return Err(PackError::InvalidData("Universes are not unique"));
                    }
                    if self.universes[i] <= self.universes[i - 1] {
                        return Err(PackError::InvalidData("Universes are not sorted"));
                    }
                }
                NetworkEndian::write_u16_into(
                    &self.universes[..self.universes.len()],
                    &mut buf[8..8 + self.universes.len() * 2],
                );

                Ok(())
            }

            fn len(&self) -> usize {
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

        impl$( $lt )* Clone for UniverseDiscoveryPacketUniverseDiscoveryLayer$( $lt )* {
            fn clone(&self) -> Self {
                UniverseDiscoveryPacketUniverseDiscoveryLayer {
                    page: self.page,
                    last_page: self.last_page,
                    #[cfg(feature = "std")]
                    universes: self.universes.clone(),
                    #[cfg(not(feature = "std"))]
                    universes: {
                        let mut universes = Vec::new();
                        universes.extend_from_slice(&self.universes[..]).unwrap();
                        universes
                    },
                }
            }
        }

        impl$( $lt )* Hash for UniverseDiscoveryPacketUniverseDiscoveryLayer$( $lt )* {
            #[inline]
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                self.page.hash(state);
                self.last_page.hash(state);
                (&*self.universes).hash(state);
            }
        }
    };
}

#[cfg(feature = "std")]
impl_universe_discovery_packet_universe_discovery_layer!(<'a>);

#[cfg(not(feature = "std"))]
impl_universe_discovery_packet_universe_discovery_layer!();

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
}
