#[macro_use]
extern crate afl;
extern crate sacn;

use sacn::packet::*;

fn main() {
    fuzz!(|data: &[u8]| {
            // Key aim is to check that parse does not crash given a wide variety of data. 
            // The specific error or packet produced is not the aim of these tests.
            let _ = AcnRootLayerProtocol::parse(data);
        }
    );
}
