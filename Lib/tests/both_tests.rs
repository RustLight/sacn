#![allow(dead_code)]
#![allow(unused_imports)]

extern crate lazy_static;
extern crate sacn;

use std::{thread};
use std::thread::sleep;
use std::option;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver, RecvTimeoutError};
use std::io::Error;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};
use sacn::{DmxSource};
use sacn::recieve::{SacnReceiver, DMXData};
use sacn::packet::{UNIVERSE_CHANNEL_CAPACITY, ACN_SDT_MULTICAST_PORT};

use std::time::Duration;

// Report: Should start code be seperated out when receiving? Causes input and output to differ and is technically part of another protocol.
// - Decided it shouldn't be seperated.

/// For some tests to work multiple instances of the protocol must be on the same network with the same port for example to test multiple simultaneous receivers, this means multiple IP's are needed.
/// This is achieved by assigning multiple static IP's to the test machine and theses IP's are specified below.
/// Theses must be changed depending on the network that the test machine is on.
const TEST_NETWORK_INTERFACE_IP_1: &'static str = "192.168.1.10";
const TEST_NETWORK_INTERFACE_IP_2: &'static str = "192.168.1.9";

/// 
#[test]
fn test_send_recv_partial_capacity_universe_multicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>, Error>>, Receiver<Result<Vec<DMXData>, Error>>) = mpsc::channel();

    let thread_tx = tx.clone();

    let universe = 1;

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = match SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT)){
            Ok(sr) => sr,
            Err(_) => panic!("Failed to create sacn receiver!")
        };

        dmx_recv.set_nonblocking(false).unwrap();

        dmx_recv.listen_universes(&[universe]).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap();

        thread_tx.send(dmx_recv.recv()).unwrap();
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    // Note: Localhost / loopback doesn't always support IPv6 multicast. Therefore this may have to be modified to select a specific network using the line below
    // where PUT_IPV6_ADDR_HERE is replaced with the ipv6 address of the interface to use. https://stackoverflow.com/questions/55308730/java-multicasting-how-to-test-on-localhost (04/01/2020)
    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT + 1);

    let mut dmx_source = DmxSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    dmx_source.register_universe(universe);

    let _ = dmx_source.send(&[universe], &TEST_DATA_PARTIAL_CAPACITY_UNIVERSE, Some(priority), None, None).unwrap();

    let received_result: Result<Vec<DMXData>, Error> = rx.recv().unwrap();

    rcv_thread.join().unwrap();

    assert!(!received_result.is_err(), "Failed: Error when receving data");

    let received_data: Vec<DMXData> = received_result.unwrap();

    assert_eq!(received_data.len(), 1); // Check only 1 universe received as expected.

    let received_universe: DMXData = received_data[0].clone();

    assert_eq!(received_universe.universe, universe); // Check that the universe received is as expected.

    assert_eq!(received_universe.values, TEST_DATA_PARTIAL_CAPACITY_UNIVERSE.to_vec(), "Received payload values don't match sent!");
}

/// 
#[test]
fn test_send_recv_single_alternative_startcode_universe_multicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>, Error>>, Receiver<Result<Vec<DMXData>, Error>>) = mpsc::channel();

    let thread_tx = tx.clone();

    let universe = 1;

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = match SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT)){
            Ok(sr) => sr,
            Err(_) => panic!("Failed to create sacn receiver!")
        };

        dmx_recv.set_nonblocking(false).unwrap();

        dmx_recv.listen_universes(&[universe]).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap();

        thread_tx.send(dmx_recv.recv()).unwrap();
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    // Note: Localhost / loopback doesn't always support IPv6 multicast. Therefore this may have to be modified to select a specific network using the line below
    // where PUT_IPV6_ADDR_HERE is replaced with the ipv6 address of the interface to use. https://stackoverflow.com/questions/55308730/java-multicasting-how-to-test-on-localhost (04/01/2020)
    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT + 1);

    let mut dmx_source = DmxSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    dmx_source.register_universe(universe);

    let _ = dmx_source.send(&[universe], &TEST_DATA_SINGLE_ALTERNATIVE_STARTCODE_UNIVERSE, Some(priority), None, None).unwrap();

    let received_result: Result<Vec<DMXData>, Error> = rx.recv().unwrap();

    rcv_thread.join().unwrap();

    assert!(!received_result.is_err(), "Failed: Error when receving data");

    let received_data: Vec<DMXData> = received_result.unwrap();

    assert_eq!(received_data.len(), 1); // Check only 1 universe received as expected.

    let received_universe: DMXData = received_data[0].clone();

    assert_eq!(received_universe.universe, universe); // Check that the universe received is as expected.

    assert_eq!(received_universe.values, TEST_DATA_SINGLE_ALTERNATIVE_STARTCODE_UNIVERSE.to_vec(), "Received payload values don't match sent!");
}

