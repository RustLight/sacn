#![allow(dead_code)]
#![allow(unused_imports)]

extern crate lazy_static;
extern crate sacn;

use std::{thread};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::io::Error;
use std::net::{SocketAddr, Ipv4Addr};
use sacn::{DmxSource};
use sacn::recieve::{SacnReceiver, DMXData, ACN_SDT_MULTICAST_PORT};
use sacn::packet::UNIVERSE_CHANNEL_CAPACITY;

// Report: Should start code be seperated out when receiving? Causes input and output to differ and is technically part of another protocol.
// - Decided it shouldn't be seperated.

/// ANSI E1.31-2018 Sections with tests that show compliance:
/// 1.1 Scope - 
/// 1.2 Overview and Architecture
/// 1.3 Appropriate Use of This Standard
/// 1.4 Classes of Data Appropriate for Transmission
/// 1.5 Universe Synchronization
/// 1.6 Universe Discovery
/// 1.7 Compliance
/// 2 Normative References
/// 3 Definitions
/// 4 Protocol Packet Structure Summary
/// 4.1 E1.31 Data Packet
/// 4.2 E1.31 Synchronization Packet
/// 4.3 E1.31 Universe Discovery Packet
/// 5 E1.31 use of the ACN Root Layer Protocol
/// 5.1 Preamble Size
/// 5.2 Post-amble Size
/// 5.3 ACN Packet Identifier
/// 5.4 Flags & Length
/// 5.5 Vector
/// .5.6 CID (Component Identifier)
/// 6 E1.31 Framing Layer Protocol
/// 6.1 Flags & Length
/// 6.2 E1.31 Data Packet Framing Layer
/// 6.2.1 E1.31 Data Packet: Vector
/// 6.2.2 E1.31 Data Packet: Source Name
/// 6.2.3 E1.31 Data Packet: Priority
/// 6.2.3.1 Multiple Sources at Highest Priority
/// 6.2.3.2 Note on Merge and Arbitration Algorithms
/// 6.2.3.3 Note on Resolution of Sources Exceeded Condition
/// 6.2.3.4 Requirements for Merging and Arbitrating
/// 6.2.3.5 Requirements for Sources Exceeded Resolution
/// 6.2.3.6 Requirements for Devices with Multiple Operating Modes
/// 6.2.4 E1.31 Data Packet: Synchronization Address
/// 6.2.4.1 Synchronization Address Usage in an E1.31 Data Packet
/// 6.2.5 E1.31 Data Packet: Sequence Number
/// 6.2.6 E1.31 Data Packet: Options
/// 6.2.7 E1.31 Data Packet: Universe
/// 6.3 E1.31 Synchronization Packet Framing Layer
/// 6.3.1 E1.31 Synchronization Packet: Vector
/// 6.3.2 E1.31 Synchronization Packet: Sequence Number
/// 6.3.3 E1.31 Synchronization Packet: Synchronization Address
/// 6.3.3.1 Synchronization Address Usage in an E1.31 Synchronization Packet
/// 6.3.4 E1.31 Synchronization Packet: Reserved
/// 6.4 E1.31 Universe Discovery Packet Framing Layer
/// 6.4.1 E1.31 Universe Discovery Packet: Vector
/// 6.4.2 E1.31 Universe Discovery Packet: Source Name
/// 6.4.3 E1.31 Universe Discovery Packet: Reserved
/// 6.5 Processing by Receivers
/// 6.6 Framing Layer Operation and Timing - Source Requirements
/// 6.6.1 Transmission Rate
/// 6.6.2 Null START Code Transmission Requirements in E1.31 Data Packets
/// 6.7 Framing Layer Operation and Timing - Receiver Requirements
/// 6.7.1 Network Data Loss
/// 6.7.1.1 Network Data Loss and Universe Discovery
/// 6.7.2 Sequence Numbering
/// 7 DMP Layer Protocol
/// 7.1 DMP Layer: Flags & Length
/// 7.2 DMP Layer: Vector
/// 7.3 Address Type and Data Type
/// 7.4 First Property Address
/// 7.5 Address Increment
/// 7.6 Property Value Count
/// 7.7 Property Values (DMX512-A Data)
/// 8 Universe Discovery Layer
/// 8.1 Flags and Length
/// 8.2 Universe Discovery Layer: Vector
/// 8.3 Page
/// 8.4 Last Page
/// 8.5 List of Universes
/// 9 Operation of E1.31 in IPv4 and IPv6 Networks
/// 9.1 Association of Multicast Addresses and Universe
/// 9.1.1 Multicast Addressing
/// 9.2 Multicast Subscription
/// 9.3 Allocation of Multicast Addresses
/// 9.3.1 Allocation of IPv4 Multicast Addresses
/// 9.3.2 Allocation of IPv6 Multicast Addresses
/// 9.4 IPv4 and IPv6 Support Requirements
/// 10 Translation between DMX512-A and E1.31 Data Transmission
/// 10.1 DMX512-A to E1.31 Translation
/// 10.1.1 Boot Condition
/// 10.1.2 Temporal Sequence
/// 10.1.3 Loss of Data
/// 10.2 E1.31 to DMX512-A Translation
/// 10.2.1 General
/// 10.2.2 Loss of Data
/// 11 Universe Synchronization
/// 11.1 Synchronized and Unsynchronized Data
/// 11.1.1 When to Begin Synchronizing Data
/// 11.1.2 When to Stop Synchronizing Data
/// 11.2 Synchronization Timings in a Streaming Environment
/// 11.2.1 Arrival of Multiple Packets Before Processing
/// 11.2.2 Delays Before Universe Synchronization
/// 12 Universe Discovery
/// 12.1 Universe Discovery and Termination
/// 12.2 Termination of Stream Transmission
///
/// Appendix A: Defined Parameters (Normative)
/// Appendix B: An Example of Universe Synchronization For Implementors (Informative)
/// B.1 Universe Synchronization for Sources
/// B.2 Universe Synchronization for Receivers
///
/// Table 4-1: E1.31 Data Packet
/// Table 4-2: E1.31 Synchronization Packet Format
/// Table 4-3: E1.31 Universe Discovery Packet Format
/// Table 5-1: E1.31 Root Layer
/// Table 6-1: E1.31 Data Packet Framing Layer
/// Table 6-2: E1.31 Synchronization Packet Framing Layer
/// Table 6-3: E1.31 Universe Discovery Packet Framing Layer
/// Table 7-1: E1.31 Data Packet DMP Layer
/// Table 8-1: E1.31 Universe Discovery Packet Universe Discovery Layer
/// Table 9-1: IPv4 Universe - IP mapping
/// Table 9-2: IPv6 Multicast Address Format
/// Table 9-3: IPv6 Universe - IP mapping
/// Table B-1: Universe Synchronization Example E1.31 Data Packet
/// Table B-2: Universe Synchronization Example E1.31 Synchronization Packet
///
/// Figure 5-1: RLP Flags and Length
/// Figure 6-1: E1.31 Flags and Length
/// Figure 7-1: DMP Flags and Length
/// Figure 8-1: Universe Discovery Flags and Length

