// Copyright 2020 sacn Developers
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

/// Uses the sACN error-chain errors.
use error::errors::*;
use sacn_parse_pack_error::sacn_parse_pack_error;

/// The core crate is used for string processing during packet parsing/packing aswell as to provide access to the Hash trait.
use core::hash::{self, Hash};
use core::str;

use std::borrow::Cow;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::vec::Vec;
use std::{time, time::Duration};

use socket2::SockAddr;

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

/// The synchronisation address used to indicate that there is no synchronisation required for the data packet.
/// As defined in ANSI E1.31-2018 Section 6.2.4.1
pub const E131_NO_SYNC_ADDR: u16 = 0;

/// The interval between universe discovery packets (adverts) as defined by ANSI E1.31-2018 Appendix A.
pub const E131_UNIVERSE_DISCOVERY_INTERVAL: Duration = time::Duration::from_secs(10);

/// The exclusive lower bound on the different between the received and expected sequence numbers within which a
/// packet will be discarded. Outside of the range specified by (E131_SEQ_DIFF_DISCARD_LOWER_BOUND, E131_SEQ_DIFF_DISCARD_UPPER_BOUND]
/// the packet won't be discarded.
///
/// Having a range allows receivers to catch up if packets are lost.
/// Value as specified in ANSI E1.31-2018 Section 6.7.2 Sequence Numbering.
pub const E131_SEQ_DIFF_DISCARD_LOWER_BOUND: isize = -20;

/// The inclusive upper bound on the different between the received and expected sequence numbers within which a
/// packet will be discarded. Outside of the range specified by (E131_SEQ_DIFF_DISCARD_LOWER_BOUND, E131_SEQ_DIFF_DISCARD_UPPER_BOUND]
/// the packet won't be discarded.
///
/// Having a range allows receivers to catch up if packets are lost.
/// Value as specified in ANSI E1.31-2018 Section 6.7.2 Sequence Numbering.
pub const E131_SEQ_DIFF_DISCARD_UPPER_BOUND: isize = 0;

/// The bit mask used to get the preview-data option within the packet option field as per
/// ANSI E1.31-2018 Section 6.2.6
pub const E131_PREVIEW_DATA_OPTION_BIT_MASK: u8 = 0b1000_0000;

/// The bit mask used to get the stream-termination option within the packet option field as per
/// ANSI E1.31-2018 Section 6.2.6
pub const E131_STREAM_TERMINATION_OPTION_BIT_MASK: u8 = 0b0100_0000;

/// The bit mask used to get the force-synchronisation option within the packet option field as per
/// ANSI E1.31-2018 Section 6.2.6
pub const E131_FORCE_SYNCHRONISATION_OPTION_BIT_MASK: u8 = 0b0010_0000;

/// The minimum allowed length of the discovery layer of an ANSI E1.31-2018 universe discovery packet.
/// As per ANSI E1.31-2018 Section 8 Table 8-9.
pub const E131_UNIVERSE_DISCOVERY_LAYER_MIN_LENGTH: usize = 8;

/// The maximum allowed length of the discovery layer of an ANSI E1.31-2018 universe discovery packet.
/// As per ANSI E1.31-2018 Section 8 Table 8-9.
pub const E131_UNIVERSE_DISCOVERY_LAYER_MAX_LENGTH: usize = 1032;

/// The expected value of the root layer length field for a synchronisation packet.
/// 33 bytes as per ANSI E1.31-2018 Section 4.2 Table 4-2.
pub const E131_UNIVERSE_SYNC_PACKET_ROOT_LENGTH: usize = 33;

/// The expected value of the framing layer length field for a synchronisation packet.
/// 11 bytes as per ANSI E1.31-2018 Section 4.2 Table 4-2.
pub const E131_UNIVERSE_SYNC_PACKET_FRAMING_LAYER_LENGTH: usize = 11;

/// The minimum expected value of the framing layer length field for a discovery packet.
/// 84 bytes as per ANSI E1.31-2018 Section 4.3 Table 4-3.
pub const E131_UNIVERSE_DISCOVERY_FRAMING_LAYER_MIN_LENGTH: usize = 82;

/// The number of stream termination packets sent when a source terminates a stream.
/// Set to 3 as per section 6.2.6 , Stream_Terminated: Bit 6 of ANSI E1.31-2018.
pub const E131_TERMINATE_STREAM_PACKET_COUNT: usize = 3;

/// The length of the pdu flags and length field in bytes.
pub const E131_PDU_LENGTH_FLAGS_LENGTH: usize = 2;

/// The pdu flags expected for an ANSI E1.31-2018 packet as per ANSI E1.31-2018 Section 4 Table 4-1, 4-2, 4-3.
pub const E131_PDU_FLAGS: u8 = 0x70;

/// The length in bytes of the root layer vector field as per ANSI E1.31-2018 Section 4 Table 4-1, 4-2, 4-3.
pub const E131_ROOT_LAYER_VECTOR_LENGTH: usize = 4;

/// The length in bytes of the E1.31 framing layer vector field as per ANSI E1.31-2018 Section 4 Table 4-1, 4-2, 4-3.
pub const E131_FRAMING_LAYER_VECTOR_LENGTH: usize = 4;

/// The length in bytes of the priority field within an ANSI E1.31-2018 data packet as defined in ANSI E1.31-2018 Section 4, Table 4-1.
const E131_PRIORITY_FIELD_LENGTH: usize = 1;

/// The length in bytes of the sequence number field within an ANSI E1.31-2018 packet as defined in ANSI E1.31-2018 Section 4, Table 4-1, 4-2.
const E131_SEQ_NUM_FIELD_LENGTH: usize = 1;

/// The length in bytes of the options field within an ANSI E1.31-2018 data packet as defined in ANSI E1.31-2018 Section 4, Table 4-1.
const E131_OPTIONS_FIELD_LENGTH: usize = 1;

/// The length in bytes of a universe field within an ANSI E1.31-2018 packet as defined in ANSI E1.31-2018 Section 4, Table 4-1, 4-3. 
const E131_UNIVERSE_FIELD_LENGTH: usize = 2;

/// The length in bytes of the Vector field within the DMP layer of an ANSI E1.31-2018 data packet as per ANSI E1.31-2018
/// Section 4, Table 4-1.
const E131_DATA_PACKET_DMP_LAYER_VECTOR_FIELD_LENGTH: usize = 1;

/// The length in bytes of the "Address Type and Data Type" field within an ANSI E1.31-2018 data packet DMP layer as per 
/// ANSI E1.31-2018 Section 4, Table 4-1.
const E131_DATA_PACKET_DMP_LAYER_ADDRESS_DATA_FIELD_LENGTH: usize = 1;

/// The length in bytes of the "First Property Address" field within an ANSI E1.31-2018 data packet DMP layer as per 
/// ANSI E1.31-2018 Section 4, Table 4-1.
const E131_DATA_PACKET_DMP_LAYER_FIRST_PROPERTY_ADDRESS_FIELD_LENGTH: usize = 2;

