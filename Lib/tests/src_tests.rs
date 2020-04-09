// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was created as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

extern crate sacn;

use sacn::error::errors::*;

use sacn::source::SacnSource;
use sacn::packet::*;

#[test]
fn test_send_without_registering(){
    let mut src = SacnSource::new_v4("Controller").unwrap();
    
    let priority = 100;

    match src.send(&[1], &TEST_DATA_SINGLE_UNIVERSE, Some(priority), None, None) {
        Ok(_) => {assert!(false, "Source didn't prevent sending without registering")},
        Err(e) => 
            match e.kind() {
                &ErrorKind::UniverseNotRegistered(ref _s) => assert!(true),
                _ => assert!(false, format!("Unexpected error type returned, {}", e.kind()))
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
            match e.kind() {
                ErrorKind::InvalidPriority(_) => {
                    assert!(true, "Expected error returned");
                }
                x => {
                    assert!(false, format!("Unexpected error type returned, {:?}", x));
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
            match e.kind() {
                ErrorKind::IllegalUniverse(_) => {
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
            match e.kind() {
                ErrorKind::IllegalUniverse(_) => {
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
            assert!(false, format!("Unexpected error returned when attempting to register discovery universe, {:?}", e));
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
            assert!(false, format!("Unexpected error returned when attempting to register the maximum allowed universe, {:?}", e));
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
            assert!(false, format!("Unexpected error returned when attempting to register the maximum allowed universe, {:?}", e));
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
            match e.kind() {
                ErrorKind::IllegalUniverse(_) => {
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