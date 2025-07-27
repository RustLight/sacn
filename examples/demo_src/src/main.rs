#![recursion_limit="1024"] // Recursion limit for error-chain.

// Copyright 2020 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// This file was created as part of a University of St Andrews Computer Science BSC Senior Honours Dissertation Project.

//! An example demo sACN source which utilises the sACN library.
//! 
//! Primarily used for testing the library including real-world conformance, compliance, integration and acceptance tests.
//! 
//! As this program is used for testing the library and isn't part of the actual library it doesn't follow the same standards of error handling and as not user
//! facing it was more helpful to have errors flagged up immediately and explicitly at the source to help development rather than trying to handle the errors.
//! 
//! Usage instructions are described by either running the receiver and using the help command or by the get_usage_str function
//! below.
//! 
//! The ACTION_... constants describe the various user input strings possible once the program has started, with more details described in get_usage_str within
//! the code. The details aren't repeated outside of that to minimise the amount of references that have to be kept upto date and which could diverge over time.
//! 
//! Note the lack of top level constant strings used in the place of output format strings is due to a limitation in rust where the format string cannot be a 
//! const.
//! 

/// The demo itself utilises a small error-chain which wraps the errors from the sACN crate and a few standard crates.
pub mod error;
use error::errors::{DemoError, Result};

extern crate sacn;

use sacn::source::SacnSource;
use sacn::packet::{ACN_SDT_MULTICAST_PORT, UNIVERSE_CHANNEL_CAPACITY};

use std::time::{Duration, Instant};
use std::io;
use std::net::{IpAddr, SocketAddr};
use std::env;
use std::thread::sleep;
use std::str::FromStr;

/// The start code used in termination packets.
const TERMINATE_START_CODE: u8 = 0;

/// The period between updates to the values send during the shape generation command.
/// Default value is approximately 30 updates per second choosen fairly arbitarily to be less than the DMX refresh rate (44 fps).
const SHAPE_DATA_SEND_PERIOD: Duration = Duration::from_millis(33);

/// User string for the preview command to set if preview data should be received.
const ACTION_PREVIEW_OPTION:        &str = "p";

/// User string for the data command to send a packet of data.
const ACTION_DATA_OPTION:           &str = "d";

/// User string for the full data command to send a full universe of data.
const ACTION_FULL_DATA_OPTION:      &str = "f";

/// User string for the unicast data command to send a packet of data using unicast.
const ACTION_UNICAST_OPTION:        &str = "u"; 

/// User string for the register command to register a universe for sending.
const ACTION_REGISTER_OPTION:       &str = "r"; 

/// User string for the terminate/quit command to terminate sending on a universe or terminate the sender entirely.
const ACTION_TERMINATE_OPTION:      &str = "q";

/// User string for the sleep command to make the sender wait for a certain period of time.
const ACTION_SLEEP_OPTION:          &str = "w"; 

/// User string for the sync command to send a synchronisation packet.
const ACTION_SYNC_OPTION:           &str = "s";

/// User string for the unicast syncronisation command to send a synchronisation packet over unicast.
const ACTION_UNICAST_SYNC_OPTION:   &str = "us"; 

/// User string for the send data over time command which sends data to a specific universe that varies over time.
const ACTION_DATA_OVER_TIME_OPTION: &str = "x";

/// User string for the test preset command which runs on of the interoperability test presets.
const ACTION_TEST_PRESENT_OPTION:   &str = "t";

/// User string to indicate that the input line should be ignored. This is mainly used for comments within the automated test input files.
const ACTION_IGNORE:                &str = "#";

/// User string for the all data option which sends an entire universe of data to a given address with all values set to the given value.
const ACTION_ALL_DATA_OPTION:       &str = "a";

/// The test preset numbers which correspond to the various preset tests described in the sender-interoperability testing document.
/// The test number for the two universes sender interoperability test (3).
const TEST_PRESET_TWO_UNIVERSE:         usize = 3;
/// The test number for the two universes unicast sender interoperability test (4).
const TEST_PRESET_TWO_UNIVERSE_UNICAST: usize = 4;
/// The test number for the moving channels sender interoperability test (7).
const TEST_PRESET_MOVING_CHANNELS:      usize = 7;
/// The test number for the preset rapid changes sender interoperability test (8).
const TEST_PRESET_RAPID_CHANGES:        usize = 8;
/// The test number for the high data rate sender interoperability test (9).
const TEST_PRESET_HIGH_DATA_RATE:       usize = 9;

/// Test preset number for acceptance test 100. 
const TEST_PRESET_ACCEPTANCE_TEST:      usize = 100;

/// The duration of one of the preset tests. 
/// Each preset test run for 20 seconds.
const TEST_PRESET_DURATION:         Duration = Duration::from_secs(20);

/// The number of universes to send on during the high data rate test preset.
const TEST_PRESET_HIGH_DATA_RATE_UNI_COUNT: u16 = 16;

/// The period of the wave used in the test moving channel preset.
/// In milliseconds.
const MOVING_CHANNEL_TEST_WAVE_PERIOD: f64 = 4000.0;

/// The offset between each channel in the wave used in the test moving channel preset.
/// The value is dimensionless and corresponds to the scale factor used on the position of the channel in the universe.
/// E.g. 1 is the first channel after the startcode and with a scale factor of 10 it becomes 1 * 10 = 10. This is then added
/// onto x value used for the sine wave. This allows each channel to move slightly seperately. 
const MOVING_CHANNEL_TEST_WAVE_OFFSET: f64 = 10.0;