/// The length in bytes of the "Address Increment" field within an ANSI E1.31-2018 data packet DMP layer as per 
/// ANSI E1.31-2018 Section 4, Table 4-1.
const E131_DATA_PACKET_DMP_LAYER_ADDRESS_INCREMENT_FIELD_LENGTH: usize = 2;

/// The length in bytes of the "Property value count" field within an ANSI E1.31-2018 data packet DMP layer as per 
/// ANSI E1.31-2018 Section 4, Table 4-1.
const E131_DATA_PACKET_DMP_LAYER_PROPERTY_VALUE_COUNT_FIELD_LENGTH: usize = 2;

/// The value of the "Address Type and Data Type" field within an ANSI E1.31-2018 data packet DMP layer as per ANSI E1.31-2018 
/// Section 4, Table 4-1.
const E131_DMP_LAYER_ADDRESS_DATA_FIELD: u8 = 0xa1;

/// The value of the "First Property Address" field within an ANSI E1.31-2018 data packet DMP layer as per ANSI E1.31-2018 
/// Section 4, Table 4-1.
const E131_DATA_PACKET_DMP_LAYER_FIRST_PROPERTY_FIELD: u16 = 0x0000;

/// The value of the "Address Increment" field within an ANSI E1.31-2018 data packet DMP layer as per ANSI E1.31-2018 
/// Section 4, Table 4-1.
const E131_DATA_PACKET_DMP_LAYER_ADDRESS_INCREMENT: u16 = 0x0001;

/// The size of the ACN root layer preamble, must be 0x0010 bytes as per ANSI E1.31-2018 Section 5.1.
/// Often treated as a usize for comparison or use with arrays however stored as u16 as this represents its field size
/// within a packet and converting u16 -> usize is always safe as len(usize) is always greater than len(u16), usize -> u16 is unsafe.
const E131_PREAMBLE_SIZE: u16 = 0x0010;

/// The size of the ACN root layer postamble, must be 0x0 bytes as per ANSI E1.31-2018 Section 5.2.
const E131_POSTAMBLE_SIZE: u16 = 0x0;

/// The E131 ACN packet identifier field value. Must be 0x41 0x53 0x43 0x2d 0x45 0x31 0x2e 0x31 0x37 0x00 0x00 0x00 as per
/// ANSI E1.31-2018 Section 5.3.
const E131_ACN_PACKET_IDENTIFIER: [u8; 12] = [0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00];

/// The E131 CID field length in bytes as per ANSI E1.31-2018 Section 4 Table 4-1, 4-2, 4-3.
pub const E131_CID_FIELD_LENGTH: usize = 16;

// The exclusive end index of the CID field. Calculated based on previous values defined in ANSI E1.31-2018 Section 4 Table 4-1, 4-2, 4-3.
const E131_CID_END_INDEX: usize = E131_PDU_LENGTH_FLAGS_LENGTH + E131_ROOT_LAYER_VECTOR_LENGTH + E131_CID_FIELD_LENGTH;

/// The length of the Source Name field in bytes in an ANSI E1.31-2018 packet as per ANSI E1.31-2018 Section 4, Table 4-1, 4-2, 4-3.
pub const E131_SOURCE_NAME_FIELD_LENGTH: usize = 64;

/// The length of the Synchronisation Address field in bytes in an ANSI E1.31-2018 packet as per ANSI E1.31-2018 Section 4, Table 4-1, 4-2, 4-3.
pub const E131_SYNC_ADDR_FIELD_LENGTH: usize = 2;

/// The initial/starting sequence number used.
pub const STARTING_SEQUENCE_NUMBER: u8 = 0;

/// The vector field value used to identify the ACN packet as an ANSI E1.31 data packet.
/// This is used at the ACN packet layer not the E1.31 layer.
/// Value as defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative).
pub const VECTOR_ROOT_E131_DATA: u32 = 0x0000_0004;

/// The vector field value used to identify the packet as an ANSI E1.31 universe discovery or synchronisation packet.
/// This is used at the ACN packet layer not the E1.31 layer.
/// Value as defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative).
pub const VECTOR_ROOT_E131_EXTENDED: u32 = 0x0000_0008;

/// The E1.31 packet vector field value used to identify the E1.31 packet as a synchronisation packet.
/// This is used at the E1.31 layer and shouldn't be confused with the VECTOR values used for the ACN layer (i.e. VECTOR_ROOT_E131_DATA and VECTOR_ROOT_E131_EXTENDED).
/// Value as defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative).
pub const VECTOR_E131_EXTENDED_SYNCHRONIZATION: u32 = 0x0000_0001;

/// The E1.31 packet vector field value used to identify the E1.31 packet as a universe discovery packet.
/// This is used at the E1.31 layer and shouldn't be confused with the VECTOR values used for the ACN layer (i.e. VECTOR_ROOT_E131_DATA and VECTOR_ROOT_E131_EXTENDED).
/// This VECTOR value is shared by E1.31 data packets, distinguished by the value of the ACN ROOT_VECTOR.
/// Value as defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative).
pub const VECTOR_E131_EXTENDED_DISCOVERY: u32 = 0x0000_0002;

/// The E1.31 packet vector field value used to identify the E1.31 packet as a data packet.
/// This is used at the E1.31 layer and shouldn't be confused with the VECTOR values used for the ACN layer (i.e. VECTOR_ROOT_E131_DATA and VECTOR_ROOT_E131_EXTENDED).
/// This VECTOR value is shared by E1.31 universe discovery packets, distinguished by the value of the ACN ROOT_VECTOR.
/// Value as defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative).
pub const VECTOR_E131_DATA_PACKET: u32 = 0x0000_0002;

/// Used at the DMP layer in E1.31 data packets to identify the packet as a set property message.
/// Not to be confused with the other VECTOR values used at the E1.31, ACN etc. layers.
/// Value as defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative).
pub const VECTOR_DMP_SET_PROPERTY: u8 = 0x02;

/// Used at the universe discovery packet universe discovery layer to identify the packet as a universe discovery list of universes.
/// Not to be confused with the other VECTOR values used at the E1.31, ACN, DMP, etc. layers.
/// Value as defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative).
pub const VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST: u32 = 0x0000_0001;

/// The port number used for the ACN family of protocols and therefore the sACN protocol.
/// As defined in ANSI E1.31-2018 Appendix A: Defined Parameters (Normative)
pub const ACN_SDT_MULTICAST_PORT: u16 = 5568;

/// The payload capacity for a sacn packet, for DMX data this would translate to 512 frames + a startcode byte.
pub const UNIVERSE_CHANNEL_CAPACITY: usize = 513;

/// The synchronisation universe/address of packets which do not require synchronisation as specified in section 6.2.4.1 of ANSI E1.31-2018.
pub const NO_SYNC_UNIVERSE: u16 = 0;

/// The timeout before data loss is assumed for an E131 source, as defined in Appendix A of ANSI E1.31-2018.
pub const E131_NETWORK_DATA_LOSS_TIMEOUT: Duration = Duration::from_millis(2500);

