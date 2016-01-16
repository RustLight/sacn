# Rust sACN implementation
[![Build Status](https://travis-ci.org/lschmierer/sacn.svg)](https://travis-ci.org/lschmierer/sacn)
[![Crates.io](http://meritbadge.herokuapp.com/sacn)](https://crates.io/crates/sacn)

[Documentation](http://lschmierer.github.io/sacn/)

This is an implementation of the Streaming ACN (ANSI E1.31) network protocol.

Currently only the sending part is implemented.

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]

sacn = "0.1"
```

Create a DmxSource and start sending DMX data to an universe.

```rust
extern crate sacn;
use sacn::DmxSource;

let mut dmx_source = DmxSource::new("Controller").unwrap();

dmx_source.send(1, &[0, 1, 2]);
// ...

// terminate the stream for a specific universe
dmx_source.terminate_stream(1);
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
