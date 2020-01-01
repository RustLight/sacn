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
/// 1.1 Scope - No specific test
/// 1.2 Overview and Architecture 
    /// - Allows transfer of arbitary START code DMX512-A data:
    /// - DMX data can be synchronized across multiple receivers using universe syncronisation
    /// - Uses a ACN wrapper meaning it is compatiable with devices following the ANSI E.1.17 [ACN] standard: 
    /// - Uses UDP as the transport/IP layer protocol:
    /// - Supports multicast addressing:
    /// - Supports unicast addressing: 
/// 1.3 Appropriate Use of This Standard
    /// - Uses UDP to provide a non-reliable IP transport mechanism
    /// - Allows multiple senders and receivers
/// 1.4 Classes of Data Appropriate for Transmission
    /// - Allows transfer of arbitary START code DMX512-A data:
/// 1.5 Universe Synchronization
    /// - Allows synchronisation through the universe synchronisation mechanism:
/// 1.6 Universe Discovery
    /// - Allows universe discovery through the universe discovery mechanism
/// 3 Definitions
/// 3.5 Source
    /// - A source is uniquely identified by a number in the header of the packet:
    /// - A source may send multiple streams of data for different universes:
    /// - Multiple sources may output data for a given universe:
/// 3.6 Receiver
    /// - A receiever may listen on multiple universes
/// 3.7 Active Data Slots
    /// - Sources for E1.31 should specify the location and amount of active data slots
    ///     using the DMP First Property Address and DMP Property Count fields (shown in Table 4-1):
/// 3.8 E1.31 Data Packet
    /// - Identified by being transmitted with the VECTOR_E131_DATA_PACKET vector:
/// 3.9 E.31 Synchronization Packet
    /// - Contains only universe synchronisation information and no additional data:
    /// - Identified by being transmitted with the VECTOR_E131_EXTENDED_SYNCHRONIZATION vector:
/// 3.10 E1.31 Universe Discovery Packet
    /// - Identified by being transmitted with the VECTOR_E131_EXTENDED_DISCOVERY vector:
/// 4 Protocol Packet Structure Summary
    /// - E1.31 components must support the £1.31 Data Packet and E1.31. Universe Discovery Packet:
    /// - E1.31 components may support the E1.31 synchronization packet:
/// 4.1 E1.31 Data Packet
    /// - Data is formatted as specified in Table 4-1 with all fields being correctly populated:
    /// - Detection of malformed packets:
    /// - All packet content must be transmitted in network byte order (big endian):
/// 4.2 E1.31 Synchronization Packet
    /// - A universe can be used as a synchronisation universe and to transmit data on simultaneously:
    /// - Packet is formatted as specified in Table 4-2 with all fields being correctly populated:
    /// - Detection of malformed packets:
    /// - All packet content must be transmitted in network byte order (big endian):
/// 4.3 E1.31 Universe Discovery Packet
    /// - A set of universe discovery packets shall be sent once every E131_UNIVERSE_DISCOVERY_INTERVAL:
    /// - The list of E1.31 universes must be sorted:
    /// - The list of universes may includes synchronisation universes:
    /// - If the list of universes changes within an E131_UNIVERSE_DISCOVERY_INTERVAL a source may send 
    ///     upto one additional set of packets to update the information:
    /// - Packet is formatted as specified in Table 4-3 with all fields being correctly populated:
    /// - Detection of malformed packets:
    /// - All packet content must be transmitted in network byte order (big endian):
/// 5 E1.31 use of the ACN Root Layer Protocol
    /// - All E1.31 packets should use the ACN Root Layer Protocol as defined in ANSI E1.17 [ACN] specifically
    ///     the fields specified in Table 5-4 which is for E1.31 on UDP.
    /// - Detection of malformed packets:
/// 5.1 Preamble Size
    /// - The preamble size field must be 0x0010:
    /// - Packets with a different preamble size must be discarded:
    /// - The preamble (preamble size field, post-amble size field and ACN packet identifier) length must 
    ///     match the size given in the field (0x10 octets):
/// 5.2 Post-amble Size
    /// - There is no post amble for RLP over UDP so the post-amble size field must be 0:
    /// - E1.31 receivers must discard packets if the post-amble size is not 0x0000. 
/// 5.3 ACN Packet Identifier
    /// - The ACN packet identifier must be exactly 0x41 0x53 0x43 0x2d 0x45 0x31 0x2e 0x31 0x37 0x00 0x00 0x00:
    /// - E1.31 receivers must discard packets if the ACN packet identifier doesn't match above:
