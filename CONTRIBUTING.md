# Developing the sacn library

This guide covers building the library from source and running its tests. To use the library in your own project, see [Installation](README.md#installation) in the README.

## Prerequisites

* Rust 1.93.1 or later, the crate's minimum supported version (`rust-version` in `Cargo.toml`). Install it with [rustup](https://rustup.rs) and add Clippy with `rustup component add clippy`.
* Git.
* For the network integration tests: Linux with root access, or Docker (see [Network tests](#network-tests)).

## Building

```sh
git clone https://github.com/RustLight/sacn.git
cd sacn
cargo build
```

`.cargo/config.toml` turns on a standard set of Clippy lints and denies `unsafe` code for every build in the repository. Check your changes with:

```sh
cargo clippy
```

## Documentation

The getting started guide is the crate-level documentation in `src/lib.rs`. Build the documentation and open it with:

```sh
cargo doc --no-deps --open
```

The code examples in the documentation are compiled, and some are run, by `cargo test --doc`. Keep them working when you change the API.

## Tests

`cargo test` runs the unit tests, the packet parsing tests and the documentation examples. They don't need any network setup and run on Linux, macOS and Windows.

### Network tests

The tests in `tests/ipv4_tests.rs` and `tests/ipv6_tests.rs` send real sACN between sources and receivers on the local machine. They are ignored by default because they need specific addresses on a network interface, and they must run on a single thread so their sockets don't clash.

The IPv4 tests need the addresses `192.168.0.6`, `192.168.0.7` and `192.168.0.8` on an interface with multicast enabled. `cargo test_ip` is an alias for:

```sh
cargo test --test ipv4_tests -- --ignored --test-threads=1
```

The easiest way to get the right setup is the Docker image in `docker-linux`, which CI also uses. Docker must be running Linux containers.

```sh
docker build -t sacn-test -f docker-linux/Dockerfile .

# IPv4: adds the test addresses to the loopback interface, then runs cargo test and the IPv4 tests.
docker run --cap-add=NET_ADMIN sacn-test

# IPv6: needs a docker network with IPv6 enabled.
docker network create --ipv6 --subnet fd00:5ac0::/64 sacn-ipv6
docker run --cap-add=NET_ADMIN --network sacn-ipv6 sacn-test ./run_tests_ipv6.sh
```

To run them on a Linux host instead, see `docker-linux/run_tests.sh` and `docker-linux/run_tests_ipv6.sh` for the addresses and routes they add. Both need root.

## Continuous integration

`.github/workflows/rust.yml` runs on every pull request to `main`:

* build, Clippy and `cargo test` on Linux, Windows and macOS;
* the IPv4 and IPv6 network tests in the Docker image on Linux.

## Other testing material

* `documentation/` holds the ANSI E1.31-2018 compliance checklist and design notes from the original project, as PDFs.
* `further testing/` holds fuzzing inputs and notes, and scripts for manual tests with several sources and receivers.
* `examples/demo_src` and `examples/demo_rcv` are interactive programs for testing against other sACN equipment. See [Demo Implementation](README.md#demo-implementation).
