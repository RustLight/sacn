// Copyright 2025 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
//  Uses the sACN library to send a sine wave on all channels on universe 1 to localhost.

extern crate sacn;

use sacn::source::SacnSource;
use sacn::packet::{ACN_SDT_MULTICAST_PORT, UNIVERSE_CHANNEL_CAPACITY};

use std::time::{Duration, Instant};
use std::net::SocketAddr;
use std::thread::sleep;

fn main(){
    let interface_ip = "127.0.0.1";
    let source_name = "sine-wave-sender";
    let universe = 1;
    let duration = Duration::from_secs(12);
    let refresh_rate = Duration::from_millis(33);

    // Uses the next along port to allow usage on the same machine as a receiver which is using the ACN_SDT port.
    let mut src = SacnSource::with_ip(source_name, SocketAddr::new(interface_ip.parse().unwrap(), ACN_SDT_MULTICAST_PORT + 1)).unwrap();
    src.register_universe(universe).unwrap();

    let start_time = Instant::now();

    let mut data: [u8; UNIVERSE_CHANNEL_CAPACITY] = [0; UNIVERSE_CHANNEL_CAPACITY];
    while start_time.elapsed() < duration {
        sine_wave(start_time, &mut data);
        src.send(&[universe], &data, None, None, None).unwrap();
        sleep(refresh_rate);
    }
}

fn sine_wave(start_time: Instant, data: &mut [u8; UNIVERSE_CHANNEL_CAPACITY]) {
    // Don't worry too much about this. Its just creating a sine wave in an array of bytes.
    // If you were using the sACN library then this is where you would do all the fancy things you want to do.
    let wave_period_ms = 4000.0; 
    let wave_offset_ms = 10.0;
    for i in 1 .. data.len() { // Use a 0 startcode so skip first value.
        let x: f64 = ((start_time.elapsed().as_millis() as f64) + (i as f64) * wave_offset_ms) / wave_period_ms;
        data[i] = ((std::u8::MAX as f64) * x.sin()).abs() as u8;
    }
}
