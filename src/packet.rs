// Copyright 2017 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[cfg(feature = "std")]
use std::vec::Vec;

use byteorder::{ByteOrder, BigEndian};
use uuid::Uuid;
use arrayvec::ArrayString;

pub struct RootLayerProtocol<'a> {
    pub pdu_block: &'a [RootLayerProtocolDataUnit<'a>],
}

pub trait ProtocolDataUnit {
    /// Returns the number of bytes the packet would occupy when packed.
    fn len(&self) -> usize;

    #[cfg(feature = "std")]
    /// Packs the PDU into the given vector.
    ///
    /// Grows the vector if necessary.
    fn pack_vec(&self, buf: &mut Vec<u8>) {
        buf.clear();
        buf.reserve_exact(self.len());
        self.pack(buf)
    }

    /// Packs the PDU into the given slice.
    ///
    /// # Panics
    /// Panics if the given slice is not large enough.
    fn pack(&self, &mut [u8]);
}

pub struct RootLayerProtocolDataUnit<'a> {
    pub cid: Uuid,
    pub data: &'a ProtocolDataUnit,
}

impl<'a> ProtocolDataUnit for RootLayerProtocolDataUnit<'a> {
    fn len(&self) -> usize {
        22 + self.data.len()
    }

    fn pack(&self, buf: &mut [u8]) {
        // length
        BigEndian::write_u16(&mut buf[0..1], self.len() as u16);
        // flags
        buf[0] &= 0x7f;
        // vector VECTOR_ROOT_E131_DATA
        BigEndian::write_u32(&mut buf[2..5], 0x00000004);
        // cid
        buf[6..22].copy_from_slice(self.cid.as_bytes());
        // data
        self.data.pack(&mut buf[23..])
    }
}

pub struct E131DataPacketFramingLayer<'a> {
    pub source_name: ArrayString<[u8; 16]>,
    pub priority: u8,
    pub synchronization_address: u16,
    pub sequence_number: u8,
    pub preview_data: bool,
    pub stream_terminated: bool,
    pub force_synchronization: bool,
    pub universe: u16,
    pub data: E131DataPacketDeviceManagementProtocolLayer<'a>,
}

impl<'a> ProtocolDataUnit for E131DataPacketFramingLayer<'a> {
    fn len(&self) -> usize {
        77 + self.data.len()
    }

    fn pack(&self, buf: &mut [u8]) {
        unimplemented!()
    }
}

/// DeviceManagementProtocol PDU with SET PROPERTY vector.
pub struct E131DataPacketDeviceManagementProtocolLayer<'a> {
    pub property_values: &'a [u8],
}

impl<'a> ProtocolDataUnit for E131DataPacketDeviceManagementProtocolLayer<'a> {
    fn len(&self) -> usize {
        10 + self.property_values.len()
    }

    fn pack(&self, buf: &mut [u8]) {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_root_layer_protocol() {
        let root_layer_protocol = RootLayerProtocol { pdu_block: &[] };
    }

    #[test]
    fn test_data_packet_framing_layer() {
        let data_packet_framing_layer = E131DataPacketFramingLayer {
            source_name: ArrayString::from("").unwrap(),
            priority: 100,
            synchronization_address: 0,
            sequence_number: 0,
            preview_data: false,
            stream_terminated: false,
            force_synchronization: false,
            universe: 0,
            data: E131DataPacketDeviceManagementProtocolLayer { property_values: &[] },
        };
        let root_layer_protocol = RootLayerProtocol {
            pdu_block: &[
                RootLayerProtocolDataUnit {
                    cid: Uuid::new_v4(),
                    data: &data_packet_framing_layer,
                },
            ],
        };
    }
}