#[test]
fn test_send_recv_single_universe(){
    let (tx, rx): (Sender<Result<Vec<DMXData>, Error>>, Receiver<Result<Vec<DMXData>, Error>>) = mpsc::channel();

    let thread_tx = tx.clone();

    let universe = 1;

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = match SacnReceiver::new(SocketAddr::new(Ipv4Addr::new(0,0,0,0).into(), ACN_SDT_MULTICAST_PORT)){
            Ok(sr) => sr,
            Err(_) => panic!("Failed to create sacn receiver!")
        };

        dmx_recv.set_nonblocking(false).unwrap();

        dmx_recv.listen_universes(&[universe]).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap();

        thread_tx.send(dmx_recv.recv()).unwrap();
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let mut dmx_source = DmxSource::new("Controller").unwrap();

    let priority = 100;

    dmx_source.register_universe(universe);

    let _ = dmx_source.send(&[universe], &TEST_DATA_SINGLE_UNIVERSE, priority).unwrap();

    let received_result: Result<Vec<DMXData>, Error> = rx.recv().unwrap();

    rcv_thread.join().unwrap();

    assert!(!received_result.is_err(), "Failed: Error when receving data");

    let received_data: Vec<DMXData> = received_result.unwrap();

    assert_eq!(received_data.len(), 1); // Check only 1 universe received as expected.

    let received_universe: DMXData = received_data[0].clone();

    assert_eq!(received_universe.universe, universe); // Check that the universe received is as expected.

    assert_eq!(received_universe.values, TEST_DATA_SINGLE_UNIVERSE.to_vec(), "Received payload values don't match sent!");
}