/// The minimum length of time to wait between sending data packets during the test preset tests.
const TEST_PRESET_UPDATE_PERIOD: Duration = Duration::from_millis(33);

/// The period of the square wave generated as part of this test.
/// Measured in packets. 
const TEST_PRESET_RAPID_CHANGE_PERIOD: usize = 10;

/// The range of values for each universe within the high data rate test preset.
const TEST_PRESET_HIGH_DATA_RATE_VARIATION_RANGE: f64 = 10.0;

/// The 2 universes used for the acceptance test.
/// ACCEPT_TEST_UNI_1 contains the backlight fixtures and ACCEPT_TEST_UNI_2 the frontlight fixtures.
const ACCEPT_TEST_UNI_1: u16 = 1;
const ACCEPT_TEST_UNI_2: u16 = 2;

/// The start addresses for each of the fixtures in the acceptance test (universe 1).
/// These are the backlights which are the colour changing lights far from the camera.
const ACCEPT_TEST_BACKLIGHT_ADDR_1: usize = 1;
const ACCEPT_TEST_BACKLIGHT_ADDR_2: usize = 50;
const ACCEPT_TEST_BACKLIGHT_ADDR_3: usize = 100;
const ACCEPT_TEST_BACKLIGHT_ADDR_4: usize = 150;
const ACCEPT_TEST_BACKLIGHT_ADDR_5: usize = 200;
const ACCEPT_TEST_BACKLIGHT_ADDR_6: usize = 250;
const ACCEPT_TEST_BACKLIGHT_ADDR_7: usize = 300;
const ACCEPT_TEST_BACKLIGHT_ADDR_8: usize = 350;

/// The start addresses for each of the fixtures in the acceptance test.
/// Offset by the universe channel capacity as they are universe 2. 
/// These are the frontlights which are near the camera.
const ACCEPT_TEST_FRONTLIGHT_ADDR_1: usize = UNIVERSE_CHANNEL_CAPACITY + 1;
const ACCEPT_TEST_FRONTLIGHT_ADDR_2: usize = UNIVERSE_CHANNEL_CAPACITY + 2;
const ACCEPT_TEST_FRONTLIGHT_ADDR_3: usize = UNIVERSE_CHANNEL_CAPACITY + 3;

/// The number of addresses taken up by the backlights in the acceptance test.
const ACCEPT_TEST_BACKLIGHT_CH_COUNT: usize = 16;

/// The number of addresses taken up by the frontlights in the acceptance test.
const ACCEPT_TEST_FRONTLIGHT_CH_COUNT: usize = 1;

/// The time the acceptance test sequence should keep cycling for.
const ACCEPT_TEST_DURATION: Duration = Duration::from_secs(30);

