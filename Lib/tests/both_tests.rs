#[allow(dead_code)]

// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was created as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

extern crate sacn;

use std::{thread};
use std::thread::sleep;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, SyncSender, Receiver, RecvTimeoutError};

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use sacn::source::SacnSource;
use sacn::recieve::{SacnReceiver, DMXData, htp_dmx_merge};
use sacn::packet::{UNIVERSE_CHANNEL_CAPACITY, ACN_SDT_MULTICAST_PORT};

extern crate socket2;
use socket2::{SockAddr};

use std::time::Duration;

use sacn::error::errors::*;

// Report: Should start code be seperated out when receiving? Causes input and output to differ and is technically part of another protocol.
// - Decided it shouldn't be seperated.

/// For some tests to work multiple instances of the protocol must be on the same network with the same port for example to test multiple simultaneous receivers, this means multiple IP's are needed.
/// This is achieved by assigning multiple static IP's to the test machine and theses IP's are specified below.
/// Theses must be changed depending on the network that the test machine is on.
const TEST_NETWORK_INTERFACE_IPV4: [&'static str; 3] = ["192.168.1.10", "192.168.1.9", "192.168.1.8"];
// const TEST_NETWORK_INTERFACE_IPV4: [&'static str; 3] = ["138.251.29.246", "138.251.29.246", "138.251.29.246"];
const TEST_NETWORK_INTERFACE_IPV6: [&'static str; 1] = ["fe80::2077:cb6:7b9b:a144"];

/// 
fn test_send_recv_partial_capacity_universe_multicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>>>, Receiver<Result<Vec<DMXData>>>) = mpsc::channel();

    let thread_tx = tx.clone();

    let universe = 1;

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&[universe]).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap();

        thread_tx.send(dmx_recv.recv(None)).unwrap();
    });

    rx.recv().unwrap().unwrap(); // Blocks until the receiver says it is ready. 

    // Note: Localhost / loopback doesn't always support IPv6 multicast. Therefore this may have to be modified to select a specific network using the line below
    // where PUT_IPV6_ADDR_HERE is replaced with the ipv6 address of the interface to use. https://stackoverflow.com/questions/55308730/java-multicasting-how-to-test-on-localhost (04/01/2020)
    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);

    let mut src = SacnSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    src.register_universe(universe).unwrap();

    src.send(&[universe], &TEST_DATA_PARTIAL_CAPACITY_UNIVERSE, Some(priority), None, None).unwrap();

    let received_result: Result<Vec<DMXData>> = rx.recv().unwrap();

    rcv_thread.join().unwrap();

    assert!(!received_result.is_err(), "Failed: Error when receving data");

    let received_data: Vec<DMXData> = received_result.unwrap();

    assert_eq!(received_data.len(), 1); // Check only 1 universe received as expected.

    let received_universe: DMXData = received_data[0].clone();

    assert_eq!(received_universe.universe, universe); // Check that the universe received is as expected.

    assert_eq!(received_universe.values, TEST_DATA_PARTIAL_CAPACITY_UNIVERSE.to_vec(), "Received payload values don't match sent!");
}

/// 
fn test_send_recv_single_alternative_startcode_universe_multicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>>>, Receiver<Result<Vec<DMXData>>>) = mpsc::channel();

    let thread_tx = tx.clone();

    let universe = 1;

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&[universe]).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap();

        thread_tx.send(dmx_recv.recv(None)).unwrap();
    });

    rx.recv().unwrap().unwrap(); // Blocks until the receiver says it is ready. 

    // Note: Localhost / loopback doesn't always support IPv6 multicast. Therefore this may have to be modified to select a specific network using the line below
    // where PUT_IPV6_ADDR_HERE is replaced with the ipv6 address of the interface to use. https://stackoverflow.com/questions/55308730/java-multicasting-how-to-test-on-localhost (04/01/2020)
    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);

    let mut src = SacnSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    src.register_universe(universe).unwrap();

    src.send(&[universe], &TEST_DATA_SINGLE_ALTERNATIVE_STARTCODE_UNIVERSE, Some(priority), None, None).unwrap();

    let received_result: Result<Vec<DMXData>> = rx.recv().unwrap();

    rcv_thread.join().unwrap();

    assert!(!received_result.is_err(), "Failed: Error when receving data");

    let received_data: Vec<DMXData> = received_result.unwrap();

    assert_eq!(received_data.len(), 1); // Check only 1 universe received as expected.

    let received_universe: DMXData = received_data[0].clone();

    assert_eq!(received_universe.universe, universe); // Check that the universe received is as expected.

    assert_eq!(received_universe.values, TEST_DATA_SINGLE_ALTERNATIVE_STARTCODE_UNIVERSE.to_vec(), "Received payload values don't match sent!");
}

