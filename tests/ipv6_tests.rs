// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was created as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

extern crate sacn;
extern crate socket2;
extern crate uuid;

pub mod ipv4_tests;

const TEST_NETWORK_INTERFACE_IPV6: [&'static str; 3] = ["2a02:c7f:d20a:c600:a502:2dae:7716:601b", "2a02:c7f:d20a:c600:a502:2dae:7716:601c", "2a02:c7f:d20a:c600:a502:2dae:7716:601d"];

// Split the IPv6 tests into 2 modules.
// This allows only running the IPv6 Multicast tests on Linux as they are unsupported on Windows. 
#[cfg(test)]
#[cfg(target_os = "linux")]
mod sacn_ipv6_multicast_test {

use std::{thread};
use std::thread::sleep;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, SyncSender, Receiver, RecvTimeoutError};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::time::Duration;
use std::iter;

use socket2::{Socket, Domain, Type};

use sacn::source::SacnSource;
use sacn::receive::{SacnReceiver, DMXData};
use sacn::packet::{UNIVERSE_CHANNEL_CAPACITY, ACN_SDT_MULTICAST_PORT, universe_to_ipv4_multicast_addr, 
    universe_to_ipv6_multicast_addr, E131_DISCOVERY_UNIVERSE, E131_TERMINATE_STREAM_PACKET_COUNT};
use sacn::error::errors::*;

/// UUID library used to handle the UUID's used in the CID fields.
use uuid::Uuid;

use ipv4_tests::{TEST_DATA_SINGLE_UNIVERSE, 
    TEST_DATA_MULTIPLE_UNIVERSE, TEST_DATA_PARTIAL_CAPACITY_UNIVERSE, 
    TEST_DATA_FULL_CAPACITY_MULTIPLE_UNIVERSE, TEST_DATA_MULTIPLE_ALTERNATIVE_STARTCODE_UNIVERSE,
    TEST_DATA_SINGLE_ALTERNATIVE_STARTCODE_UNIVERSE};
use TEST_NETWORK_INTERFACE_IPV6;

#[test]
#[ignore]
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
    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[1].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);

    let mut src = SacnSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    src.register_universe(universe).unwrap();

    src.send(&[universe], &TEST_DATA_PARTIAL_CAPACITY_UNIVERSE, Some(priority), None, None).unwrap();

    let received_result: Result<Vec<DMXData>> = rx.recv().unwrap();

    rcv_thread.join().unwrap();

    assert!(!received_result.is_err(), "Failed: Error when receiving data");

    let received_data: Vec<DMXData> = received_result.unwrap();

    assert_eq!(received_data.len(), 1); // Check only 1 universe received as expected.

    let received_universe: DMXData = received_data[0].clone();

    assert_eq!(received_universe.universe, universe); // Check that the universe received is as expected.

    assert_eq!(received_universe.values, TEST_DATA_PARTIAL_CAPACITY_UNIVERSE.to_vec(), "Received payload values don't match sent!");
}

#[test]
#[ignore]
fn test_send_recv_single_universe_alternative_startcode_multicast_ipv6(){
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

    assert!(!received_result.is_err(), "Failed: Error when receiving data");

    let received_data: Vec<DMXData> = received_result.unwrap();

    assert_eq!(received_data.len(), 1); // Check only 1 universe received as expected.

    let received_universe: DMXData = received_data[0].clone();

    assert_eq!(received_universe.universe, universe); // Check that the universe received is as expected.

    assert_eq!(received_universe.values, TEST_DATA_SINGLE_ALTERNATIVE_STARTCODE_UNIVERSE.to_vec(), "Received payload values don't match sent!");
}

#[test]
#[ignore]
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

    assert!(!sync_pkt_res.is_err(), "Failed: Error when receiving packets");

    let mut received_data: Vec<DMXData> = sync_pkt_res.unwrap();

    received_data.sort(); // No guarantee on the ordering of the received data so sort it first to allow easier checking.

    assert_eq!(received_data.len(), 2); // Check 2 universes received as expected.

    assert_eq!(received_data[0].universe, 2); // Check that the universe received is as expected.

    assert_eq!(received_data[0].sync_uni, 2); // Check that the sync universe is as expected.

    assert_eq!(received_data[0].values, TEST_DATA_MULTIPLE_ALTERNATIVE_STARTCODE_UNIVERSE[..UNIVERSE_CHANNEL_CAPACITY].to_vec(), "Universe 1 received payload values don't match sent!");

    assert_eq!(received_data[1].universe, 3); // Check that the universe received is as expected.

    assert_eq!(received_data[1].sync_uni, 2); // Check that the sync universe is as expected.

    assert_eq!(received_data[1].values, TEST_DATA_MULTIPLE_ALTERNATIVE_STARTCODE_UNIVERSE[UNIVERSE_CHANNEL_CAPACITY..].to_vec(), "Universe 2 received payload values don't match sent!");
}