/// Describes the various commands / command-line arguments avaliable and what they do.
/// Displayed to the user if they ask for help or enter an unrecognised input.
/// Not a const as const with format! not supported in rust.
fn get_usage_str() -> String{
    format!("Usage ./main <interface_ip> <source_name>\n
    
    Reads data from stdin and sends it using the protocol. \n
    Data must be formatted as, a sync_universe of 0 means no synchronisation, this uses multicast: \n
    {} <universe> <sync_uni> <priority> <data_as_u8_space_seperated> \n
    
    Sends a full universe of data (512 channels + 0 startcode) with the first bytes of the data as specified 
    below (remainder is 0's) \n
    {} <universe> <sync_uni> <priority> <data_as_u8_space_seperate> \n
    
    To send data unicast use: \n
    {} <universe> <sync_uni> <priority> <dst_addr> <data_as_u8_space_seperated> \n
    
    Register a sending universe as: \n
    {} <universe> \n
    
    Terminate a universe, if universe is 0 then will terminate entirely: \n
    {} <universe> \n
    
    Sleep for x milliseconds \n
    {} <milliseconds> \n
    
    Send a synchronisation packet for the given universe \n
    {} <universe> \n
    
    Send a synchronisation packet for the given universe to the given address \n
    {} <universe> <dst_addr> \n
    
    Start a demo shape which continuously sends data to the given universe for the given number of milliseconds \n
    {} <universe> <duration_millis> <priority>\n

    Set the preview data option flag to the given value, this is reflected in packets sent using the other actions\n
    {} <'true'/'false'>\n

    Sends a full universe of data to the given universe with all values (except the startcode which is set to 0) set to the
    given value. This uses the default priority and no synchronisation.\n
    {} <universe> <value>\n

    Generates output based on the scenario described in the corresponding sender interoperability test.\n
    Preset 3: Two Universes Distinct Values, Arguments: <Universe 1> <Universe 2>\n
    Preset 4: Two Universes Distinct Values Unicast, Arguments: <Universe 1> <Universe 2> <dst_ip>\n
    Preset 7: Independent moving channels (sine wave through universe)\n
    Preset 8: Rapid Changes (Quick moves to 255 to 0 to 255 on repeat)\n
    Preset 9: High data rate (Uses given universe as start universe + the next 15 universes (16 universes total)). Raw value, r, for each universe x, r = [(x - 1) * 10, x * 10). \n
            E.g. for universe 2 the range is [20, 30). \n
    Preset 100: Acceptance Test, Union Demo Sequence, Parameters are ignored as making this work in the general case with any lights patched anywhere would involve writing
                an entire lighting board patch system which is significantly beyond the scope of a single developers year work. \n
                \n
    {} <preset> <universe> <... optionally more arguments>\n

    All input is ignored on lines starting with '{} '. 
    ", ACTION_DATA_OPTION, ACTION_FULL_DATA_OPTION, ACTION_UNICAST_OPTION, ACTION_REGISTER_OPTION, ACTION_TERMINATE_OPTION, 
    ACTION_SLEEP_OPTION, ACTION_SYNC_OPTION, ACTION_UNICAST_SYNC_OPTION, ACTION_DATA_OVER_TIME_OPTION, ACTION_PREVIEW_OPTION, 
    ACTION_ALL_DATA_OPTION, ACTION_TEST_PRESENT_OPTION, ACTION_IGNORE)
}

/// Entry point to the demo source. Details of usage can be found in the get_usage_str function or by running the program and typing "h" or "help".
/// 
/// # Arguments:
/// Usage ./main <interface_ip> <source_name>
fn main(){
    let cmd_args: Vec<String> = env::args().collect();

    if cmd_args.len() < 3 {
        return display_help();
    }

    let interface_ip = &cmd_args[1];

    let source_name = &cmd_args[2];

    // Uses the next along port to allow usage on the same machine as a receiver which is using the ACN_SDT port.
    let mut src = SacnSource::with_ip(source_name, SocketAddr::new(interface_ip.parse().unwrap(), ACN_SDT_MULTICAST_PORT + 1)).unwrap();

    println!("Started");

    loop {
        // https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line (03/02/2020)
        match handle_input(&mut src) {
            Ok(should_continue) => {
                if !should_continue {
                    break;
                }
            }
            Err(e) => {
                println!("Error: Input line unusable: {}", e);
            }
        }
    } 
}

/// Displays the usage/help string to stdout.
/// 
fn display_help(){
    println!("{}", get_usage_str());
}

/// Handles the user command to send a full universe of data which starts with the data given and then is padded with 0's upto the full length.
/// 
/// # Arguments
/// src: A mutable reference to the SacnSource to use to send the data with.
/// 
/// split_input: The parts of the user command which have been split up by white space.
/// 
/// split_input[1] is expected to be the universe.
/// 
/// split_input[2] is expected to be the syncronisation universe.
/// 
/// split_input[3] is expected to be the priority.
/// 
/// split_input[4] is expected to be the start of the data to send.
/// 
fn handle_full_data_option(src: &mut SacnSource, split_input: Vec<&str>) -> Result<bool> {
    if split_input.len() < 4 {
        bail!(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts for data line ( < 4 )"));
    }

    let universe: u16 = split_input[1].parse().unwrap();

    let sync_uni: u16 = split_input[2].parse().unwrap();

    let priority: u8 = split_input[3].parse().unwrap();

    let mut data: [u8; 513] = [0; 513];

    for i in 4 .. split_input.len() {
        data[i - 4] = split_input[i].parse().unwrap();
    }

    if sync_uni == 0 {
        src.send(&[universe], &data, Some(priority), None, None)?;
    } else {
        src.send(&[universe], &data, Some(priority), None, Some(sync_uni))?;
    }

    Ok(true)
}

/// Handles the user command to send a full universe of data with all the payload being the same value (with a zero startcode).
/// 
/// # Arguments
/// src: A mutable reference to the SacnSource to use for sending data.
/// 
/// split_input: The input from the user as part of the command split by white space.
/// 
/// split_input[1] is expected to be the universe.
/// 
/// split_input[2] is expected to be the value to set all the payload values to.
/// 
fn handle_all_data_option(src: &mut SacnSource, split_input: Vec<&str>) -> Result<bool> {
    if split_input.len() < 3 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts for data line ( < 3 )").into());
    }
    
    let universe: u16 = split_input[1].parse().unwrap();

    let value: u8 = split_input[2].parse().unwrap();

    let mut data: [u8; 513] = [value; 513];

    data[0] = 0; // Zero startcode used.

    src.send(&[universe], &data, None, None, None)?;

    Ok(true)
}

/// Sends data from the given SacnSource to the multicast address for the given data universe.
///
/// # Arguments
/// src: A mutable reference to the SacnSource to use as the sender to send the unicast data from.
/// 
/// split_input: The input from the user as part of the command split by white space.
/// 
/// split_input[1] is expected to be the universe to send the data to.
/// 
/// split_input[2] is expected to be the synchronisation address to use for the data, 0 means none.
/// 
/// split_input[3] is expected to be the priority to send the data with.
/// 
/// The rest of the input is expected to be the data to send.
/// 
fn handle_data_option(src: &mut SacnSource, split_input: Vec<&str>) -> Result<bool> {
    let universe: u16 = split_input[1].parse().unwrap();

    if split_input.len() < 4 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts for data line ( < 3 )").into());
    }

    let sync_uni: u16 = split_input[2].parse().unwrap();

    let priority: u8 = split_input[3].parse().unwrap();

    let mut data: Vec<u8> = Vec::new();

    for i in 4 .. split_input.len() {
        data.push(split_input[i].parse().unwrap());
    }

    if sync_uni == 0 {
        src.send(&[universe], &data, Some(priority), None, None)?;
    } else {
        src.send(&[universe], &data, Some(priority), None, Some(sync_uni))?;
    }

    Ok(true)
}