fn test_across_alternative_startcode_universe_multicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>>>, Receiver<Result<Vec<DMXData>>>) = mpsc::channel();

    let thread_tx = tx.clone();

    const UNIVERSES: [u16; 2] = [2, 3];

    let rcv_thread = thread::spawn(move || {
        let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT);
        let mut dmx_recv = SacnReceiver::with_ip(addr, None).unwrap();

        dmx_recv.listen_universes(&UNIVERSES).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap(); // Signal that the receiver is ready to receive.

        thread_tx.send(dmx_recv.recv(None)).unwrap(); // Receive the sync packet, the data packets shouldn't have caused .recv to return as forced to wait for sync.
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);
    let mut src = SacnSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    src.register_universes(&UNIVERSES).unwrap();

    src.send(&UNIVERSES, &TEST_DATA_MULTIPLE_ALTERNATIVE_STARTCODE_UNIVERSE, Some(priority), None, Some(UNIVERSES[0])).unwrap();
    sleep(Duration::from_millis(500)); // Small delay to allow the data packets to get through as per NSI-E1.31-2018 Appendix B.1 recommendation.
    src.send_sync_packet(UNIVERSES[0], None).unwrap();

    let sync_pkt_res: Result<Vec<DMXData>> = rx.recv().unwrap();

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
fn test_send_recv_full_capacity_across_universe_multicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>>>, Receiver<Result<Vec<DMXData>>>) = mpsc::channel();

    let thread_tx = tx.clone();

    const UNIVERSES: [u16; 2] = [2, 3];

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&UNIVERSES).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap(); // Signal that the receiver is ready to receive.

        thread_tx.send(dmx_recv.recv(None)).unwrap(); // Receive the sync packet, the data packets shouldn't have caused .recv to return as forced to wait for sync.
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);
    let mut src = SacnSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    src.register_universes(&UNIVERSES).unwrap();

    src.send(&UNIVERSES, &TEST_DATA_FULL_CAPACITY_MULTIPLE_UNIVERSE, Some(priority), None, Some(UNIVERSES[0])).unwrap();
    sleep(Duration::from_millis(500)); // Small delay to allow the data packets to get through as per NSI-E1.31-2018 Appendix B.1 recommendation.
    src.send_sync_packet(UNIVERSES[0], None).unwrap();

    let sync_pkt_res: Result<Vec<DMXData>> = rx.recv().unwrap();

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
// 2 different IP's to the network. Theses IPs are manually specified in the TEST_NETWORK_INTERFACE_IPV4 constant and so to run it must be changed
// depending on the environment.
#[test]
fn test_send_single_universe_multiple_receivers_multicast_ipv4(){
    let (tx, rx): (Sender<Result<Vec<DMXData>>>, Receiver<Result<Vec<DMXData>>>) = mpsc::channel();

    let thread1_tx = tx.clone();
    let thread2_tx = tx.clone();

    let universe = 1;

    let rcv_thread1 = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&[universe]).unwrap();

        thread1_tx.send(Ok(Vec::new())).unwrap();

        thread1_tx.send(dmx_recv.recv(None)).unwrap();
    });

    let rcv_thread2 = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[1].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&[universe]).unwrap();

        thread2_tx.send(Ok(Vec::new())).unwrap();

        thread2_tx.send(dmx_recv.recv(None)).unwrap();
    });

    rx.recv().unwrap().unwrap(); // Blocks until both receivers say they are ready.
    rx.recv().unwrap().unwrap();

    let ip: SocketAddr = SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);

    let mut src = SacnSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    src.register_universe(universe).unwrap();

    src.send(&[universe], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), None, None).unwrap();

    let received_result1: Result<Vec<DMXData>> = rx.recv().unwrap();
    let received_result2: Result<Vec<DMXData>> = rx.recv().unwrap();

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

#[test]
fn test_send_across_universe_multiple_receivers_sync_multicast_ipv4(){
    let (tx, rx): (Sender<Result<Vec<DMXData>>>, Receiver<Result<Vec<DMXData>>>) = mpsc::channel();

    let thread1_tx = tx.clone();
    let thread2_tx = tx.clone();

    let universe1 = 1;
    let universe2 = 2;

    let sync_uni = 3;

    let rcv_thread1 = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&[universe1]).unwrap();
        dmx_recv.listen_universes(&[sync_uni]).unwrap();

        thread1_tx.send(Ok(Vec::new())).unwrap();

        thread1_tx.send(dmx_recv.recv(None)).unwrap();
    });

    let rcv_thread2 = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[1].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&[universe2]).unwrap();
        dmx_recv.listen_universes(&[sync_uni]).unwrap();

        thread2_tx.send(Ok(Vec::new())).unwrap();

        thread2_tx.send(dmx_recv.recv(None)).unwrap();
    });

    rx.recv().unwrap().unwrap(); // Blocks until both receivers say they are ready.
    rx.recv().unwrap().unwrap();

    let ip: SocketAddr = SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);

    let mut src = SacnSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    src.register_universe(universe1).unwrap();
    src.register_universe(universe2).unwrap();
    src.register_universe(sync_uni).unwrap();

    src.send(&[universe1], &TEST_DATA_MULTIPLE_UNIVERSE[..513], Some(priority), None, Some(sync_uni)).unwrap();
    src.send(&[universe2], &TEST_DATA_MULTIPLE_UNIVERSE[513..], Some(priority), None, Some(sync_uni)).unwrap();

    // Waiting to receive, if anything is received it indicates one of the receivers progressed without waiting for synchronisation.
    // This has the issue that is is possible that even though they could have progressed the receive threads may not have leading them to pass this part 
    // when they shouldn't. This is difficult to avoid using this method of testing. It is also possible for the delay on the network to be so high that it 
    // causes the timeout, this is also difficult to avoid. Both of these reasons should be considered if this test passes occasionally but not consistently. 
    // The timeout should be large enough to make this unlikely although must be lower than the protocol's in-built timeout.
    const WAIT_RECV_TIMEOUT: u64 = 2;
    let attempt_recv = rx.recv_timeout(Duration::from_secs(WAIT_RECV_TIMEOUT));

    match attempt_recv {
        Ok(o) => {
            println!("{:#?}", o);
            assert!(false, "Receivers received without waiting for sync");
        },
        Err(e) => assert_eq!(e, RecvTimeoutError::Timeout)
    }

    src.send_sync_packet(sync_uni, None).unwrap();

    println!("Waiting to receive");

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

fn test_send_recv_single_universe_unicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>>>, Receiver<Result<Vec<DMXData>>>) = mpsc::channel();

    let thread_tx = tx.clone();

    let universe = 1;

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&[universe]).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap();

        thread_tx.send(dmx_recv.recv(None)).unwrap();
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT + 1);
    let mut src = SacnSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    src.register_universe(universe).unwrap();

    let dst_ip: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT);

    let _ = src.send(&[universe], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), Some(dst_ip), None).unwrap();

    let received_result: Result<Vec<DMXData>> = rx.recv().unwrap();

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
    let (tx, rx): (Sender<Result<Vec<DMXData>>>, Receiver<Result<Vec<DMXData>>>) = mpsc::channel();

    let thread_tx = tx.clone();

    let universe = 1;

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(Ipv4Addr::LOCALHOST.into(), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&[universe]).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap();

        thread_tx.send(dmx_recv.recv(None)).unwrap();
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), ACN_SDT_MULTICAST_PORT + 1);
    let mut src = SacnSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    src.register_universe(universe).unwrap();

    let dst_ip: SocketAddr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), ACN_SDT_MULTICAST_PORT);

    let _ = src.send(&[universe], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), Some(dst_ip), None).unwrap();

    let received_result: Result<Vec<DMXData>> = rx.recv().unwrap();

    rcv_thread.join().unwrap();

    assert!(!received_result.is_err(), "Failed: Error when receving data");

    let received_data: Vec<DMXData> = received_result.unwrap();

    assert_eq!(received_data.len(), 1); // Check only 1 universe received as expected.

    let received_universe: DMXData = received_data[0].clone();

    assert_eq!(received_universe.universe, universe); // Check that the universe received is as expected.

    assert_eq!(received_universe.values, TEST_DATA_SINGLE_UNIVERSE.to_vec(), "Received payload values don't match sent!");
}