#[test]
#[ignore]
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

    assert!(!sync_pkt_res.is_err(), "Failed: Error when receiving packets");

    let mut received_data: Vec<DMXData> = sync_pkt_res.unwrap();

    received_data.sort(); // No guarantee on the ordering of the received data so sort it first to allow easier checking.

    assert_eq!(received_data.len(), 2); // Check 2 universes received as expected.

    assert_eq!(received_data[0].universe, 2); // Check that the universe received is as expected.

    assert_eq!(received_data[0].sync_uni, 2); // Check that the sync universe is as expected.

    assert_eq!(received_data[0].values, TEST_DATA_FULL_CAPACITY_MULTIPLE_UNIVERSE[..UNIVERSE_CHANNEL_CAPACITY].to_vec(), "Universe 1 received payload values don't match sent!");

    assert_eq!(received_data[1].universe, 3); // Check that the universe received is as expected.

    assert_eq!(received_data[1].sync_uni, 2); // Check that the sync universe is as expected.

    assert_eq!(received_data[1].values, TEST_DATA_FULL_CAPACITY_MULTIPLE_UNIVERSE[UNIVERSE_CHANNEL_CAPACITY..].to_vec(), "Universe 2 received payload values don't match sent!");
}

#[test]
#[ignore]
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

    assert!(!received_result.is_err(), "Failed: Error when receiving data");

    let received_data: Vec<DMXData> = received_result.unwrap();

    assert_eq!(received_data.len(), 1); // Check only 1 universe received as expected.

    let received_universe: DMXData = received_data[0].clone();

    assert_eq!(received_universe.universe, universe); // Check that the universe received is as expected.

    assert_eq!(received_universe.values, TEST_DATA_SINGLE_UNIVERSE.to_vec(), "Received payload values don't match sent!");
}

#[test]
#[ignore]
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

    assert!(!sync_pkt_res.is_err(), "Failed: Error when receiving packets");

    let mut received_data: Vec<DMXData> = sync_pkt_res.unwrap();

    received_data.sort(); // No guarantee on the ordering of the received data so sort it first to allow easier checking.

    assert_eq!(received_data.len(), 2); // Check 2 universes received as expected.

    assert_eq!(received_data[0].universe, 2); // Check that the universe received is as expected.

    assert_eq!(received_data[0].sync_uni, 2); // Check that the sync universe is as expected.

    assert_eq!(received_data[0].values, TEST_DATA_MULTIPLE_UNIVERSE[..UNIVERSE_CHANNEL_CAPACITY].to_vec(), "Universe 1 received payload values don't match sent!");

    assert_eq!(received_data[1].universe, 3); // Check that the universe received is as expected.

    assert_eq!(received_data[1].sync_uni, 2); // Check that the sync universe is as expected.

    assert_eq!(received_data[1].values, TEST_DATA_MULTIPLE_UNIVERSE[UNIVERSE_CHANNEL_CAPACITY..].to_vec(), "Universe 2 received payload values don't match sent!");
}

#[test]
#[ignore]
fn test_send_across_universe_multiple_receivers_sync_multicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>>>, Receiver<Result<Vec<DMXData>>>) = mpsc::channel();

    let thread1_tx = tx.clone();
    let thread2_tx = tx.clone();

    let universe1 = 1;
    let universe2 = 2;

    let sync_uni = 3;

    let rcv_thread1 = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&[universe1]).unwrap();
        dmx_recv.listen_universes(&[sync_uni]).unwrap();

        thread1_tx.send(Ok(Vec::new())).unwrap();

        thread1_tx.send(dmx_recv.recv(None)).unwrap();
    });

    let rcv_thread2 = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[1].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&[universe2]).unwrap();
        dmx_recv.listen_universes(&[sync_uni]).unwrap();

        thread2_tx.send(Ok(Vec::new())).unwrap();

        thread2_tx.send(dmx_recv.recv(None)).unwrap();
    });

    rx.recv().unwrap().unwrap(); // Blocks until both receivers say they are ready.
    rx.recv().unwrap().unwrap();

    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);

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

