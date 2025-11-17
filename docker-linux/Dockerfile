# Dockerfile
FROM rust:1-bullseye

SHELL ["/bin/bash", "-c"]

# optional: tools for quick network debug
RUN apt-get update && apt-get install iproute2 -y && rm -rf /var/lib/apt/lists/*

WORKDIR /work
COPY Cargo.toml Cargo.lock ./
# (if workspace, copy all manifest files here instead)
COPY examples/ examples/
COPY tests/ tests/
COPY src/ src/

# build tests to warm cache
RUN cargo test --no-run --locked

COPY docker-linux/run_tests.sh run_tests.sh
RUN chmod +x run_tests.sh

# RUN cargo test

# default command runs your multicast suite; tweak if needed
CMD ["./run_tests.sh"]