/// Sends data from the given SacnSource to the receiver at the given destination using unicast 
/// (or broadcast if a broadcast IP is provided).
///
/// # Arguments
/// src: A mutable reference to the SacnSource to use as the sender to send the unicast data from.
/// 
/// split_input: The input from the user as part of the command split by white space.
/// 
/// split_input[1] is expected to be the universe to send the data to.
/// 
/// split_input[2] is expected to be the synchronisation address to use for the data, 0 means none.
/// 
/// split_input[3] is expected to be the priority to send the data with.
/// 
/// split_input[4] is expected to be the ip to send the data to.
/// 
/// The rest of the input is expected to be the data to send.
/// 
fn handle_unicast_option(src: &mut SacnSource, split_input: Vec<&str>) -> Result<bool> {
    let universe: u16 = split_input[1].parse().unwrap();

    if split_input.len() < 5 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts for data line ( < 5 )").into());
    }

    let sync_uni: u16 = split_input[2].parse().unwrap();

    let priority: u8 = split_input[3].parse().unwrap();

    let dst_ip = split_input[4];

    let mut data: Vec<u8> = Vec::new();

    for i in 5 .. split_input.len() {
        data.push(split_input[i].parse().unwrap());
    }

    if sync_uni == 0 {
        src.send(&[universe], &data, Some(priority), Some(SocketAddr::new(IpAddr::V4(dst_ip.parse().unwrap()), ACN_SDT_MULTICAST_PORT).into()), None)?;
    } else {
        src.send(&[universe], &data, Some(priority), Some(SocketAddr::new(IpAddr::V4(dst_ip.parse().unwrap()), ACN_SDT_MULTICAST_PORT).into()), Some(sync_uni))?;
    }

    Ok(true)
}

/// Handles the user command to send data over time. This sends arbitary data that changes over time. 
/// The specific data isn't important as this is more to show the receiver and sender are connected properly.
/// 
/// # Arguments
/// src: A mutable reference to the SacnSource to use as the sender in this test.
/// 
/// split_input: The input from the user as part of the command split by white space.
/// 
/// split_input[1] is expected to be the universe to send the data to.
/// 
/// split_input[2] is expected to be the time to keep sending data for in milliseconds.
/// 
/// split_input[3] is expected to be the priority to send the data with.
/// 
fn handle_data_over_time_option(src: &mut SacnSource, split_input: Vec<&str>) -> Result<bool> {
    if split_input.len() < 4 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts for data line ( < 4 )").into());
    }

    let universe: u16 = split_input[1].parse().unwrap();
    let duration_millis: u64 = split_input[2].parse().unwrap();
    let priority: u8 = split_input[3].parse().unwrap();

    let duration: Duration = Duration::from_millis(duration_millis);

    let start_time = Instant::now();

    while start_time.elapsed() < duration {
        let x: f64 = (start_time.elapsed().as_millis() as f64) / (1000 as f64);
        let d: u8 = (255.0 * x.sin()) as u8;

        let mut data: [u8; 513] = [d; 513];
        data[0] = 0; // Use 0 startcode

        src.send(&[universe], &data, Some(priority), None, None)?;

        sleep(SHAPE_DATA_SEND_PERIOD);
    }

    Ok(true)
}

/// Handles the user command to run one of the test presets. These test presets are used as part of the interoperability testing as described in the
/// Interoperability Testing document.
/// 
/// # Arguments
/// src: A mutable reference to the SacnSource to use as the sender in this test.
/// 
/// split_input: The input from the user as part of the command split by white space.
/// 
/// split_input[1] is expected to be the preset to run.
/// 
/// split_input[2] is expected to be the universe to use.
/// 
/// More input is dependent on the test preset being run as described in the usage / get_usage_str().
/// 
fn handle_test_preset_option(src: &mut SacnSource, split_input: Vec<&str>) -> Result<bool> {
    if split_input.len() < 3 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts for test preset option ( < 3 )").into());
    }

    let preset: usize = split_input[1].parse().unwrap();
    let universe: u16 = split_input[2].parse().unwrap();

    match preset {
        TEST_PRESET_MOVING_CHANNELS => {
            run_test_moving_channel_preset(src, universe)?;
        },
        TEST_PRESET_RAPID_CHANGES => {
            run_test_rapid_changes_preset(src, universe)?;
        },
        TEST_PRESET_HIGH_DATA_RATE => {
            run_test_high_data_rate(src, universe, TEST_PRESET_HIGH_DATA_RATE_UNI_COUNT)?;
        },
        TEST_PRESET_TWO_UNIVERSE => {
            if split_input.len() < 4 {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts for test preset 2 universes option ( < 4 )").into());
            }

            let universe_2: u16 = split_input[3].parse().unwrap();

            run_test_2_universes_distinct_values(src, universe, universe_2, std::u8::MAX / 2, std::u8::MAX, None)?;
        },
        TEST_PRESET_TWO_UNIVERSE_UNICAST => {
            if split_input.len() < 5 {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts for test preset 2 universes option ( < 4 )").into());
            }

            let universe_2: u16 = split_input[3].parse().unwrap();
            let addr = SocketAddr::new(split_input[4].parse().unwrap(), ACN_SDT_MULTICAST_PORT);

            run_test_2_universes_distinct_values(src, universe, universe_2, std::u8::MAX / 2, std::u8::MAX, Some(addr))?;
        },
        TEST_PRESET_ACCEPTANCE_TEST => {
            run_acceptance_test_demo(src)?;
        }
        _ => {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unrecognised test preset option").into());
        }
    }

    Ok(true)
}

