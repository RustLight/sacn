#![cfg_attr(rustfmt, rustfmt_skip)]
// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was created as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

extern crate sacn;
extern crate uuid;

use sacn::error::errors::*;

use sacn::source::SacnSource;
use sacn::packet::*;

use std::net::{SocketAddr, Ipv4Addr, IpAddr};

/// UUID library used to handle the UUID's used in the CID fields.
use uuid::Uuid;

/// Attempts to create an ipv4 source with the source name longer than expected.
#[test]
fn test_new_ipv4_one_too_long_source_name() {
    const SRC_NAME: &str = "01234567890123456789012345678901234567890123456789012345678901234";
    match SacnSource::new_v4(SRC_NAME) {
        Err(e) => {
            match e {
                SacnError::MalformedSourceName(_) => {
                    assert!(true, "Expected error returned");
                }
                _ => {
                    assert!(false, "Unexpected error returned");
                }
            }
        }
        Ok(_) => {
            assert!(false, "SacnSource created with a source name length greater than the allowed maximum");
        }
    }
}

#[test]
fn test_new_ipv6_one_too_long_source_name() {
    const SRC_NAME: &str = "01234567890123456789012345678901234567890123456789012345678901234";
    match SacnSource::new_v6(SRC_NAME) {
        Err(e) => {
            match e {
                SacnError::MalformedSourceName(_) => {
                    assert!(true, "Expected error returned");
                }
                _ => {
                    assert!(false, "Unexpected error returned");
                }
            }
        }
        Ok(_) => {
            assert!(false, "SacnSource created with a source name length greater than the allowed maximum");
        }
    }
}

#[test]
fn test_new_with_cid_ip_too_long_source_name() {
    const SRC_NAME: &str = "01234567890123456789012345678901234567890123456789012345678901234";
    match SacnSource::with_cid_ip(SRC_NAME, Uuid::new_v4(), SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT)) {
        Err(e) => {
            match e {
                SacnError::MalformedSourceName(_) => {
                    assert!(true, "Expected error returned");
                }
                _ => {
                    assert!(false, "Unexpected error returned");
                }
            }
        }
        Ok(_) => {
            assert!(false, "SacnSource created with a source name length greater than the allowed maximum");
        }
    }
}

#[test]
fn test_new_with_cid_ip_v4_too_long_source_name() {
    const SRC_NAME: &str = "01234567890123456789012345678901234567890123456789012345678901234";
    match SacnSource::with_cid_v4(SRC_NAME, Uuid::new_v4()) {
        Err(e) => {
            match e {
                SacnError::MalformedSourceName(_) => {
                    assert!(true, "Expected error returned");
                }
                _ => {
                    assert!(false, "Unexpected error returned");
                }
            }
        }
        Ok(_) => {
            assert!(false, "SacnSource created with a source name length greater than the allowed maximum");
        }
    }
}

#[test]
fn test_new_with_cid_ip_v6_too_long_source_name() {
    const SRC_NAME: &str = "01234567890123456789012345678901234567890123456789012345678901234";
    match SacnSource::with_cid_v6(SRC_NAME, Uuid::new_v4()) {
        Err(e) => {
            match e {
                SacnError::MalformedSourceName(_) => {
                    assert!(true, "Expected error returned");
                }
                _ => {
                    assert!(false, "Unexpected error returned");
                }
            }
        }
        Ok(_) => {
            assert!(false, "SacnSource created with a source name length greater than the allowed maximum");
        }
    }
}

#[test]
fn test_new_with_ip_too_long_source_name() {
    const SRC_NAME: &str = "01234567890123456789012345678901234567890123456789012345678901234";
    match SacnSource::with_ip(SRC_NAME, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT)) {
        Err(e) => {
            match e {
                SacnError::MalformedSourceName(_) => {
                    assert!(true, "Expected error returned");
                }
                _ => {
                    assert!(false, "Unexpected error returned");
                }
            }
        }
        Ok(_) => {
            assert!(false, "SacnSource created with a source name length greater than the allowed maximum");
        }
    }
}

#[test]
fn test_set_name_too_long_source_name() {
    const SRC_NAME: &str = "01234567890123456789012345678901234567890123456789012345678901234";
    let mut src = SacnSource::new_v4("Initial name").unwrap();

    match src.set_name(SRC_NAME) {
        Err(e) => {
            match e {
                SacnError::MalformedSourceName(_) => {
                    assert!(true, "Expected error returned");
                }
                _ => {
                    assert!(false, "Unexpected error returned");
                }
            }
        }
        Ok(_) => {
            assert!(false, "SacnSource created with a source name length greater than the allowed maximum");
        }
    }
}

