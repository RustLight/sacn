#[macro_use]
extern crate afl;
extern crate sacn;

use sacn::packet::*;

fn main() {
    fuzz!(|data: &[u8]| {
            let _ = AcnRootLayerProtocol::parse(data);
        }
    );
}