#[test]
#[ignore]
fn test_three_senders_three_recv_multicast_ipv6(){
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

    assert!(RCV_THREADS <= TEST_NETWORK_INTERFACE_IPV6.len(), "Number of test network interface ips less than number of recv threads!");

    const BASE_UNIVERSE: u16 = 2;

    for i in 0 .. SND_THREADS {
        let tx = snd_tx.clone();

        let data = snd_data[i].clone();

        snd_threads.push(thread::spawn(move || {
            let ip: SocketAddr = SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1 + (i as u16));
            // https://www.programming-idioms.org/idiom/153/concatenate-string-with-integer/1975/rust (11/01/2020)
            let mut src = SacnSource::with_ip(&format!("Source {}", i), ip).unwrap();
    
            let priority = 100;

            let universe: u16 = (i as u16) + BASE_UNIVERSE; 
    
            src.register_universe(universe).unwrap(); // Senders all send on different universes.

            tx.send(()).unwrap(); // Forces each sender thread to wait till the controlling thread receives which stops sending before the receivers are ready.
    
            src.send(&[universe], &data, Some(priority), None, None).unwrap();
        }));
    }

    for i in 0 .. RCV_THREADS {
        let tx = rcv_tx.clone();

        rcv_threads.push(thread::spawn(move || {
            // Port kept the same so must use multiple IP's.
            let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[i].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

            // Receivers listen to all universes
            for i in (BASE_UNIVERSE as u16) .. ((SND_THREADS as u16) + (BASE_UNIVERSE as u16)) {
                dmx_recv.listen_universes(&[i]).unwrap();
            }

            let mut res: Vec<Result<Vec<DMXData>>> = Vec::new();

            tx.send(Vec::new()).unwrap(); // Receiver notifies controlling thread it is ready.

            for _i in 0 .. SND_THREADS { // Receiver should receive from every universe.
                res.push(dmx_recv.recv(None)); // Receiver won't complete this until it receives from the senders which are all held waiting on the controlling thread.
            }

            // Results of each receive are sent back, this allows checking that each receiver was an expected universe, all universes were received and there were no errors.
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
            assert_eq!(data.len(), 1); // Check that each universe was received separately.
            rcv_dmx_datas.push(data[0].clone());
        }

        rcv_dmx_datas.sort_unstable(); // Sorting by universe allows easier checking as order received may vary depending on network.

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
#[ignore]
fn test_universe_discovery_one_universe_one_source_ipv6(){
    const SND_THREADS: usize = 1;
    const BASE_UNIVERSE: u16 = 2;
    const UNIVERSE_COUNT: usize = 1;
    const SOURCE_NAMES: [&'static str; 1] = ["Source 1"];

    let (snd_tx, snd_rx): (SyncSender<()>, Receiver<()>) = mpsc::sync_channel(0);

    let mut snd_threads = Vec::new();

    for i in 0 .. SND_THREADS {
        let tx = snd_tx.clone();

        snd_threads.push(thread::spawn(move || {
            let ip: SocketAddr = SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1 + (i as u16));

            let mut src = SacnSource::with_ip(SOURCE_NAMES[i], ip).unwrap();

            let mut universes: Vec<u16> = Vec::new();
            for j in 0 .. UNIVERSE_COUNT {
                universes.push(((i + j) as u16) + BASE_UNIVERSE);
            }

            src.register_universes(&universes).unwrap();

            tx.send(()).unwrap(); // Used to force the sender to wait till the receiver has received a universe discovery.
        }));
    }

    let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

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
#[ignore]
fn test_universe_discovery_multiple_universe_one_source_ipv6(){
    const SND_THREADS: usize = 1;
    const BASE_UNIVERSE: u16 = 2;
    const UNIVERSE_COUNT: usize = 5;
    const SOURCE_NAMES: [&'static str; 1] = ["Source 1"];

    let (snd_tx, snd_rx): (SyncSender<()>, Receiver<()>) = mpsc::sync_channel(0);

    let mut snd_threads = Vec::new();

    for i in 0 .. SND_THREADS {
        let tx = snd_tx.clone();

        snd_threads.push(thread::spawn(move || {
            let ip: SocketAddr = SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1 + (i as u16));

            let mut src = SacnSource::with_ip(SOURCE_NAMES[i], ip).unwrap();

            let mut universes: Vec<u16> = Vec::new();
            for j in 0 .. UNIVERSE_COUNT {
                universes.push(((i + j) as u16) + BASE_UNIVERSE);
            }

            src.register_universes(&universes).unwrap();

            tx.send(()).unwrap(); // Used to force the sender to wait till the receiver has received a universe discovery.
        }));
    }

    let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

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
#[ignore]
fn test_universe_discovery_multiple_pages_one_source_ipv6(){
    const SND_THREADS: usize = 1;
    const BASE_UNIVERSE: u16 = 2;
    const UNIVERSE_COUNT: usize = 600;
    const SOURCE_NAMES: [&'static str; 1] = ["Source 1"];

    let (snd_tx, snd_rx): (SyncSender<()>, Receiver<()>) = mpsc::sync_channel(0);

    let mut snd_threads = Vec::new();

    for i in 0 .. SND_THREADS {
        let tx = snd_tx.clone();

        snd_threads.push(thread::spawn(move || {
            let ip: SocketAddr = SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1 + (i as u16));

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

    let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

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
#[ignore]
fn test_send_recv_two_universe_multicast_ipv6(){
    let (tx, rx): (Sender<Result<Vec<DMXData>>>, Receiver<Result<Vec<DMXData>>>) = mpsc::channel();

    let thread_tx = tx.clone();

    let universes = [1, 2];

    let rcv_thread = thread::spawn(move || {
        let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT), None).unwrap();

        dmx_recv.listen_universes(&universes).unwrap();

        thread_tx.send(Ok(Vec::new())).unwrap(); // Notify the sender that the receiver is ready.

        thread_tx.send(dmx_recv.recv(None)).unwrap(); // Receive and pass on 2 lots of data, blocking.
        thread_tx.send(dmx_recv.recv(None)).unwrap();
    });

    rx.recv().unwrap().unwrap(); // Blocks until the receiver says it is ready. 

    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);
    let mut src = SacnSource::with_ip("Source", ip).unwrap();

    src.register_universes(&universes).unwrap();

    // Send 2 universes of data with default priority, no synchronisation and use multicast.
    src.send(&universes, &TEST_DATA_MULTIPLE_UNIVERSE, None, None, None).unwrap();

    // Get the data that was sent to the receiver.
    let received_result: Result<Vec<DMXData>> = rx.recv().unwrap();
    let received_result_2: Result<Vec<DMXData>> = rx.recv().unwrap();

    // Receiver can be terminated.
    rcv_thread.join().unwrap();

    assert!(!received_result.is_err(), "Failed: Error when receiving 1st universe of data");
    assert!(!received_result_2.is_err(), "Failed: Error when receiving 2nd universe of data");

    let received_data: Vec<DMXData> = received_result.unwrap();
    let received_data_2: Vec<DMXData> = received_result_2.unwrap();

    assert_eq!(received_data.len(), 1);   // Check only 1 universe received from each individual recv() as expected, if this wasn't the case it would
    assert_eq!(received_data_2.len(), 1); // indicate that the data has been synchronised incorrectly or that less data than expected was received.

    assert_eq!(received_data[0].universe, universes[0]);   // Check that the universe received is as expected.
    assert_eq!(received_data_2[0].universe, universes[1]);

    assert_eq!(received_data[0].values, TEST_DATA_MULTIPLE_UNIVERSE[..513].to_vec());
    assert_eq!(received_data_2[0].values, TEST_DATA_MULTIPLE_UNIVERSE[513..].to_vec());
}

#[test]
#[ignore]
fn test_two_senders_one_recv_same_universe_no_sync_multicast_ipv6(){
    let universe = 1;

    let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT), None).unwrap();

    dmx_recv.listen_universes(&[universe]).unwrap();

    let snd_thread_1 = thread::spawn(move || {
        let ip: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT + 1);
        let mut src = SacnSource::with_ip("Source", ip).unwrap();

        let priority = 100;

        src.register_universe(universe).unwrap();

        let _ = src.send(&[universe], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), None, None).unwrap();
    });

    let snd_thread_2 = thread::spawn(move || {
        let ip: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT + 2);
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

/// Setups and runs through the scenario as described in ANSI E1.31-2018 Appendix B.
/// This asserts that the behaviour of this implementation is exactly as outlined within that section.
/// This shows that the implementation handles universe synchronisation in the way specified by the protocol document. 
/// As the force synchronisation option is not implemented as part of this library that section is ignored.
/// 
/// This is exactly the same as the IPv4 variant test of the same name but done over IPv6 to show equivalence.
/// 
#[test]
#[ignore]
fn test_ansi_e131_appendix_b_runthrough_ipv6() {
    // The number of set of (data packets + sync packet) to send.
    const SYNC_PACKET_COUNT: usize = 5;

    // The number of data packets sent before each sync packet.
    const DATA_PACKETS_PER_SYNC_PACKET: usize = 2;

    // The 'slight pause' as specified by ANSI E1.31-2018 Section 11.2.2 between data and sync packets.
    const PAUSE_PERIOD: Duration = Duration::from_millis(100);

    let (tx, rx): (SyncSender<()>, Receiver<()>) = mpsc::sync_channel(0);
    
    let thread_tx = tx.clone();

    let data_universes = [1, 2];
    let sync_universe = 7962;
    let priority = 100;
    let source_name = "Source_A";
    let data = [0x00, 0xe, 0x0, 0xc, 0x1, 0x7, 0x1, 0x4, 0x8, 0x0, 0xd, 0xa, 0x7, 0xa];
    let data2 = [0x00, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0xa];
    let src_cid: Uuid = Uuid::from_bytes(&[0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e]).unwrap();

    let snd_thread = thread::spawn(move || {
        let ip: SocketAddr = SocketAddr::new(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap(), ACN_SDT_MULTICAST_PORT + 1);
        let mut src = SacnSource::with_cid_ip(source_name, src_cid, ip).unwrap();

        src.register_universes(&data_universes).unwrap();
        src.register_universe(sync_universe).unwrap();

        // Sender waits till the receiver says it is ready.
        thread_tx.send(()).unwrap();

        for _ in 0 .. SYNC_PACKET_COUNT {
            // Sender sends data packets to the 2 data universes using the same synchronisation address.
            src.send(&[data_universes[0]], &data, Some(priority), None, Some(sync_universe)).unwrap();
            src.send(&[data_universes[1]], &data2, Some(priority), None, Some(sync_universe)).unwrap();

            // Sender observes a slight pause to allow for processing delays (ANSI E1.31-2018 Section 11.2.2).
            sleep(PAUSE_PERIOD);

            // Sender sends a synchronisation packet to the sync universe.
            src.send_sync_packet(sync_universe, None).unwrap();
        }

        // Sender sends a data packet to the data universe using a zero synchronisation address indicating synchronisation is now over.
        src.send(&[data_universes[0]], &data, Some(priority), None, None).unwrap();
        src.send(&[data_universes[1]], &data2, Some(priority), None, None).unwrap();
    });
    
    let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(TEST_NETWORK_INTERFACE_IPV6[1].parse().unwrap(), ACN_SDT_MULTICAST_PORT), None).unwrap();

    // Receiver only listening to the data universe, the sync universe should be joined automatically when a data packet that requires it arrives.
    dmx_recv.listen_universes(&data_universes).unwrap();

    // Receiver created successfully so allow the sender to progress.
    rx.recv().unwrap();

    for _ in 0 .. SYNC_PACKET_COUNT {
        // "When the E1.31 Synchronization Packet arrives from Source A, Receiver B acts on the data."
        match dmx_recv.recv(None) {
            Ok(p) => { 
                assert_eq!(p.len(), DATA_PACKETS_PER_SYNC_PACKET);
                if p[0].universe == data_universes[0] {
                    assert_eq!(p[0].values, data, "Unexpected data within first data packet of a set of synchronised packets");

                    assert_eq!(p[1].universe, data_universes[1], "Unrecognised universe as second data packet in set of synchronised packets");
                    assert_eq!(p[1].values, data2, "Unexpected data within second data packet of a set of synchronised packets");
                } else if p[0].universe == data_universes[1] {
                    assert_eq!(p[0].values, data2, "Unexpected data within first data packet of a set of synchronised packets");

                    assert_eq!(p[1].universe, data_universes[0], "Unrecognised universe as second data packet in set of synchronised packets");
                    assert_eq!(p[1].values, data, "Unexpected data within second data packet of a set of synchronised packets");
                } else {
                    assert!(false, "Unrecognised universe within data packet");
                }
            }
            Err(e) => {
                assert!(false, format!("Unexpected error returned: {:?}", e));
            }
        }
    }
    // "This process continues until Receiver B receives an E1.31 Data Packet with a Synchronization Address of 0."
    // "Receiver B may then unsubscribe from the synchronization multicast address" - This implementation does not automatically unsubscribe
    //        This is based on the reasoning that a synchronisation universe will be used multiple times and subscribing/un-subscribing is unneeded overhead.
    // Synchronisation is now over so should receive 2 packets individually.
    let rcv_data = dmx_recv.recv(None).unwrap();
    assert_eq!(rcv_data.len(), 1);
    assert_eq!(rcv_data[0].universe, data_universes[0]);
    assert_eq!(rcv_data[0].values, data);

    let rcv_data2 = dmx_recv.recv(None).unwrap();
    assert_eq!(rcv_data2.len(), 1);
    assert_eq!(rcv_data2[0].universe, data_universes[1]);
    assert_eq!(rcv_data2[0].values, data2);

    // "If, at any time, Receiver B receives more than one E1.31 Data Packet with the same Synchronization
    // Address in it, before receiving an E1.31 Universe Synchronization Packet, it will discard all but the most
    // recent E1.31 Data Packet. Those packets are only acted upon when the synchronization command
    // arrives."
    // This is taken to refer to a data packet within the same universe and synchronisation address not a packet with any universe
    // this assumption is based on the wording "Universe synchronization is required for applications where receivers require more than one universe to
    // be controlled, multiple receivers produce synchronized output, or unsynchronized control of receivers may
    // result in undesired visual effects." from ANSI E1.31-2018 Section 11. This wording indicates that one use case of synchronisation is to allow
    // receivers with more than one universe to be controlled however this would be impossible if the statement above (from ANSI E1.31-2018 Appendix B) 
    // indicated that data packets for all but one universe should be discarded.

    // "Since the the Force_Synchronization bit in the Options field of the E1.31 Data Packet has been set to 0,
    // even if Source A times out the E131_NETWORK_DATA_LOSS_TIMEOUT, Receiver B will stay in its last
    // look until a new E1.31 Synchronization Packet arrives."
    // The implementation does not support the force synchronisation bit so always acts as if is set to 1 and times out.

    snd_thread.join().unwrap();
}

/// Sets up a single source and receiver. Like in a real-world scenario the source sends data continuously on 2 universes synchronised 
/// on a third universe with a small interval between data sends.
/// The receiver starts with no knowledge of what universe the source is sending on so starts by using Universe Discovery to discover the universes
/// it then joins these universes and receives the data. The sender eventually stops sending data and terminates by sending stream termination packets.
/// The receiver receives these packets and also terminates.
/// This shows that the implementation works in a simulated scenario that makes use of multiple features / parts.
/// It also shows the receiver 'jumping into' a stream of data that has already started (meaning sequence numbers are already > 0).
/// 
/// This is exactly the same as the IPv4 variant test of the same name but done over IPv6 to show equivalence.
/// 
#[test]
#[ignore]
fn test_discover_recv_sync_runthrough_ipv6() {
    // The number of set of (data packets + sync packet) to send.
    const SYNC_PACKET_COUNT: usize = 250;

    // The number of data packets sent before each sync packet.
    const DATA_PACKETS_PER_SYNC_PACKET: usize = 2;

    // The 'slight pause' as specified by ANSI E1.31-2018 Section 11.2.2 between data and sync packets.
    const PAUSE_PERIOD: Duration = Duration::from_millis(50);
    
    // The interval between sets of sync/data packets.
    const INTERVAL: Duration = Duration::from_millis(100);

    // The universes used for data.
    const DATA_UNIVERSES: [u16; 2] = [1, 2];

    // The universe used for synchronisation packets.
    const SYNC_UNIVERSE: u16 = 4;

    // The source name
    const SOURCE_NAME: &str = "Test Source";

    // The data send on the first and second universes.
    const DATA: [u8; 16] = [0x00, 0xe, 0x0, 0xc, 0x1, 0x7, 0x1, 0x4, 0x8, 0x0, 0xd, 0xa, 0x7, 0xa, 0x9, 0x8];
    const DATA2: [u8; 16] =[0x00, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0xa, 0x9, 0x8];

    // The source CID.
    let src_cid: Uuid = Uuid::from_bytes(&[0xef, 0x07, 0xc8, 0xdd, 0x00, 0x64, 0x44, 0x01, 0xa3, 0xa2, 0x45, 0x9e, 0xf8, 0xe6, 0x14, 0x3e]).unwrap();

    let snd_thread = thread::spawn(move || {
        let ip: SocketAddr = SocketAddr::new(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap(), ACN_SDT_MULTICAST_PORT + 1);
        let mut src = SacnSource::with_cid_ip(SOURCE_NAME, src_cid, ip).unwrap();

        src.register_universes(&DATA_UNIVERSES).unwrap();
        src.register_universe(SYNC_UNIVERSE).unwrap();

        for _ in 0 .. SYNC_PACKET_COUNT {
            // Sender sends data packets to the 2 data universes using the same synchronisation address.
            src.send(&[DATA_UNIVERSES[0]], &DATA, None, None, Some(SYNC_UNIVERSE)).unwrap();
            src.send(&[DATA_UNIVERSES[1]], &DATA2, None, None, Some(SYNC_UNIVERSE)).unwrap();

            // Sender observes a slight pause to allow for processing delays (ANSI E1.31-2018 Section 11.2.2).
            sleep(PAUSE_PERIOD);

            // Sender sends a synchronisation packet to the sync universe.
            src.send_sync_packet(SYNC_UNIVERSE, None).unwrap();

            sleep(INTERVAL);
        }

        // Sender goes out of scope so will automatically send termination packets.
    });
    
    let mut dmx_recv = SacnReceiver::with_ip(SocketAddr::new(TEST_NETWORK_INTERFACE_IPV6[1].parse().unwrap(), ACN_SDT_MULTICAST_PORT), None).unwrap();

    // Receiver starts by not listening to any data universes (automatically listens to discovery universe).
    
    dmx_recv.set_announce_source_discovery(true);

    let universes: Vec<u16> = match dmx_recv.recv(None) {
        Err(e) => {
            match e.kind() {
                ErrorKind::SourceDiscovered(_name) => {
                    let discovered_sources = dmx_recv.get_discovered_sources();
                    assert_eq!(discovered_sources.len(), 1);

                    // Found the source so don't want to be notified about other sources.
                    dmx_recv.set_announce_source_discovery(false);

                    // Do want to be notified about stream termination in this case.
                    dmx_recv.set_announce_stream_termination(true);

                    discovered_sources[0].get_all_universes()
                }
                _ => {
                    // A real-user would want to look at using more detailed error handling as appropriate to their use case but for this test panic 
                    // (which will fail the test) is suitable.
                    panic!("Unexpected error");
                }
            }
        }
        Ok(_) => {
            panic!("Unexpected data packet before any data universes registered");
        }
    };

    dmx_recv.listen_universes(&universes).unwrap(); // Assert Successful

    loop {
        match dmx_recv.recv(None) {
            Err(e) => {
                match e.kind() {
                    ErrorKind::UniverseTerminated(_src_cid, _universe) => {
                        // A real use-case may also want to not terminate when the source does and instead remain waiting but in this
                        // case the for the test the receiver terminates with the source.
                        break;
                    }
                    _ => {
                        assert!(false, "Unexpected error returned");
                    }
                }
            }
            Ok(rcv_data) => {
                assert_eq!(rcv_data.len(), DATA_PACKETS_PER_SYNC_PACKET);
                if rcv_data[0].universe == DATA_UNIVERSES[0] {
                    assert_eq!(rcv_data[0].values, DATA, "Unexpected data within first data packet of a set of synchronised packets");

                    assert_eq!(rcv_data[1].universe, DATA_UNIVERSES[1], "Unrecognised universe as second data packet in set of synchronised packets");
                    assert_eq!(rcv_data[1].values, DATA2, "Unexpected data within second data packet of a set of synchronised packets");
                } else if rcv_data[0].universe == DATA_UNIVERSES[1] {
                    assert_eq!(rcv_data[0].values, DATA2, "Unexpected data within first data packet of a set of synchronised packets");

                    assert_eq!(rcv_data[1].universe, DATA_UNIVERSES[0], "Unrecognised universe as second data packet in set of synchronised packets");
                    assert_eq!(rcv_data[1].values, DATA, "Unexpected data within second data packet of a set of synchronised packets");
                } else {
                    assert!(false, "Unrecognised universe within data packet");
                }
            } 
        }
    }

    // Finished receiving from the sender.
    snd_thread.join().unwrap();
}

/// Creates an IPv4 sender and an IPv6 sender as well as 2 receiver sockets (one for each IP version).
/// 
/// Both senders then send a data packet, sync packet, discovery packet and termination packet to their respective receiver socket and the test asserts 
/// that all packets received are identical regardless of IP version used as per ANSI E1.31-2018 Section 9.1
/// 
#[test]
#[ignore]
fn test_ip_equivalence() {
    /* Packet parameters, not directly the focus of the test */
    const CID: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    const PRIORITY: u8 = 150;

    let universe: u16 = 1;

    let source_name = "SourceName".to_string() +
                        "\0\0\0\0\0\0\0\0\0\0" +
                        "\0\0\0\0\0\0\0\0\0\0" +
                        "\0\0\0\0\0\0\0\0\0\0" +
                        "\0\0\0\0\0\0\0\0\0\0" +
                        "\0\0\0\0\0\0\0\0\0\0" +
                        "\0\0\0\0";
    let mut dmx_data: Vec<u8> = Vec::new();
    dmx_data.push(0); // Start code
    dmx_data.extend(iter::repeat(100).take(255));

    /*  */

    // Create and setup the ipv4 source.
    let ipv4: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT + 1);
    let mut ipv4_source = SacnSource::with_cid_ip(&source_name.clone(), Uuid::from_bytes(&CID).unwrap(), ipv4).unwrap();
    ipv4_source.set_preview_mode(false).unwrap();
    ipv4_source.set_multicast_loop_v4(true).unwrap();
    ipv4_source.register_universes(&[universe as u16]).unwrap();

    // Create and setup the ipv4 receiver socket.
    let ipv4_recv = Socket::new(Domain::ipv4(), Type::dgram(), None).unwrap();
    let ipv4_multicast_addr = universe_to_ipv4_multicast_addr(universe).unwrap();
    let ipv4_discovery_multicast_addr = universe_to_ipv4_multicast_addr(E131_DISCOVERY_UNIVERSE).unwrap();

    // To allow joining multiple multicast groups like this reuse port/address must be true.
    ipv4_recv.set_reuse_port(true).unwrap();
    ipv4_recv.set_reuse_address(true).unwrap();

    // Bind to the unspecified 0.0.0.0 address allowing receiving any data on that port then join the universe and the discovery multicast groups.
    // Binding to unspecified required to allow receiving from multiple multicast addresses.
    ipv4_recv.bind(&SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT).into()).unwrap();
    ipv4_recv.join_multicast_v4(&ipv4_multicast_addr.as_inet().unwrap().ip(), &Ipv4Addr::UNSPECIFIED).unwrap();
    ipv4_recv.join_multicast_v4(&ipv4_discovery_multicast_addr.as_inet().unwrap().ip(), &Ipv4Addr::UNSPECIFIED).unwrap();
    
    // Create and setup the ipv6 source.
    let ipv6: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT + 1);
    let mut ipv6_source = SacnSource::with_cid_ip(&source_name.clone(), Uuid::from_bytes(&CID).unwrap(), ipv6).unwrap();
    ipv6_source.set_preview_mode(false).unwrap();
    ipv6_source.register_universes(&[universe]).unwrap();

    // Create and setup the ipv6 receiver socket.
    let ipv6_recv = Socket::new(Domain::ipv6(), Type::dgram(), None).unwrap();
    let ipv6_multicast_addr = universe_to_ipv6_multicast_addr(universe).unwrap();
    let ipv6_discovery_multicast_addr = universe_to_ipv6_multicast_addr(E131_DISCOVERY_UNIVERSE).unwrap();

    // To allow joining multiple multicast groups like this reuse port/address must be true.
    ipv6_recv.set_reuse_port(true).unwrap();
    ipv6_recv.set_reuse_address(true).unwrap();

    // Bind to the unspecified :: address for same reason as for IPv4.
    ipv6_recv.bind(&SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), ACN_SDT_MULTICAST_PORT).into()).unwrap();
    ipv6_recv.join_multicast_v6(&ipv6_multicast_addr.as_inet6().unwrap().ip(), 0).unwrap();
    ipv6_recv.join_multicast_v6(&ipv6_discovery_multicast_addr.as_inet6().unwrap().ip(), 0).unwrap();
    
    // Send and receive the data packet over IPv4.
    let mut ipv4_recv_buf = [0; 1024];
    ipv4_source.send(&[universe], &dmx_data, Some(PRIORITY), None, None).unwrap();
    let (ipv4_len, _) = ipv4_recv.recv_from(&mut ipv4_recv_buf).unwrap();

    // Send and receive the data packet over IPv6.
    let mut ipv6_recv_buf = [0; 1024];
    ipv6_source.send(&[universe], &dmx_data, Some(PRIORITY), None, None).unwrap();
    let (ipv6_len, _) = ipv6_recv.recv_from(&mut ipv6_recv_buf).unwrap();

    // Check that the data packets match.
    assert_eq!(ipv4_recv_buf[.. ipv4_len], ipv6_recv_buf[.. ipv6_len], "IPv4 and IPv6 data packets aren't identical");

    // Send and receive the sync packet over IPv4.
    ipv4_recv_buf = [0; 1024];
    ipv4_source.send_sync_packet(universe, None).unwrap();
    let (ipv4_len, _) = ipv4_recv.recv_from(&mut ipv4_recv_buf).unwrap();

    // Send and receive the sync packet over IPv6.
    ipv6_recv_buf = [0; 1024];
    ipv6_source.send_sync_packet(universe, None).unwrap();
    let (ipv6_len, _) = ipv6_recv.recv_from(&mut ipv6_recv_buf).unwrap();

    // Check the sync packets match.
    assert_eq!(ipv4_recv_buf[.. ipv4_len], ipv6_recv_buf[.. ipv6_len], "IPv4 and IPv6 sync packets aren't identical");

    // Wait for discovery packet over IPv4.
    ipv4_recv_buf = [0; 1024];
    let (ipv4_len, _) = ipv4_recv.recv_from(&mut ipv4_recv_buf).unwrap();

    // Wait for discovery packet over IPv6.
    ipv6_recv_buf = [0; 1024];
    let (ipv6_len, _) = ipv6_recv.recv_from(&mut ipv6_recv_buf).unwrap();

    // Check the discovery packets match.
    assert_eq!(ipv4_recv_buf[.. ipv4_len], ipv6_recv_buf[.. ipv6_len], "IPv4 and IPv6 discovery packets aren't identical");

    // Terminate sending data on the universe.
    ipv4_source.terminate_stream(universe, 0).unwrap();
    ipv6_source.terminate_stream(universe, 0).unwrap();

    // Termination packets are sent multiple times so check that they are all received.
    for _ in 0 .. E131_TERMINATE_STREAM_PACKET_COUNT {
        // Send and receive a termination packet over IPv4.
        ipv4_recv_buf = [0; 1024];
        let (ipv4_len, _) = ipv4_recv.recv_from(&mut ipv4_recv_buf).unwrap();

        // Send and receive a termination packet over IPv6.
        ipv6_recv_buf = [0; 1024];
        let (ipv6_len, _) = ipv6_recv.recv_from(&mut ipv6_recv_buf).unwrap();

        // Check that the termination packets match
        assert_eq!(ipv4_recv_buf[.. ipv4_len], ipv6_recv_buf[.. ipv6_len], "IPv4 and IPv6 termination packets aren't identical");
    }
}
}