fn test_send_recv_single_universe_multicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>>>, Receiver<Result<Vec<DMXData>>>) = mpsc::channel();

    let thread_tx = tx.clone();

    let universe = 1;

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&[universe]).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap();

        thread_tx.send(dmx_recv.recv(None)).unwrap();
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    // Note: Localhost / loopback doesn't always support IPv6 multicast. Therefore this may have to be modified to select a specific network using the line below
    // where PUT_IPV6_ADDR_HERE is replaced with the ipv6 address of the interface to use. https://stackoverflow.com/questions/55308730/java-multicasting-how-to-test-on-localhost (04/01/2020)
    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);

    let mut src = SacnSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    src.register_universe(universe).unwrap();

    let _ = src.send(&[universe], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), None, None).unwrap();

    let received_result: Result<Vec<DMXData>> = rx.recv().unwrap();

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
    let (tx, rx): (Sender<Result<Vec<DMXData>>>, Receiver<Result<Vec<DMXData>>>) = mpsc::channel();

    let thread_tx = tx.clone();

    let universe = 1;

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&[universe]).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap();

        thread_tx.send(dmx_recv.recv(None)).unwrap();
    });

    rx.recv().unwrap().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);
    let mut src = SacnSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    src.register_universe(universe).unwrap();

    src.send(&[universe], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), None, None).unwrap();

    let received_result: Result<Vec<DMXData>> = rx.recv().unwrap();

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
fn test_send_recv_across_universe_multicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>>>, Receiver<Result<Vec<DMXData>>>) = mpsc::channel();

    let thread_tx = tx.clone();

    const UNIVERSES: [u16; 2] = [2, 3];

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&UNIVERSES).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap(); // Signal that the receiver is ready to receive.

        thread_tx.send(dmx_recv.recv(None)).unwrap(); // Receive the sync packet, the data packets shouldn't have caused .recv to return as forced to wait for sync.
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);
    let mut src = SacnSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    src.register_universes(&UNIVERSES).unwrap();

    src.send(&UNIVERSES, &TEST_DATA_MULTIPLE_UNIVERSE, Some(priority), None, Some(UNIVERSES[0])).unwrap();
    sleep(Duration::from_millis(500)); // Small delay to allow the data packets to get through as per NSI-E1.31-2018 Appendix B.1 recommendation. See other warnings about the possibility of theses tests failing if the network isn't perfect.
    src.send_sync_packet(UNIVERSES[0], None).unwrap();

    let sync_pkt_res: Result<Vec<DMXData>> = rx.recv().unwrap();

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
    let (tx, rx): (Sender<Result<Vec<DMXData>>>, Receiver<Result<Vec<DMXData>>>) = mpsc::channel();

    let thread_tx = tx.clone();

    const UNIVERSES: [u16; 2] = [2, 3];

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(Ipv4Addr::new(0,0,0,0).into(), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&UNIVERSES).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap(); // Signal that the receiver is ready to receive.

        thread_tx.send(dmx_recv.recv(None)).unwrap(); // Receive the sync packet, the data packets shouldn't have caused .recv to return as forced to wait for sync.
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT + 1);
    let mut src = SacnSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    src.register_universes(&UNIVERSES).unwrap();

    src.send(&UNIVERSES, &TEST_DATA_MULTIPLE_UNIVERSE, Some(priority), None, Some(UNIVERSES[0])).unwrap();
    sleep(Duration::from_millis(500)); // Small delay to allow the data packets to get through as per NSI-E1.31-2018 Appendix B.1 recommendation. See other warnings about the possibility of theses tests failing if the network isn't perfect.
    src.send_sync_packet(UNIVERSES[0], None).unwrap();

    let sync_pkt_res: Result<Vec<DMXData>> = rx.recv().unwrap();

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
fn test_send_recv_across_universe_unicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>>>, Receiver<Result<Vec<DMXData>>>) = mpsc::channel();

    let thread_tx = tx.clone();

    const UNIVERSES: [u16; 2] = [2, 3];

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&UNIVERSES).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap(); // Signal that the receiver is ready to receive.

        thread_tx.send(dmx_recv.recv(None)).unwrap(); // Receive the sync packet, the data packets shouldn't have caused .recv to return as forced to wait for sync.
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT + 1);
    let mut src = SacnSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    src.register_universes(&UNIVERSES).unwrap();

    let _ = src.send(&UNIVERSES, &TEST_DATA_MULTIPLE_UNIVERSE, Some(priority), Some(SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT).into()), Some(UNIVERSES[0])).unwrap();
    sleep(Duration::from_millis(500)); // Small delay to allow the data packets to get through as per NSI-E1.31-2018 Appendix B.1 recommendation.
    src.send_sync_packet(UNIVERSES[0], Some(SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT).into())).unwrap();

    let sync_pkt_res: Result<Vec<DMXData>> = rx.recv().unwrap();

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
    let (tx, rx): (Sender<Result<Vec<DMXData>>>, Receiver<Result<Vec<DMXData>>>) = mpsc::channel();

    let thread_tx = tx.clone();

    const UNIVERSES: [u16; 2] = [2, 3];

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(Ipv4Addr::new(127,0,0,1).into(), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&UNIVERSES).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap(); // Signal that the receiver is ready to receive.

        thread_tx.send(dmx_recv.recv(None)).unwrap(); // Receive the sync packet, the data packets shouldn't have caused .recv to return as forced to wait for sync.
    });

    let _ = rx.recv().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), ACN_SDT_MULTICAST_PORT + 1);
    let mut src = SacnSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    src.register_universes(&UNIVERSES).unwrap();

    let _ = src.send(&UNIVERSES, &TEST_DATA_MULTIPLE_UNIVERSE, Some(priority), Some(SocketAddr::new(Ipv4Addr::new(127,0,0,1).into(), ACN_SDT_MULTICAST_PORT).into()), Some(UNIVERSES[0])).unwrap();
    sleep(Duration::from_millis(500)); // Small delay to allow the data packets to get through as per NSI-E1.31-2018 Appendix B.1 recommendation.
    src.send_sync_packet(UNIVERSES[0], Some(SocketAddr::new(Ipv4Addr::new(127,0,0,1).into(), ACN_SDT_MULTICAST_PORT).into())).unwrap();

    let sync_pkt_res: Result<Vec<DMXData>> = rx.recv().unwrap();

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

