# Rust sACN implementation
[![Build Status](https://travis-ci.org/lschmierer/sacn.svg)](https://travis-ci.org/lschmierer/sacn)

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