/// Constantly sends data packets to 2 universes with the given values. Used as part of the interoperability testing as described in the
/// the Interoperability Testing document.
/// 
/// # Arguments:
/// src: A mutable reference to the SacnSource to use as the sender in this test.
/// 
/// uni_1: The first universe to send data on.
/// 
/// uni_2: The second universe to send data on.
/// 
/// uni1_val: The value to send on the first universe.
/// 
/// uni2_val: The value to send on the second universe.
/// 
/// dst_ip: None to use multicast or Some(addr) to use unicast to a specific address.
/// 
fn run_test_2_universes_distinct_values(src: &mut SacnSource, uni_1: u16, uni_2: u16, uni1_val: u8, uni2_val: u8, dst_ip: Option<SocketAddr>) -> Result<()> {
    let start_time = Instant::now();

    let mut data_1: [u8; UNIVERSE_CHANNEL_CAPACITY] = [uni1_val; UNIVERSE_CHANNEL_CAPACITY];
    let mut data_2: [u8; UNIVERSE_CHANNEL_CAPACITY] = [uni2_val; UNIVERSE_CHANNEL_CAPACITY];
    data_1[0] = 0; // Uses 0 zero-start code for both universes.
    data_2[0] = 0; 
    
    while start_time.elapsed() < TEST_PRESET_DURATION {
        src.send(&[uni_1], &data_1, None, dst_ip, None)?;
        src.send(&[uni_2], &data_2, None, dst_ip, None)?;

        sleep(TEST_PRESET_UPDATE_PERIOD);
    }

    Ok(())
}

/// Runs the moving channel test preset as part of the interoperability testing. As described in more detail within the Interoperability Testing document.
/// 
/// # Arguments:
/// src: A mutable reference to the SacnSource to use as the sender in this test.
/// 
/// universe: The universe to send data on in the test.
/// 
fn run_test_moving_channel_preset(src: &mut SacnSource, universe: u16) -> Result<()> {
    let start_time = Instant::now();

    let mut data: [u8; UNIVERSE_CHANNEL_CAPACITY] = [0; UNIVERSE_CHANNEL_CAPACITY];

    while start_time.elapsed() < TEST_PRESET_DURATION {

        // Use a 0 startcode so skip first value.
        for i in 1 .. data.len() {
            let x: f64 = ((start_time.elapsed().as_millis() as f64) + (i as f64) * MOVING_CHANNEL_TEST_WAVE_OFFSET) / MOVING_CHANNEL_TEST_WAVE_PERIOD;
            let d: u8 = ((std::u8::MAX as f64) * x.sin()) as u8;
            data[i] = d;
        }

        src.send(&[universe], &data, None, None, None)?;

        sleep(TEST_PRESET_UPDATE_PERIOD);
    }

    Ok(())
}

/// Runs the rapid changes test preset as part of the interoperability testing. As described in more detail within the Interoperability Testing document.
/// 
/// # Arguments:
/// src: A mutable reference to the SacnSource to use as the sender in this test.
/// 
/// universe: The universe to send data on in the test.
/// 
fn run_test_rapid_changes_preset(src: &mut SacnSource, universe: u16) -> Result<()> {
    let start_time = Instant::now();

    let mut counter = 0;

    while start_time.elapsed() < TEST_PRESET_DURATION {
        let mut data = if counter < (TEST_PRESET_RAPID_CHANGE_PERIOD / 2) {
            [0; UNIVERSE_CHANNEL_CAPACITY]
        } else {
            [std::u8::MAX; UNIVERSE_CHANNEL_CAPACITY]
        };

        // Use a zero startcode.
        data[0] = 0;

        src.send(&[universe], &data, None, None, None)?;

        counter = (counter + 1) % TEST_PRESET_RAPID_CHANGE_PERIOD;
        sleep(TEST_PRESET_UPDATE_PERIOD);
    }

    Ok(())
}

/// Runs the high data rate interoperability test preset. As described in more detail within the Interoperability Testing document.
/// 
/// # Arguments:
/// src: A mutable reference to the SacnSource to use as the sender in this test.
/// 
/// start_universe: The universe to use as the first universe in the test.
/// 
/// universe_count: The number of universes starting at the start_universe (inclusive) to send data on.
/// 
fn run_test_high_data_rate(src: &mut SacnSource, start_universe: u16, universe_count: u16) -> Result<()> {
    let start_time = Instant::now();

    let mut counter: f64 = 0.0;

    while start_time.elapsed() < TEST_PRESET_DURATION {
        for universe in start_universe .. start_universe + universe_count {
            let d = ((universe - start_universe) as f64) * (TEST_PRESET_HIGH_DATA_RATE_VARIATION_RANGE * counter.sin());
            let mut data: [u8; UNIVERSE_CHANNEL_CAPACITY] = [d as u8; UNIVERSE_CHANNEL_CAPACITY];
            // Use a zero startcode.
            data[0] = 0;
            src.send(&[universe], &data, None, None, None)?;
        }
        

        counter = counter + 0.05;
        sleep(TEST_PRESET_UPDATE_PERIOD);
    }

    Ok(())
}