#[test]
fn test_across_alternative_startcode_universe_multicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>, Error>>, Receiver<Result<Vec<DMXData>, Error>>) = mpsc::channel();

    let thread_tx = tx.clone();

    const UNIVERSES: [u16; 2] = [2, 3];

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = match SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT)) {
            Ok(sr) => sr,
            Err(_) => panic!("Failed to create sacn receiver!")
        };

        dmx_recv.set_nonblocking(false).unwrap();

        dmx_recv.listen_universes(&UNIVERSES).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap(); // Signal that the receiver is ready to receive.

        thread_tx.send(dmx_recv.recv()).unwrap(); // Receive the sync packet, the data packets shouldn't have caused .recv to return as forced to wait for sync.
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT + 1);
    let mut dmx_source = DmxSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    dmx_source.register_universes(&UNIVERSES);

    dmx_source.send(&UNIVERSES, &TEST_DATA_MULTIPLE_ALTERNATIVE_STARTCODE_UNIVERSE, Some(priority), None, Some(UNIVERSES[0])).unwrap();
    sleep(Duration::from_millis(500)); // Small delay to allow the data packets to get through as per NSI-E1.31-2018 Appendix B.1 recommendation.
    dmx_source.send_sync_packet(UNIVERSES[0], &None);

    let sync_pkt_res: Result<Vec<DMXData>, Error> = rx.recv().unwrap();

    rcv_thread.join().unwrap();

    assert!(!sync_pkt_res.is_err(), "Failed: Error when receving packets");

    let mut received_data: Vec<DMXData> = sync_pkt_res.unwrap();

    received_data.sort(); // No guarantee on the ordering of the receieved data so sort it first to allow easier checking.

    assert_eq!(received_data.len(), 2); // Check 2 universes received as expected.

    assert_eq!(received_data[0].universe, 2); // Check that the universe received is as expected.

    assert_eq!(received_data[0].sync_uni, 2); // Check that the sync universe is as expected.

    assert_eq!(received_data[0].values, TEST_DATA_MULTIPLE_ALTERNATIVE_STARTCODE_UNIVERSE[..UNIVERSE_CHANNEL_CAPACITY].to_vec(), "Universe 1 received payload values don't match sent!");

    assert_eq!(received_data[1].universe, 3); // Check that the universe received is as expected.

    assert_eq!(received_data[1].sync_uni, 2); // Check that the sync universe is as expected.

    assert_eq!(received_data[1].values, TEST_DATA_MULTIPLE_ALTERNATIVE_STARTCODE_UNIVERSE[UNIVERSE_CHANNEL_CAPACITY..].to_vec(), "Universe 2 received payload values don't match sent!");
}

