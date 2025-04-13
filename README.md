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
## Testing
## Demo Implementation

## What is ESTA 1.31-2018 (ANSI E1.31-2018)?

**ESTA 1.31-2018**, also known as **ANSI E1.31-2018**, defines how to transmit **DMX512 lighting control data** over standard **Ethernet networks** using the **sACN (Streaming Architecture for Control Networks)** protocol.

It is widely used in **theatrical, concert, architectural, and broadcast lighting** environments.

---

### Key Terms

- **DMX512**: A digital control protocol used to manage stage lighting and effects. It transmits values (0–255) for up to 512 control channels per line, called a **universe**.

- **Universe**: A group of 512 DMX channels, each corresponding to a controllable parameter like brightness or color. Large shows often use many universes to control numerous fixtures.

- **Ethernet**: A standard computer networking technology that allows data to be transmitted over cables like CAT5e or CAT6. In this context, it enables DMX data to travel over modern IP-based networks.

- **sACN (Streaming ACN)**: A protocol built on IP networking that transports DMX data efficiently over Ethernet, allowing fast and reliable lighting control.

---

### What the Standard Does

- **Defines how to run the DMX512 protocol over an Ethernet-based network**, using specific IP addresses and multicast to send data.
- **Enables multiple senders and receivers** to operate on the same network.
- **Scales to large systems** with many universes and devices.
- **Includes rules for data merging and priority**, so multiple controllers can interact with the same fixtures without conflict.

---

### Why It Matters

- Simplifies wiring for large or distributed lighting systems.
- Enables flexible control setups, including remote operation.
- Scales easily to support complex or high-density lighting installations.
- Widely supported and adopted in professional lighting environments.

---

### TL;DR

**ESTA 1.31-2018 allows high-speed lighting control data to be transmitted over regular computer networks instead of older DMX cables, making modern lighting systems more powerful, flexible, and scalable.**

Originally forked from https://github.com/lschmierer/sacn and then further developed as part of a final year project at the University of St Andrews.
