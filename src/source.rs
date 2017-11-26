// Copyright 2016 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Result, Error, ErrorKind};
use std::net::UdpSocket;
use net2::UdpBuilder;
use uuid::Uuid;

use packet;

fn universe_to_ip(universe: u16) -> Result<String> {
    if universe == 0 || universe > 63999 {
        return Err(Error::new(ErrorKind::InvalidInput,
                              "universe is limited to the range 1 to 63999"));
    }
    let high_byte = (universe >> 8) & 0xff;
    let low_byte = universe & 0xff;
    Ok(format!("239.255.{}.{}:5568", high_byte, low_byte))
}

fn create_cid() -> [u8; 16] {
    let uuid = Uuid::new_v4();
    let mut cid = [0u8; 16];
    for (&x, p) in uuid.as_bytes().iter().zip(cid.iter_mut()) {
        *p = x;
    }
    cid
}

/// A DMX over sACN sender.
///
/// DmxSource is used for sending sACN packets over ethernet.
///
/// Each universe will be sent to a dedicated multicast address
/// "239.255.{universe_high_byte}.{universe_low_byte}".
///
/// # Examples
///
/// ```
/// use sacn::DmxSource;
///
/// let mut dmx_source = DmxSource::new("Controller").unwrap();
///
/// dmx_source.send(1, &[100, 100, 100, 100, 100, 100]);
/// dmx_source.terminate_stream(1);
/// ```
#[derive(Debug)]
pub struct DmxSource {
    socket: UdpSocket,
    cid: [u8; 16],
    name: String,
    preview_data: bool,
    start_code: u8,
    sequences: RefCell<HashMap<u16, u8>>,
}

impl DmxSource {
    /// Constructs a new DmxSource with DMX START code set to 0.
    pub fn new(name: &str) -> Result<DmxSource> {
        let cid = create_cid();
        DmxSource::with_cid(name, &cid)
    }
    /// Consturcts a new DmxSource with binding to the supplied ip and a DMX START code set to 0.
    pub fn with_ip(name: &str, ip: &str) -> Result<DmxSource> {
        let cid = create_cid();
        DmxSource::with_cid_ip(name, &cid, ip)
    }

    /// Constructs a new DmxSource with DMX START code set to 0 with specified CID.
    pub fn with_cid(name: &str, cid: &[u8; 16]) -> Result<DmxSource> {
        let ip = "0.0.0.0";
        DmxSource::with_cid_ip(name, &cid, &ip)
    }
    /// Constructs a new DmxSource with DMX START code set to 0 with specified CID and IP address.
    pub fn with_cid_ip(name: &str, cid: &[u8; 16], ip: &str) -> Result<DmxSource> {
        let ip_port = format!("{}:0", ip);
        let sock_builder = try!(UdpBuilder::new_v4());
        let sock = try!(sock_builder.bind(&ip_port));

        Ok(DmxSource {
            socket: sock,
            cid: cid.clone(),
            name: name.to_string(),
            preview_data: false,
            start_code: 0,
            sequences: RefCell::new(HashMap::new()),
        })
    }

    /// Sends DMX data to specified universe.
    pub fn send(&self, universe: u16, data: &[u8]) -> Result<()> {
        self.send_with_priority(universe, data, 100)
    }

    /// Sends DMX data to specified universe with specified priority.
    pub fn send_with_priority(&self, universe: u16, data: &[u8], priority: u8) -> Result<()> {
        if priority > 200 {
            return Err(Error::new(ErrorKind::InvalidInput, "priority must be <= 200"));
        }
        let ip = try!(universe_to_ip(universe));
        let mut sequence = match self.sequences.borrow().get(&universe) {
            Some(s) => s.clone(),
            None => 0,
        };

        let sacn_packet = try!(packet::pack_acn(&self.cid,
                                                universe,
                                                &*self.name,
                                                priority,
                                                sequence,
                                                self.preview_data,
                                                false,
                                                self.start_code,
                                                data));
        try!(self.socket.send_to(&sacn_packet, &*ip));

        if sequence == 255 {
            sequence = 0;
        } else {
            sequence += 1;
        }
        self.sequences.borrow_mut().insert(universe, sequence);
        Ok(())
    }