/// Note: this test assumes perfect network conditions (0% reordering, loss, duplication etc.), this should be the case for
/// the loopback adapter with the low amount of data sent but this may be a possible cause if integration tests fail unexpectedly.
#[test]
fn test_send_recv_full_capacity_across_universe_multicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>, Error>>, Receiver<Result<Vec<DMXData>, Error>>) = mpsc::channel();

    let thread_tx = tx.clone();

    const UNIVERSES: [u16; 2] = [2, 3];

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = match SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT)) {
            Ok(sr) => sr,
            Err(_) => panic!("Failed to create sacn receiver!")
        };

        dmx_recv.set_nonblocking(false).unwrap();

        dmx_recv.listen_universes(&UNIVERSES).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap(); // Signal that the receiver is ready to receive.

        thread_tx.send(dmx_recv.recv()).unwrap(); // Receive the sync packet, the data packets shouldn't have caused .recv to return as forced to wait for sync.
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT + 1);
    let mut dmx_source = DmxSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    dmx_source.register_universes(&UNIVERSES);

    dmx_source.send(&UNIVERSES, &TEST_DATA_FULL_CAPACITY_MULTIPLE_UNIVERSE, Some(priority), None, Some(UNIVERSES[0])).unwrap();
    sleep(Duration::from_millis(500)); // Small delay to allow the data packets to get through as per NSI-E1.31-2018 Appendix B.1 recommendation.
    dmx_source.send_sync_packet(UNIVERSES[0], &None);

    let sync_pkt_res: Result<Vec<DMXData>, Error> = rx.recv().unwrap();

    rcv_thread.join().unwrap();

    assert!(!sync_pkt_res.is_err(), "Failed: Error when receving packets");

    let mut received_data: Vec<DMXData> = sync_pkt_res.unwrap();

    received_data.sort(); // No guarantee on the ordering of the receieved data so sort it first to allow easier checking.

    assert_eq!(received_data.len(), 2); // Check 2 universes received as expected.

    assert_eq!(received_data[0].universe, 2); // Check that the universe received is as expected.

    assert_eq!(received_data[0].sync_uni, 2); // Check that the sync universe is as expected.

    assert_eq!(received_data[0].values, TEST_DATA_FULL_CAPACITY_MULTIPLE_UNIVERSE[..UNIVERSE_CHANNEL_CAPACITY].to_vec(), "Universe 1 received payload values don't match sent!");

    assert_eq!(received_data[1].universe, 3); // Check that the universe received is as expected.

    assert_eq!(received_data[1].sync_uni, 2); // Check that the sync universe is as expected.

    assert_eq!(received_data[1].values, TEST_DATA_FULL_CAPACITY_MULTIPLE_UNIVERSE[UNIVERSE_CHANNEL_CAPACITY..].to_vec(), "Universe 2 received payload values don't match sent!");
}