#[cfg(test)]
mod sacn_ipv6_unicast_test {

use std::{thread};
use std::thread::sleep;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

use std::net::{IpAddr, SocketAddr};
use sacn::source::SacnSource;
use sacn::receive::{SacnReceiver, DMXData};
use sacn::packet::{UNIVERSE_CHANNEL_CAPACITY, ACN_SDT_MULTICAST_PORT};

use std::time::Duration;

use sacn::error::errors::*;

use ipv4_tests::{TEST_DATA_SINGLE_UNIVERSE, TEST_DATA_MULTIPLE_UNIVERSE};
use TEST_NETWORK_INTERFACE_IPV6;

#[test]
#[ignore]
fn test_send_recv_single_universe_unicast_ipv6(){
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

    let ip: SocketAddr = SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[1].parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);
    let mut src = SacnSource::with_ip("Source", ip).unwrap();

    let priority = 100;

    src.register_universe(universe).unwrap();

    let dst_ip: SocketAddr = SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT);

    let _ = src.send(&[universe], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), Some(dst_ip), None).unwrap();

    let received_result: Result<Vec<DMXData>> = rx.recv().unwrap();

    rcv_thread.join().unwrap();

    assert!(!received_result.is_err(), "Failed: Error when receiving data");

    let received_data: Vec<DMXData> = received_result.unwrap();

    assert_eq!(received_data.len(), 1); // Check only 1 universe received as expected.

    let received_universe: DMXData = received_data[0].clone();

    assert_eq!(received_universe.universe, universe); // Check that the universe received is as expected.

    assert_eq!(received_universe.values, TEST_DATA_SINGLE_UNIVERSE.to_vec(), "Received payload values don't match sent!");
}