/// 5.4 Flags & Length
    /// - The PDU length must be encoded in the low 12 bits of the root layer flags and length field:
    /// - The flags (top 4 bits) must be 0x7:
    /// - The PDU length is computed started with octet 16 and counting all remaining octets in the packet including
    ///     all payload:
    /// - A ful payload data packet should have a length of 638 octets:
    /// - A synchronisation packet should have a length of 49 octets:
    /// - A universe discovery packet length should be computed to the end of the list of universes field:
/// 5.5 Vector
    /// The root layer vector must be VECTOR_ROOT_E131_DATA if the packet contains E1.31 data:
    /// The root layer vector must be VECTOR_ROOT_E131_EXTENDED if the packet is for univers discovery or synchronisation:
    /// The packet type / root layer vector cannot be both simultaneously:
    /// Receivers must discard a packet if the vector isn't one of the above:
/// 5.6 CID (Component Identifier)
    /// Must be a UUID - a universally unique identifier that is 128 bit number unique across space and time:
    /// The CID must be compliant with RFC 4122 [UUID]:
    /// A piece of equipment must maintain the same CID for its entire lifetime: 
    /// Must be transmitted in network byte order (big endian):
/// 6 E1.31 Framing Layer Protocol
/// 6.1 Flags & Length
    /// - Each framing layer must start with the flags & length field:
    /// - The field must be 16 bit with the PDU length encoded in the low 12 bits and 0x7 in the top 4 bits:
    /// - The PDU length must be computed starting with octet 38 and continue through the last octet provided by the underlying layer:
    /// - An E1.31 Data Packet with full payload must have a length of 638:
    /// - An E1.31 Universe Discovery Packet must have a length between 120 and 1144 depending on the list of universes:
/// 6.2 E1.31 Data Packet Framing Layer
    /// - The packet must be formatted as specified in Table 6-5:
/// 6.2.1 E1.31 Data Packet: Vector
    /// - The E1.31 layer vector must be VECTOR_E131_DATA_PACKET for an E1.31 Data Packet
/// 6.2.2 E1.31 Data Packet: Source Name
    /// - The source name must be null-terminated:
    /// - The source name of a component must match the UACN field as specified in EPI 19 [ACN]:
    /// - The source name may be the same across multiple universes sourced by the same component:
    /// - The source name should be unique: Left to the implementer / user-configuration
/// 6.2.3 E1.31 Data Packet: Priority
    /// - The most recent E1.31 Data Packet from a single source must supersede any previous packet
    ///     from that source:
    /// - A receiver may receiver data for the same universe from multiple sources which is distinguished by examining
    ///     the CID in the packet:
    /// - The priority field must be in the range 0 to 200
    /// - Data from sources with a higher priority (e.g. 200 vs 100) will be treated as the defininive data for that universe.
    /// - If the E1.31 receiver is also doing universe syncronisation then the behaviour is undefined:
/// 6.2.3.1 Multiple Sources at Highest Priority
    /// - If there are multiple sources transmitting data at the same highest currently active priority for a given
    ///     universe then this must be handled:
    /// - If a receiver is only capable of processing a certain number of sources of data it will encounter a sources exceeded
    ///     condition when a greater number of sources are present:
/// 6.2.3.2 Note on Merge and Arbitration Algorithms
    /// - Allow various merging algorithms for combining data from multiple sources:
/// 6.2.3.3 Note on Resolution of Sources Exceeded Condition
    /// - Various possible resolution mechanisms shouldbe possible:
    /// - Resolution mechanisms are recommended to not generate different results from the same 
    ///     source combination on different occasions as it can make troubleshooting more difficult:
/// 6.2.3.4 Requirements for Merging and Arbitrating
    /// - The ability to merge/arbitrate between multiple sources, the maximum number of sources which
    ///     can be handled and the algorithm used should all be declared in user documentation for the device: Left to the implementer
/// 6.2.3.5 Requirements for Sources Exceeded Resolution
    /// - The resolution behaviour for equipment to resolve a source exceeded condition should be specified in the user documentation:
    /// - The sources exceeded condition is highly recommended to be easily detected at the device aswell as potentially through the network:
/// 6.2.3.6 Requirements for Devices with Multiple Operating Modes
    /// - All different operating modes for a device should be compliant with the standard or or non-compliant configurations should be 
    ///     clearly declared as such.
/// 6.2.4 E1.31 Data Packet: Synchronization Address
    /// 
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