/// Runs the acceptance test sender to vision visualiser demo.
/// 
/// Made to work with the corresponding vision visualiser "Student-Union-Model.v3s" file with patch as follows:
/// Format:
/// <fixture_name> <channel_count>ch: <sACN_universe>-<address>, ....
/// 
/// Patch:
/// Robe Robin LedBeam 150 16ch: 1-1, 1-50, 1-100, 1-150, 1-200, 1-250, 1-300, 1-350.
/// Fresnel-Front-Light 1ch: 2-1, 2-2, 2-3
/// 
/// Step 1, 150 + Front On at full.
/// Step 2, Red
/// Step 3, Blue
/// Step 4, All off
/// 
/// # Arguments
/// src: A mutable reference to the SacnSource to use as the sender in the acceptance test.
/// 
fn run_acceptance_test_demo(src: &mut SacnSource) -> Result<()> {
    // The number of steps and the length (in packets) of each step.
    const STEP_COUNT: usize = 4;
    const STEP_LENGTH: usize = 100;

    let start_time = Instant::now();

    let mut step_counter: usize = 0; // Used as an animation / key-frame timeline to allow different actions to happen in sequence.

    // 
    let mut step1_data: [u8; UNIVERSE_CHANNEL_CAPACITY * 2] = [0; UNIVERSE_CHANNEL_CAPACITY * 2];
    let mut step2_data: [u8; UNIVERSE_CHANNEL_CAPACITY * 2] = [0; UNIVERSE_CHANNEL_CAPACITY * 2];
    let mut step3_data: [u8; UNIVERSE_CHANNEL_CAPACITY * 2] = [0; UNIVERSE_CHANNEL_CAPACITY * 2];
    let mut step4_data: [u8; UNIVERSE_CHANNEL_CAPACITY * 2] = [0; UNIVERSE_CHANNEL_CAPACITY * 2];

    gen_acceptance_test_step_1(&mut step1_data);
    gen_acceptance_test_step_2(&mut step2_data);
    gen_acceptance_test_step_3(&mut step3_data);
    gen_acceptance_test_step_4(&mut step4_data);

    // Put each step into a data structure to cycle through.
    let data = [step1_data, step2_data, step3_data, step4_data];

    while start_time.elapsed() < ACCEPT_TEST_DURATION {
        // Cycle through each step.
        let pos: usize = step_counter / STEP_LENGTH;

        src.send(&[ACCEPT_TEST_UNI_1, ACCEPT_TEST_UNI_2], &data[pos], None, None, None)?;

        step_counter = (step_counter + 1) % (STEP_LENGTH * STEP_COUNT); // Loop back around at the end.
        sleep(TEST_PRESET_UPDATE_PERIOD);
    }

    Ok(())
}

/// Acceptance Test.
/// Step 1, Backlight + Front On at full.
/// 
/// # Arguments
/// buf: The buffer to put the payload data into.
/// 
fn gen_acceptance_test_step_1(buf: &mut [u8; UNIVERSE_CHANNEL_CAPACITY * 2]) {
    // Backlights.
    gen_acceptance_test_step_1_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_1 .. ACCEPT_TEST_BACKLIGHT_ADDR_1 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_1_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_2 .. ACCEPT_TEST_BACKLIGHT_ADDR_2 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_1_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_3 .. ACCEPT_TEST_BACKLIGHT_ADDR_3 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_1_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_4 .. ACCEPT_TEST_BACKLIGHT_ADDR_4 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_1_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_5 .. ACCEPT_TEST_BACKLIGHT_ADDR_5 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_1_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_6 .. ACCEPT_TEST_BACKLIGHT_ADDR_6 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_1_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_7 .. ACCEPT_TEST_BACKLIGHT_ADDR_7 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_1_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_8 .. ACCEPT_TEST_BACKLIGHT_ADDR_8 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);

    // Frontlights.
    gen_acceptance_test_step_1_frontlight_state(&mut buf[ACCEPT_TEST_FRONTLIGHT_ADDR_1 .. ACCEPT_TEST_FRONTLIGHT_ADDR_1 + ACCEPT_TEST_FRONTLIGHT_CH_COUNT]);
    gen_acceptance_test_step_1_frontlight_state(&mut buf[ACCEPT_TEST_FRONTLIGHT_ADDR_2 .. ACCEPT_TEST_FRONTLIGHT_ADDR_2 + ACCEPT_TEST_FRONTLIGHT_CH_COUNT]);
    gen_acceptance_test_step_1_frontlight_state(&mut buf[ACCEPT_TEST_FRONTLIGHT_ADDR_3 .. ACCEPT_TEST_FRONTLIGHT_ADDR_3 + ACCEPT_TEST_FRONTLIGHT_CH_COUNT]);
}

/// Acceptance Test.
/// 
/// The backlights use 16 dmx channels each. The usage of each channel is as described by the DMX chart found at the manufacture website:
/// https://www.robe.cz/ledbeam-150/download/#dmx-charts (12/04/2020).
/// The backlights are in 16 channel mode (mode 2).
/// Channel : Usage (explaination)
/// 1: Pan (positioning)
/// 2: Pan Fine (positioning)
/// 3: Tilt (positioning)
/// 4: Tilt Fine (positioning)
/// 5: Pan/Tilt Speed (positioning)
/// 6: Power/Special Functions (Control channel)
/// 7: Virtual Colour Wheel (colour)
/// 8: Red
/// 9: Green
/// 10:Blue
/// 11:White
/// 12: CTC (colour temperature)
/// 13: Colour-Mix-Control 
/// 14: Zoom (beam wideness)
/// 15: Shutter/Strobe (should the fixture rapidly flash)
/// 16: Dimmer Intensity (brightness)
/// 
/// This is why this demo is locked to this specific lighting fixture and mode. There is no standard to these channel orderings and so any different
/// fixture or mode wouldn't behave as expected.
/// 
/// # Arguments
/// buf: The buffer to put the payload data into.
/// 
fn gen_acceptance_test_step_1_backlight_state(buf: &mut [u8]) {
    // Position the fixtures pointing at the stage.
    buf[0] = 128;  // Pan
    buf[1] = 0;    // Pan Fine
    buf[2] = 148;  // Tilt
    buf[3] = 114;  // Tilt Fine
    buf[4] = 0;    // Pan/Tilt Speed

    // Leave the fixtures in their default options state.
    buf[5] = 0;    // Power/Special Functions (Control channel)

    // Set the fixture to white.
    buf[6] = 0;    // Using Red-Green-Blue-White mixing so don't use the virtual colour wheel.
    buf[7] = 0;    // Red at 0.
    buf[8] = 0;    // Green at 0.
    buf[9] = 0;    // Blue at 0.
    buf[10] = 255; // White at full.
    buf[11] = 0;   // Colour temperature at 0 (meaning default colour temperature of around 6500k).
    buf[12] = 45;  // Use additive colour mixing. 45 is the default value.

    // Set the fixture so it covers the stage.
    buf[13] = 91;  // Zoom the fixture so it is wide enough to get reasonable coverage.

    // Set the fixture to full.
    buf[14] = 255; // The shutter is set to 255 to indicate it is fully open meaning all light can pass.
    buf[15] = 255; // The brightness is set to 255 to indicate it should be at full.
}