/// The timeout before a discovered source is assumed to be lost as defined in section 12.2 of ANSI E1.31-2018.
pub const UNIVERSE_DISCOVERY_SOURCE_TIMEOUT: Duration = E131_NETWORK_DATA_LOSS_TIMEOUT;

/// Converts the given ANSI E1.31-2018 universe into an Ipv4 multicast address with the port set to the acn multicast port as defined
/// in packet::ACN_SDT_MULTICAST_PORT.
///
/// Conversion done as specified in section 9.3.1 of ANSI E1.31-2018
///
/// Returns the multicast address.
///
/// # Errors
/// IllegalUniverse: Returned if the given universe is outwith the allowed range of universes,
///     see (is_universe_in_range)[fn.is_universe_in_range.packet].
/// 
pub fn universe_to_ipv4_multicast_addr(universe: u16) -> Result<SockAddr> {
    is_universe_in_range(universe)?;

    let high_byte: u8 = ((universe >> 8) & 0xff) as u8;
    let low_byte: u8 = (universe & 0xff) as u8;

    // As per ANSI E1.31-2018 Section 9.3.1 Table 9-10.
    Ok(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(239, 255, high_byte, low_byte)),
        ACN_SDT_MULTICAST_PORT,
    )
    .into())
}

/// Converts the given ANSI E1.31-2018 universe into an Ipv6 multicast address with the port set to the acn multicast port as defined
/// in packet::ACN_SDT_MULTICAST_PORT.
///
/// Conversion done as specified in section 9.3.2 of ANSI E1.31-2018
///
/// Returns the multicast address.
///
/// # Errors
/// IllegalUniverse: Returned if the given universe is outwith the allowed range of universes,
///     see (is_universe_in_range)[fn.is_universe_in_range.packet].
/// 
pub fn universe_to_ipv6_multicast_addr(universe: u16) -> Result<SockAddr> {
    is_universe_in_range(universe)?;

    // As per ANSI E1.31-2018 Section 9.3.2 Table 9-12.
    Ok(SocketAddr::new(
        IpAddr::V6(Ipv6Addr::new(0xFF18, 0, 0, 0, 0, 0, 0x8300, universe)),
        ACN_SDT_MULTICAST_PORT,
    )
    .into())
}

/// Checks if the given universe is a valid universe to send on (within allowed range).
///
/// # Errors
/// IllegalUniverse: Returned if the universe is outwith the allowed range of universes
///     [E131_MIN_MULTICAST_UNIVERSE, E131_MAX_MULTICAST_UNIVERSE] + E131_DISCOVERY_UNIVERSE.
///
pub fn is_universe_in_range(universe: u16) -> Result<()> {
    if (universe != E131_DISCOVERY_UNIVERSE)
        && (universe < E131_MIN_MULTICAST_UNIVERSE || universe > E131_MAX_MULTICAST_UNIVERSE)
    {
        bail!(ErrorKind::IllegalUniverse(
            format!(
                "Universe must be in the range [{} - {}], universe: {}",
                E131_MIN_MULTICAST_UNIVERSE, E131_MAX_MULTICAST_UNIVERSE, universe
            )
            .to_string()
        ));
    }
    Ok(())
}

/// Fills the given array of bytes with the given length n with bytes of value 0.
#[inline]
fn zeros(buf: &mut [u8], n: usize) {
    for b in buf.iter_mut().take(n) {
        *b = 0;
    }
}

/// Takes the given byte buffer (e.g. a c char array) and parses it into a rust &str.
/// 
/// # Arguments
/// buf: The byte buffer to parse into a str.
/// 
/// # Errors
/// SourceNameInvalid: Returned if the source name is not null terminated as required by ANSI E1.31-2018 Section 6.2.2
/// 
#[inline]
fn parse_source_name_str(buf: &[u8]) -> Result<&str> {
    let mut source_name_length = buf.len();
    for (i, b) in buf.iter().enumerate() {
        if *b == 0 {
            source_name_length = i;
            break;
        }
    }

    if source_name_length == buf.len() && buf[buf.len() - 1] != 0 {
        bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::SourceNameInvalid("Packet source name not null terminated".to_string())));
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
                if buf.len() <  (E131_PREAMBLE_SIZE as usize) {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInsufficientData("Insufficient data for ACN root layer preamble".to_string())));
                }

                // Preamble Size
                if NetworkEndian::read_u16(&buf[0..2]) != E131_PREAMBLE_SIZE {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInvalidData("invalid Preamble Size".to_string())));
                }

                // Post-amble Size
                if NetworkEndian::read_u16(&buf[2..4]) != E131_POSTAMBLE_SIZE {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInvalidData("invalid Post-amble Size".to_string())));
                }

                // ACN Packet Identifier
                if &buf[4 .. (E131_PREAMBLE_SIZE as usize)] != E131_ACN_PACKET_IDENTIFIER {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInvalidData("invalid ACN packet identifier".to_string())));
                }

                // PDU block
                Ok(AcnRootLayerProtocol {
                    pdu: E131RootLayer::parse(&buf[(E131_PREAMBLE_SIZE as usize) ..])?,
                })
            }

            /// Packs the packet into heap allocated memory.
            pub fn pack_alloc(&self) -> Result<Vec<u8>> {
                let mut buf = Vec::with_capacity(self.len());
                self.pack_vec(&mut buf)?;
                Ok(buf)
            }

            /// Packs the packet into the given vector.
            ///
            /// Grows the vector `buf` if necessary.
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
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInvalidData("invalid ACN packet identifier".to_string())));
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
                // Preamble Field Size (Bytes)
                2 +
                // Post-amble Field Size (Bytes)
                2 +
                // ACN Packet Identifier Field Size (Bytes)
                E131_ACN_PACKET_IDENTIFIER.len() +
                // PDU block
                self.pdu.len()
            }
        }
    };
}

