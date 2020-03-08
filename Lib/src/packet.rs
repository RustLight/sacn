// Copyright 2018 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was modified as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

#![warn(missing_docs)]

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

// Uses the sACN error-chain errors.
use error::errors::*;
use error::errors::ErrorKind::*;

/// The core crate is used for string processing during packet parsing/packing aswell as to provide access to the Hash trait.
use core::hash::{self, Hash};
use core::str;

use std::borrow::Cow;
use std::vec::Vec;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::time::Duration;

/// The byteorder crate is used for marshalling data on/off the network in Network Byte Order.
use byteorder::{ByteOrder, NetworkEndian};

/// The uuid crate is used for working with/generating UUIDs which sACN uses as part of the cid field in the protocol.
use uuid::Uuid;

/// The maximum number of universes per page in a universe discovery packet.
pub const DISCOVERY_UNI_PER_PAGE: usize = 512;

/// The universe used for universe discovery as defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative)
pub const E131_DISCOVERY_UNIVERSE: u16 = 64214;

/// The default priority used for the E1.31 packet priority field, as per ANSI E1.31-2018 Section 4.1 Table 4-1
pub const E131_DEFAULT_PRIORITY: u8 = 100;

/// The maximum allowed priority for a E1.31 packet, as per ANSI E1.31-2018 Section 6.2.3
pub const E131_MAX_PRIORITY: u8 = 200;

/// Value of the highest byte of the IPV4 multicast address as specified in section 9.3.1 of ANSI E1.31-2018.
pub const E131_MULTICAST_IPV4_HIGHEST_BYTE: u8 = 239;

/// Value of the second highest byte of the IPV4 multicast address as specified in section 9.3.1 of ANSI E1.31-2018.
pub const E131_MULTICAST_IPV4_SECOND_BYTE: u8 = 255;

/// The maximum universe number that can be used with the E1.31 protocol as specified in section 9.1.1 of ANSI E1.31-2018.
pub const E131_MAX_MULTICAST_UNIVERSE: u16 = 63999;

/// The lowest / minimum universe number that can be used with the E1.31 protocol as specified in section 9.1.1 of ANSI E1.31-2018.
pub const E131_MIN_MULTICAST_UNIVERSE: u16 = 1;

/// The vector field value used to identify the ACN packet as an ANSI E1.31 data packet.
/// This is used at the ACN packet layer not the E1.31 layer.
/// Value as defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative).
const VECTOR_ROOT_E131_DATA: u32 = 0x0000_0004;

/// The vector field value used to identify the packet as an ANSI E1.31 universe discovery or synchronisation packet.
/// This is used at the ACN packet layer not the E1.31 layer.
/// Value as defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative).
const VECTOR_ROOT_E131_EXTENDED: u32 = 0x0000_0008;

/// The E1.31 packet vector field value used to identify the E1.31 packet as a synchronisation packet.
/// This is used at the E1.31 layer and shouldn't be confused with the VECTOR values used for the ACN layer (i.e. VECTOR_ROOT_E131_DATA and VECTOR_ROOT_E131_EXTENDED).
/// Value as defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative).
const VECTOR_E131_EXTENDED_SYNCHRONIZATION: u32 = 0x0000_0001;

/// The E1.31 packet vector field value used to identify the E1.31 packet as a universe discovery packet.
/// This is used at the E1.31 layer and shouldn't be confused with the VECTOR values used for the ACN layer (i.e. VECTOR_ROOT_E131_DATA and VECTOR_ROOT_E131_EXTENDED).
/// This VECTOR value is shared by E1.31 data packets, distinguished by the value of the ACN ROOT_VECTOR.
/// Value as defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative).
const VECTOR_E131_EXTENDED_DISCOVERY: u32 = 0x0000_0002;