#[test]
fn test_two_senders_one_recv_different_universes_multicast_ipv4(){
    let universe_1 = 1;
    let universe_2 = 2;

    let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(Ipv4Addr::new(0,0,0,0).into(), ACN_SDT_MULTICAST_PORT), None).unwrap();

    dmx_recv.listen_universes(&[universe_1]).unwrap();
    dmx_recv.listen_universes(&[universe_2]).unwrap();

    let snd_thread_1 = thread::spawn(move || {
        let ip: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT + 1);
        let mut src = SacnSource::with_ip("Source", ip).unwrap();

        let priority = 100;

        src.register_universe(universe_1).unwrap();

        let _ = src.send(&[universe_1], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), None, None).unwrap();
    });

    let snd_thread_2 = thread::spawn(move || {
        let ip: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT + 2);
        let mut src = SacnSource::with_ip("Source", ip).unwrap();

        let priority = 100;

        src.register_universe(universe_2).unwrap();

        let _ = src.send(&[universe_2], &TEST_DATA_PARTIAL_CAPACITY_UNIVERSE, Some(priority), None, None).unwrap();
    });

    let res1: Vec<DMXData> = dmx_recv.recv(None).unwrap();
    let res2: Vec<DMXData> = dmx_recv.recv(None).unwrap();

    snd_thread_1.join().unwrap();
    snd_thread_2.join().unwrap();


    assert_eq!(res1.len(), 1);
    assert_eq!(res2.len(), 1);

    let mut res = vec![res1[0].clone(), res2[0].clone()];
    res.sort_unstable();

    assert_eq!(res[0].universe, universe_1);
    assert_eq!(res[1].universe, universe_2);

    assert_eq!(res[0].values, TEST_DATA_SINGLE_UNIVERSE.to_vec());
    assert_eq!(res[1].values, TEST_DATA_PARTIAL_CAPACITY_UNIVERSE.to_vec());
}

#[test]
fn test_two_senders_one_recv_same_universe_no_sync_multicast_ipv4(){
    let universe = 1;

    let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(Ipv4Addr::new(0,0,0,0).into(), ACN_SDT_MULTICAST_PORT), None).unwrap();

    dmx_recv.listen_universes(&[universe]).unwrap();

    let snd_thread_1 = thread::spawn(move || {
        let ip: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT + 1);
        let mut src = SacnSource::with_ip("Source", ip).unwrap();

        let priority = 100;

        src.register_universe(universe).unwrap();

        let _ = src.send(&[universe], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), None, None).unwrap();
    });

    let snd_thread_2 = thread::spawn(move || {
        let ip: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), ACN_SDT_MULTICAST_PORT + 2);
        let mut src = SacnSource::with_ip("Source", ip).unwrap();

        let priority = 100;

        src.register_universe(universe).unwrap();

        let _ = src.send(&[universe], &TEST_DATA_PARTIAL_CAPACITY_UNIVERSE, Some(priority), None, None).unwrap();
    });

    let res1: Vec<DMXData> = dmx_recv.recv(None).unwrap();
    let res2: Vec<DMXData> = dmx_recv.recv(None).unwrap();

    snd_thread_1.join().unwrap();
    snd_thread_2.join().unwrap();

    assert_eq!(res1.len(), 1);
    assert_eq!(res2.len(), 1);

    let res = vec![res1[0].clone(), res2[0].clone()];

    assert_eq!(res[0].universe, universe);
    assert_eq!(res[1].universe, universe);

    if res[0].values == TEST_DATA_SINGLE_UNIVERSE.to_vec() {
        assert_eq!(res[1].values, TEST_DATA_PARTIAL_CAPACITY_UNIVERSE.to_vec());
    } else {
        assert_eq!(res[0].values, TEST_DATA_PARTIAL_CAPACITY_UNIVERSE.to_vec());
        assert_eq!(res[1].values, TEST_DATA_SINGLE_UNIVERSE.to_vec());
    }
}

#[test]
fn test_two_senders_one_recv_same_universe_custom_merge_fn_sync_multicast_ipv4(){
    let (tx, rx): (SyncSender<()>, Receiver<()>) = mpsc::sync_channel(0); // Used for handshaking

    let snd_tx = tx.clone();

    let universe = 1;
    let sync_uni = 2;

    let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap(), ACN_SDT_MULTICAST_PORT), None).unwrap();

    dmx_recv.listen_universes(&[universe, sync_uni]).unwrap();

    dmx_recv.set_merge_fn(htp_dmx_merge).unwrap();

    let snd_thread_1 = thread::spawn(move || {
        let ip: SocketAddr = SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);
        let mut src = SacnSource::with_ip("Source", ip).unwrap();

        let priority = 100;

        src.register_universe(universe).unwrap();
        src.register_universe(sync_uni).unwrap();

        src.send(&[universe], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), None, Some(sync_uni)).unwrap();
        snd_tx.send(()).unwrap();
    });

    let snd_thread_2 = thread::spawn(move || {
        let ip: SocketAddr = SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 2);
        let mut src = SacnSource::with_ip("Source 2", ip).unwrap();

        let priority = 100;

        src.register_universe(universe).unwrap();
        src.register_universe(sync_uni).unwrap();

        src.send(&[universe], &TEST_DATA_PARTIAL_CAPACITY_UNIVERSE, Some(priority), None, Some(sync_uni)).unwrap();
        rx.recv().unwrap(); // Must only send once both threads have sent for this test to test what happens in that situation (where there will be a merge).
        src.send_sync_packet(sync_uni, None).unwrap();
    });

    let res1: Vec<DMXData> = dmx_recv.recv(None).unwrap();

    snd_thread_1.join().unwrap();
    snd_thread_2.join().unwrap();

    assert_eq!(res1.len(), 1);
    assert_eq!(res1[0].values, htp_dmx_merge(&DMXData {
        universe: universe,
        values: TEST_DATA_SINGLE_UNIVERSE.to_vec(),
        sync_uni: sync_uni,
        priority: 100,
        src_cid: None
    },
    &DMXData {
        universe: universe,
        values: TEST_DATA_PARTIAL_CAPACITY_UNIVERSE.to_vec(),
        sync_uni: sync_uni,
        priority: 100,
        src_cid: None
    },).unwrap().values);
}

