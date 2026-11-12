# RUST Sacn
A Rust implementation of the ANSI E1.31 Streaming ACN protocol, tested against protocol version ANSI E1.31-2018. 

### Support for
* Sending and receiving data using the ANSI E1.31-2018 protocol over IPv4 and IPv6
* Unicast, Multicast and Broadcast Supported
* Tested on Windows and Linux
* Universe Synchronisation
* Universe Discovery

### Examples
#### Simple application to send a sine wave to universe 1 on localhost
`cargo run --example sine-wave-sender`

## INSTALLATION
### Prerequisites
## Getting Started
## Compliance
Compliance with the ANSI E1.31-2018 protocol was tested (April 2020) and the results are shown in 'documentation/ANSI-E1.31-2018-Compliance-Check-List.pdf'.

### Left to the application
Some parts of the standard depend on how the library is used, so the library leaves them to the application:

* **Transmission rate and keep-alive (Section 6.6).** `SacnSource` only sends when you call `send`. Don't send a universe faster than the DMX512-A refresh rate (about 44 Hz for Null START Code data). When Null START Code data stops changing, send it three more times. After that, resend the last data every 800 to 1000 ms as a keep-alive. Otherwise receivers enter network data loss after `E131_NETWORK_DATA_LOSS_TIMEOUT` (2.5 s).
* **Priority and merging between sources (Section 6.2.3).** For unsynchronised data, `SacnReceiver::recv` returns each source's data as it arrives, with its priority and source CID in `DMXData`. The application chooses which source to use (the highest priority wins) or merges same-priority sources, for example with `htp_dmx_merge`. Data held for universe synchronisation is merged with the function set by `set_merge_fn`.
## Testing
Quick library logic and parse testing: `cargo test`

Ipv4 testing requires that the computer have (a) network interface(s) with the IPs of `192.168.0.6`, `192.168.0.7`, and `192.168.0.8`. These IPs are set in the testing file. Ip tests are ignored by default. Run the ip tests in a single thread to avoid socket conflicts within the OS. To run ipv4 tests, use `cargo test_ip`, which is an alias for
`cargo test --test ipv4_tests -- --ignored --test-threads=1`.

There is a Dockerfile to run the tests, inluding ipv4 tests, on linux. In the project root, run `docker build -t sacn-test -f docker-linux/Dockerfile .` to build the image. Then, run `docker run --cap-add=NET_ADMIN sacn-test` to run the ipv4 tests within the image. The docker engine must be running in linux container mode.

## Demo Implementation

## What is ESTA 1.31-2018 (ANSI E1.31-2018)?

**ESTA 1.31-2018**, also known as **ANSI E1.31-2018**, defines how to transmit **DMX512 lighting control data** over standard **Ethernet networks** using the **sACN (Streaming Architecture for Control Networks)** protocol.

It is widely used in **theatrical, concert, architectural, and broadcast lighting** environments.

#### Key Terms

- **DMX512**: A digital control protocol used to manage stage lighting and effects. It transmits values (0–255) for up to 512 control channels per line, called a **universe**.
- **Universe**: A group of 512 DMX channels, each corresponding to a controllable parameter like brightness or color. Large shows often use many universes to control numerous fixtures.
- **Ethernet**: A standard computer networking technology that allows data to be transmitted over cables like CAT5e or CAT6. In this context, it enables DMX data to travel over modern IP-based networks.
- **sACN (Streaming ACN)**: A protocol built on IP networking that transports DMX data efficiently over Ethernet, allowing fast and reliable lighting control.


#### What the Standard Does

- **Defines how to run the DMX512 protocol over an Ethernet-based network**, using specific IP addresses and multicast to send data.
- **Enables multiple senders and receivers** to operate on the same network.
- **Scales to large systems** with many universes and devices.
- **Includes rules for data merging and priority**, so multiple controllers can interact with the same fixtures without conflict.

#### Why It Matters

- Simplifies wiring for large or distributed lighting systems.
- Enables flexible control setups, including remote operation.
- Scales easily to support complex or high-density lighting installations.
- Widely supported and adopted in professional lighting environments.

#### TL;DR

ESTA 1.31-2018 allows high-speed lighting control data to be transmitted over regular computer networks instead of older DMX cables, making modern lighting systems more powerful, flexible, and scalable.

---

Originally forked from https://github.com/lschmierer/sacn and then further developed as part of a final year project at the University of St Andrews.
