# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.12.0](https://github.com/RustLight/sacn/compare/v0.11.1...v0.12.0) - 2026-11-26

### Fixed

- [**breaking**] Require Rust 1.88 or later ([#123](https://github.com/RustLight/sacn/pull/123)) by @stewartadam

### Documentation

- *(contributing)* Require a scope and mark breaking changes with ! ([#124](https://github.com/RustLight/sacn/pull/124)) by @stewartadam
- *(readme)* Move badges below the title and add a build badge ([#114](https://github.com/RustLight/sacn/pull/114)) by @claude
- *(readme)* Add crates.io, license, downloads and docs.rs badges ([#113](https://github.com/RustLight/sacn/pull/113)) by @claude
- Add AGENTS.md ([#109](https://github.com/RustLight/sacn/pull/109)) by @claude

### Other

- *(packet)* Remove single-use lifetime macros ([#115](https://github.com/RustLight/sacn/pull/115)) by @claude
- Fix rustdoc warnings and deny them in CI ([#103](https://github.com/RustLight/sacn/pull/103)) by @claude
- Lock CI dependencies: workflow lockfile and cargo --locked ([#102](https://github.com/RustLight/sacn/pull/102)) by @claude
- Terminate preview streams and keep sources alive only on Null START Code ([#99](https://github.com/RustLight/sacn/pull/99)) by @claude
- Start synchronisation before synchronised data in the network tests ([#98](https://github.com/RustLight/sacn/pull/98)) by @claude
- Act on synchronised data until the first sync packet arrives ([#98](https://github.com/RustLight/sacn/pull/98)) by @claude
- Describe discovery paging without referencing other implementations ([#95](https://github.com/RustLight/sacn/pull/95)) by @claude
- Note discovery paging and that sources without universes send none ([#95](https://github.com/RustLight/sacn/pull/95)) by @claude
- Send no empty universe discovery pages ([#95](https://github.com/RustLight/sacn/pull/95)) by @claude
- Fix the build: waiting data is keyed by universe and start code ([#107](https://github.com/RustLight/sacn/pull/107)) by @claude
- Merge main (crate docs from #101) and link the logging docs ([#94](https://github.com/RustLight/sacn/pull/94)) by @claude
- Document when SourceDiscovered fires in the crate docs ([#97](https://github.com/RustLight/sacn/pull/97)) by @claude
- Document per-START-code holding of synchronised data in the guide ([#96](https://github.com/RustLight/sacn/pull/96)) by @claude
- Add getting started guide and installation instructions ([#101](https://github.com/RustLight/sacn/pull/101)) by @claude
- Name sACN in every log message ([#94](https://github.com/RustLight/sacn/pull/94)) by @claude
- Log silent failures, dropped packets and per-packet traces ([#94](https://github.com/RustLight/sacn/pull/94)) by @claude
- Use the same start code in the held-data replacement integration tests ([#96](https://github.com/RustLight/sacn/pull/96)) by @claude
- Use a default-on logging feature backed by log instead of tracing ([#94](https://github.com/RustLight/sacn/pull/94)) by @claude
- Only raise SourceDiscovered when a source is new or changes ([#97](https://github.com/RustLight/sacn/pull/97)) by @claude
- Hold synchronised data per universe and start code ([#96](https://github.com/RustLight/sacn/pull/96)) by @claude
- Use an optional, default-on tracing feature instead of log ([#94](https://github.com/RustLight/sacn/pull/94)) by @claude
- Report periodic source errors through the log facade instead of println ([#94](https://github.com/RustLight/sacn/pull/94)) by @claude
- Merge remote-tracking branch 'origin/main' into claude/project-thread-vi2vxe ([#93](https://github.com/RustLight/sacn/pull/93)) by @claude
- Add IPv4 tests for alternate START code and full-capacity cross-universe sends ([#93](https://github.com/RustLight/sacn/pull/93)) by @claude
- Support the Force_Synchronization option ([#83](https://github.com/RustLight/sacn/pull/83)) by @claude
- Run the IPv6 TTL/loop test in the IPv6 CI job ([#88](https://github.com/RustLight/sacn/pull/88)) by @claude
- Don't store repeated universe discovery pages twice ([#80](https://github.com/RustLight/sacn/pull/80)) by @claude
- Make TTL and multicast loop settings work on IPv6 sources ([#88](https://github.com/RustLight/sacn/pull/88)) by @claude
- Run IPv6 integration tests in an IPv6-enabled docker network ([#92](https://github.com/RustLight/sacn/pull/92)) by @claude
- Keep discovered sources for two discovery intervals ([#79](https://github.com/RustLight/sacn/pull/79)) by @claude
- Bind default sources to an ephemeral port ([#84](https://github.com/RustLight/sacn/pull/84)) by @claude
- Track synchronisation universes separately from data universes ([#91](https://github.com/RustLight/sacn/pull/91)) by @claude
- Add terminate_stream_to for receivers sent data by unicast ([#90](https://github.com/RustLight/sacn/pull/90)) by @claude
- Return an error when the source socket can't be created ([#82](https://github.com/RustLight/sacn/pull/82)) by @claude
- Keep valid packets when another source has timed out ([#81](https://github.com/RustLight/sacn/pull/81)) by @stewartadam
- Pass unpadded source names to SacnSource in IP tests ([#76](https://github.com/RustLight/sacn/pull/76)) by @claude
- Limit source names to 63 bytes so they stay null-terminated ([#76](https://github.com/RustLight/sacn/pull/76)) by @claude
- Reject PDUs whose length is shorter than their fixed fields ([#75](https://github.com/RustLight/sacn/pull/75)) by @stewartadam
- Fix simple-receiver example for short frames and row layout ([#89](https://github.com/RustLight/sacn/pull/89)) by @claude
- Document what E1.31 behaviour is left to the application ([#87](https://github.com/RustLight/sacn/pull/87)) by @claude
- Terminate every universe even if one fails, and don't panic in Drop ([#86](https://github.com/RustLight/sacn/pull/86)) by @claude
- Add backticks in test doc comment for clippy ([#85](https://github.com/RustLight/sacn/pull/85)) by @claude
- Check every source for timeouts when announce_timeout is set ([#85](https://github.com/RustLight/sacn/pull/85)) by @claude
- Don't send data or sync packets on the discovery universe ([#78](https://github.com/RustLight/sacn/pull/78)) by @claude
- Parse only the bytes of the received datagram ([#77](https://github.com/RustLight/sacn/pull/77)) by @claude
- Allow sync packets on unregistered universes ([#73](https://github.com/RustLight/sacn/pull/73)) by @claude
- Raise MSRV to 1.93.1 and update serial_test to 4.x ([#74](https://github.com/RustLight/sacn/pull/74)) by @claude
- Move CI container to Debian trixie and refresh Cargo.lock ([#74](https://github.com/RustLight/sacn/pull/74)) by @claude
- Handle port reuse errors when port reuse isn't allowed on target platform. ([#71](https://github.com/RustLight/sacn/pull/71)) by @DAlexanderNZ
- Clippy suggestions ([#65](https://github.com/RustLight/sacn/pull/65)) by @bruingineer
- Refactor recv to a loop instead of recursion ([#63](https://github.com/RustLight/sacn/pull/63)) by @bruingineer
- Update packet.rs ([#67](https://github.com/RustLight/sacn/pull/67)) by @bruingineer