#[test]
fn test_two_senders_two_recv_multicast_ipv4(){
    const SND_THREADS: usize = 2;
    const RCV_THREADS: usize = 2;
    const SND_DATA_LEN: usize = 100;

    let mut snd_data: Vec<Vec<u8>> = Vec::new();

    for i in 1 .. SND_THREADS + 1 {
        let mut d: Vec<u8> = Vec::new();
        for _k in 0 .. SND_DATA_LEN {
            d.push(i as u8);
        }
        snd_data.push(d);
    }

    let mut snd_threads = Vec::new();
    let mut rcv_threads = Vec::new();

    let (rcv_tx, rcv_rx): (SyncSender<Vec<Result<Vec<DMXData>>>>, Receiver<Vec<Result<Vec<DMXData>>>>) = mpsc::sync_channel(0);
    let (snd_tx, snd_rx): (SyncSender<()>, Receiver<()>) = mpsc::sync_channel(0); // Used for handshaking, allows syncing the sender states.

    assert!(RCV_THREADS <= TEST_NETWORK_INTERFACE_IPV4.len(), "Number of test network interface ips less than number of recv threads!");

    const BASE_UNIVERSE: u16 = 2;

    for i in 0 .. SND_THREADS {
        let tx = snd_tx.clone();

        let data = snd_data[i].clone();

        snd_threads.push(thread::spawn(move || {
            let ip: SocketAddr = SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1 + (i as u16));
            // https://www.programming-idioms.org/idiom/153/concatenate-string-with-integer/1975/rust (11/01/2020)
            let mut src = SacnSource::with_ip(&format!("Source {}", i), ip).unwrap();
    
            let priority = 100;

            let universe: u16 = (i as u16) + BASE_UNIVERSE; 
    
            src.register_universe(universe).unwrap(); // Senders all send on different universes.

            tx.send(()).unwrap(); // Forces each sender thread to wait till the controlling thread recveives which stops sending before the receivers are ready.
    
            src.send(&[universe], &data, Some(priority), None, None).unwrap();
        }));
    }

    for i in 0 .. RCV_THREADS {
        let tx = rcv_tx.clone();

        rcv_threads.push(thread::spawn(move || {
            // Port kept the same so must use multiple IP's.
            let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[i].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

            // Receivers listen to all universes
            for i in (BASE_UNIVERSE as u16) .. ((SND_THREADS as u16) + (BASE_UNIVERSE as u16)) {
                dmx_recv.listen_universes(&[i]).unwrap();
            }

            let mut res: Vec<Result<Vec<DMXData>>> = Vec::new();

            tx.send(Vec::new()).unwrap(); // Receiver notifies controlling thread it is ready.

            for _i in 0 .. SND_THREADS { // Receiver should receive from every universe.
                res.push(dmx_recv.recv(None)); // Receiver won't complete this until it receives from the senders which are all held waiting on the controlling thread.
            }

            // Results of each receive are sent back, this allows checking that each reciever was an expected universe, all universes were received and there were no errors.
            tx.send(res).unwrap(); 
        }));

        assert_eq!(rcv_rx.recv().unwrap().len(), 0); // Wait till the receiver has notified controlling thread it is ready.
    }

    for _i in 0 .. SND_THREADS {
        snd_rx.recv().unwrap(); // Allow each sender to progress
    }

    for _i in 0 .. RCV_THREADS {
        let res: Vec<Result<Vec<DMXData>>> = rcv_rx.recv().unwrap();

        assert_eq!(res.len(), SND_THREADS);

        let mut rcv_dmx_datas: Vec<DMXData> = Vec::new();

        for r in res {
            let data: Vec<DMXData> = r.unwrap(); // Check that there are no errors when receiving.
            assert_eq!(data.len(), 1); // Check that each universe was received seperately.
            rcv_dmx_datas.push(data[0].clone());
        }

        rcv_dmx_datas.sort_unstable(); // Sorting by universe allows easier checking as order recieved may vary depending on network.

        for k in 0 .. SND_THREADS {
            assert_eq!(rcv_dmx_datas[k].universe, ((k as u16) + BASE_UNIVERSE)); // Check that the universe received is as expected.

            assert_eq!(rcv_dmx_datas[k].values, snd_data[k], "Received payload values don't match sent!");
        }
    }

    for s in snd_threads {
        s.join().unwrap();
    }

    for r in rcv_threads {
        r.join().unwrap();
    }
}