// Note: For this test to work the PC must be capable of connecting to the network on 2 IP's, this was done in windows by adding another static IP so the PC was connecting through
// 2 different IP's to the network. Theses IPs are manually specified as the constants TEST_NETWORK_INTERFACE_IP_1 and TEST_NETWORK_INTERFACE_IP_2 in this test and so to run it must be changed
// depending on the environment.
#[test]
fn test_send_single_universe_multiple_receivers_multicast_ipv4(){
    let (tx, rx): (Sender<Result<Vec<DMXData>, Error>>, Receiver<Result<Vec<DMXData>, Error>>) = mpsc::channel();

    let thread1_tx = tx.clone();
    let thread2_tx = tx.clone();

    let universe = 1;

    let rcv_thread1 = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IP_1.parse().unwrap()), ACN_SDT_MULTICAST_PORT)).unwrap();

        dmx_recv.set_nonblocking(false).unwrap();

        dmx_recv.listen_universes(&[universe]).unwrap();

        thread1_tx.send(Ok(Vec::new())).unwrap();

        thread1_tx.send(dmx_recv.recv()).unwrap();
    });

    let rcv_thread2 = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IP_2.parse().unwrap()), ACN_SDT_MULTICAST_PORT)).unwrap();

        dmx_recv.set_nonblocking(false).unwrap();

        dmx_recv.listen_universes(&[universe]).unwrap();

        thread2_tx.send(Ok(Vec::new())).unwrap();

        thread2_tx.send(dmx_recv.recv()).unwrap();
    });

    let _ = rx.recv().unwrap(); // Blocks until both receivers say they are ready.
    let _ = rx.recv().unwrap();

    let ip: SocketAddr = SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IP_1.parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);

    let mut dmx_source = DmxSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    dmx_source.register_universe(universe);

    let _ = dmx_source.send(&[universe], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), None, None).unwrap();

    let received_result1: Result<Vec<DMXData>, Error> = rx.recv().unwrap();
    let received_result2: Result<Vec<DMXData>, Error> = rx.recv().unwrap();

    rcv_thread1.join().unwrap();
    rcv_thread2.join().unwrap();

    assert!(!received_result1.is_err(), "Failed: Error when receving data");
    let received_data1: Vec<DMXData> = received_result1.unwrap();
    assert_eq!(received_data1.len(), 1); // Check only 1 universe received as expected.
    let received_universe1: DMXData = received_data1[0].clone();
    assert_eq!(received_universe1.universe, universe); // Check that the universe received is as expected.
    assert_eq!(received_universe1.values, TEST_DATA_SINGLE_UNIVERSE.to_vec(), "Received payload values don't match sent!");

    assert!(!received_result2.is_err(), "Failed: Error when receving data");
    let received_data2: Vec<DMXData> = received_result2.unwrap();
    assert_eq!(received_data2.len(), 1); // Check only 1 universe received as expected.
    let received_universe2: DMXData = received_data2[0].clone();
    assert_eq!(received_universe2.universe, universe); // Check that the universe received is as expected.
    assert_eq!(received_universe2.values, TEST_DATA_SINGLE_UNIVERSE.to_vec(), "Received payload values don't match sent!");
}

// Note: For this test to work the PC must be capable of connecting to the network on 2 IP's, this was done in windows by adding another static IP so the PC was connecting through
// 2 different IP's to the network. Theses IPs are manually specified as the constants TEST_NETWORK_INTERFACE_IP_1 and TEST_NETWORK_INTERFACE_IP_2 in this test and so to run it must be changed
// depending on the environment.
#[test]
fn test_send_across_universe_multiple_receivers_sync_multicast_ipv4(){
    let (tx, rx): (Sender<Result<Vec<DMXData>, Error>>, Receiver<Result<Vec<DMXData>, Error>>) = mpsc::channel();

    let thread1_tx = tx.clone();
    let thread2_tx = tx.clone();

    let universe1 = 1;
    let universe2 = 2;

    let sync_uni = 2;

    let rcv_thread1 = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IP_1.parse().unwrap()), ACN_SDT_MULTICAST_PORT)).unwrap();

        dmx_recv.set_nonblocking(false).unwrap();

        dmx_recv.listen_universes(&[universe1]).unwrap();

        thread1_tx.send(Ok(Vec::new())).unwrap();

        thread1_tx.send(dmx_recv.recv()).unwrap();
    });

    let rcv_thread2 = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IP_2.parse().unwrap()), ACN_SDT_MULTICAST_PORT)).unwrap();

        dmx_recv.set_nonblocking(false).unwrap();

        dmx_recv.listen_universes(&[universe2]).unwrap();

        thread2_tx.send(Ok(Vec::new())).unwrap();

        thread2_tx.send(dmx_recv.recv()).unwrap();
    });

    let _ = rx.recv().unwrap(); // Blocks until both receivers say they are ready.
    let _ = rx.recv().unwrap();

    let ip: SocketAddr = SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IP_1.parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);

    let mut dmx_source = DmxSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    dmx_source.register_universe(universe1);
    dmx_source.register_universe(universe2);

    let _ = dmx_source.send(&[universe1], &TEST_DATA_MULTIPLE_UNIVERSE[..513], Some(priority), None, Some(sync_uni)).unwrap();
    let _ = dmx_source.send(&[universe2], &TEST_DATA_MULTIPLE_UNIVERSE[513..], Some(priority), None, Some(sync_uni)).unwrap();

    // Waiting to receive, if anything is received it indicates one of the receivers progressed without waiting for synchronisation.
    // This has the issue that is is possible that even though they could have progressed the receive threads may not have leading them to pass this part 
    // when they shouldn't. This is difficult to avoid using this method of testing. It is also possible for the delay on the network to be so high that it 
    // causes the timeout, this is also difficult to avoid. Both of these reasons should be considered if this test passes occasionally but not consistently. 
    // The timeout should be large enough to make this unlikely although must be lower than the protocol's in-built timeout.
    const wait_receive_timeout: u64 = 1;
    let attemptRecv = rx.recv_timeout(Duration::from_secs(wait_receive_timeout));

    match attemptRecv {
        Ok(o) => {
            println!("{:#?}", o);
            assert!(false, "Receivers received without waiting for sync");
        },
        Err(e) => assert_eq!(e, RecvTimeoutError::Timeout)
    }

    dmx_source.send_sync_packet(sync_uni, &None);

    let received_result1: Vec<DMXData> = rx.recv().unwrap().unwrap();
    let received_result2: Vec<DMXData> = rx.recv().unwrap().unwrap();

    rcv_thread1.join().unwrap();
    rcv_thread2.join().unwrap();

    assert_eq!(received_result1.len(), 1); // Check only 1 universe received as expected.
    assert_eq!(received_result2.len(), 1); // Check only 1 universe received as expected.

    let mut results = vec![received_result1[0].clone(), received_result2[0].clone()];
    results.sort_unstable(); // Ordering of received data is undefined, to make it easier to check sort first.

    assert_eq!(results[0].universe, universe1); // Check that the universe 1 received is as expected.
    assert_eq!(results[1].universe, universe2); // Check that the universe 2 received is as expected.

    assert_eq!(results[0].values, TEST_DATA_MULTIPLE_UNIVERSE[..513].to_vec());
    assert_eq!(results[1].values, TEST_DATA_MULTIPLE_UNIVERSE[513..].to_vec());
}