impl_acn_root_layer_protocol!(<'a>);

struct PduInfo {
    length: usize,
    vector: u32,
}

/// Takes the given byte buffer and parses the flags, length and vector fields into a PduInfo struct.
/// 
/// # Arguments
/// buf: The raw byte buffer.
/// 
/// vector_length: The length of the vectorfield in bytes.
/// 
/// # Errors
/// ParseInsufficientData: If the length of the buffer is less than the flag, length and vector fields (E131_PDU_LENGTH_FLAGS_LENGTH + vector_length).
/// 
/// ParsePduInvalidFlags: If the flags parsed don't match the flags expected for an ANSI E1.31-2018 packet as per ANSI E1.31-2018 Section 4 Table 4-1, 4-2, 4-3.
/// 
fn pdu_info(buf: &[u8], vector_length: usize) -> Result<PduInfo> {
    if buf.len() < E131_PDU_LENGTH_FLAGS_LENGTH + vector_length {
        bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInsufficientData("Insufficient data when parsing pdu_info, no flags or length field".to_string())));
    }

    // Flags
    let flags = buf[0] & 0xf0; // Flags are stored in the top 4 bits.
    if flags != E131_PDU_FLAGS {
        bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParsePduInvalidFlags(flags)));
    }
    // Length
    let length = (NetworkEndian::read_u16(&buf[0 .. E131_PDU_LENGTH_FLAGS_LENGTH]) & 0x0fff) as usize;

    // Vector
    let vector = NetworkEndian::read_uint(&buf[E131_PDU_LENGTH_FLAGS_LENGTH .. ], vector_length) as u32;

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
                let PduInfo { length, vector } = pdu_info(&buf, E131_ROOT_LAYER_VECTOR_LENGTH)?;
                if buf.len() < length {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInsufficientData("Buffer contains insufficient data based on ACN root layer pdu length field".to_string())));
                }

                if vector != VECTOR_ROOT_E131_DATA && vector != VECTOR_ROOT_E131_EXTENDED {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PduInvalidVector(vector)));
                }
                
                // CID
                let cid = Uuid::from_bytes(&buf[E131_PDU_LENGTH_FLAGS_LENGTH + E131_ROOT_LAYER_VECTOR_LENGTH .. E131_CID_END_INDEX])?;

                // Data
                let data = match vector {
                    VECTOR_ROOT_E131_DATA => {
                        E131RootLayerData::DataPacket(DataPacketFramingLayer::parse(&buf[E131_CID_END_INDEX .. length])?)
                    }
                    VECTOR_ROOT_E131_EXTENDED => {
                        let data_buf = &buf[E131_CID_END_INDEX .. length];
                        let PduInfo { length, vector} = pdu_info(&data_buf, E131_FRAMING_LAYER_VECTOR_LENGTH)?;
                        if buf.len() < length {
                            bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInsufficientData("Buffer contains insufficient data based on E131 framing layer pdu length field".to_string())));
                        }

                        match vector {
                            VECTOR_E131_EXTENDED_SYNCHRONIZATION => {
                                E131RootLayerData::SynchronizationPacket(
                                    SynchronizationPacketFramingLayer::parse(data_buf)?,
                                )
                            }
                            VECTOR_E131_EXTENDED_DISCOVERY => {
                                E131RootLayerData::UniverseDiscoveryPacket(
                                    UniverseDiscoveryPacketFramingLayer::parse(data_buf)?,
                                )
                            }
                            vector => bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PduInvalidVector(vector))),
                        }
                    }
                    vector => bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PduInvalidVector(vector))),
                };

                Ok(E131RootLayer {
                    cid,
                    data,
                })
            }

            fn pack(&self, buf: &mut [u8]) -> Result<()> {
                if buf.len() < self.len() {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PackBufferInsufficient("".to_string())))
                }

                // Flags and Length, flags are stored in the top 4 bits.
                let flags_and_length = NetworkEndian::read_u16(&[E131_PDU_FLAGS, 0x0]) | (self.len() as u16) & 0x0fff;
                NetworkEndian::write_u16(&mut buf[0 .. E131_PDU_LENGTH_FLAGS_LENGTH], flags_and_length);

                // Vector
                match self.data {
                    E131RootLayerData::DataPacket(_) => {
                        NetworkEndian::write_u32(&mut buf[E131_PDU_LENGTH_FLAGS_LENGTH .. E131_PDU_LENGTH_FLAGS_LENGTH + E131_ROOT_LAYER_VECTOR_LENGTH], VECTOR_ROOT_E131_DATA)
                    }
                    E131RootLayerData::SynchronizationPacket(_)
                    | E131RootLayerData::UniverseDiscoveryPacket(_) => {
                        NetworkEndian::write_u32(&mut buf[E131_PDU_LENGTH_FLAGS_LENGTH .. E131_PDU_LENGTH_FLAGS_LENGTH + E131_ROOT_LAYER_VECTOR_LENGTH], VECTOR_ROOT_E131_EXTENDED)
                    }
                }

                // CID
                buf[E131_PDU_LENGTH_FLAGS_LENGTH + E131_ROOT_LAYER_VECTOR_LENGTH .. E131_CID_END_INDEX].copy_from_slice(self.cid.as_bytes());

                // Data
                match self.data {
                    E131RootLayerData::DataPacket(ref data) => Ok(data.pack(&mut buf[E131_CID_END_INDEX .. ])?),
                    E131RootLayerData::SynchronizationPacket(ref data) => Ok(data.pack(&mut buf[E131_CID_END_INDEX .. ])?),
                    E131RootLayerData::UniverseDiscoveryPacket(ref data) => Ok(data.pack(&mut buf[E131_CID_END_INDEX .. ])?),
                }
            }

            fn len(&self) -> usize {
                // Length and Flags
                E131_PDU_LENGTH_FLAGS_LENGTH +
                // Vector
                E131_ROOT_LAYER_VECTOR_LENGTH +
                // CID
                E131_CID_FIELD_LENGTH +
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

impl_e131_root_layer!(<'a>);

macro_rules! impl_data_packet_framing_layer {
    ( $( $lt:tt )* ) => {
        /// Framing layer PDU for sACN data packets.
        #[derive(Eq, PartialEq, Debug)]
        pub struct DataPacketFramingLayer$( $lt )* {
            /// The name of the source.
            pub source_name: Cow<'a, str>,

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

        // Calculate the indexes of the fields within the buffer based on the size of the fields previous.
        // Constants are replaced inline so this increases readability by removing magic numbers without affecting runtime performance.
        // Theses indexes are only valid within the scope of this part of the protocol (DataPacketFramingLayer).
        const SOURCE_NAME_INDEX: usize = E131_PDU_LENGTH_FLAGS_LENGTH + E131_FRAMING_LAYER_VECTOR_LENGTH;
        const PRIORITY_INDEX: usize = SOURCE_NAME_INDEX + E131_SOURCE_NAME_FIELD_LENGTH;
        const SYNC_ADDR_INDEX: usize = PRIORITY_INDEX + E131_PRIORITY_FIELD_LENGTH;
        const SEQ_NUM_INDEX: usize = SYNC_ADDR_INDEX + E131_SYNC_ADDR_FIELD_LENGTH;
        const OPTIONS_FIELD_INDEX: usize = SEQ_NUM_INDEX + E131_SEQ_NUM_FIELD_LENGTH;
        const UNIVERSE_INDEX: usize = OPTIONS_FIELD_INDEX + E131_OPTIONS_FIELD_LENGTH;
        const DATA_INDEX: usize = UNIVERSE_INDEX + E131_UNIVERSE_FIELD_LENGTH;

        impl$( $lt )* Pdu for DataPacketFramingLayer$( $lt )* {
            fn parse(buf: &[u8]) -> Result<DataPacketFramingLayer$( $lt )*> {
                // Length and Vector
                let PduInfo { length, vector } = pdu_info(&buf, E131_FRAMING_LAYER_VECTOR_LENGTH)?;
                if buf.len() < length {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInsufficientData("Buffer contains insufficient data based on data packet framing layer pdu length field".to_string())));
                }

                if vector != VECTOR_E131_DATA_PACKET {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PduInvalidVector(vector)));
                }

                // Source Name
                let source_name = String::from(
                    parse_source_name_str(
                        &buf[SOURCE_NAME_INDEX .. PRIORITY_INDEX]
                    )?
                );

                // Priority
                let priority = buf[PRIORITY_INDEX];
                if priority > E131_MAX_PRIORITY {
                    bail!(
                        ErrorKind::SacnParsePackError(
                            sacn_parse_pack_error::ErrorKind::ParseInvalidPriority(
                                format!("Priority value: {} is outwith the allowed range", priority).to_string())));
                }

                // Synchronization Address
                let synchronization_address = NetworkEndian::read_u16(&buf[SYNC_ADDR_INDEX .. SEQ_NUM_INDEX]);
                if synchronization_address > E131_MAX_MULTICAST_UNIVERSE {
                    bail!(
                        ErrorKind::SacnParsePackError(
                        sacn_parse_pack_error::ErrorKind::ParseInvalidSyncAddr(
                            format!("Sync_addr value: {} is outwith the allowed range", synchronization_address).to_string()))
                    );
                }

                // Sequence Number
                let sequence_number = buf[SEQ_NUM_INDEX];

                // Options, Stored as bit flag.
                let preview_data = buf[OPTIONS_FIELD_INDEX] & E131_PREVIEW_DATA_OPTION_BIT_MASK != 0;
                let stream_terminated = buf[OPTIONS_FIELD_INDEX] & E131_STREAM_TERMINATION_OPTION_BIT_MASK != 0;
                let force_synchronization = buf[OPTIONS_FIELD_INDEX] & E131_FORCE_SYNCHRONISATION_OPTION_BIT_MASK != 0;

                // Universe
                let universe = NetworkEndian::read_u16(&buf[UNIVERSE_INDEX .. DATA_INDEX]);

                if universe < E131_MIN_MULTICAST_UNIVERSE || universe > E131_MAX_MULTICAST_UNIVERSE {
                    bail!(
                        ErrorKind::SacnParsePackError(
                        sacn_parse_pack_error::ErrorKind::ParseInvalidUniverse(
                            format!("Universe value: {} is outwith the allowed range", universe).to_string()))
                    );
                }

                // Data layer.
                let data = DataPacketDmpLayer::parse(&buf[DATA_INDEX .. length])?;

                Ok(DataPacketFramingLayer {
                    source_name: source_name.into(),
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
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PackBufferInsufficient("".to_string())));
                }

                // Flags and Length
                let flags_and_length = NetworkEndian::read_u16(&[E131_PDU_FLAGS, 0x0]) | (self.len() as u16) & 0x0fff;
                NetworkEndian::write_u16(&mut buf[0.. E131_PDU_LENGTH_FLAGS_LENGTH], flags_and_length);

                // Vector
                NetworkEndian::write_u32(&mut buf[E131_PDU_LENGTH_FLAGS_LENGTH .. SOURCE_NAME_INDEX], VECTOR_E131_DATA_PACKET);

                // Source Name, padded with 0's up to the required 64 byte length.
                zeros(&mut buf[SOURCE_NAME_INDEX .. PRIORITY_INDEX], E131_SOURCE_NAME_FIELD_LENGTH);
                buf[SOURCE_NAME_INDEX .. SOURCE_NAME_INDEX + self.source_name.len()].copy_from_slice(self.source_name.as_bytes());

                // Priority
                buf[PRIORITY_INDEX] = self.priority;

                // Synchronization Address
                NetworkEndian::write_u16(&mut buf[SYNC_ADDR_INDEX .. SEQ_NUM_INDEX], self.synchronization_address);

                // Sequence Number
                buf[SEQ_NUM_INDEX] = self.sequence_number;

                // Options, zero out all the bits to start including bits 0-4 as per ANSI E1.31-2018 Section 6.2.6.
                buf[OPTIONS_FIELD_INDEX] = 0;

                // Preview Data
                if self.preview_data {
                    buf[OPTIONS_FIELD_INDEX] = E131_PREVIEW_DATA_OPTION_BIT_MASK;
                }

                // Stream Terminated
                if self.stream_terminated {
                    buf[OPTIONS_FIELD_INDEX] |= E131_STREAM_TERMINATION_OPTION_BIT_MASK;
                }

                // Force Synchronization
                if self.force_synchronization {
                    buf[OPTIONS_FIELD_INDEX] |= E131_FORCE_SYNCHRONISATION_OPTION_BIT_MASK;
                }

                // Universe
                NetworkEndian::write_u16(&mut buf[UNIVERSE_INDEX .. DATA_INDEX], self.universe);

                // Data
                Ok(self.data.pack(&mut buf[DATA_INDEX .. ])?)
            }

            fn len(&self) -> usize {
                // Length and Flags
                E131_PDU_LENGTH_FLAGS_LENGTH +
                // Vector
                E131_FRAMING_LAYER_VECTOR_LENGTH +
                // Source Name
                E131_SOURCE_NAME_FIELD_LENGTH +
                // Priority
                E131_PRIORITY_FIELD_LENGTH +
                // Synchronization Address
                E131_SYNC_ADDR_FIELD_LENGTH +
                // Sequence Number
                E131_SEQ_NUM_FIELD_LENGTH +
                // Options
                E131_OPTIONS_FIELD_LENGTH +
                // Universe
                E131_UNIVERSE_FIELD_LENGTH +
                // Data
                self.data.len()
            }
        }

        impl$( $lt )* Clone for DataPacketFramingLayer$( $lt )* {
            fn clone(&self) -> Self {
                DataPacketFramingLayer {
                    source_name: self.source_name.clone(),
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

impl_data_packet_framing_layer!(<'a>);

macro_rules! impl_data_packet_dmp_layer {
    ( $( $lt:tt )* ) => {
        /// Device Management Protocol PDU with SET PROPERTY vector.
        ///
        /// Used for sACN data packets.
        #[derive(Eq, PartialEq, Debug)]
        pub struct DataPacketDmpLayer$( $lt )* {
            /// DMX data property values (DMX start coder + 512 slots).
            pub property_values: Cow<'a, [u8]>,
        }

        // Calculate the indexes of the fields within the buffer based on the size of the fields previous.
        // Constants are replaced inline so this increases readability by removing magic numbers without affecting runtime performance.
        // Theses indexes are only valid within the scope of this part of the protocol (DataPacketDmpLayer).
        const VECTOR_FIELD_INDEX: usize = E131_PDU_LENGTH_FLAGS_LENGTH;
        const ADDRESS_DATA_FIELD_INDEX: usize = VECTOR_FIELD_INDEX + E131_DATA_PACKET_DMP_LAYER_VECTOR_FIELD_LENGTH;
        const FIRST_PRIORITY_FIELD_INDEX: usize = ADDRESS_DATA_FIELD_INDEX + E131_DATA_PACKET_DMP_LAYER_ADDRESS_DATA_FIELD_LENGTH;
        const ADDRESS_INCREMENT_FIELD_INDEX: usize = FIRST_PRIORITY_FIELD_INDEX + E131_DATA_PACKET_DMP_LAYER_FIRST_PROPERTY_ADDRESS_FIELD_LENGTH;
        const PROPERTY_VALUE_COUNT_FIELD_INDEX: usize = ADDRESS_INCREMENT_FIELD_INDEX + E131_DATA_PACKET_DMP_LAYER_ADDRESS_INCREMENT_FIELD_LENGTH;
        const PROPERTY_VALUES_FIELD_INDEX: usize = PROPERTY_VALUE_COUNT_FIELD_INDEX + E131_DATA_PACKET_DMP_LAYER_PROPERTY_VALUE_COUNT_FIELD_LENGTH;

        impl$( $lt )* Pdu for DataPacketDmpLayer$( $lt )* {

            fn parse(buf: &[u8]) -> Result<DataPacketDmpLayer$( $lt )*> {
                // Length and Vector
                let PduInfo { length, vector } = pdu_info(&buf, E131_DATA_PACKET_DMP_LAYER_VECTOR_FIELD_LENGTH)?;
                if buf.len() < length {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInsufficientData("Buffer contains insufficient data based on data packet dmp layer pdu length field".to_string())));
                }

                if vector != u32::from(VECTOR_DMP_SET_PROPERTY) {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PduInvalidVector(vector)));
                }

                // Address and Data Type
                if buf[ADDRESS_DATA_FIELD_INDEX] != E131_DMP_LAYER_ADDRESS_DATA_FIELD {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInvalidData("invalid Address and Data Type".to_string())));
                }

                // First Property Address
                if NetworkEndian::read_u16(&buf[FIRST_PRIORITY_FIELD_INDEX .. ADDRESS_INCREMENT_FIELD_INDEX]) != E131_DATA_PACKET_DMP_LAYER_FIRST_PROPERTY_FIELD {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInvalidData("invalid First Property Address".to_string())));
                }

                // Address Increment
                if NetworkEndian::read_u16(&buf[ADDRESS_INCREMENT_FIELD_INDEX .. PROPERTY_VALUE_COUNT_FIELD_INDEX]) != E131_DATA_PACKET_DMP_LAYER_ADDRESS_INCREMENT {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInvalidData("invalid Address Increment".to_string())));
                }

                // Property value count
                let property_value_count = NetworkEndian::read_u16(&buf[PROPERTY_VALUE_COUNT_FIELD_INDEX .. PROPERTY_VALUES_FIELD_INDEX]);

                // Check that the property value count matches the expected count based on the pdu length given previously.
                if property_value_count as usize + PROPERTY_VALUES_FIELD_INDEX != length {
                    bail!(ErrorKind::SacnParsePackError(
                        sacn_parse_pack_error::ErrorKind::ParseInsufficientData(
                            format!("Invalid data packet dmp layer property value count, pdu length indicates {} property values, property value count field indicates {} property values",
                                length , property_value_count)
                            .to_string()
                        )
                    ));
                }

                // Property values
                // The property value length is only of the property values and not the headers so start counting at the index that the property values start.
                let property_values_length = length - PROPERTY_VALUES_FIELD_INDEX;
                if property_values_length > UNIVERSE_CHANNEL_CAPACITY {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInvalidData("only 512 DMX slots allowed".to_string())));
                }

                let mut property_values = Vec::with_capacity(property_values_length);

                property_values.extend_from_slice(&buf[PROPERTY_VALUES_FIELD_INDEX .. length]);

                Ok(DataPacketDmpLayer {
                    property_values: property_values.into(),
                })
            }

            fn pack(&self, buf: &mut [u8]) -> Result<()> {
                if self.property_values.len() > UNIVERSE_CHANNEL_CAPACITY {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PackInvalidData("only 512 DMX values allowed".to_string())));
                }

                if buf.len() < self.len() {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PackBufferInsufficient("DataPacketDmpLayer pack buffer length insufficient".to_string())));
                }

                // Flags and Length
                let flags_and_length = NetworkEndian::read_u16(&[E131_PDU_FLAGS, 0x0]) | (self.len() as u16) & 0x0fff;
                NetworkEndian::write_u16(&mut buf[0.. E131_PDU_LENGTH_FLAGS_LENGTH], flags_and_length);

                // Vector
                buf[VECTOR_FIELD_INDEX] = VECTOR_DMP_SET_PROPERTY;

                // Address and Data Type
                buf[ADDRESS_DATA_FIELD_INDEX] = E131_DMP_LAYER_ADDRESS_DATA_FIELD;

                // First Property Address
                zeros(&mut buf[FIRST_PRIORITY_FIELD_INDEX .. ADDRESS_INCREMENT_FIELD_INDEX], E131_DATA_PACKET_DMP_LAYER_FIRST_PROPERTY_ADDRESS_FIELD_LENGTH);

                // Address Increment
                NetworkEndian::write_u16(&mut buf[ADDRESS_INCREMENT_FIELD_INDEX .. PROPERTY_VALUE_COUNT_FIELD_INDEX], E131_DATA_PACKET_DMP_LAYER_ADDRESS_INCREMENT);

                // Property value count
                NetworkEndian::write_u16(&mut buf[PROPERTY_VALUE_COUNT_FIELD_INDEX .. PROPERTY_VALUES_FIELD_INDEX], self.property_values.len() as u16);

                // Property values
                buf[PROPERTY_VALUES_FIELD_INDEX .. PROPERTY_VALUES_FIELD_INDEX + self.property_values.len()].copy_from_slice(&self.property_values);

                Ok(())
            }

            fn len(&self) -> usize {
                // Length and Flags
                E131_PDU_LENGTH_FLAGS_LENGTH +
                // Vector
                E131_DATA_PACKET_DMP_LAYER_VECTOR_FIELD_LENGTH +
                // Address and Data Type
                E131_DATA_PACKET_DMP_LAYER_ADDRESS_DATA_FIELD_LENGTH +
                // First Property Address
                E131_DATA_PACKET_DMP_LAYER_FIRST_PROPERTY_ADDRESS_FIELD_LENGTH +
                // Address Increment
                E131_DATA_PACKET_DMP_LAYER_ADDRESS_INCREMENT_FIELD_LENGTH +
                // Property value count
                E131_DATA_PACKET_DMP_LAYER_PROPERTY_VALUE_COUNT_FIELD_LENGTH +
                // Property values
                self.property_values.len()
            }
        }

        impl$( $lt )* Clone for DataPacketDmpLayer$( $lt )* {
            fn clone(&self) -> Self {
                DataPacketDmpLayer {
                    property_values: self.property_values.clone(),
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

impl_data_packet_dmp_layer!(<'a>);

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
        let PduInfo { length, vector } = pdu_info(&buf, 4)?;
        if buf.len() < length {
            bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInsufficientData("Buffer contains insufficient data based on synchronisation packet framing layer pdu length field".to_string())));
        }

        if vector != VECTOR_E131_EXTENDED_SYNCHRONIZATION {
            bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PduInvalidVector(vector)));
        }

        if length != E131_UNIVERSE_SYNC_PACKET_FRAMING_LAYER_LENGTH {
            bail!(ErrorKind::SacnParsePackError(
                sacn_parse_pack_error::ErrorKind::PduInvalidLength(length))); 
        }

        // Sequence Number
        let sequence_number = buf[6];

        // Synchronization Address
        let synchronization_address = NetworkEndian::read_u16(&buf[7..9]);

        if synchronization_address > E131_MAX_MULTICAST_UNIVERSE || synchronization_address < E131_MIN_MULTICAST_UNIVERSE {
            bail!(
                ErrorKind::SacnParsePackError(
                sacn_parse_pack_error::ErrorKind::ParseInvalidUniverse(
                    format!("Synchronisation address value: {} is outwith the allowed range", synchronization_address).to_string()))
            );
        }

        // Reserved fields (buf[9..11]) should be ignored by receivers as per ANSI E1.31-2018 Section 6.3.4.

        Ok(SynchronizationPacketFramingLayer {
            sequence_number,
            synchronization_address,
        })
    }

    fn pack(&self, buf: &mut [u8]) -> Result<()> {
        if buf.len() < self.len() {
            bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PackBufferInsufficient("SynchronizationPacketFramingLayer pack buffer length insufficient".to_string())));
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
            pub source_name: Cow<'a, str>,

            /// Universe discovery layer.
            pub data: UniverseDiscoveryPacketUniverseDiscoveryLayer$( $lt )*,
        }

        impl$( $lt )* Pdu for UniverseDiscoveryPacketFramingLayer$( $lt )* {
            fn parse(buf: &[u8]) -> Result<UniverseDiscoveryPacketFramingLayer$( $lt )*> {
                // Length and Vector
                let PduInfo { length, vector } = pdu_info(&buf, 4)?;
                if buf.len() < length {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInsufficientData("Buffer contains insufficient data based on universe discovery packet framing layer pdu length field".to_string())));
                }

                if vector != VECTOR_E131_EXTENDED_DISCOVERY {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PduInvalidVector(vector)));
                }

                if length < E131_UNIVERSE_DISCOVERY_FRAMING_LAYER_MIN_LENGTH {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PduInvalidLength(length)));
                }

                // Source Name
                let source_name = String::from(parse_source_name_str(&buf[6..70])?);

                // Reserved data (buf[70..74]) ignored as per ANSI E1.31-2018 Section 6.4.3.

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
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PackBufferInsufficient("UniverseDiscoveryPacketFramingLayer pack buffer length insufficient".to_string())));
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

