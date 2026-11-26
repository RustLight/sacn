# Dockerfile
#
# Two stages:
# - `env`: the test environment without the sources: Rust at the version in
#   rust-toolchain.toml, iproute2 and cargo-llvm-cov. CI publishes it to the
#   GitHub Container Registry (see docker-linux/ci_image.sh) and runs the tests
#   from the checkout, mounted into the container.
# - the default stage adds the sources and builds the tests, for running them
#   locally with `docker run`, see CONTRIBUTING.md.
FROM rust:1-trixie AS env

LABEL org.opencontainers.image.source="https://github.com/RustLight/sacn"
LABEL org.opencontainers.image.description="Test environment for the sacn network tests"

SHELL ["/bin/bash", "-c"]

# ip, used by run_isolated.sh to set up each test's network namespace
RUN apt-get update && apt-get install iproute2 -y && rm -rf /var/lib/apt/lists/*

# Install the pinned toolchain and cargo-llvm-cov (for run_coverage.sh) in a
# directory with rust-toolchain.toml, so they are for the pinned Rust.
WORKDIR /work
COPY rust-toolchain.toml ./
RUN rustup toolchain install && \
    rustup component add llvm-tools-preview && \
    cargo install cargo-llvm-cov --locked --version 0.9.1 && \
    rm -rf /usr/local/cargo/registry

FROM env

COPY Cargo.toml Cargo.lock ./
COPY examples/ examples/
COPY tests/ tests/
COPY src/ src/

# build tests to warm cache
RUN cargo test --no-run --locked

COPY docker-linux/run_tests.sh docker-linux/run_tests_ipv6.sh docker-linux/run_isolated.sh docker-linux/run_coverage.sh ./
RUN chmod +x run_tests.sh run_tests_ipv6.sh run_isolated.sh run_coverage.sh

# default command runs the IPv4 multicast suite; run ./run_tests_ipv6.sh for the IPv6 suite
CMD ["./run_tests.sh"]