#[test]
fn test_send_recv_single_universe_unicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>, Error>>, Receiver<Result<Vec<DMXData>, Error>>) = mpsc::channel();

    let thread_tx = tx.clone();

    let universe = 1;

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = match SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT)){
            Ok(sr) => sr,
            Err(_) => panic!("Failed to create sacn receiver!")
        };

        dmx_recv.set_nonblocking(false).unwrap();

        dmx_recv.listen_universes(&[universe]).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap();

        thread_tx.send(dmx_recv.recv()).unwrap();
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT + 1);
    let mut dmx_source = DmxSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    dmx_source.register_universe(universe);

    let dst_ip: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT);

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
fn test_send_recv_single_universe_unicast_ipv4(){
    let (tx, rx): (Sender<Result<Vec<DMXData>, Error>>, Receiver<Result<Vec<DMXData>, Error>>) = mpsc::channel();

    let thread_tx = tx.clone();

    let universe = 1;

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = match SacnReceiver::with_ip(SocketAddr::new(Ipv4Addr::LOCALHOST.into(), ACN_SDT_MULTICAST_PORT)){
            Ok(sr) => sr,
            Err(_) => panic!("Failed to create sacn receiver!")
        };

        dmx_recv.set_nonblocking(false).unwrap();

        dmx_recv.listen_universes(&[universe]).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap();

        thread_tx.send(dmx_recv.recv()).unwrap();
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), ACN_SDT_MULTICAST_PORT + 1);
    let mut dmx_source = DmxSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    dmx_source.register_universe(universe);

    let dst_ip: SocketAddr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), ACN_SDT_MULTICAST_PORT);

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
fn test_send_recv_single_universe_multicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>, Error>>, Receiver<Result<Vec<DMXData>, Error>>) = mpsc::channel();

    let thread_tx = tx.clone();

    let universe = 1;

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = match SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT)){
            Ok(sr) => sr,
            Err(_) => panic!("Failed to create sacn receiver!")
        };

        dmx_recv.set_nonblocking(false).unwrap();

        dmx_recv.listen_universes(&[universe]).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap();

        thread_tx.send(dmx_recv.recv()).unwrap();
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    // Note: Localhost / loopback doesn't always support IPv6 multicast. Therefore this may have to be modified to select a specific network using the line below
    // where PUT_IPV6_ADDR_HERE is replaced with the ipv6 address of the interface to use. https://stackoverflow.com/questions/55308730/java-multicasting-how-to-test-on-localhost (04/01/2020)
    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT + 1);

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

