#![allow(dead_code)]
#![allow(unused_imports)]

extern crate lazy_static;
extern crate sacn;

use std::{thread};
use std::option;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::io::Error;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};
use sacn::{DmxSource};
use sacn::recieve::{SacnReceiver, DMXData};
use sacn::packet::{UNIVERSE_CHANNEL_CAPACITY, ACN_SDT_MULTICAST_PORT};

// Report: Should start code be seperated out when receiving? Causes input and output to differ and is technically part of another protocol.
// - Decided it shouldn't be seperated.

/// ANSI E1.31-2018 Sections with tests that show compliance:
/// 1.1 Scope - No specific test
/// 1.2 Overview and Architecture 
    /// - Allows transfer of arbitary START code DMX512-A data:
    /// - DMX data can be synchronized across multiple receivers using universe syncronisation:
    /// - Uses a ACN wrapper meaning it is compatiable with devices following the ANSI E.1.17 [ACN] standard: 
    /// - Uses UDP as the transport/IP layer protocol:
    /// - Supports multicast addressing: 
    /// - Supports unicast addressing: 
/// 1.3 Appropriate Use of This Standard
    /// - Uses UDP to provide a non-reliable IP transport mechanism:
    /// - Allows multiple senders and receivers:
/// 1.4 Classes of Data Appropriate for Transmission
    /// - Allows transfer of arbitary START code DMX512-A data:
/// 1.5 Universe Synchronization
    /// - Allows synchronisation through the universe synchronisation mechanism:
/// 1.6 Universe Discovery
    /// - Allows universe discovery through the universe discovery mechanism:
/// 3 Definitions
/// 3.5 Source
    /// - A source is uniquely identified by a number in the header of the packet:
    /// - A source may send multiple streams of data for different universes:
    /// - Multiple sources may output data for a given universe:
/// 3.6 Receiver
    /// - A receiever may listen on multiple universes:
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
/// 6.2.4.1 Synchronization Address Usage in an E1.31 Data Packet
    /// - A synchronisation address of value 0 indicates that the data isn't synchronised meaning any waiting
    ///     data must be discarded and the data acted on immediately.
    /// - A nonzero synchronization address means that the data is synchronised, if the receiever doesn't support universe
    ///     universe synchronisation the packet should be processed normally:  Doesn't apply as the implementation supports universe synchronisation.
    /// - A nonzero synchronisation address means that the data packet should be held until the arrival of the corresponding E1.31 synchronisation
    ///     packet to release it:
    /// - A receiver must not synchronise any data until it has receieved its first E1.31 synchronisation packet on the synchronisation address:
/// 6.2.5 E1.31 Data Packet: Sequence Number
    /// - Sources must maintain a sequence number for each universe transmitted:
    /// - The sequence number should be incremented by one for each packet sent on the universe:
/// 6.2.6 E1.31 Data Packet: Options
    /// - The most significant bit is the Preview_Data, when set to 1 this means that the data is intended for use that doesn't affect the live output e.g.
    ///     for visualisers or media server preview applications:
    /// - The Stream_Terminated bit (2nd most significant) triggers the termination of a stream or universe synchronisation without waiting
    ///     for timeout and to indicate that the termination is not due to a fault condition. When set to 1 the source of data for the universe
    ///     specified has terminated transmission of the universe:
    /// - A source should send three packets when terminating the universe source:
    /// - A receiver should enter network data loss condition when a packet with the stream terminated bit is set:
    /// - A receiver should ignore any property values in a packet with the stream termination bit set:
    /// - The Force_Synchronisation bit (3rd most significant) says how a receiver should handle the loss of synchronisation, if set to 0 then 
    ///     on synchronisation loss the reciever must not update / process any new packets until syncronisation is re-established / resumes:
    /// - If the Force_Synchronisation bit is set to 1 then if synchronisation is lost receivers may continue to process new E1.31 data packets
    ///     without having to wait for synchronisation to resume / re-etablish:
    /// - The least significant 5 bits of the field are reserved for future use and must be transmitted as 0:
    /// - The least significant 5 bits of the field should be ignored by receivers:
/// 6.2.7 E1.31 Data Packet: Universe
    /// - Universe values must be in the range 1 to 63999 inclusive:
    /// - Other universe values are reserved for future use and must not be used except for the E131_DISCOVERY UNIVERSE:
    /// - The E131_DISCOVERY_UNIVERSE: is used for universe discovery:
/// 6.3 E1.31 Synchronization Packet Framing Layer
    /// - The synchronisation packet framing layer must conform to Table 6-6:
/// 6.3.1 E1.31 Synchronization Packet: Vector
    /// - The E1.31 layer vector must have a value of VECTOR_E131_EXTENDED_SYNCHRONIZATION for an E1.31 Synchronization Packet:
/// 6.3.2 E1.31 Synchronization Packet: Sequence Number
    /// - Sources must maintain a sequence number for each universe transmitted:
    /// - The sequence number should be incremented by one for each packet sent on the universe:
/// 6.3.3 E1.31 Synchronization Packet: Synchronization Address
/// 6.3.3.1 Synchronization Address Usage in an E1.31 Synchronization Packet
    /// - A synchronisation packet with a synchronisation address of 0 is meaningless as the entire purpose of the packet is to
    ///     be used for universe synchronisation so should never be transmitted:
    /// - A synchronisation packet with a synchronisation address of 0 should be ignored by receievers:
    /// - When sending via multicast synchronisation packets must be sent only to the address corresponding to the synchronisation address:
    /// - Receievers may ignore synchronization packets sent to multicast address not corresponding to synchronisation addresses:
/// 6.3.4 E1.31 Synchronization Packet: Reserved
    /// - Octets 47-48 of a E1.31 Synchronisation packet are reserved for future used and must be transmitted as 0:
    /// - Octets 47-48 of a E1.31 Synchronisation packet must be ignored by receievers:
/// 6.4 E1.31 Universe Discovery Packet Framing Layer
    /// - Packets must be formatted as specified in Table 6-7:
/// 6.4.1 E1.31 Universe Discovery Packet: Vector
    /// - E1.31 Universe Discovery Packets must have the E1.31 layer vector set to VECTOR_E131_EXTENDED_DISCOVERY: 
/// 6.4.2 E1.31 Universe Discovery Packet: Source Name
    /// - The source name must be null-terminated:
    /// - The source name of a component must match the UACN field as specified in EPI 19 [ACN]:
    /// - The source name may be the same across multiple universes sourced by the same component:
    /// - The source name should be unique: Left to the implementer / user-configuration
/// 6.4.3 E1.31 Universe Discovery Packet: Reserved
    /// - Octets 108-111 of the E1.31 Universe Discovery Packets are reserved for future use and must be transmitted as 0:
    /// - Octets 108-111 of the E1.31 Universe Discovery Packets must be ignored by receievers:
/// 6.5 Processing by Receivers
    /// - Receievers must discard packets if the receieved value is not VECTOR_E131_DATA_PACKET, 
    ///     VECTOR_E131_EXTENDED_SYNCHRONIZATION or VECTOR_E131_EXTENDED_DISCOVERY:
    /// - Receivers that do not support universe synchronisation may ignore packets with VECTOR_E131_EXTENDED_SYNCHRONISATION: 
    ///     Doesn't apply as implementation supports universe synchronisation.
/// 6.6 Framing Layer Operation and Timing - Source Requirements
/// 6.6.1 Transmission Rate
    /// - E1.31 sources must not transmit packets for a given universe number at a rate which exceeds the maximum refresh 
    ///     rate specified in E1.11 [DMX] unless configured by the user to do so:
    /// - E1.11 places special restrictions on the maximum rate for alternate START Code packets in Section 8.5.3.2:
/// 6.6.2 Null START Code Transmission Requirements in E1.31 Data Packets
    /// - Transmission of Null START code data should only be done when it changes:
    /// - Before entering this period of transmission suppression three packets of the values should be sent:
    /// - During transmission suppression a single keep-alive packet should be transmitted at intervals of between
    ///     800mS and 1000mS, each keep-alive packet should have identical content to the last Null START Code data 
    ///     packet sent (but with sequence number still incremented normally):
    /// - These requirements do not apply to alternate START code data:
/// 6.7 Framing Layer Operation and Timing - Receiver Requirements
/// 6.7.1 Network Data Loss
    /// - Network data loss is a conditional defined as teh absence of reception of E1.31 packets from a given source for a
    ///     period of E131_NETWORK_DATA_LOSS_TIMEOUT:
    /// - or the recepit of a packet containing the options field Stream_Terminated set to 1:
    /// - Data loss is specific to a universe not a source:
    /// - A specific universe is considered disconnected on data loss:
/// 6.7.1.1 Network Data Loss and Universe Discovery
    /// - Sources experiencing a network data loss condition must reflect the change in the E1.31 Universe 
    ///     discovery list of universes no later than 2 E131_UNIVERSE_DISCOVER_INTERVAL's
/// 6.7.2 Sequence Numbering
    /// - Receivers that do not support sequence numbering of packets should ignore these fields: 
    /// - Receivers that support sequence numbering should evaluate sequence numbers seperately for each E1.31 
    ///     packet type and within each packet type seperately for each universe:
    /// - Receivers should process packets in the order received unless the sequence number of the packet receieved
    ///     minus the sequence number of the last accepted sequence number is less than or equal to 0 but greater than -20:
/// 7 DMP Layer Protocol
    /// - DMP data should only appear in E1.31 Data Packets and not E1.31 Sync or Discovery packets
    /// - The DMP data should be formatted as specified in Table 7-8
/// 7.1 DMP Layer: Flags & Length
    /// - The PDU length is encoded at the low 12 bits:
    /// - 0x7 must appear in the top 4 bits:
    /// - The DMP layer PDU length is computed starting at octet 115 and ends including the last value in the DMP PDU (octet 637 for a full payload):
/// 7.2 DMP Layer: Vector
    /// - The DMP layer vector must be set to VECTOR_DMP_SET_PROPERTY:
    /// - Receivers should discard packets if the receieved value is not VECTOR_DMP_SET_PROPERTY:
/// 7.3 Address Type and Data Type
    /// - The DMP layer address type and data type must be 0xa1:
    /// - Receivers must discard packets if the value is not 0xa1
/// 7.4 First Property Address
    /// - The DMP Layers first property address must be 0x0000:
    /// - Receivers must discard packets if the value is not 0x0000:
/// 7.5 Address Increment
    /// - The DMP layer address increment must be 0x0001:
    /// - Receivers must discard packets if the value is not 0x0001:
/// 7.6 Property Value Count
    /// - Must contain the number of DMX512-A [DMX] slots including the START code slot:
/// 7.7 Property Values (DMX512-A Data)
    /// - The first octet of the property values field is the DMX512-A START Code
    /// - The maximum number of data slots excluding the START Code is 512 data slots:
    /// - Alternate START Code data much be processed in compliance with ANSI E1.11 [DMX] Section 8.5.3.3:
/// 8 Universe Discovery Layer
    /// - The packet must be formatted as specified in Table 8-9:
/// 8.1 Flags and Length
    /// - The PDU length is encoded in the low 12 bits:
    /// - 0x7 must be encoded in the top 4 bits:
    /// - The PDU length is computed from octet 112 upto and including the last universe in the universe 
    ///     discovery PDU (octet 1143 for a full payload):
/// 8.2 Universe Discovery Layer: Vector
    /// - The university discovery layer vector must be VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST:
    /// - Receievers should discard packets if the received value is not VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST:
/// 8.3 Page
    /// - Indicates the page being specified in the set of universe discovery packets starting at 0:
/// 8.4 Last Page
    /// - Indicates the index of the last page in the set of universe discovery packets:
/// 8.5 List of Universes
    /// - Must be numerically sorted:
    /// - May be empty:
    /// - Should contain all of the universes upon which a source is actively transmitting 
    ///     E1.31 Data and Synchronisation information:
/// 9 Operation of E1.31 in IPv4 and IPv6 Networks
    /// - The standard can work over either and which modes are supported should be indicated:
/// 9.1 Association of Multicast Addresses and Universe
    /// - The standard should work over multicasting: Compliance shown by test_send_recv_single_universe_multicast_ipv4
    /// - The standard should also work using unicast: Compliance shown by test_send_recv_single_universe_unicast_ipv4
    /// - Addressing of multicast traffic done by setting 2 least significant bytes to the desired universe number 
    ///     or synchronisation address:
    /// - Sources operating over IPv4 and IPv6 simultaneously should transmit identical E1.31 packets regardless of IP transport used:
    /// - Recievers operating in IPv4 and IPV6 simultaneously should not process E1.31 packets differently based on the IP transport:
    /// - Receivers operating in IPv4 and IPv6 simultaneously seeing the same packet via both IP transports shall only act on one instance of that packet:
/// 9.1.1 Multicast Addressing
    /// - E1.31 devices should not transmit on address 239.255.255.0 through 239.255.255.255:
    /// - E1.31 devices shall not used universe number 0 or univere numbers [64000 - 65535] excluding universe 64214 (used for universe discovery only):
    /// - The identity of the universe must be determined by the universe number in packet and not assumed from multicast address:
    /// - E1.31 devices should also respond to E1.31 data receieved on its unicast address: Compliance shown by test_send_recv_single_universe_unicast_ipv4
    /// - When multicast addressing is used the UDP destination port shall be set to the standard ACN-SDT multicast port ACN_SDT_MULTICAST_PORT:
    /// - For unicast communication the ACN-SDT multicast port shall be used by default but may be configured differently: Compliance shown by test_send_recv_single_universe_unicast_ipv4
/// 9.2 Multicast Subscription
    /// - Receivers supporting IPv4 must support IGMP v2 or any subsequent superset of IGMPv2's functionality:
    /// - Receivers supporting IPv6 shall support MLD V1 or any subsequent subset of MLD1's functionality:
/// 9.3 Allocation of Multicast Addresses
/// 9.3.1 Allocation of IPv4 Multicast Addresses
    /// - Multicast IPv4 addresses must be defined as in Table 9-10
/// 9.3.2 Allocation of IPv6 Multicast Addresses
    /// - Multicast IPv6 addresses must be defined as in Table 9-11 and Table 9-12
/// 9.4 IPv4 and IPv6 Support Requirements
    /// - E1.31 sources need to be able to operate on both IPv4 and IPv6 potentially simultaneously: 
    /// - The state of IPv4 / IPv6 operation should be configurable by the end user:
/// 10 Translation between DMX512-A and E1.31 Data Transmission
/// 10.1 DMX512-A to E1.31 Translation
/// 10.1.1 Boot Condition
    /// - A DMX512-A [DMX] to E1.31 translator shall not transmit E1.31 data packets for a given universe until it has received at least one valid DMX512-A input packet for that universe:
/// 10.1.2 Temporal Sequence
    /// - A DMX512-A [DMX] to E1.31 translator shall transmit packets in the order in which they were received from the DMX512-A source:
/// 10.1.3 Loss of Data
    /// - On loss of data as defined in DMX512-A a source shall terminate transmission as per Section 6.7.1:
/// 10.2 E1.31 to DMX512-A Translation
/// 10.2.1 General
/// 10.2.2 Loss of Data
    /// - There must be an operating mode where upon detection of loss of data as defined in 6.7.1 for all sources of a universe a source shall
    ///     immediately stop transmitting DMX512-A packets:
/// 11 Universe Synchronization
    /// - There is no restriction on the number of synchronisation addresses allowed on a single network:
    /// - It is possible to have multiple independent universes configured for E1.31 synchronisation concurrently:
/// 11.1 Synchronized and Unsynchronized Data
/// 11.1.1 When to Begin Synchronizing Data
    /// - A receiever should begin universe synchronisation upon receipt of the first syncronisation packet for that universe:
/// 11.1.2 When to Stop Synchronizing Data
    /// - A receiever should stop universe synchronisation if it does not receieve an E1.31 synchronisation packet on that universe within E131_NETWORK_DATA_LOSS_TIMEOUT:
    /// - The behaviour on timeout may be determined by the Force Synchronisation Option bit:
/// 11.2 Synchronization Timings in a Streaming Environment
/// 11.2.1 Arrival of Multiple Packets Before Processing
    /// - An E1.31 receiever should only synchronise using the definitive E1.31 data for that universe:
    /// - If there is a single source the definitive data is the data packet with the most recent valid sequence number:
    /// - If there are multiple active syncrhonisation sources on the same synchronisation address it is beyond the scope of the standard:
/// 11.2.2 Delays Before Universe Synchronization
    /// - Recommended to add a configurable delay between data packets and transmission of an E1.31 synchronisation packet: 
/// 12 Universe Discovery
    /// - Legacy devices may not implement it even though to be compliant they should:
/// 12.1 Universe Discovery and Termination
    /// - A source that is not sending any universe data may stop sending E1.31 Universe Discovery Packets until transmission resumse:
    /// - Alternatively a source could send an empty list of universes:
/// 12.2 Termination of Stream Transmission
    /// - A E1.31 data stream is terminated when either a Stream_Terminated packet is receieved:
    /// - or if no packet is receieved for an interval of E131_NETWORK_DATA_LOSS_TIMEOUT:
    /// - A source that has terminated transmission for an E1.31 univers must refelct the change no later than the end of the second E131_UNIVERSE_DISCOVERY_INTERVAL

/// Appendix A: Defined Parameters (Normative)
    /// - All parameters used must match those specified in Appendix A:
/// Appendix B: An Example of Universe Synchronization For Implementors (Informative)
/// B.1 Universe Synchronization for Sources
    /// - The completed implementation must produce exactly the example response given for the given conditions / inputs:
/// B.2 Universe Synchronization for Receivers
    /// - The completed implementation must produce exactly the example response given for the given conditions / inputs:
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
fn test_send_recv_single_universe_unicast_ipv4(){
    let (tx, rx): (Sender<Result<Vec<DMXData>, Error>>, Receiver<Result<Vec<DMXData>, Error>>) = mpsc::channel();

    let thread_tx = tx.clone();

    let universe = 1;

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = match SacnReceiver::new(SocketAddr::new(Ipv4Addr::new(127,0,0,1).into(), ACN_SDT_MULTICAST_PORT)){
            Ok(sr) => sr,
            Err(_) => panic!("Failed to create sacn receiver!")
        };

        dmx_recv.set_nonblocking(false).unwrap();

        dmx_recv.listen_universes(&[universe]).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap();

        thread_tx.send(dmx_recv.recv()).unwrap();
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT + 1);
    let mut dmx_source = DmxSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    dmx_source.register_universe(universe);

    let dst_ip: SocketAddr = SocketAddr::new(Ipv4Addr::new(127,0,0,1).into(), ACN_SDT_MULTICAST_PORT);

    let _ = dmx_source.send(&[universe], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), Some(dst_ip), None).unwrap();

    let received_result: Result<Vec<DMXData>, Error> = rx.recv().unwrap();

    rcv_thread.join().unwrap();

    assert!(!received_result.is_err(), "Failed: Error when receving data");

    let received_data: Vec<DMXData> = received_result.unwrap();

    assert_eq!(received_data.len(), 1); // Check only 1 universe received as expected.

    let received_universe: DMXData = received_data[0].clone();

    assert_eq!(received_universe.universe, universe); // Check that the universe received is as expected.

    assert_eq!(received_universe.values, TEST_DATA_SINGLE_UNIVERSE.to_vec(), "Received payload values don't match sent!");
}

#[test]
fn test_send_recv_single_universe_multicast_ipv4(){
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

    let ip: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT + 1);
    let mut dmx_source = DmxSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    dmx_source.register_universe(universe);

    let _ = dmx_source.send(&[universe], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), None, None).unwrap();

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
fn test_send_recv_across_universe_multicast_ipv4(){
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

    let ip: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT + 1);
    let mut dmx_source = DmxSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    dmx_source.register_universes(&UNIVERSES);

    dmx_source.send(&UNIVERSES, &TEST_DATA_MULTIPLE_UNIVERSE, Some(priority), None, None).unwrap();

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