/// Acceptance Test.
/// The front-lights only use a single channel which is brightness.
/// For step 1 this is set to full (255).
/// 
/// # Arguments
/// buf: The buffer to put the payload data into.
/// 
fn gen_acceptance_test_step_1_frontlight_state(buf: &mut [u8]) {
    buf[0] = 255;
}

/// Acceptance Test.
/// Step 2, Red
/// 
/// # Arguments
/// buf: The buffer to put the payload data into.
/// 
fn gen_acceptance_test_step_2(buf: &mut [u8; UNIVERSE_CHANNEL_CAPACITY * 2]) {
    // Many of the channels will stay the same, therefore can just layer the changes on top.
    // This is refered to as 'tracking' within the lighting industry and internally is key to how modern lighting control systems deal with having so
    // many parameters in real-time (by only changing the ones necessary).
    gen_acceptance_test_step_1(buf);

    // The backlight fixtures change colour so therefore the colour channels within them need changing.
    gen_acceptance_test_step_2_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_1 .. ACCEPT_TEST_BACKLIGHT_ADDR_1 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_2_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_2 .. ACCEPT_TEST_BACKLIGHT_ADDR_2 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_2_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_3 .. ACCEPT_TEST_BACKLIGHT_ADDR_3 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_2_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_4 .. ACCEPT_TEST_BACKLIGHT_ADDR_4 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_2_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_5 .. ACCEPT_TEST_BACKLIGHT_ADDR_5 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_2_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_6 .. ACCEPT_TEST_BACKLIGHT_ADDR_6 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_2_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_7 .. ACCEPT_TEST_BACKLIGHT_ADDR_7 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_2_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_8 .. ACCEPT_TEST_BACKLIGHT_ADDR_8 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);

    // Note that because the front light is not changing in this step it doesn't have to be modified.
}

/// Acceptance Test.
/// Apply the changes to the backlight fixtures for step 2 (set to red). Note that only the color changes so only colour channels are affected.
/// This relies on the buffer containing the values from step 1 already.
/// 
/// # Arguments
/// buf: The buffer to put the payload data into.
/// 
fn gen_acceptance_test_step_2_backlight_state(buf: &mut [u8]) {
    // Set the fixture to red.
    buf[7] = 255;  // Red at full.
    buf[10] = 0;   // White at 0.
}

/// Acceptance Test.
/// Step 3, Blue
/// 
/// # Arguments
/// buf: The buffer to put the payload data into.
/// 
fn gen_acceptance_test_step_3(buf: &mut [u8; UNIVERSE_CHANNEL_CAPACITY * 2]) {
    gen_acceptance_test_step_2(buf);

    // The backlight fixtures change colour so therefore the colour channels within them need changing.
    gen_acceptance_test_step_3_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_1 .. ACCEPT_TEST_BACKLIGHT_ADDR_1 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_3_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_2 .. ACCEPT_TEST_BACKLIGHT_ADDR_2 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_3_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_3 .. ACCEPT_TEST_BACKLIGHT_ADDR_3 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_3_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_4 .. ACCEPT_TEST_BACKLIGHT_ADDR_4 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_3_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_5 .. ACCEPT_TEST_BACKLIGHT_ADDR_5 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_3_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_6 .. ACCEPT_TEST_BACKLIGHT_ADDR_6 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_3_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_7 .. ACCEPT_TEST_BACKLIGHT_ADDR_7 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_3_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_8 .. ACCEPT_TEST_BACKLIGHT_ADDR_8 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
}

/// Acceptance Test.
/// Apply the changes to the backlight fixtures for step 3 (set to blue). Note that only the color changes so only colour channels are affected.
/// This relies on the buffer containing the values from step 2 already.
/// 
/// # Arguments
/// buf: The buffer to put the payload data into.
/// 
fn gen_acceptance_test_step_3_backlight_state(buf: &mut [u8]) {
    // Set the fixture to blue.
    buf[7] = 0;     // Red at 0.
    buf[9] = 255;   // Blue at full.
}