#[test]
fn test_send_recv_single_universe_multicast_ipv4(){
    let (tx, rx): (Sender<Result<Vec<DMXData>, Error>>, Receiver<Result<Vec<DMXData>, Error>>) = mpsc::channel();

    let thread_tx = tx.clone();

    let universe = 1;

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = match SacnReceiver::with_ip(SocketAddr::new(Ipv4Addr::new(0,0,0,0).into(), ACN_SDT_MULTICAST_PORT)){
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
fn test_send_recv_across_universe_multicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>, Error>>, Receiver<Result<Vec<DMXData>, Error>>) = mpsc::channel();

    let thread_tx = tx.clone();

    const UNIVERSES: [u16; 2] = [2, 3];

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = match SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT)) {
            Ok(sr) => sr,
            Err(_) => panic!("Failed to create sacn receiver!")
        };

        dmx_recv.set_nonblocking(false).unwrap();

        dmx_recv.listen_universes(&UNIVERSES).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap(); // Signal that the receiver is ready to receive.

        thread_tx.send(dmx_recv.recv()).unwrap(); // Receive the sync packet, the data packets shouldn't have caused .recv to return as forced to wait for sync.
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT + 1);
    let mut dmx_source = DmxSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    dmx_source.register_universes(&UNIVERSES);

    dmx_source.send(&UNIVERSES, &TEST_DATA_MULTIPLE_UNIVERSE, Some(priority), None, Some(UNIVERSES[0])).unwrap();
    sleep(Duration::from_millis(500)); // Small delay to allow the data packets to get through as per NSI-E1.31-2018 Appendix B.1 recommendation. See other warnings about the possibility of theses tests failing if the network isn't perfect.
    dmx_source.send_sync_packet(UNIVERSES[0], &None);

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

/// Note: this test assumes perfect network conditions (0% reordering, loss, duplication etc.), this should be the case for
/// the loopback adapter with the low amount of data sent but this may be a possible cause if integration tests fail unexpectedly.
#[test]
fn test_send_recv_across_universe_multicast_ipv4(){
    let (tx, rx): (Sender<Result<Vec<DMXData>, Error>>, Receiver<Result<Vec<DMXData>, Error>>) = mpsc::channel();

    let thread_tx = tx.clone();

    const UNIVERSES: [u16; 2] = [2, 3];

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = match SacnReceiver::with_ip(SocketAddr::new(Ipv4Addr::new(0,0,0,0).into(), ACN_SDT_MULTICAST_PORT)){
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

    dmx_source.send(&UNIVERSES, &TEST_DATA_MULTIPLE_UNIVERSE, Some(priority), None, Some(UNIVERSES[0])).unwrap();
    sleep(Duration::from_millis(500)); // Small delay to allow the data packets to get through as per NSI-E1.31-2018 Appendix B.1 recommendation. See other warnings about the possibility of theses tests failing if the network isn't perfect.
    dmx_source.send_sync_packet(UNIVERSES[0], &None);

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

/// Note: this test assumes perfect network conditions (0% reordering, loss, duplication etc.), this should be the case for
/// the loopback adapter with the low amount of data sent but this may be a possible cause if integration tests fail unexpectedly.
#[test]
fn test_send_recv_across_universe_unicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>, Error>>, Receiver<Result<Vec<DMXData>, Error>>) = mpsc::channel();

    let thread_tx = tx.clone();

    const UNIVERSES: [u16; 2] = [2, 3];

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = match SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT)) {
            Ok(sr) => sr,
            Err(_) => panic!("Failed to create sacn receiver!")
        };

        dmx_recv.set_nonblocking(false).unwrap();

        dmx_recv.listen_universes(&UNIVERSES).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap(); // Signal that the receiver is ready to receive.

        thread_tx.send(dmx_recv.recv()).unwrap(); // Receive the sync packet, the data packets shouldn't have caused .recv to return as forced to wait for sync.
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT + 1);
    let mut dmx_source = DmxSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    dmx_source.register_universes(&UNIVERSES);

    let dst_ip: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT);

    let _ = dmx_source.send(&UNIVERSES, &TEST_DATA_MULTIPLE_UNIVERSE, Some(priority), Some(dst_ip), Some(UNIVERSES[0])).unwrap();
    sleep(Duration::from_millis(500)); // Small delay to allow the data packets to get through as per NSI-E1.31-2018 Appendix B.1 recommendation.
    dmx_source.send_sync_packet(UNIVERSES[0], &Some(dst_ip));

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

