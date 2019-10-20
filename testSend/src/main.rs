#[macro_use]
extern crate lazy_static;
extern crate sacn;
use sacn::DmxSource;
use std::{thread, time}; // https://doc.rust-lang.org/std/thread/fn.sleep.html (20/09/2019)

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {
    let dmx_source = DmxSource::new("Controller").unwrap();

    let wait_time = time::Duration::from_millis(500);

    loop {
        dmx_source.send(1, &[0, 1, 2]);
        println!("Sent!");
        thread::sleep(wait_time);
    }
    

    dmx_source.terminate_stream(1);
}