#[test]
fn test_three_senders_two_recv_multicast_ipv4(){
    const SND_THREADS: usize = 3;
    const RCV_THREADS: usize = 2;
    const SND_DATA_LEN: usize = 100;

    let mut snd_data: Vec<Vec<u8>> = Vec::new();

    for i in 1 .. SND_THREADS + 1 {
        let mut d: Vec<u8> = Vec::new();
        for _k in 0 .. SND_DATA_LEN {
            d.push(i as u8);
        }
        snd_data.push(d);
    }

    let mut snd_threads = Vec::new();
    let mut rcv_threads = Vec::new();

    let (rcv_tx, rcv_rx): (SyncSender<Vec<Result<Vec<DMXData>>>>, Receiver<Vec<Result<Vec<DMXData>>>>) = mpsc::sync_channel(0);
    let (snd_tx, snd_rx): (SyncSender<()>, Receiver<()>) = mpsc::sync_channel(0); // Used for handshaking, allows syncing the sender states.

    assert!(RCV_THREADS <= TEST_NETWORK_INTERFACE_IPV4.len(), "Number of test network interface ips less than number of recv threads!");

    const BASE_UNIVERSE: u16 = 2;

    for i in 0 .. SND_THREADS {
        let tx = snd_tx.clone();

        let data = snd_data[i].clone();

        snd_threads.push(thread::spawn(move || {
            let ip: SocketAddr = SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1 + (i as u16));
            // https://www.programming-idioms.org/idiom/153/concatenate-string-with-integer/1975/rust (11/01/2020)
            let mut src = SacnSource::with_ip(&format!("Source {}", i), ip).unwrap();
    
            let priority = 100;

            let universe: u16 = (i as u16) + BASE_UNIVERSE; 
    
            src.register_universe(universe).unwrap(); // Senders all send on different universes.

            tx.send(()).unwrap(); // Forces each sender thread to wait till the controlling thread recveives which stops sending before the receivers are ready.
    
            src.send(&[universe], &data, Some(priority), None, None).unwrap();
        }));
    }

    for i in 0 .. RCV_THREADS {
        let tx = rcv_tx.clone();

        rcv_threads.push(thread::spawn(move || {
            // Port kept the same so must use multiple IP's.
            let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[i].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

            // Receivers listen to all universes
            for i in (BASE_UNIVERSE as u16) .. ((SND_THREADS as u16) + (BASE_UNIVERSE as u16)) {
                dmx_recv.listen_universes(&[i]).unwrap();
            }

            let mut res: Vec<Result<Vec<DMXData>>> = Vec::new();

            tx.send(Vec::new()).unwrap(); // Receiver notifies controlling thread it is ready.

            for _i in 0 .. SND_THREADS { // Receiver should receive from every universe.
                res.push(dmx_recv.recv(None)); // Receiver won't complete this until it receives from the senders which are all held waiting on the controlling thread.
            }

            // Results of each receive are sent back, this allows checking that each reciever was an expected universe, all universes were received and there were no errors.
            tx.send(res).unwrap(); 
        }));

        assert_eq!(rcv_rx.recv().unwrap().len(), 0); // Wait till the receiver has notified controlling thread it is ready.
    }

    for _i in 0 .. SND_THREADS {
        snd_rx.recv().unwrap(); // Allow each sender to progress
    }

    for _i in 0 .. RCV_THREADS {
        let res: Vec<Result<Vec<DMXData>>> = rcv_rx.recv().unwrap();

        assert_eq!(res.len(), SND_THREADS);

        let mut rcv_dmx_datas: Vec<DMXData> = Vec::new();

        for r in res {
            let data: Vec<DMXData> = r.unwrap(); // Check that there are no errors when receiving.
            assert_eq!(data.len(), 1); // Check that each universe was received seperately.
            rcv_dmx_datas.push(data[0].clone());
        }

        rcv_dmx_datas.sort_unstable(); // Sorting by universe allows easier checking as order recieved may vary depending on network.

        for k in 0 .. SND_THREADS {
            assert_eq!(rcv_dmx_datas[k].universe, ((k as u16) + BASE_UNIVERSE)); // Check that the universe received is as expected.

            assert_eq!(rcv_dmx_datas[k].values, snd_data[k], "Received payload values don't match sent!");
        }
    }

    for s in snd_threads {
        s.join().unwrap();
    }

    for r in rcv_threads {
        r.join().unwrap();
    }
}

#[test]
fn test_two_senders_three_recv_multicast_ipv4(){
    const SND_THREADS: usize = 2;
    const RCV_THREADS: usize = 3;
    const SND_DATA_LEN: usize = 100;

    let mut snd_data: Vec<Vec<u8>> = Vec::new();

    for i in 1 .. SND_THREADS + 1 {
        let mut d: Vec<u8> = Vec::new();
        for _k in 0 .. SND_DATA_LEN {
            d.push(i as u8);
        }
        snd_data.push(d);
    }

    let mut snd_threads = Vec::new();
    let mut rcv_threads = Vec::new();

    let (rcv_tx, rcv_rx): (SyncSender<Vec<Result<Vec<DMXData>>>>, Receiver<Vec<Result<Vec<DMXData>>>>) = mpsc::sync_channel(0);
    let (snd_tx, snd_rx): (SyncSender<()>, Receiver<()>) = mpsc::sync_channel(0); // Used for handshaking, allows syncing the sender states.

    assert!(RCV_THREADS <= TEST_NETWORK_INTERFACE_IPV4.len(), "Number of test network interface ips less than number of recv threads!");

    const BASE_UNIVERSE: u16 = 2;

    for i in 0 .. SND_THREADS {
        let tx = snd_tx.clone();

        let data = snd_data[i].clone();

        snd_threads.push(thread::spawn(move || {
            let ip: SocketAddr = SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1 + (i as u16));
            // https://www.programming-idioms.org/idiom/153/concatenate-string-with-integer/1975/rust (11/01/2020)
            let mut src = SacnSource::with_ip(&format!("Source {}", i), ip).unwrap();
    
            let priority = 100;

            let universe: u16 = (i as u16) + BASE_UNIVERSE; 
    
            src.register_universe(universe).unwrap(); // Senders all send on different universes.

            tx.send(()).unwrap(); // Forces each sender thread to wait till the controlling thread recveives which stops sending before the receivers are ready.
    
            src.send(&[universe], &data, Some(priority), None, None).unwrap();
        }));
    }

    for i in 0 .. RCV_THREADS {
        let tx = rcv_tx.clone();

        rcv_threads.push(thread::spawn(move || {
            // Port kept the same so must use multiple IP's.
            let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[i].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

            // Receivers listen to all universes
            for i in (BASE_UNIVERSE as u16) .. ((SND_THREADS as u16) + (BASE_UNIVERSE as u16)) {
                dmx_recv.listen_universes(&[i]).unwrap();
            }

            let mut res: Vec<Result<Vec<DMXData>>> = Vec::new();

            tx.send(Vec::new()).unwrap(); // Receiver notifies controlling thread it is ready.

            for _i in 0 .. SND_THREADS { // Receiver should receive from every universe.
                res.push(dmx_recv.recv(None)); // Receiver won't complete this until it receives from the senders which are all held waiting on the controlling thread.
            }

            // Results of each receive are sent back, this allows checking that each reciever was an expected universe, all universes were received and there were no errors.
            tx.send(res).unwrap(); 
        }));

        assert_eq!(rcv_rx.recv().unwrap().len(), 0); // Wait till the receiver has notified controlling thread it is ready.
    }

    for _i in 0 .. SND_THREADS {
        snd_rx.recv().unwrap(); // Allow each sender to progress
    }

    for _i in 0 .. RCV_THREADS {
        let res: Vec<Result<Vec<DMXData>>> = rcv_rx.recv().unwrap();

        assert_eq!(res.len(), SND_THREADS);

        let mut rcv_dmx_datas: Vec<DMXData> = Vec::new();

        for r in res {
            let data: Vec<DMXData> = r.unwrap(); // Check that there are no errors when receiving.
            assert_eq!(data.len(), 1); // Check that each universe was received seperately.
            rcv_dmx_datas.push(data[0].clone());
        }

        rcv_dmx_datas.sort_unstable(); // Sorting by universe allows easier checking as order recieved may vary depending on network.

        for k in 0 .. SND_THREADS {
            assert_eq!(rcv_dmx_datas[k].universe, ((k as u16) + BASE_UNIVERSE)); // Check that the universe received is as expected.

            assert_eq!(rcv_dmx_datas[k].values, snd_data[k], "Received payload values don't match sent!");
        }
    }

    for s in snd_threads {
        s.join().unwrap();
    }

    for r in rcv_threads {
        r.join().unwrap();
    }
}