/// Note: this test assumes perfect network conditions (0% reordering, loss, duplication etc.), this should be the case for
/// the loopback adapter with the low amount of data sent but this may be a possible cause if integration tests fail unexpectedly.
#[test]
fn test_send_recv_across_universe_unicast_ipv4(){
    let (tx, rx): (Sender<Result<Vec<DMXData>, Error>>, Receiver<Result<Vec<DMXData>, Error>>) = mpsc::channel();

    let thread_tx = tx.clone();

    const UNIVERSES: [u16; 2] = [2, 3];

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = match SacnReceiver::with_ip(SocketAddr::new(Ipv4Addr::new(127,0,0,1).into(), ACN_SDT_MULTICAST_PORT)){
            Ok(sr) => sr,
            Err(_) => panic!("Failed to create sacn receiver!")
        };

        dmx_recv.set_nonblocking(false).unwrap();

        dmx_recv.listen_universes(&UNIVERSES).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap(); // Signal that the receiver is ready to receive.

        thread_tx.send(dmx_recv.recv()).unwrap(); // Receive the sync packet, the data packets shouldn't have caused .recv to return as forced to wait for sync.
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), ACN_SDT_MULTICAST_PORT + 1);
    let mut dmx_source = DmxSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    dmx_source.register_universes(&UNIVERSES);

    let dst_ip: SocketAddr = SocketAddr::new(Ipv4Addr::new(127,0,0,1).into(), ACN_SDT_MULTICAST_PORT);

    let _ = dmx_source.send(&UNIVERSES, &TEST_DATA_MULTIPLE_UNIVERSE, Some(priority), Some(dst_ip), Some(UNIVERSES[0])).unwrap();
    sleep(Duration::from_millis(500)); // Small delay to allow the data packets to get through as per NSI-E1.31-2018 Appendix B.1 recommendation.
    dmx_source.send_sync_packet(UNIVERSES[0], &Some(dst_ip));

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


const TEST_DATA_PARTIAL_CAPACITY_UNIVERSE: [u8; 313] = [0,
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

const TEST_DATA_SINGLE_ALTERNATIVE_STARTCODE_UNIVERSE: [u8; 513] = [1,
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

const TEST_DATA_SINGLE_UNIVERSE: [u8; 513] = [0,
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

const TEST_DATA_MULTIPLE_ALTERNATIVE_STARTCODE_UNIVERSE: [u8; 714] = [1,
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

        3,
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

const TEST_DATA_MULTIPLE_UNIVERSE: [u8; 714] = [0,
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

        0,
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
const TEST_DATA_FULL_CAPACITY_MULTIPLE_UNIVERSE: [u8; 1026] = [0,
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
        0,
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