#[test]
fn test_get_name() {
    let name = "Test_Name";
    let src = SacnSource::new_v4(name).unwrap();

    assert_eq!(name, src.name().unwrap(), "Name retrieved does not match name set");
}

#[test]
fn test_set_name_get_name() {
    let name = "Test_Name";
    let mut src = SacnSource::new_v4("Initial Name").unwrap();

    src.set_name(name).unwrap();

    assert_eq!(name, src.name().unwrap(), "Name retrieved does not match name set");
}

#[test]
fn test_get_cid() {
    let cid = Uuid::new_v4();

    let src = SacnSource::with_cid_ip("Test name", cid, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT)).unwrap();

    assert_eq!(src.cid().unwrap(), cid, "CID does not match CID set");
}

#[test]
fn test_set_get_cid() {
    let cid = Uuid::new_v4();

    let mut src = SacnSource::with_cid_ip("Test name", cid, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT)).unwrap();

    let new_cid = Uuid::new_v4();

    src.set_cid(new_cid).unwrap();

    assert_eq!(src.cid().unwrap(), new_cid, "CID does not match CID set");
}

#[test]
fn test_get_preview() {
    let src = SacnSource::with_cid_ip("Test name", Uuid::new_v4(), SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT)).unwrap();

    assert!(!src.preview_mode().unwrap(), "Preview mode not set to false initially");
}

#[test]
fn test_set_get_preview() {
    let mut src = SacnSource::with_cid_ip("Test name", Uuid::new_v4(), SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT)).unwrap();

    src.set_preview_mode(true).unwrap();

    assert!(src.preview_mode().unwrap(), "Preview mode not set correctly");
}

#[test]
fn test_set_get_multicast_ttl() {
    let mut src = SacnSource::with_cid_ip("Test name", Uuid::new_v4(), SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT)).unwrap();

    let ttl = 3;

    src.set_multicast_ttl(ttl).unwrap();

    assert_eq!(src.multicast_ttl().unwrap(), ttl, "TTL not set correctly");
}

#[test]
fn test_set_get_ttl() {
    let mut src = SacnSource::with_cid_ip("Test name", Uuid::new_v4(), SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT)).unwrap();

    let ttl = 3;

    src.set_ttl(ttl).unwrap();

    assert_eq!(src.ttl().unwrap(), ttl, "TTL not set correctly");
}

#[test]
fn test_get_multicast_loop() {
    let src = SacnSource::with_cid_ip("Test name", Uuid::new_v4(), SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT)).unwrap();

    assert!(src.multicast_loop().unwrap(), "Multicast loop set to false initially when expected true");
}

#[test]
fn test_set_get_multicast_loop() {
    let mut src = SacnSource::with_cid_ip("Test name", Uuid::new_v4(), SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), ACN_SDT_MULTICAST_PORT)).unwrap();

    src.set_multicast_loop_v4(false).unwrap();

    assert!(!src.multicast_loop().unwrap(), "Multicast loop not set to false correctly");
}

#[test]
fn test_send_without_registering(){
    let mut src = SacnSource::new_v4("Controller").unwrap();

    let priority = 100;

    match src.send(&[1], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), None, None) {
        Ok(_) => {assert!(false, "Source didn't prevent sending without registering")},
        Err(ref e) =>
            match e {
                &SacnError::UniverseNotRegistered(ref _s) => assert!(true),
                _ => assert!(false, "Unexpected error type returned, {}", e)
            }
    }
}

/// Attempts to send a packet with a priority higher (> 200) than the maximum allowed as per ANSI E1.31-2018 Section 6.2.3.
#[test]
fn test_send_above_priority(){
    let mut src = SacnSource::new_v4("Controller").unwrap();
    let universe = 1;
    let priority = 201;

    src.register_universe(universe).unwrap();

    match src.send(&[universe], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), None, None) {
        Err(e) => {
            match e {
                SacnError::InvalidPriority(_) => {
                    assert!(true, "Expected error returned");
                }
                x => {
                    assert!(false, "Unexpected error type returned, {:?}", x);
                }
            }

        }
        Ok(_) => {
            assert!(
                false,
                "Invalid priority (> limit) was not rejected"
            );
        }
    }
}

