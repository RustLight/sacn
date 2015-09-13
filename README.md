# Rust sACN implementation

This is an implementation of the Streaming ACN (ANSI E1.31) network protocol.

Currently only the sending part is implemented.

## Usage

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
