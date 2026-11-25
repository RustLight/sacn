# sACN (E.131) for Rust

A Rust library implementing the E1.31/sACN protocol for transmitting DMX512 control data over standard Ethernet for network-based DMX light control. For more details, see [What is E1.31](#what-is-esta-131-2018-ansi-e131-2018) and [Key Terms](#key-terms) below.

Key features:

* Sending and receiving data using the ANSI E1.31-2018 protocol over IPv4 and IPv6
* Unicast, Multicast and Broadcast packets
* Tested on Windows, Linux and MacOS
* Universe discovery and synchronisation

## Using the library

### Prerequisites

* Rust 1.93.1 or later (the crate uses the 2024 edition). Install Rust with [rustup](https://rustup.rs).
* Linux, macOS or Windows. On Windows, IPv6 multicast isn't supported, so an IPv6 receiver there only receives unicast and broadcast.
* A network that carries UDP on port 5568, the sACN port. Allow it through the host firewall.

  For multicast, the network must pass IP multicast packets; on managed switches this usually means IGMP snooping with a querier.

### Installation

Add the crate to your project:

```sh
cargo add sacn
```

or add it to `Cargo.toml` by hand:

```toml
[dependencies]
sacn = "0.11"
```

### Getting Started

The [crate documentation](https://docs.rs/sacn) has a getting started guide covering the key concepts, sending, synchronisation, receiving and universe discovery.

Send a universe of data with `SacnSource`:

```rust
use sacn::packet::UNIVERSE_CHANNEL_CAPACITY;
use sacn::source::SacnSource;

let mut src = SacnSource::new_v4("My lighting desk")?;
src.register_universe(1)?;

// Index 0 is the DMX START code (0 for dimmer data); indices 1 to 512 are the slots.
let mut data = [0u8; UNIVERSE_CHANNEL_CAPACITY];
data[1] = 255;
// Arguments: universes, data, priority (None = 100), destination (None = multicast), sync universe (None = unsynchronised).
src.send(&[1], &data, None, None, None)?;
```

Receive it with `SacnReceiver`:

```rust
use sacn::packet::ACN_SDT_MULTICAST_PORT;
use sacn::receive::SacnReceiver;
use std::net::SocketAddr;
use std::time::Duration;

// The address of the network interface to receive on, with the sACN port.
let mut dmx_rcv = SacnReceiver::with_ip(SocketAddr::new("0.0.0.0".parse()?, ACN_SDT_MULTICAST_PORT), None)?;
dmx_rcv.listen_universes(&[1])?;

loop {
    // Returns an error when the timeout passes with no data; None blocks.
    if let Ok(universes) = dmx_rcv.recv(Some(Duration::from_secs(1))) {
        for dmx in universes {
            println!("Universe {} at priority {}: {:?}", dmx.universe, dmx.priority, dmx.values);
        }
    }
}
```

The source only sends when you call `send`, so your application must resend data to keep the stream alive, as described [in the documentation](https://docs.rs/sacn/#keeping-a-universe-alive).

## Examples

`examples/demo_src` and `examples/demo_rcv` are the interactive source and receiver used to test the library against other sACN equipment during development. Start each with the IP address of the network interface to use, for example `cargo run --example demo_rcv -- 192.168.0.6`, then type `h` for the list of commands. Between them they cover sending and receiving data, unicast, synchronisation, preview data, stream termination and universe discovery.

## What is ESTA 1.31-2018 (ANSI E1.31-2018)?

[ESTA 1.31-2018](https://tsp.esta.org/tsp/documents/docs/E1-31-2018.pdf), also known as [ANSI E1.31-2018](https://webstore.ansi.org/standards/esta/ansie1312018), defines how to transmit DMX512 lighting control data over standard Ethernet networks using the Streaming Architecture for Control Networks (sACN) protocol.

It allows high-speed lighting control data to be transmitted over regular computer networks instead of older DMX cables, making modern lighting systems more powerful, flexible, and scalable.

It is widely used in theatrical, concert, architectural, and broadcast lighting environments.

### Key Terms

* **[DMX512](https://en.wikipedia.org/wiki/DMX512)**: A digital control protocol used to manage stage lighting and effects. It transmits values (0–255) for up to 512 control channels per line, called a **universe**.
* **DMX Universe**: A group of 512 DMX channels, with each channel corresponding to a controllable parameter like brightness or color. Large shows often use many universes to control numerous fixtures.
* **[Ethernet](https://en.wikipedia.org/wiki/Ethernet)**: Networking standards that allow data to be transmitted over common cables like CAT5e or CAT6. In this context, it enables multiplexing hundreds of universes of DMX512 over modern IP-based networks instead of requiring a XLR5 cable per universe.
* **[sACN (Streaming ACN)](https://en.wikipedia.org/wiki/Architecture_for_Control_Networks)**: A protocol built on IP networking that transports DMX data efficiently over Ethernet, allowing fast and reliable lighting control.

What the sACN standard does:

* **Defines how to run the DMX512 protocol over an Ethernet-based network**, using specific IP addresses and multicast to send data.
* **Enables multiple senders and receivers** to operate on the same network.
* **Scales to large systems** with many universes and devices.
* **Includes rules for data merging and priority**, so multiple controllers can interact with the same fixtures without conflict.

---

## About the project

## License

The project are made available dual-licensed under either [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) at your discretion.

See [CONTRIBUTING](CONTRIBUTING.md) for details on contributing to the source code, including building the library from source, running the tests and the network test setup.

## Development History

This crate started as [lschmierer/sacn](https://github.com/lschmierer/sacn) by [Lukas Schmierer](https://github.com/lschmierer).

[Paul Lancaster](https://github.com/Lan2u) then extended it substantially as a final year project at the University of St Andrews in 2020.

It is now developed by [Stewart Adam](https://github.com/stewartadam) and its contributors at [RustLight/sacn](https://github.com/RustLight/sacn).

## AI disclosure

Development on this project is AI-assisted. We treat coding agents such as Claude Code as an important tool among many others like CI and tests that can help accelerate development and catch quality issues, and not something to be trusted blindly.

Agents might help develop code fixes, catch bugs by checking the library against the E1.31 standard, review code changes, and write tests for edge cases we might have missed. If you author a change with an agent, we expect you to:

* **Plan up front.** Make key design decisions and decide on change scope, usually grounded in a reported issue and behavior described in the E1.31 standard, before the agent starts.
* **Review the code.** Carefully read what the agent produced and make sure you understand it before asking for a merge. It's still **your** code, even if it was written by an agent.

Every change is validated by our CI gates, no matter how it was written:

* **Tests:** CI builds the crate on Linux, Windows and macOS and runs the unit tests, the doc examples, and the IPv4 and IPv6 network tests, which send real sACN between sources and receivers.
* **Human review:** a maintainer reviews every pull request and decides whether to merge it. Agents don't merge changes.