/// Acceptance test.
/// Step 4, All Off.
/// 
/// # Arguments
/// buf: The buffer to put the payload data into.
/// 
fn gen_acceptance_test_step_4(buf: &mut [u8; UNIVERSE_CHANNEL_CAPACITY * 2]) {
    gen_acceptance_test_step_3(buf);

    // All fixtures need to change in this state to off so need to update them all.

    // Backlights.
    gen_acceptance_test_step_4_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_1 .. ACCEPT_TEST_BACKLIGHT_ADDR_1 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_4_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_2 .. ACCEPT_TEST_BACKLIGHT_ADDR_2 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_4_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_3 .. ACCEPT_TEST_BACKLIGHT_ADDR_3 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_4_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_4 .. ACCEPT_TEST_BACKLIGHT_ADDR_4 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_4_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_5 .. ACCEPT_TEST_BACKLIGHT_ADDR_5 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_4_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_6 .. ACCEPT_TEST_BACKLIGHT_ADDR_6 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_4_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_7 .. ACCEPT_TEST_BACKLIGHT_ADDR_7 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);
    gen_acceptance_test_step_4_backlight_state(&mut buf[ACCEPT_TEST_BACKLIGHT_ADDR_8 .. ACCEPT_TEST_BACKLIGHT_ADDR_8 + ACCEPT_TEST_BACKLIGHT_CH_COUNT]);

    // Frontlights.
    gen_acceptance_test_step_4_frontlight_state(&mut buf[ACCEPT_TEST_FRONTLIGHT_ADDR_1 .. ACCEPT_TEST_FRONTLIGHT_ADDR_1 + ACCEPT_TEST_FRONTLIGHT_CH_COUNT]);
    gen_acceptance_test_step_4_frontlight_state(&mut buf[ACCEPT_TEST_FRONTLIGHT_ADDR_2 .. ACCEPT_TEST_FRONTLIGHT_ADDR_2 + ACCEPT_TEST_FRONTLIGHT_CH_COUNT]);
    gen_acceptance_test_step_4_frontlight_state(&mut buf[ACCEPT_TEST_FRONTLIGHT_ADDR_3 .. ACCEPT_TEST_FRONTLIGHT_ADDR_3 + ACCEPT_TEST_FRONTLIGHT_CH_COUNT]);
}

/// Acceptance Test.
/// Apply the changes to the backlight fixtures for step 4 (turn off). Note that only the brightness changes so only the brightness channel is changed.
/// This relies on the buffer containing the values from step 3 already.
/// 
/// # Arguments
/// buf: The buffer to put the payload data into.
/// 
fn gen_acceptance_test_step_4_backlight_state(buf: &mut [u8]) {
    // Set the fixture brightness to 0.
    buf[15] = 0;
}

/// Acceptance Test.
/// Apply the changes to the frontlight fixtures for step 4 (turn off). Note that only the brightness changes so only the brightness channel is changed.
/// This relies on the buffer containing the values from step 3 already.
/// 
/// # Arguments
/// buf: The buffer to put the payload data into.
/// 
fn gen_acceptance_test_step_4_frontlight_state(buf: &mut [u8]) {
    // Set the fixture brightness to 0.
    buf[0] = 0;
}

/// Handles input from the user.
/// 
/// Returns Ok(true) to continue or Ok(false) if no more input.
/// 
/// # Arguments
/// src: A mutable reference to the SacnSource to perform the user instructions on.
/// 
fn handle_input(src: &mut SacnSource) -> Result <bool>{
    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            if n == 0 {
                // Means EOF is reached so terminate
                return Ok(false)
            }

            // https://www.tutorialspoint.com/rust/rust_string.htm (03/02/2020)
            let split_input: Vec<&str> = input.split_whitespace().collect();
            if split_input.len() < 2 {
                display_help();
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts ( < 2 )").into());
            }

            match split_input[0] {
                ACTION_IGNORE => {
                    // Ignore Input
                    Ok(true)
                }
                ACTION_DATA_OPTION => {
                    handle_data_option(src, split_input)
                }
                ACTION_FULL_DATA_OPTION => {
                    handle_full_data_option(src, split_input)
                }
                ACTION_UNICAST_OPTION => {
                    handle_unicast_option(src, split_input)
                }
                ACTION_DATA_OVER_TIME_OPTION => {
                    handle_data_over_time_option(src, split_input)
                }
                ACTION_SYNC_OPTION => {
                    let universe: u16 = split_input[1].parse().unwrap();
                    src.send_sync_packet(universe, None)?;
                    Ok(true)
                }
                ACTION_UNICAST_SYNC_OPTION => {
                    if split_input.len() < 3 {
                        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts for data line ( < 3 )").into());
                    }

                    let universe: u16 = split_input[1].parse().unwrap();
                    let dst_ip = split_input[2];
                    src.send_sync_packet(universe, Some(SocketAddr::from_str(dst_ip).unwrap().into()))?;
                    Ok(true)
                }
                ACTION_REGISTER_OPTION => {
                    let universe: u16 = split_input[1].parse().unwrap();
                    src.register_universe(universe)?;
                    Ok(true)
                }
                ACTION_PREVIEW_OPTION => {
                    let val = split_input[1].parse();

                    match val {
                        Ok(v) => {
                            src.set_preview_mode(v)?;
                        },
                        Err(_e) => {
                            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Preview flag option not 'true'/'false' or otherwise parsable as boolean").into());
                        }
                    }
                    Ok(true)
                }
                ACTION_TERMINATE_OPTION => {
                    let universe: u16 = split_input[1].parse().unwrap();
                    if universe == 0 {
                        return Ok(false)
                    } else {
                        src.terminate_stream(universe, TERMINATE_START_CODE)?;
                    }
                    Ok(true)
                }
                ACTION_SLEEP_OPTION => {
                    if split_input.len() < 2 {
                        display_help();
                        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient parts ( < 2 )").into());
                    }
                    let millis: u64 = split_input[1].parse().unwrap();
                    sleep(Duration::from_millis(millis));
                    Ok(true)
                }
                ACTION_ALL_DATA_OPTION => {
                    handle_all_data_option(src, split_input)
                }
                ACTION_TEST_PRESENT_OPTION => {
                    handle_test_preset_option(src, split_input)
                }
                x => {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Unknown input type: {}", x)).into());
                }
            }
        }
        Err(e) => {
            return Err(e.into());
        }
    }
}