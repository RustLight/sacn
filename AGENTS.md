# AGENTS.md

Guidance for AI coding agents working on this repository. Read [CONTRIBUTING.md](CONTRIBUTING.md) first: it covers prerequisites, building, the documentation, the tests and CI. This file adds the rules that agents should follow on top of it.

## Project

`sacn` is a Rust implementation of ANSI E1.31 (Streaming ACN, "sACN"), a protocol for sending DMX512 data over IP networks. The crate provides a source (sender) and a receiver, with universe synchronisation and universe discovery.

## E1.31 compliance

* Evaluate every change against the E1.31 specification. The current edition is [E1.31-2016](https://tsp.esta.org/tsp/documents/docs/E1-31-2016.pdf). If the PDF can't be fetched, `documentation/ANSI-E1.31-2018-Compliance-Check-List.pdf` paraphrases each clause (read it with `pdftotext`).
* In pull requests that change protocol behaviour, name the spec sections involved (for example "E1.31 §6.2.6") and say how the change meets them.
* Where the spec leaves a choice to the implementation, or is silent, say so and ask the maintainers before picking a behaviour.

## Code

* Code and doc comments explain the behaviour itself or cite the E1.31 section that requires it. Don't justify behaviour by pointing at another sACN implementation.
* Log only through the crate's internal `log_warn!`, `log_debug!` and `log_trace!` macros, and include "sACN" in each message. See [CONTRIBUTING.md](CONTRIBUTING.md#building).
* Don't break the public API without discussing it first. Some types, such as `SacnError`, are not `#[non_exhaustive]`, so even adding a variant is a breaking change that needs a major version bump.
* `unsafe` code is denied by `.cargo/config.toml`. Don't work around it.
* Keep changes focused. Don't reformat or fix lints in code you aren't otherwise changing; do that in a separate pull request.

## Tests

* Add or update tests with every behaviour change.
* Tests that need real network interfaces go in `tests/ipv4_tests.rs` or `tests/ipv6_tests.rs`, marked `#[ignore]` like the existing ones, so `cargo test` keeps working without network setup. Run them in the Docker image described in [CONTRIBUTING.md](CONTRIBUTING.md#network-tests).
* Install the Git hooks described in [CONTRIBUTING.md](CONTRIBUTING.md) with prek. They run the checks on commit and push; don't skip them with `--no-verify`. Don't add new Clippy or rustdoc warnings.

## Documentation

Update the documentation in the same pull request as the behaviour it describes:

* `src/lib.rs`: the crate-level getting started guide, and its compiled examples.
* `README.md`: features, usage, and what the library leaves to the application.
* `CONTRIBUTING.md`: building, testing and CI.

## Git and pull requests

* Use descriptive branch names with a type prefix, for example `fix/no-empty-universe-pages` or `chore/update-uuid`.
* Write commit messages and pull request titles in [Conventional Commits](https://www.conventionalcommits.org) format, always with a scope naming the area changed, for example `fix(sender): do not send empty universe discovery pages` or `chore(deps): update uuid to 1.16`, not `fix: ...`. Common scopes are `sender`, `receiver`, `packet`, `docs`, `ci`, `deps` and `cargo`.
* Mark a breaking change with `!` after the scope, for example `fix(cargo)!: require Rust 1.88 or later`. A change is breaking if it can break code or builds that depend on the crate: a public API change, a raised minimum supported Rust version, or a behaviour change that existing callers may rely on. release-plz then bumps the minor version (the major version from 1.0) and tags the changelog entry as breaking. cargo-semver-checks only catches public API changes.
* Pull requests are squash-merged and their titles become the changelog entries (see [Releasing](CONTRIBUTING.md#releasing)), so write titles that describe the change for users of the crate.
* Keep pull request branches up to date with `main` by rebasing, not merging, and push with `--force-with-lease`.
* One pull request per fix or feature. Split unrelated findings into separate pull requests, and say in the body when one depends on another.
* The pull request body holds only content useful to a reviewer: what changed, why, the spec sections involved, and how it was tested. Leave out tool attribution such as "Generated with ..." footers or session links.
* Ask before making a design decision the maintainers haven't settled, rather than picking one and opening a pull request.