#[test]
#[ignore]
/// Note: this test assumes perfect network conditions (0% reordering, loss, duplication etc.), this should be the case for
/// the loopback adapter with the low amount of data sent but this may be a possible cause if integration tests fail unexpectedly.
fn test_send_recv_across_universe_unicast_ipv6(){
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

    let _ = src.send(&UNIVERSES, &TEST_DATA_MULTIPLE_UNIVERSE, Some(priority), Some(SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT).into()), Some(UNIVERSES[0])).unwrap();
    sleep(Duration::from_millis(500)); // Small delay to allow the data packets to get through as per NSI-E1.31-2018 Appendix B.1 recommendation.
    src.send_sync_packet(UNIVERSES[0], Some(SocketAddr::new(IpAddr::V6(TEST_NETWORK_INTERFACE_IPV6[0].parse().unwrap()), ACN_SDT_MULTICAST_PORT).into())).unwrap();

    let sync_pkt_res: Result<Vec<DMXData>> = rx.recv().unwrap();

    rcv_thread.join().unwrap();

    assert!(!sync_pkt_res.is_err(), "Failed: Error when receiving packets");

    let mut received_data: Vec<DMXData> = sync_pkt_res.unwrap();

    received_data.sort(); // No guarantee on the ordering of the received data so sort it first to allow easier checking.

    assert_eq!(received_data.len(), 2); // Check 2 universes received as expected.

    assert_eq!(received_data[0].universe, 2); // Check that the universe received is as expected.

    assert_eq!(received_data[0].sync_uni, 2); // Check that the sync universe is as expected.

    assert_eq!(received_data[0].values, TEST_DATA_MULTIPLE_UNIVERSE[..UNIVERSE_CHANNEL_CAPACITY].to_vec(), "Universe 1 received payload values don't match sent!");

    assert_eq!(received_data[1].universe, 3); // Check that the universe received is as expected.

    assert_eq!(received_data[1].sync_uni, 2); // Check that the sync universe is as expected.

    assert_eq!(received_data[1].values, TEST_DATA_MULTIPLE_UNIVERSE[UNIVERSE_CHANNEL_CAPACITY..].to_vec(), "Universe 2 received payload values don't match sent!");
}
}