/// The E1.31 packet vector field value used to identify the E1.31 packet as a data packet.
/// This is used at the E1.31 layer and shouldn't be confused with the VECTOR values used for the ACN layer (i.e. VECTOR_ROOT_E131_DATA and VECTOR_ROOT_E131_EXTENDED).
/// This VECTOR value is shared by E1.31 universe discovery packets, distinguished by the value of the ACN ROOT_VECTOR.
/// Value as defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative).
const VECTOR_E131_DATA_PACKET: u32 = 0x0000_0002;

/// Used at the DMP layer in E1.31 data packets to identify the packet as a set property message.
/// Not to be confused with the other VECTOR values used at the E1.31, ACN etc. layers.
/// Value as defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative).
const VECTOR_DMP_SET_PROPERTY: u8 = 0x02;

/// Used at the universe discovery packet universe discovery layer to identify the packet as a universe discovery list of universes.
/// Not to be confused with the other VECTOR values used at the E1.31, ACN, DMP, etc. layers.
/// Value as defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative).
const VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST: u32 = 0x0000_0001;

/// The port number used for the ACN family of protocols and therefore the sACN protocol.
/// As defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative)
pub const ACN_SDT_MULTICAST_PORT: u16 = 5568; 

/// The payload capacity for a sacn packet, for DMX data this would translate to 512 frames + a startcode byte.
pub const UNIVERSE_CHANNEL_CAPACITY: usize = 513;

/// The synchronisation universe/address of packets which do not require synchronisation as specified in section 6.2.4.1 of ANSI E1.31-2018.
pub const NO_SYNC_UNIVERSE: u16 = 0;

/// The timeout before data loss is assumed for an E131 source, as defined in Apendix A of ANSI E1.31-2018.
pub const E131_NETWORK_DATA_LOSS_TIMEOUT: Duration = Duration::from_millis(2500);

/// The timeout before a discovered source is assumed to be lost as defined in section 12.2 of ANSI E1.31-2018.
pub const UNIVERSE_DISCOVERY_SOURCE_TIMEOUT: Duration = E131_NETWORK_DATA_LOSS_TIMEOUT;

/// Converts the given ANSI E1.31-2018 universe into an Ipv4 multicast address with the port set to the acn multicast port as defined 
/// in packet::ACN_SDT_MULTICAST_PORT.
/// 
/// Converstion done as specified in section 9.3.1 of ANSI E1.31-2018
///
/// Returns the multicast address.
/// 
/// # Errors
/// Returns an ErrorKind::IllegalUniverse error if the given universe is outwith the allowed range of universes which is
/// [E131_MIN_MULTICAST_UNIVERSE - E131_MAX_MULTICAST_UNIVERSE] inclusive excluding the discovery universe, E131_DISCOVERY_UNIVERSE.
pub fn universe_to_ipv4_multicast_addr(universe: u16) -> Result<SocketAddr>{
    if (universe != E131_DISCOVERY_UNIVERSE) && (universe < E131_MIN_MULTICAST_UNIVERSE || universe > E131_MAX_MULTICAST_UNIVERSE) {
        bail!(ErrorKind::IllegalUniverse( format!("Universe to convert: {} is out of allowed range", universe)));
    }

    let high_byte: u8 = ((universe >> 8) & 0xff) as u8;
    let low_byte: u8 = (universe & 0xff) as u8;

    // As per ANSI E1.31-2018 Section 9.3.1 Table 9-10.
    Ok(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(239, 255, high_byte, low_byte)), ACN_SDT_MULTICAST_PORT))
}

