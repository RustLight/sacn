# Developing the sacn library

This guide covers building the library from source and running its tests. To use the library in your own project, see [Installation](README.md#installation) in the README.

## Prerequisites

* [rustup](https://rustup.rs). It installs the Rust version pinned in `rust-toolchain.toml`, with Clippy and rustfmt, the first time you run `cargo` in the repository. The crate itself supports Rust 1.88 or later (`rust-version` in `Cargo.toml`), which the `msrv` CI job checks. The tests and dev-dependencies can need a newer Rust.
* Git.
* For the network integration tests: Linux with root access, or Docker (see [Network tests](#network-tests)).

## Building

```sh
git clone https://github.com/RustLight/sacn.git
cd sacn
cargo build
```

`.cargo/config.toml` turns on a standard set of Clippy lints and denies `unsafe` code for every build in the repository. `rust-toolchain.toml` pins the Rust version used for development and CI, and rustup installs it automatically. CI fails on any Clippy warning or unformatted code, so check your changes with:

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo clippy --all-targets --no-default-features --features std -- -D warnings
```

To move to a newer Rust release, change the version in `rust-toolchain.toml`, run the commands above, and commit the version change together with any fixes for new warnings.

The last command checks the build without the `logging` feature. Log messages go through the crate's internal `log_warn!`, `log_debug!` and `log_trace!` macros, never `println!` or the `log` macros directly, so the code still builds when the feature is off. Start each message with or include "sACN" so it stands out in an application's logs.

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

The network interface tests (the `interface_selection` module of `tests/ipv4_tests.rs` and `sacn_ipv6_interface_test` in `tests/ipv6_tests.rs`) send on chosen interfaces. Besides the addresses above, they need multicast enabled on the loopback interface and a second interface with an IPv4 address and multicast; the IPv6 test needs the IPv6 test addresses on an interface with an IPv6 multicast route. `docker-linux/run_isolated.sh` gives each test namespace a veth interface for this.

The easiest way to get the right setup is the Docker image in `docker-linux`, which CI also uses. Docker must be running Linux containers.

```sh
docker build -t sacn-test -f docker-linux/Dockerfile .

# IPv4: runs the IPv4 tests. SYS_ADMIN lets each test run in its own network namespace.
docker run --cap-add=NET_ADMIN --cap-add=SYS_ADMIN sacn-test

# IPv6: needs a docker network with IPv6 enabled.
docker network create --ipv6 --subnet fd00:5ac0::/64 sacn-ipv6
docker run --cap-add=NET_ADMIN --cap-add=SYS_ADMIN --network sacn-ipv6 sacn-test ./run_tests_ipv6.sh
```

CI doesn't build the image on every run. Its `env` stage, the test environment without the sources, is published to the GitHub Container Registry as `ghcr.io/rustlight/sacn/test-env`, tagged with a hash of `docker-linux/Dockerfile` and `rust-toolchain.toml`. The test jobs pull it and run the tests from the checkout; when either file changes, `docker-linux/ci_image.sh` builds the new image and pushes it. Pull requests from forks can't push packages, so they build it in each job.

The scripts run the network tests in parallel, one network namespace per test so their sockets can't clash, as many at once as there are CPU cores (set `SACN_TEST_JOBS` to change that). To run them on a Linux host instead, run `docker-linux/run_tests.sh` or `docker-linux/run_tests_ipv6.sh` as root; `docker-linux/run_isolated.sh` sets up each namespace's addresses and routes.

### Coverage

[cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) measures which lines `cargo test` runs. Install it once, then run the tests with coverage and open the HTML report:

```sh
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov --locked
cargo llvm-cov --open
```

That measures `cargo test` only. CI also measures the [network tests](#network-tests): the Linux job records the coverage of `cargo test`, the IPv4 and IPv6 jobs run their tests through `docker-linux/run_coverage.sh` in the Docker image, which includes cargo-llvm-cov, and the coverage job merges what the three recorded. To measure all of them locally in one report, mount the repository at the same path in the container, so the reports land in `target/llvm-cov/` with paths that match your checkout:

```sh
docker build -t sacn-test -f docker-linux/Dockerfile .
docker network create --ipv6 --subnet fd00:5ac0::/64 sacn-ipv6
docker run --cap-add=NET_ADMIN --cap-add=SYS_ADMIN --network sacn-ipv6 \
    -v "$PWD:$PWD" -w "$PWD" sacn-test ./docker-linux/run_coverage.sh unit ipv4 ipv6
```

The files in `target/` are then owned by root; take them back with `sudo chown -R "$(id -u):$(id -g)" target`. Doctests aren't measured, because cargo-llvm-cov needs nightly Rust for them. CI publishes the report for `main` at <https://rustlight.github.io/sacn/>.

## Git hooks

`.pre-commit-config.yaml` defines hooks that run the checks from CI on your machine. Install [prek](https://prek.j178.dev) (or pre-commit), then enable them once:

```sh
prek install
```

Before each commit they check formatting with `cargo fmt --check`, run Clippy with warnings denied and `cargo test` (including the doctests), and build the documentation with warnings denied. They also lint changed Markdown files with rumdl (configured in `.rumdl.toml`), check their web links with lychee (configured in `lychee.toml`), and lint changed workflows with actionlint and audit them with zizmor. Run every hook on the whole repository with `prek run --all-files`.

## Continuous integration

`.github/workflows/rust.yml` runs on every pull request. A first job looks at which files changed, so the other jobs only run when they're needed:

* When code changes (`src/`, `tests/`, `examples/`, Cargo files, `docker-linux/` or the workflows): a rustfmt check, then build, Clippy with warnings denied and `cargo test` on Linux, Windows and macOS, the IPv4 and IPv6 network tests in the Docker image on Linux, a job that merges the [coverage](#coverage) recorded by the Linux test jobs, a check that the library builds with the minimum supported Rust version, and a check of the workflow lockfile. Cargo runs with `--locked`, so update `Cargo.lock` alongside `Cargo.toml`.
* When code or Markdown changes: the documentation build with warnings denied, and the doctests.
* When Markdown or the workflows change: the rumdl, lychee, actionlint and zizmor hooks from `.pre-commit-config.yaml`.

Every action the workflows use is pinned in `.github/workflows/actions.lock`. After changing a `uses:` line, regenerate it with [`gh actions-lock`](https://github.com/github/gh-actions-lock) (`gh extension install github/gh-actions-lock`) and commit it with the workflow change.

The coverage job's summary lists the coverage of each file and, from [diff-cover](https://github.com/Bachmann1234/diff_cover), of the lines the pull request changes. Its full HTML report is attached to the workflow run as the `coverage-report` artifact. Coverage is reported, not enforced.

Pushes to `main` run everything, and also publish the coverage report to GitHub Pages along with the data for the README's coverage badge. The `all-checks` job passes only when every job above passed or was skipped, and is the check that branch protection requires.

## Releasing

Releases are automated with [release-plz](https://release-plz.dev), configured in `release-plz.toml` and run by `.github/workflows/release-plz.yml`.

Pull requests are squash-merged, so each one becomes a single commit on `main` whose message is the pull request title. Write titles in [Conventional Commits](https://www.conventionalcommits.org) format with a scope, such as `fix(receiver): ...`, because they become the changelog entries: `feat` goes under "Added", `fix` under "Fixed", `perf` under "Performance", `docs` under "Documentation" and `chore(deps)` under "Dependencies". `ci`, `test`, `style`, `build` and other `chore` changes are left out of the changelog, unless they are marked as breaking with `!`.

Mark a breaking change with `!` after the scope, such as `fix(cargo)!: require Rust 1.88 or later`. Breaking changes include public API changes, a raised minimum supported Rust version, and behaviour changes that existing users may rely on. release-plz then bumps the minor version (the major version from 1.0) and tags the changelog entry as breaking. cargo-semver-checks detects public API breaks even when a title isn't marked, but not the other kinds.

On every push to `main` that includes a `feat`, `fix`, `perf`, `docs`, `refactor` or `revert` change, or any change marked breaking, release-plz opens or updates a release pull request. Other changes, such as `ci` or `chore(deps)`, wait for the next release. Only the files listed under `include` in `Cargo.toml` are published. It sets the next version in `Cargo.toml` and `Cargo.lock`, following the Cargo SemVer rules (on 0.x, a breaking change bumps the minor version and anything else the patch version), and checks the public API with [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks). It also adds the new section to `CHANGELOG.md`, crediting each pull request's author. Edit the pull request to reword the changelog or to pick a different version.

Merging the release pull request makes the release: release-plz publishes the crate to crates.io, pushes the `vX.Y.Z` tag and creates the GitHub Release with the changelog section as its notes. Nothing is published until then.

### One-time setup

Maintainers set these up once in the repository and on crates.io:

* A GitHub App with read and write access to "Contents" and "Pull requests", installed on the repository. Its tokens let the release pull request run CI, which the default `GITHUB_TOKEN` can't. Add the App as a bypass actor on the tag ruleset so it can push release tags.
* A `release` deployment environment (Settings, Environments) with the App's client ID as the variable `RELEASE_APP_CLIENT_ID` and its private key as the secret `RELEASE_APP_PRIVATE_KEY`.
* [Trusted publishing](https://crates.io/docs/trusted-publishing) for the crate on crates.io, for the `RustLight/sacn` repository, the `release-plz.yml` workflow and the `release` environment. No crates.io token is stored in the repository.
* Squash merging, with the pull request title as the default commit message, as the only merge method.

## Other testing material

* `documentation/` holds the ANSI E1.31-2018 compliance checklist and design notes from the original project, as PDFs.
* `further testing/` holds fuzzing inputs and notes, and scripts for manual tests with several sources and receivers.
* `examples/demo_src` and `examples/demo_rcv` are interactive programs for testing against other sACN equipment. See [Examples](README.md#examples).
