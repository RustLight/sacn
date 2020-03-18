extern crate sacn;
extern crate uuid;

#[cfg(test)]
pub mod sync_parse_tests {

use sacn::packet::*;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};
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

#[test]
fn test_synchronization_packet_parse_pack() {
    let packet = AcnRootLayerProtocol {
        pdu: E131RootLayer {
            cid: Uuid::from_bytes(&TEST_SYNCHRONIZATION_PACKET[22..38]).unwrap(),
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
}