/// Converts the given ANSI E1.31-2018 universe into an Ipv6 multicast address with the port set to the acn multicast port as defined 
/// in packet::ACN_SDT_MULTICAST_PORT.
/// 
/// Converstion done as specified in section 9.3.2 of ANSI E1.31-2018
/// 
/// Returns the multicast address.
/// 
/// # Errors
/// Returns an ErrorKind::IllegalUniverse error if the given universe is outwith the allowed range of universes which is
/// [E131_MIN_MULTICAST_UNIVERSE - E131_MAX_MULTICAST_UNIVERSE] inclusive excluding the discovery universe, E131_DISCOVERY_UNIVERSE.
pub fn universe_to_ipv6_multicast_addr(universe: u16) -> Result<SocketAddr>{
    if (universe != E131_DISCOVERY_UNIVERSE) && (universe < E131_MIN_MULTICAST_UNIVERSE || universe > E131_MAX_MULTICAST_UNIVERSE) {
        bail!(ErrorKind::IllegalUniverse( format!("Universe to convert: {} is out of allowed range", universe)));
    }

    // As per ANSI E1.31-2018 Section 9.3.2 Table 9-12.
    Ok(SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0xFF18, 0, 0, 0, 0, 0, 0x8300, universe)), ACN_SDT_MULTICAST_PORT))
}

/// Fills the given array of bytes with the given length n with bytes of value 0.
#[inline]
fn zeros(buf: &mut [u8], n: usize) {
    for b in buf.iter_mut().take(n) {
        *b = 0;
    }
}