#[test]
fn test_three_senders_three_recv_multicast_ipv4(){
    const SND_THREADS: usize = 3;
    const RCV_THREADS: usize = 3;
    const SND_DATA_LEN: usize = 100;

    let mut snd_data: Vec<Vec<u8>> = Vec::new();

    for i in 1 .. SND_THREADS + 1 {
        let mut d: Vec<u8> = Vec::new();
        for _k in 0 .. SND_DATA_LEN {
            d.push(i as u8);
        }
        snd_data.push(d);
    }

    let mut snd_threads = Vec::new();
    let mut rcv_threads = Vec::new();

    let (rcv_tx, rcv_rx): (SyncSender<Vec<Result<Vec<DMXData>>>>, Receiver<Vec<Result<Vec<DMXData>>>>) = mpsc::sync_channel(0);
    let (snd_tx, snd_rx): (SyncSender<()>, Receiver<()>) = mpsc::sync_channel(0); // Used for handshaking, allows syncing the sender states.

    assert!(RCV_THREADS <= TEST_NETWORK_INTERFACE_IPV4.len(), "Number of test network interface ips less than number of recv threads!");

    const BASE_UNIVERSE: u16 = 2;

    for i in 0 .. SND_THREADS {
        let tx = snd_tx.clone();

        let data = snd_data[i].clone();

        snd_threads.push(thread::spawn(move || {
            let ip: SocketAddr = SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1 + (i as u16));
            // https://www.programming-idioms.org/idiom/153/concatenate-string-with-integer/1975/rust (11/01/2020)
            let mut src = SacnSource::with_ip(&format!("Source {}", i), ip).unwrap();
    
            let priority = 100;

            let universe: u16 = (i as u16) + BASE_UNIVERSE; 
    
            src.register_universe(universe).unwrap(); // Senders all send on different universes.

            tx.send(()).unwrap(); // Forces each sender thread to wait till the controlling thread recveives which stops sending before the receivers are ready.
    
            src.send(&[universe], &data, Some(priority), None, None).unwrap();
        }));
    }

    for i in 0 .. RCV_THREADS {
        let tx = rcv_tx.clone();

        rcv_threads.push(thread::spawn(move || {
            // Port kept the same so must use multiple IP's.
            let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[i].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

            // Receivers listen to all universes
            for i in (BASE_UNIVERSE as u16) .. ((SND_THREADS as u16) + (BASE_UNIVERSE as u16)) {
                dmx_recv.listen_universes(&[i]).unwrap();
            }

            let mut res: Vec<Result<Vec<DMXData>>> = Vec::new();

            tx.send(Vec::new()).unwrap(); // Receiver notifies controlling thread it is ready.

            for _i in 0 .. SND_THREADS { // Receiver should receive from every universe.
                res.push(dmx_recv.recv(None)); // Receiver won't complete this until it receives from the senders which are all held waiting on the controlling thread.
            }

            // Results of each receive are sent back, this allows checking that each reciever was an expected universe, all universes were received and there were no errors.
            tx.send(res).unwrap(); 
        }));

        assert_eq!(rcv_rx.recv().unwrap().len(), 0); // Wait till the receiver has notified controlling thread it is ready.
    }

    for _i in 0 .. SND_THREADS {
        snd_rx.recv().unwrap(); // Allow each sender to progress
    }

    for _i in 0 .. RCV_THREADS {
        let res: Vec<Result<Vec<DMXData>>> = rcv_rx.recv().unwrap();

        assert_eq!(res.len(), SND_THREADS);

        let mut rcv_dmx_datas: Vec<DMXData> = Vec::new();

        for r in res {
            let data: Vec<DMXData> = r.unwrap(); // Check that there are no errors when receiving.
            assert_eq!(data.len(), 1); // Check that each universe was received seperately.
            rcv_dmx_datas.push(data[0].clone());
        }

        rcv_dmx_datas.sort_unstable(); // Sorting by universe allows easier checking as order recieved may vary depending on network.

        println!("{:?}", rcv_dmx_datas);

        for k in 0 .. SND_THREADS {
            assert_eq!(rcv_dmx_datas[k].universe, ((k as u16) + BASE_UNIVERSE)); // Check that the universe received is as expected.

            assert_eq!(rcv_dmx_datas[k].values, snd_data[k], "Received payload values don't match sent!");
        }
    }

    for s in snd_threads {
        s.join().unwrap();
    }

    for r in rcv_threads {
        r.join().unwrap();
    }
}

#[test]
fn test_universe_discovery_one_universe_one_source_ipv4(){
    const SND_THREADS: usize = 1;
    const BASE_UNIVERSE: u16 = 2;
    const UNIVERSE_COUNT: usize = 1;
    const SOURCE_NAMES: [&'static str; 1] = ["Source 1"];

    let (snd_tx, snd_rx): (SyncSender<()>, Receiver<()>) = mpsc::sync_channel(0);

    let mut snd_threads = Vec::new();

    for i in 0 .. SND_THREADS {
        let tx = snd_tx.clone();

        snd_threads.push(thread::spawn(move || {
            let ip: SocketAddr = SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1 + (i as u16));

            let mut src = SacnSource::with_ip(SOURCE_NAMES[i], ip).unwrap();

            let mut universes: Vec<u16> = Vec::new();
            for j in 0 .. UNIVERSE_COUNT {
                universes.push(((i + j) as u16) + BASE_UNIVERSE);
            }

            src.register_universes(&universes).unwrap();

            tx.send(()).unwrap(); // Used to force the sender to wait till the receiver has received a universe discovery.
        }));
    }

    let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

    loop { 
        let result = dmx_recv.recv(Some(Duration::from_secs(2)));
        match result { 
            Err(e) => {
                match e.kind() {
                    &ErrorKind::Io(ref s) => {
                        match s.kind() {
                            std::io::ErrorKind::WouldBlock => {
                                // Expected to timeout / would block.
                                // The different errors are due to windows and unix returning different errors for the same thing.
                            },
                            std::io::ErrorKind::TimedOut => {},
                            _ => {
                                assert!(false, "Unexpected error returned");
                            }
                        }
                    },
                    _ => {
                        assert!(false, "Unexpected error returned");
                    }
                }
            },
            Ok(_) => {
                assert!(false, "No data should have been passed up!");
            }
        }
        
        let discovered = dmx_recv.get_discovered_sources(); 

        if discovered.len() > 0 {
            assert_eq!(discovered.len(), 1);
            assert_eq!(discovered[0].name, SOURCE_NAMES[0]);
            let universes = discovered[0].get_all_universes();
            assert_eq!(universes.len(), UNIVERSE_COUNT);
            for j in 0 .. UNIVERSE_COUNT {
                assert_eq!(universes[j], (j as u16) + BASE_UNIVERSE);
            }
            break;
        }
    }

    snd_rx.recv().unwrap();

    for s in snd_threads {
        s.join().unwrap();
    }
}