impl_universe_discovery_packet_framing_layer!(<'a>);

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
        }

        impl$( $lt )* Pdu for UniverseDiscoveryPacketUniverseDiscoveryLayer$( $lt )* {
            fn parse(buf: &[u8]) -> Result<UniverseDiscoveryPacketUniverseDiscoveryLayer$( $lt )*> {
                // Length and Vector
                let PduInfo { length, vector } = pdu_info(&buf, 4)?;
                if buf.len() < length {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInsufficientData("Buffer contains insufficient data based on universe discovery packet universe discovery layer pdu length field".to_string())));
                }

                if vector != VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PduInvalidVector(vector)));
                }

                if length < E131_UNIVERSE_DISCOVERY_LAYER_MIN_LENGTH || length > E131_UNIVERSE_DISCOVERY_LAYER_MAX_LENGTH {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PduInvalidLength(length)));
                }

                // Page
                let page = buf[6];

                // Last Page
                let last_page = buf[7];

                if page > last_page {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::ParseInvalidPage("Page value higher than last_page".to_string())));
                }

                // Universes
                let universes_length = (length - 8) / 2;
                let mut universes = Vec::with_capacity(universes_length);

                let mut i = 8;
                let mut last_universe: i32 = -1;
                while ((i+2) <= length) {
                    let u = NetworkEndian::read_u16(&buf[i .. i+2]);

                    if ((u as i32) > last_universe) { // Enforce assending ordering of universes as per ANSI E1.31-2018 Section 8.5. 
                        universes.push(u);
                        last_universe = (u as i32);
                        i = i + 2; // Each universe takes 2 bytes so jump by 2.
                    } else {
                        bail!(ErrorKind::SacnParsePackError(
                            sacn_parse_pack_error::ErrorKind::ParseInvalidUniverseOrder(
                                format!("Universe {} is out of order, discovery packet universe list must be in accending order!", u).to_string())));
                    }
                }

                if i != length { 
                    // Indicates that there is data left over, this can happen if the bytes for the universes is not an even number meaning 
                    // that every byte cannot be used to create 16 bit universe numbers, this shouldn't happen and indicates that the packet
                    // is the wrong length / malformed (with extra data on the end).
                    bail!(ErrorKind::SacnParsePackError(
                            sacn_parse_pack_error::ErrorKind::ParseInsufficientData(
                                "A non-even (odd) amount of data left at end of packet, cannot parsed a single byte into a 16 bit universe number".to_string()
                            )
                        )
                    );
                }

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
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PackInvalidData("only 512 universes allowed".to_string())));
                }

                if buf.len() < self.len() {
                    bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PackBufferInsufficient("UniverseDiscoveryPacketUniverseDiscoveryLayer pack buffer insufficient".to_string())));
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
                        bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PackInvalidData("Universes are not unique".to_string())));
                    }
                    if self.universes[i] <= self.universes[i - 1] {
                        bail!(ErrorKind::SacnParsePackError(sacn_parse_pack_error::ErrorKind::PackInvalidData("Universes are not sorted".to_string())));
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

impl_universe_discovery_packet_universe_discovery_layer!(<'a>);

#[cfg(test)]
mod test {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

    /// The universe_to tests below check that the conversion from a universe to an IPv6 or IPv4 multicast address is done as
    /// per ANSI E1.31-2018 Section 9.3.1 Table 9-10 (IPv4) and ANSI E1.31-2018 Section 9.3.2 Table 9-11 + Table 9-12.
    #[test]
    fn test_universe_to_ipv4_lowest_byte_normal() {
        let val: u16 = 119;
        let res = universe_to_ipv4_multicast_addr(val).unwrap();
        assert!(res.as_inet().unwrap().ip().is_multicast());

        assert_eq!(
            res.as_inet().unwrap(),
            SocketAddrV4::new(
                Ipv4Addr::new(239, 255, (val / 256) as u8, (val % 256) as u8),
                ACN_SDT_MULTICAST_PORT
            )
        );
    }

    #[test]
    fn test_universe_to_ip_ipv4_both_bytes_normal() {
        let val: u16 = 300;
        let res = universe_to_ipv4_multicast_addr(val).unwrap();
        assert!(res.as_inet().unwrap().ip().is_multicast());

        assert_eq!(
            res.as_inet().unwrap(),
            SocketAddrV4::new(
                Ipv4Addr::new(239, 255, (val / 256) as u8, (val % 256) as u8),
                ACN_SDT_MULTICAST_PORT
            )
        );
    }

    #[test]
    fn test_universe_to_ip_ipv4_limit_high() {
        let res = universe_to_ipv4_multicast_addr(E131_MAX_MULTICAST_UNIVERSE).unwrap();
        assert!(res.as_inet().unwrap().ip().is_multicast());

        assert_eq!(
            res.as_inet().unwrap(),
            SocketAddrV4::new(
                Ipv4Addr::new(
                    239,
                    255,
                    (E131_MAX_MULTICAST_UNIVERSE / 256) as u8,
                    (E131_MAX_MULTICAST_UNIVERSE % 256) as u8
                ),
                ACN_SDT_MULTICAST_PORT
            )
        );
    }

    #[test]
    fn test_universe_to_ip_ipv4_limit_low() {
        let res = universe_to_ipv4_multicast_addr(E131_MIN_MULTICAST_UNIVERSE).unwrap();

        assert!(res.as_inet().unwrap().ip().is_multicast());

        assert_eq!(
            res.as_inet().unwrap(),
            SocketAddrV4::new(
                Ipv4Addr::new(
                    239,
                    255,
                    (E131_MIN_MULTICAST_UNIVERSE / 256) as u8,
                    (E131_MIN_MULTICAST_UNIVERSE % 256) as u8
                ),
                ACN_SDT_MULTICAST_PORT
            )
        );
    }

    #[test]
    fn test_universe_to_ip_ipv4_out_range_low() {
        match universe_to_ipv4_multicast_addr(0) {
            Ok(_) => assert!(
                false,
                "Universe to ipv4 multicast allowed below minimum allowed universe"
            ),
            Err(e) => match e.kind() {
                &ErrorKind::IllegalUniverse(ref _s) => assert!(true),
                _ => assert!(false, "Unexpected error type returned"),
            },
        }
    }

    #[test]
    fn test_universe_to_ip_ipv4_out_range_high() {
        match universe_to_ipv4_multicast_addr(E131_MAX_MULTICAST_UNIVERSE + 10) {
            Ok(_) => assert!(
                false,
                "Universe to ipv4 multicast allowed above maximum allowed universe"
            ),
            Err(e) => match e.kind() {
                &ErrorKind::IllegalUniverse(ref _s) => assert!(true),
                _ => assert!(false, "Unexpected error type returned"),
            },
        }
    }

    #[test]
    fn test_universe_to_ipv6_lowest_byte_normal() {
        let val: u16 = 119;
        let res = universe_to_ipv6_multicast_addr(val).unwrap();

        assert!(res.as_inet6().unwrap().ip().is_multicast());

        let low_16: u16 = (((val / 256) as u16) << 8) | ((val % 256) as u16);

        assert_eq!(
            res.as_inet6().unwrap(),
            SocketAddrV6::new(
                Ipv6Addr::new(0xFF18, 0, 0, 0, 0, 0, 0x8300, low_16),
                ACN_SDT_MULTICAST_PORT,
                0,
                0
            )
        );
    }

    #[test]
    fn test_universe_to_ip_ipv6_both_bytes_normal() {
        let val: u16 = 300;
        let res = universe_to_ipv6_multicast_addr(val).unwrap();

        assert!(res.as_inet6().unwrap().ip().is_multicast());

        let low_16: u16 = (((val / 256) as u16) << 8) | ((val % 256) as u16);

        assert_eq!(
            res.as_inet6().unwrap(),
            SocketAddrV6::new(
                Ipv6Addr::new(0xFF18, 0, 0, 0, 0, 0, 0x8300, low_16),
                ACN_SDT_MULTICAST_PORT,
                0,
                0
            )
        );
    }

    #[test]
    fn test_universe_to_ip_ipv6_limit_high() {
        let res = universe_to_ipv6_multicast_addr(E131_MAX_MULTICAST_UNIVERSE).unwrap();

        assert!(res.as_inet6().unwrap().ip().is_multicast());

        let low_16: u16 = (((E131_MAX_MULTICAST_UNIVERSE / 256) as u16) << 8)
            | ((E131_MAX_MULTICAST_UNIVERSE % 256) as u16);

        assert_eq!(
            res.as_inet6().unwrap(),
            SocketAddrV6::new(
                Ipv6Addr::new(0xFF18, 0, 0, 0, 0, 0, 0x8300, low_16),
                ACN_SDT_MULTICAST_PORT,
                0,
                0
            )
        );
    }

    #[test]
    fn test_universe_to_ip_ipv6_limit_low() {
        let res = universe_to_ipv6_multicast_addr(E131_MIN_MULTICAST_UNIVERSE).unwrap();

        assert!(res.as_inet6().unwrap().ip().is_multicast());

        let low_16: u16 = (((E131_MIN_MULTICAST_UNIVERSE / 256) as u16) << 8)
            | ((E131_MIN_MULTICAST_UNIVERSE % 256) as u16);

        assert_eq!(
            res.as_inet6().unwrap(),
            SocketAddrV6::new(
                Ipv6Addr::new(0xFF18, 0, 0, 0, 0, 0, 0x8300, low_16),
                ACN_SDT_MULTICAST_PORT,
                0,
                0
            )
        );
    }

    #[test]
    fn test_universe_to_ip_ipv6_out_range_low() {
        match universe_to_ipv6_multicast_addr(0) {
            Ok(_) => assert!(
                false,
                "Universe to ipv4 multicast allowed below minimum allowed universe"
            ),
            Err(e) => match e.kind() {
                &ErrorKind::IllegalUniverse(ref _s) => assert!(true),
                _ => assert!(false, "Unexpected error type returned"),
            },
        }
    }

    #[test]
    fn test_universe_to_ip_ipv6_out_range_high() {
        match universe_to_ipv6_multicast_addr(E131_MAX_MULTICAST_UNIVERSE + 10) {
            Ok(_) => assert!(
                false,
                "Universe to ipv4 multicast allowed above maximum allowed universe"
            ),
            Err(e) => match e.kind() {
                &ErrorKind::IllegalUniverse(ref _s) => assert!(true),
                _ => assert!(false, "Unexpected error type returned"),
            },
        }
    }

    /// Verifies that the parameters are set correctly as per ANSI E1.31-2018 Appendix A: Defined Parameters (Normative).
    /// This test is particularly useful at the maintainance stage as it will flag up if any protocol defined constant is changed. 
    #[test]
    fn check_ansi_e131_2018_parameter_values() {
        assert_eq!(VECTOR_ROOT_E131_DATA, 0x0000_0004);
        assert_eq!(VECTOR_ROOT_E131_EXTENDED, 0x0000_0008);
        assert_eq!(VECTOR_DMP_SET_PROPERTY, 0x02);
        assert_eq!(VECTOR_E131_DATA_PACKET, 0x0000_0002);
        assert_eq!(VECTOR_E131_EXTENDED_SYNCHRONIZATION, 0x0000_0001);
        assert_eq!(VECTOR_E131_EXTENDED_DISCOVERY, 0x0000_0002);
        assert_eq!(VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST, 0x0000_0001);
        assert_eq!(E131_UNIVERSE_DISCOVERY_INTERVAL, Duration::from_secs(10));
        assert_eq!(E131_NETWORK_DATA_LOSS_TIMEOUT, Duration::from_millis(2500));
        assert_eq!(E131_DISCOVERY_UNIVERSE, 64214);
        assert_eq!(ACN_SDT_MULTICAST_PORT, 5568);
    }
}
