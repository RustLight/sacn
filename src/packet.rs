// Copyright 2016 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::{Result, Error, ErrorKind};
use std::iter::repeat;

// length as low12 and flags
fn length_as_low12(length: u16) -> [u8; 2] {
    let u = 0x7000 | length;
    [(u >> 8) as u8, u as u8]
}

fn pack_dmp_layer(start_code: u8, dmx_data: &[u8]) -> Result<Vec<u8>> {
    if dmx_data.len() > 512 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "max 512 channels per universe allowed",
        ));
    }
    let mut packet = Vec::with_capacity(11 + dmx_data.len());
    // Flags and Length
    packet.extend(length_as_low12(11 + (dmx_data.len() as u16)).iter());
    // Vector
    packet.push(0x02);
    // Address Type & Data Type
    packet.push(0xa1);
    // First Property Address
    packet.extend("\x00\x00".bytes());
    // Address Increment
    packet.extend("\x00\x01".bytes());
    // Property value count
    let count = 1 + dmx_data.len();
    packet.push((count >> 8) as u8);
    packet.push(count as u8);
    // Property values
    packet.push(start_code);
    packet.extend(dmx_data);

    Ok(packet)
}

fn pack_framing_layer(
    universe: u16,
    source_name: &str,
    priority: u8,
    sequence: u8,
    preview_data: bool,
    stream_terminated: bool,
    dmp_packet: &[u8],
) -> Result<Vec<u8>> {
    let mut packet = Vec::with_capacity(77 + dmp_packet.len());
    // Flags and Length
    packet.extend(length_as_low12(77 + (dmp_packet.len() as u16)).iter());
    // Vector
    packet.extend("\x00\x00\x00\x02".bytes());
    // Source Name
    packet.extend(source_name.bytes());
    packet.extend(repeat(0).take(64 - source_name.len()));
    // Priority
    packet.push(priority);
    // Reserved
    packet.extend("\x00\x00".bytes());
    // Sequence Number
    packet.push(sequence);
    // Options
    packet.push(if preview_data && stream_terminated {
        0b1100_0000
    } else if preview_data {
        0b1000_0000
    } else if stream_terminated {
        0b0100_0000
    } else {
        0
    });
    // Universe
    packet.push((universe >> 8) as u8);
    packet.push(universe as u8);

    packet.extend(dmp_packet);

    Ok(packet)
}

fn pack_acn_root_layer(cid: &[u8; 16], framing_packet: &[u8]) -> Result<Vec<u8>> {
    let mut packet = Vec::with_capacity(38 + framing_packet.len());
    // Preamble Size
    packet.extend("\x00\x10".bytes());
    // Post-amble Size
    packet.extend("\x00\x00".bytes());
    // ACN Packet Identifie
    packet.extend("ASC-E1.17\x00\x00\x00".bytes());
    // Flags and Length
    packet.extend(length_as_low12(22 + (framing_packet.len() as u16)).iter());
    // Vector
    packet.extend("\x00\x00\x00\x04".bytes());
    // CID
    packet.extend(cid);

    packet.extend(framing_packet);

    Ok(packet)
}

pub fn pack_acn(
    cid: &[u8; 16],
    universe: u16,
    source_name: &str,
    priority: u8,
    sequence: u8,
    preview_data: bool,
    stream_terminated: bool,
    start_code: u8,
    dmx_data: &[u8],
) -> Result<Vec<u8>> {
    let dmp_packet = try!(pack_dmp_layer(start_code, dmx_data));
    let framing_packet = try!(pack_framing_layer(
        universe,
        source_name,
        priority,
        sequence,
        preview_data,
        stream_terminated,
        &dmp_packet,
    ));
    Ok(try!(pack_acn_root_layer(cid, &framing_packet)))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter;

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_pack_acn() {
        let cid = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let universe = 1;
        let source_name = "SourceName";
        let priority = 150;
        let sequence = 200;
        let preview_data = false;
        let stream_terminated = true;
        let start_code = 0;
        let mut dmx_data: Vec<u8> = Vec::new();
        dmx_data.extend(iter::repeat(100).take(255));

        // Root Layer
        let mut packet = Vec::new();
        // Preamble Size
        packet.extend("\x00\x10".bytes());
        // Post-amble Size
        packet.extend("\x00\x00".bytes());
        // ACN Packet Identifie
        packet.extend("\x41\x53\x43\x2d\x45\x31\x2e\x31\x37\x00\x00\x00".bytes());
        // Flags and Length (22 + 343)
        packet.push(0b01110001);
        packet.push(0b01101101);
        // Vector
        packet.extend("\x00\x00\x00\x04".bytes());
        // CID
        packet.extend(&cid);

        // E1.31 Framing Layer
        // Flags and Length (77 + 266)
        packet.push(0b01110001);
        packet.push(0b01010111);
        // Vector
        packet.extend("\x00\x00\x00\x02".bytes());
        // Source Name
        let source_name = source_name.to_string() +
                          "\0\0\0\0\0\0\0\0\0\0" +
                          "\0\0\0\0\0\0\0\0\0\0" +
                          "\0\0\0\0\0\0\0\0\0\0" +
                          "\0\0\0\0\0\0\0\0\0\0" +
                          "\0\0\0\0\0\0\0\0\0\0" +
                          "\0\0\0\0";

        assert_eq!(source_name.len(), 64);
        packet.extend(source_name.bytes());
        // Priority
        packet.push(priority);
        // Reserved
        packet.extend("\x00\x00".bytes());
        // Sequence Number
        packet.push(sequence);
        // Options
        packet.push(0b0100_0000);
        // Universe
        packet.push(0);
        packet.push(1);

        // DMP Layer
        // Flags and Length (266)
        packet.push(0b01110001);
        packet.push(0b00001010);
        // Vector
        packet.push(0x02);
        // Address Type & Data Type
        packet.push(0xa1);
        // First Property Address
        packet.extend("\x00\x00".bytes());
        // Address Increment
        packet.extend("\x00\x01".bytes());
        // Property value count
        packet.push(0b1);
        packet.push(0b00000000);
        // Property values
        packet.push(start_code);
        packet.extend(&dmx_data);

        assert_eq!(packet,
                   pack_acn(&cid,
                            universe,
                            &source_name,
                            priority,
                            sequence,
                            preview_data,
                            stream_terminated,
                            start_code,
                            &dmx_data)
                       .unwrap());
    }
}