/// Tests sending a single universe of data, this appear 'assertion-free' but it isn't because .unwrap() will panic
/// if a function returns an error.
/// This test therefore checks that the sender works without crashing in one of the simplest cases.
#[test]
fn test_send_single_universe(){
    let mut src = SacnSource::new_v4("Controller").unwrap();

    let priority = 100;

    let universe: u16 = 1;

    src.register_universe(universe).unwrap();

    src.send(&[1], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), None, None).unwrap();
}

#[test]
fn test_send_across_universe(){
    let mut src = SacnSource::new_v4("Controller").unwrap();

    let priority = 100;

    let universes: [u16; 2] = [1, 2];

    src.register_universes(&universes).unwrap();

    src.send(&universes, &TEST_DATA_MULTIPLE_UNIVERSE, Some(priority), None, None).unwrap();
}

/// Attempt to register a universe below the minimum allowed universe. This should fail with an IllegalUniverse error.
/// Exceptional test.
#[test]
fn test_register_below_min_universe() {
    let mut src = SacnSource::new_v4("Controller").unwrap();
    const UNIVERSE: u16 = E131_MIN_MULTICAST_UNIVERSE - 1;

    match src.register_universes(&[UNIVERSE]) {
        Err(e) => {
            match e {
                SacnError::IllegalUniverse(_) => {
                    assert!(true, "Expected error returned");
                }
                _ => {
                    assert!(false, "Unexpected error type returned");
                }
            }
        }
        _ => {
            assert!(false, "Attempt to register universe below minimum succeeded when should have failed");
        }
    }
}

/// Attempt to register a universe above the maximum allowed universe. This should fail with an IllegalUniverse error.
/// Exceptional test.
#[test]
fn test_register_above_max_universe() {
    let mut src = SacnSource::new_v4("Controller").unwrap();
    const UNIVERSE: u16 = E131_MAX_MULTICAST_UNIVERSE + 1;

    match src.register_universes(&[UNIVERSE]) {
        Err(e) => {
            match e {
                SacnError::IllegalUniverse(_) => {
                    assert!(true, "Expected error returned");
                }
                _ => {
                    assert!(false, "Unexpected error type returned");
                }
            }
        }
        _ => {
            assert!(false, "Attempt to register universe above maximum succeeded when should have failed");
        }
    }
}

/// Attempt to register the discovery universe. Even though this is higher than the maximum allowed universe this should succeed as per ANSI E1.31-2018 Section 6.2.7.
/// Extreme test.
#[test]
fn test_register_discovery_universe() {
    let mut src = SacnSource::new_v4("Controller").unwrap();
    match src.register_universes(&[E131_DISCOVERY_UNIVERSE]) {
        Err(e) => {
            assert!(false, "Unexpected error returned when attempting to register discovery universe, {:?}", e);
        }
        _ => {
            assert!(true, "Registration successful");
        }
    }
}

/// Attempt to register the maximum allowed universe, this should succeed as the allowed range is inclusive of this universe.
/// Extreme test.
#[test]
fn test_register_max_universe() {
    let mut src = SacnSource::new_v4("Controller").unwrap();
    match src.register_universes(&[E131_MAX_MULTICAST_UNIVERSE]) {
        Err(e) => {
            assert!(false, "Unexpected error returned when attempting to register the maximum allowed universe, {:?}", e);
        }
        _ => {
            assert!(true, "Registration successful");
        }
    }
}

/// Attempt to register the minimum allowed universe, this should succeed as the allowed range is inclusive of this universe.
/// Extreme test.
#[test]
fn test_register_min_universe() {
    let mut src = SacnSource::new_v4("Controller").unwrap();
    match src.register_universes(&[E131_MIN_MULTICAST_UNIVERSE]) {
        Err(e) => {
            assert!(false, "Unexpected error returned when attempting to register the maximum allowed universe, {:?}", e);
        }
        _ => {
            assert!(true, "Registration successful");
        }
    }
}

/// Attempts to send a synchronisation packet with the synchronisation address/universe set to 0 which should be rejected as per ANSI E1.31-2018 Section 6.3.3.1.
#[test]
fn test_sync_addr_0() {
    let mut src = SacnSource::new_v4("Controller").unwrap();
    const SYNC_UNI: u16 = 0;

    match src.send_sync_packet(SYNC_UNI, None) {
        Err(e) => {
            match e {
                SacnError::IllegalUniverse(_) => {
                    assert!(true, "Expected error returned");
                }
                _ => {
                    assert!(false, "Unexpected error type returned");
                }
            }
        }
        _ => {
            assert!(false, "Attempt to send a synchronisation packet with a synchronisation address of 0 succeeded when it should have been rejected");
        }
    }
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