#[test]
fn test_universe_discovery_multiple_universe_one_source_ipv4(){
    const SND_THREADS: usize = 1;
    const BASE_UNIVERSE: u16 = 2;
    const UNIVERSE_COUNT: usize = 5;
    const SOURCE_NAMES: [&'static str; 1] = ["Source 1"];

    let (snd_tx, snd_rx): (SyncSender<()>, Receiver<()>) = mpsc::sync_channel(0);

    let mut snd_threads = Vec::new();

    for i in 0 .. SND_THREADS {
        let tx = snd_tx.clone();

        snd_threads.push(thread::spawn(move || {
            let ip: SocketAddr = SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1 + (i as u16));

            let mut src = SacnSource::with_ip(SOURCE_NAMES[i], ip).unwrap();

            let mut universes: Vec<u16> = Vec::new();
            for j in 0 .. UNIVERSE_COUNT {
                universes.push(((i + j) as u16) + BASE_UNIVERSE);
            }

            src.register_universes(&universes).unwrap();

            tx.send(()).unwrap(); // Used to force the sender to wait till the receiver has received a universe discovery.
        }));
    }

    let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

    loop { 
        let result = dmx_recv.recv(Some(Duration::from_secs(2)));
        match result { 
            Err(e) => {
                match e.kind() {
                    &ErrorKind::Io(ref s) => {
                        match s.kind() {
                            std::io::ErrorKind::WouldBlock => {
                                // Expected to timeout / would block.
                                // The different errors are due to windows and unix returning different errors for the same thing.
                            },
                            std::io::ErrorKind::TimedOut => {},
                            _ => {
                                assert!(false, "Unexpected error returned");
                            }
                        }
                    },
                    _ => {
                        assert!(false, "Unexpected error returned");
                    }
                }
            },
            Ok(_) => {
                assert!(false, "No data should have been passed up!");
            }
        }
        
        let discovered = dmx_recv.get_discovered_sources(); 

        if discovered.len() > 0 {
            assert_eq!(discovered.len(), 1);
            assert_eq!(discovered[0].name, SOURCE_NAMES[0]);

            let universes = discovered[0].get_all_universes();
            assert_eq!(universes.len(), UNIVERSE_COUNT);
            for j in 0 .. UNIVERSE_COUNT {
                assert_eq!(universes[j], (j as u16) + BASE_UNIVERSE);
            }
            break;
        }
    }

    snd_rx.recv().unwrap();

    for s in snd_threads {
        s.join().unwrap();
    }
}

#[test]
fn test_universe_discovery_multiple_pages_one_source_ipv4(){
    const SND_THREADS: usize = 1;
    const BASE_UNIVERSE: u16 = 2;
    const UNIVERSE_COUNT: usize = 600;
    const SOURCE_NAMES: [&'static str; 1] = ["Source 1"];

    let (snd_tx, snd_rx): (SyncSender<()>, Receiver<()>) = mpsc::sync_channel(0);

    let mut snd_threads = Vec::new();

    for i in 0 .. SND_THREADS {
        let tx = snd_tx.clone();

        snd_threads.push(thread::spawn(move || {
            let ip: SocketAddr = SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1 + (i as u16));

            let mut src = SacnSource::with_ip(SOURCE_NAMES[i], ip).unwrap();

            src.set_is_sending_discovery(false); // To stop universe discovery packets being sent until all universes are registered.

            let mut universes: Vec<u16> = Vec::new();
            for j in 0 .. UNIVERSE_COUNT {
                universes.push(((i + j) as u16) + BASE_UNIVERSE);
            }

            src.register_universes(&universes).unwrap();

            src.set_is_sending_discovery(true);

            tx.send(()).unwrap(); // Used to force the sender to wait till the receiver has received a universe discovery.
        }));
    }

    let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V4(TEST_NETWORK_INTERFACE_IPV4[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

    loop { 
        let result = dmx_recv.recv(Some(Duration::from_secs(2)));

        match result { 
            Err(e) => {
                match e.kind() {
                    &ErrorKind::Io(ref s) => {
                        match s.kind() {
                            std::io::ErrorKind::WouldBlock => {
                                // Expected to timeout / would block.
                                // The different errors are due to windows and unix returning different errors for the same thing.
                            },
                            std::io::ErrorKind::TimedOut => {},
                            _ => {
                                assert!(false, "Unexpected error returned");
                            }
                        }
                    },
                    _ => {
                        assert!(false, "Unexpected error returned");
                    }
                }
            },
            Ok(_) => {
                assert!(false, "No data should have been passed up!");
            }
        }
        
        let discovered = dmx_recv.get_discovered_sources(); 

        if discovered.len() > 0 {
            assert_eq!(discovered.len(), 1);
            assert_eq!(discovered[0].name, SOURCE_NAMES[0]);
            let universes = discovered[0].get_all_universes();
            assert_eq!(universes.len(), UNIVERSE_COUNT);
            for j in 0 .. UNIVERSE_COUNT {
                assert_eq!(universes[j], (j as u16) + BASE_UNIVERSE);
            }
            break;
        }
    }

    snd_rx.recv().unwrap();

    for s in snd_threads {
        s.join().unwrap();
    }
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