    /// Terminates a universe stream.
    ///
    /// Terminates a stream to a specified universe by sending three packages with
    /// Stream_Terminated flag set to 1.
    pub fn terminate_stream(&self, universe: u16) -> Result<()> {
        let ip = try!(universe_to_ip(universe));
        let mut sequence = match self.sequences.borrow_mut().remove(&universe) {
            Some(s) => s,
            None => 0,
        };

        for _ in 0..3 {
            let sacn_packet = try!(packet::pack_acn(&self.cid,
                                                    universe,
                                                    &*self.name,
                                                    200,
                                                    sequence,
                                                    false,
                                                    true,
                                                    0,
                                                    &[]));
            try!(self.socket.send_to(&sacn_packet, &*ip));

            if sequence == 255 {
                sequence = 0;
            } else {
                sequence += 1;
            }
        }
        Ok(())
    }

    /// Returns the ACN CID device identifier of the DmxSource.
    pub fn cid(&self) -> &[u8; 16] {
        &self.cid
    }

    /// Sets the ACN CID device identifier.
    pub fn set_cid(&mut self, cid: [u8; 16]) -> Result<()> {
        self.cid = cid;
        Ok(())
    }

    /// Returns the ACN source name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets ACN source name.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Returns if DmxSource is in preview mode.
    pub fn preview_mode(&self) -> bool {
        self.preview_data
    }

    /// Sets the DmxSource to preview mode.
    ///
    /// All packets will be sent with Preview_Data flag set to 1.
    pub fn set_preview_mode(&mut self, preview_mode: bool) {
        self.preview_data = preview_mode;
    }

    /// Returns the current DMX START code.
    pub fn start_code(&self) -> u8 {
        self.start_code
    }

    /// Sets the DMX START code.
    pub fn set_start_code(&mut self, start_code: u8) {
        self.start_code = start_code;
    }

    /// Sets the multicast time to live.
    pub fn set_multicast_ttl(&self, multicast_ttl: u32) -> Result<()> {
        self.socket.set_multicast_ttl_v4(multicast_ttl)
    }

    /// Returns the multicast time to live of the socket.
    pub fn multicast_ttl(&self) -> Result<u32> {
        self.socket.multicast_ttl_v4()
    }

    /// Sets if multicast loop is enabled.
    pub fn set_multicast_loop(&self, multicast_loop: bool) -> Result<()> {
        self.socket.set_multicast_loop_v4(multicast_loop)
    }

    /// Returns if multicast loop of the socket is enabled.
    pub fn multicast_loop(&self) -> Result<bool> {
        self.socket.multicast_loop_v4()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter;
    use std::net::Ipv4Addr;
    use net2::UdpBuilder;

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_dmx_source() {
        let cid = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let universe = 1;
        let source_name = "SourceName";
        let priority = 150;
        let sequence = 0;
        let preview_data = false;
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
        packet.push(0);
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

        let mut source = DmxSource::with_cid(&source_name, &cid).unwrap();
        source.set_preview_mode(preview_data);
        source.set_start_code(start_code);
        source.set_multicast_loop(true).unwrap();

        let recv_socket = UdpBuilder::new_v4().unwrap().bind("0.0.0.0:5568").unwrap();
        recv_socket.join_multicast_v4(&Ipv4Addr::new(239, 255, 0, 1), &Ipv4Addr::new(0, 0, 0, 0))
                   .unwrap();

        let mut recv_buf = [0; 1024];

        source.send_with_priority(universe, &dmx_data, priority).unwrap();
        let (amt, _) = recv_socket.recv_from(&mut recv_buf).unwrap();

        assert_eq!(&packet[..], &recv_buf[0..amt]);

        drop(source);
        drop(recv_socket);
    }

    #[test]
    fn test_terminate_stream() {
        let source = DmxSource::new("Source").unwrap();
        source.set_multicast_loop(true).unwrap();

        let recv_socket = UdpBuilder::new_v4().unwrap().bind("0.0.0.0:5568").unwrap();
        recv_socket.join_multicast_v4(&Ipv4Addr::new(239, 255, 0, 1), &Ipv4Addr::new(0, 0, 0, 0))
                   .unwrap();

        let mut recv_buf = [0; 1024];

        source.terminate_stream(1).unwrap();
        for _ in 0..2 {
            recv_socket.recv_from(&mut recv_buf).unwrap();
            assert_eq!(recv_buf[112], 0b0100_0000)
        }

        drop(source);
        drop(recv_socket);
    }
}
