// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Packets whose PDU length fields are shorter than the fixed fields of that layer must be
//! rejected with an error rather than causing a panic.

use sacn::error::errors::SacnError;
use sacn::packet::AcnRootLayerProtocol;
use sacn::sacn_parse_pack_error::ParsePacketError;

/// Preamble size, post-amble size and ACN packet identifier.
fn acn_header() -> Vec<u8> {
    let mut v = vec![0x00, 0x10, 0x00, 0x00];
    v.extend_from_slice(b"ASC-E1.17\0\0\0");
    v
}

fn assert_invalid_length(buf: &[u8]) {
    match AcnRootLayerProtocol::parse(buf) {
        Err(SacnError::SacnParsePackError(ParsePacketError::PduInvalidLength(_))) => {}
        other => panic!("expected PduInvalidLength, got {other:?}"),
    }
}

#[test]
fn test_root_layer_length_shorter_than_cid() {
    let mut buf = acn_header();
    // Root layer length of 6 covers only flags/length and vector, not the CID.
    buf.extend_from_slice(&[0x70, 0x06, 0x00, 0x00, 0x00, 0x04]);
    buf.extend_from_slice(&[0u8; 40]);
    assert_invalid_length(&buf);
}

#[test]
fn test_data_framing_layer_length_shorter_than_header() {
    let mut buf = acn_header();
    // Root layer: length 28 = 22 header bytes + 6 framing bytes.
    buf.extend_from_slice(&[0x70, 0x1c, 0x00, 0x00, 0x00, 0x04]);
    buf.extend_from_slice(&[0u8; 16]); // CID
    // Data framing layer of length 6: flags/length and vector only.
    buf.extend_from_slice(&[0x70, 0x06, 0x00, 0x00, 0x00, 0x02]);
    buf.extend_from_slice(&[0u8; 100]);
    assert_invalid_length(&buf);
}

#[test]
fn test_dmp_layer_length_shorter_than_header() {
    let framing_len: u8 = 77 + 3;
    let mut buf = acn_header();
    buf.extend_from_slice(&[0x70, 22 + framing_len, 0x00, 0x00, 0x00, 0x04]);
    buf.extend_from_slice(&[0u8; 16]); // CID
    // Data framing layer header.
    buf.extend_from_slice(&[0x70, framing_len, 0x00, 0x00, 0x00, 0x02]);
    buf.extend_from_slice(&[0u8; 64]); // Source name
    buf.extend_from_slice(&[100, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]); // Priority, sync addr, seq, options, universe 1
    // DMP layer of length 3: flags/length and vector only.
    buf.extend_from_slice(&[0x70, 0x03, 0x02]);
    buf.extend_from_slice(&[0u8; 20]);
    assert_invalid_length(&buf);
}

/// Every truncation of the PDU length fields of a valid data packet must return an error, never panic.
#[test]
fn test_all_short_lengths_rejected_without_panic() {
    let mut valid = acn_header();
    let dmp_len: u16 = 10 + 513;
    let framing_len: u16 = 77 + dmp_len;
    let root_len: u16 = 22 + framing_len;
    valid.extend_from_slice(&(0x7000 | root_len).to_be_bytes());
    valid.extend_from_slice(&[0x00, 0x00, 0x00, 0x04]);
    valid.extend_from_slice(&[0u8; 16]);
    valid.extend_from_slice(&(0x7000 | framing_len).to_be_bytes());
    valid.extend_from_slice(&[0x00, 0x00, 0x00, 0x02]);
    valid.extend_from_slice(&[0u8; 64]);
    valid.extend_from_slice(&[100, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);
    valid.extend_from_slice(&(0x7000 | dmp_len).to_be_bytes());
    valid.extend_from_slice(&[0x02, 0xa1, 0x00, 0x00, 0x00, 0x01]);
    valid.extend_from_slice(&513u16.to_be_bytes());
    valid.extend_from_slice(&[0u8; 513]);
    assert!(AcnRootLayerProtocol::parse(&valid).is_ok());

    // Offsets of the root, framing and DMP flags/length fields.
    for offset in [16usize, 38, 115] {
        for len in 0u16..0x0fff {
            let mut buf = valid.clone();
            buf[offset..offset + 2].copy_from_slice(&(0x7000 | len).to_be_bytes());
            let _ = AcnRootLayerProtocol::parse(&buf);
        }
    }
}
