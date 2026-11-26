# Dockerfile
FROM rust:1-trixie

SHELL ["/bin/bash", "-c"]

# optional: tools for quick network debug
RUN apt-get update && apt-get install iproute2 -y && rm -rf /var/lib/apt/lists/*

WORKDIR /work
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
# (if workspace, copy all manifest files here instead)
COPY examples/ examples/
COPY tests/ tests/
COPY src/ src/

# build tests to warm cache
RUN cargo test --no-run --locked

# Build with --build-arg COVERAGE=1 for run_coverage.sh, which needs cargo-llvm-cov.
ARG COVERAGE=
RUN if [[ -n "$COVERAGE" ]]; then \
        rustup component add llvm-tools-preview && \
        cargo install cargo-llvm-cov --locked --version 0.9.1; \
    fi

COPY docker-linux/run_tests.sh docker-linux/run_tests_ipv6.sh docker-linux/run_isolated.sh docker-linux/run_coverage.sh ./
RUN chmod +x run_tests.sh run_tests_ipv6.sh run_isolated.sh run_coverage.sh

# RUN cargo test

# default command runs the IPv4 multicast suite; run ./run_tests_ipv6.sh for the IPv6 suite
CMD ["./run_tests.sh"]