/// Note: this test assumes perfect network conditions (0% reordering, loss, duplication etc.), this should be the case for
/// the loopback adapter with the low amount of data sent but this may be a possible cause if integration tests fail unexpectedly.
#[test]
fn test_send_recv_across_universe(){
    let (tx, rx): (Sender<Result<Vec<DMXData>, Error>>, Receiver<Result<Vec<DMXData>, Error>>) = mpsc::channel();

    let thread_tx = tx.clone();

    const UNIVERSES: [u16; 2] = [2, 3];

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = match SacnReceiver::new(SocketAddr::new(Ipv4Addr::new(0,0,0,0).into(), ACN_SDT_MULTICAST_PORT)){
            Ok(sr) => sr,
            Err(_) => panic!("Failed to create sacn receiver!")
        };

        dmx_recv.set_nonblocking(false).unwrap();

        dmx_recv.listen_universes(&UNIVERSES).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap(); // Signal that the receiver is ready to receive.

        thread_tx.send(dmx_recv.recv()).unwrap(); // Receive the sync packet, the data packets shouldn't have caused .recv to return as forced to wait for sync.
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let mut dmx_source = DmxSource::new("Controller").unwrap();

    let priority = 100;

    dmx_source.register_universes(&UNIVERSES);

    dmx_source.send(&UNIVERSES, &TEST_DATA_MULTIPLE_UNIVERSE, priority).unwrap();

    let sync_pkt_res: Result<Vec<DMXData>, Error> = rx.recv().unwrap();

    rcv_thread.join().unwrap();

    assert!(!sync_pkt_res.is_err(), "Failed: Error when receving packets");

    let mut received_data: Vec<DMXData> = sync_pkt_res.unwrap();

    received_data.sort(); // No guarantee on the ordering of the receieved data so sort it first to allow easier checking.

    assert_eq!(received_data.len(), 2); // Check 2 universes received as expected.

    assert_eq!(received_data[0].universe, 2); // Check that the universe received is as expected.

    assert_eq!(received_data[0].sync_uni, 2); // Check that the sync universe is as expected.

    assert_eq!(received_data[0].values, TEST_DATA_MULTIPLE_UNIVERSE[..UNIVERSE_CHANNEL_CAPACITY].to_vec(), "Universe 1 received payload values don't match sent!");

    assert_eq!(received_data[1].universe, 3); // Check that the universe received is as expected.

    assert_eq!(received_data[1].sync_uni, 2); // Check that the sync universe is as expected.

    assert_eq!(received_data[1].values, TEST_DATA_MULTIPLE_UNIVERSE[UNIVERSE_CHANNEL_CAPACITY..].to_vec(), "Universe 2 received payload values don't match sent!");
}

const TEST_DATA_SINGLE_UNIVERSE: [u8; 512] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100,

        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100,

        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100,

        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100,

        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100,

        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12
    ];

const TEST_DATA_MULTIPLE_UNIVERSE: [u8; 712] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100,

        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100,

        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100,

        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100,

        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100,

        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,

        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100,

        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100,
    ];