/// Takes the given byte buffer (e.g. a c char array) and parses it into a rust &str.
#[inline]
fn parse_c_str(buf: &[u8]) -> Result<&str> {
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
            pub fn parse(buf: &[u8]) -> Result<AcnRootLayerProtocol> {
                // Preamble Size
                if NetworkEndian::read_u16(&buf[0..2]) != 0x0010 {
                    bail!(ErrorKind::ParseInvalidData("invalid Preamble Size".to_string()));
                }

                // Post-amble Size
                if NetworkEndian::read_u16(&buf[2..4]) != 0 {
                    bail!(ErrorKind::ParseInvalidData("invalid Post-amble Size".to_string()));
                }

                // ACN Packet Identifier
                if &buf[4..16] != b"ASC-E1.17\x00\x00\x00" {
                    bail!(ErrorKind::ParseInvalidData("invalid ACN packet indentifier".to_string()));
                }

                // PDU block
                Ok(AcnRootLayerProtocol {
                    pdu: E131RootLayer::parse(&buf[16..])?,
                })
            }

            /// Packs the packet into heap allocated memory.
            #[cfg(feature = "std")]
            pub fn pack_alloc(&self) -> Result<Vec<u8>> {
                let mut buf = Vec::with_capacity(self.len());
                self.pack_vec(&mut buf)?;
                Ok(buf)
            }

            /// Packs the packet into the given vector.
            ///
            /// Grows the vector `buf` if necessary.
            #[cfg(feature = "std")]
            pub fn pack_vec(&self, buf: &mut Vec<u8>) -> Result<()> {
                buf.clear();
                buf.reserve_exact(self.len());
                unsafe {
                    buf.set_len(self.len());
                }
                self.pack(buf)
            }

            /// Packs the packet into the given buffer.
            pub fn pack(&self, buf: &mut [u8]) -> Result<()> {
                if buf.len() < self.len() {
                    bail!(ErrorKind::PackBufferInsufficient("".to_string()))
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

fn pdu_info(buf: &[u8], vector_length: usize) -> Result<PduInfo> {
    if buf.len() < 2 {
        bail!(ErrorKind::ParseInsufficientData("".to_string()));
    }

    // Flags
    let flags = buf[0] & 0xf0;
    if flags != 0x70 {
        bail!(ErrorKind::ParsePduInvalidFlags(flags));
    }
    // Length
    let length = (NetworkEndian::read_u16(&buf[0..2]) & 0x0fff) as usize;
    if buf.len() < length {
        bail!(ErrorKind::ParseInsufficientData("Insufficient data when parsing pdu_info".to_string()));
    }

    // Vector
    let vector = NetworkEndian::read_uint(&buf[2..], vector_length) as u32;

    Ok(PduInfo { length, vector })
}

trait Pdu: Sized {
    fn parse(buf: &[u8]) -> Result<Self>;

    fn pack(&self, buf: &mut [u8]) -> Result<()>;

    fn len(&self) -> usize;
}

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
            fn parse(buf: &[u8]) -> Result<E131RootLayer$( $lt )*> {
                // Length and Vector
                let PduInfo { length, vector } = pdu_info(&buf, 4)?;
                if vector != VECTOR_ROOT_E131_DATA && vector != VECTOR_ROOT_E131_EXTENDED {
                    bail!(ErrorKind::PduInvalidVector(vector));
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

                            vector => bail!(ErrorKind::PduInvalidVector(vector)),
                        }
                    }
                    vector => bail!(ErrorKind::PduInvalidVector(vector)),
                };

                Ok(E131RootLayer {
                    cid,
                    data,
                })
            }

            fn pack(&self, buf: &mut [u8]) -> Result<()> {
                if buf.len() < self.len() {
                    bail!(ErrorKind::PackBufferInsufficient("".to_string()));
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
            fn parse(buf: &[u8]) -> Result<DataPacketFramingLayer$( $lt )*> {
                // Length and Vector
                let PduInfo { length, vector } = pdu_info(&buf, 4)?;
                if vector != VECTOR_E131_DATA_PACKET {
                    bail!(ErrorKind::PduInvalidVector(vector));
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

            fn pack(&self, buf: &mut [u8]) -> Result<()> {
                if buf.len() < self.len() {
                    bail!(ErrorKind::PackBufferInsufficient("".to_string()));
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
            fn parse(buf: &[u8]) -> Result<DataPacketDmpLayer$( $lt )*> {
                // Length and Vector
                let PduInfo { length, vector } = pdu_info(&buf, 1)?;
                if vector != u32::from(VECTOR_DMP_SET_PROPERTY) {
                    bail!(ErrorKind::PduInvalidVector(vector));
                }

                // Address and Data Type
                if buf[3] != 0xa1 {
                    bail!(ErrorKind::ParseInvalidData("invalid Address and Data Type".to_string()));
                }

                // First Property Address
                if NetworkEndian::read_u16(&buf[4..6]) != 0 {
                    bail!(ErrorKind::ParseInvalidData("invalid First Property Address".to_string()));
                }

                // Address Increment
                if NetworkEndian::read_u16(&buf[6..8]) != 0x0001 {
                    bail!(ErrorKind::ParseInvalidData("invalid Address Increment".to_string()));
                }

                // Property value count
                if NetworkEndian::read_u16(&buf[8..10]) as usize + 10 != length {
                    bail!(ErrorKind::ParseInvalidData("invalid Property value count".to_string()));
                }

                // Property values
                let property_values_length = length - 10;
                if property_values_length > 513 {
                    bail!(ErrorKind::ParseInvalidData("only 512 DMX slots allowed".to_string()));
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

            fn pack(&self, buf: &mut [u8]) -> Result<()> {
                if self.property_values.len() > 513 {
                    bail!(ErrorKind::PackInvalidData("only 512 DMX values allowed".to_string()));
                }

                if buf.len() < self.len() {
                    bail!(ErrorKind::PackBufferInsufficient("".to_string()));
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
                // Address Increment, PackError>
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

/// sACN synchronization packet PDU.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Copy)]
pub struct SynchronizationPacketFramingLayer {
    /// The sequence number of the packet.
    pub sequence_number: u8,

    /// The address to synchronize.
    pub synchronization_address: u16,
}

impl Pdu for SynchronizationPacketFramingLayer {
    fn parse(buf: &[u8]) -> Result<SynchronizationPacketFramingLayer> {
        // Length and Vector
        let PduInfo { vector, .. } = pdu_info(&buf, 4)?;
        if vector != VECTOR_E131_EXTENDED_SYNCHRONIZATION {
            bail!(ErrorKind::PduInvalidVector(vector));
        }

        // Sequence Number
        let sequence_number = buf[6];

        // Synchronization Address
        let synchronization_address = NetworkEndian::read_u16(&buf[7..9]);

        // Reserved
        if buf[9..11] != [0, 0] {
            bail!(ErrorKind::ParseInvalidData("Reserved data is invalid and couldn't be parsed".to_string()));
        }

        Ok(SynchronizationPacketFramingLayer {
            sequence_number,
            synchronization_address,
        })
    }

    fn pack(&self, buf: &mut [u8]) -> Result<()> {
        if buf.len() < self.len() {
            bail!(ErrorKind::PackBufferInsufficient("".to_string()));
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
            fn parse(buf: &[u8]) -> Result<UniverseDiscoveryPacketFramingLayer$( $lt )*> {
                // Length and Vector
                let PduInfo { length, vector } = pdu_info(&buf, 4)?;
                if vector != VECTOR_E131_EXTENDED_DISCOVERY {
                    bail!(ErrorKind::PduInvalidVector(vector));
                }

                // Source Name
                let source_name = String::from(parse_c_str(&buf[6..70])?);

                // Reserved
                if buf[70..74] != [0, 0, 0, 0] {
                    bail!(ErrorKind::ParseInvalidData("Reserved data invalid and couldn't be parsed".to_string()));
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

            fn pack(&self, buf: &mut [u8]) -> Result<()> {
                if buf.len() < self.len() {
                    bail!(ErrorKind::PackBufferInsufficient("".to_string()));
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
            fn parse(buf: &[u8]) -> Result<UniverseDiscoveryPacketUniverseDiscoveryLayer$( $lt )*> {
                // Length and Vector
                let PduInfo { length, vector } = pdu_info(&buf, 4)?;
                if vector != VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST {
                    bail!(ErrorKind::PduInvalidVector(vector));
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

            fn pack(&self, buf: &mut [u8]) -> Result<()> {
                if self.universes.len() > 512 {
                    bail!(ErrorKind::PackInvalidData("only 512 universes allowed".to_string()));
                }

                if buf.len() < self.len() {
                    bail!(ErrorKind::PackBufferInsufficient("".to_string()));
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
                        bail!(ErrorKind::PackInvalidData("Universes are not unique".to_string()));
                    }
                    if self.universes[i] <= self.universes[i - 1] {
                        bail!(ErrorKind::PackInvalidData("Universes are not sorted".to_string()));
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

#[test]
fn test_universe_to_ipv4_lowest_byte_normal(){
    let val: u16 = 119;
    let res = universe_to_ipv4_multicast_addr(val).unwrap();
    
    assert!(res.ip().is_multicast());

    assert_eq!(res, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(239, 255, (val/256) as u8, (val % 256) as u8)), ACN_SDT_MULTICAST_PORT));
}

#[test]
fn test_universe_to_ip_ipv4_both_bytes_normal(){
    let val: u16 = 300;
    let res = universe_to_ipv4_multicast_addr(val).unwrap();
    
    assert!(res.ip().is_multicast());

    assert_eq!(res, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(239, 255, (val/256) as u8, (val % 256) as u8)), ACN_SDT_MULTICAST_PORT));
}

#[test]
fn test_universe_to_ip_ipv4_limit_high(){
    let res = universe_to_ipv4_multicast_addr(E131_MAX_MULTICAST_UNIVERSE).unwrap();
    
    assert!(res.ip().is_multicast());

    assert_eq!(res, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(239, 255, (E131_MAX_MULTICAST_UNIVERSE/256) as u8, (E131_MAX_MULTICAST_UNIVERSE % 256) as u8)), ACN_SDT_MULTICAST_PORT));
}

#[test]
fn test_universe_to_ip_ipv4_limit_low(){
    let res = universe_to_ipv4_multicast_addr(E131_MIN_MULTICAST_UNIVERSE).unwrap();

    assert!(res.ip().is_multicast());

    assert_eq!(res, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(239, 255, (E131_MIN_MULTICAST_UNIVERSE/256) as u8, (E131_MIN_MULTICAST_UNIVERSE % 256) as u8)), ACN_SDT_MULTICAST_PORT));
}

#[test]
fn test_universe_to_ip_ipv4_out_range_low(){
    match universe_to_ipv4_multicast_addr(0) {
        Ok(_) => {assert!(false, "Universe to ipv4 multicast allowed below minimum allowed universe")},
        Err(e) => 
            match e.kind() {
                &ErrorKind::IllegalUniverse(ref s) => assert!(true),
                _ => assert!(false, "Unexpected error type returned")
            }
    }
}

#[test]
fn test_universe_to_ip_ipv4_out_range_high(){
    match universe_to_ipv4_multicast_addr(E131_MAX_MULTICAST_UNIVERSE + 10) {
        Ok(_) => {assert!(false, "Universe to ipv4 multicast allowed above maximum allowed universe")},
        Err(e) => 
            match e.kind() {
                &ErrorKind::IllegalUniverse(ref s) => assert!(true),
                _ => assert!(false, "Unexpected error type returned")
            }
    }
}

#[test]
fn test_universe_to_ipv6_lowest_byte_normal(){
    let val: u16 = 119;
    let res = universe_to_ipv6_multicast_addr(val).unwrap();

    assert!(res.ip().is_multicast());

    let low_16: u16 = (((val/256) as u16) << 8) | ((val % 256) as u16);
    
    assert_eq!(res, SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0xFF18, 0, 0, 0, 0, 0, 0x8300, low_16)), ACN_SDT_MULTICAST_PORT));
}

#[test]
fn test_universe_to_ip_ipv6_both_bytes_normal(){
    let val: u16 = 300;
    let res = universe_to_ipv6_multicast_addr(val).unwrap();

    assert!(res.ip().is_multicast());

    let low_16: u16 = (((val/256) as u16) << 8) | ((val % 256) as u16);
    
    assert_eq!(res, SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0xFF18, 0, 0, 0, 0, 0, 0x8300, low_16)), ACN_SDT_MULTICAST_PORT));
}

#[test]
fn test_universe_to_ip_ipv6_limit_high(){
    let res = universe_to_ipv6_multicast_addr(E131_MAX_MULTICAST_UNIVERSE).unwrap();

    assert!(res.ip().is_multicast());

    let low_16: u16 = (((E131_MAX_MULTICAST_UNIVERSE/256) as u16) << 8) | ((E131_MAX_MULTICAST_UNIVERSE % 256) as u16);
    
    assert_eq!(res, SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0xFF18, 0, 0, 0, 0, 0, 0x8300, low_16)), ACN_SDT_MULTICAST_PORT));
}

#[test]
fn test_universe_to_ip_ipv6_limit_low(){
    let res = universe_to_ipv6_multicast_addr(E131_MIN_MULTICAST_UNIVERSE).unwrap();

    assert!(res.ip().is_multicast());

    let low_16: u16 = (((E131_MIN_MULTICAST_UNIVERSE/256) as u16) << 8) | ((E131_MIN_MULTICAST_UNIVERSE % 256) as u16);
    
    assert_eq!(res, SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0xFF18, 0, 0, 0, 0, 0, 0x8300, low_16)), ACN_SDT_MULTICAST_PORT));
}

#[test]
fn test_universe_to_ip_ipv6_out_range_low(){
    match universe_to_ipv6_multicast_addr(0) {
        Ok(_) => {assert!(false, "Universe to ipv4 multicast allowed below minimum allowed universe")},
        Err(e) => 
            match e.kind() {
                &ErrorKind::IllegalUniverse(ref s) => assert!(true),
                _ => assert!(false, "Unexpected error type returned")
            }
    }
}

#[test]
fn test_universe_to_ip_ipv6_out_range_high(){
    match universe_to_ipv6_multicast_addr(E131_MAX_MULTICAST_UNIVERSE + 10) {
        Ok(_) => {assert!(false, "Universe to ipv4 multicast allowed above maximum allowed universe")},
        Err(e) => 
            match e.kind() {
                &ErrorKind::IllegalUniverse(ref s) => assert!(true),
                _ => assert!(false, "Unexpected error type returned")
            }
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
