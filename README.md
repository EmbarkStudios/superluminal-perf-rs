# 🐏 superluminal-perf-rs

[![Build Status](https://github.com/EmbarkStudios/superluminal-perf-rs/workflows/CI/badge.svg)](https://github.com/EmbarkStudios/superluminal-perf-rs/actions?workflow=CI)
[![Crates.io](https://img.shields.io/crates/v/superluminal-perf.svg)](https://crates.io/crates/superluminal-perf)
[![Docs](https://docs.rs/superluminal-perf/badge.svg)](https://docs.rs/superluminal-perf)
[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4%20adopted-ff69b4.svg)](CODE_OF_CONDUCT.md)
[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](http://embark.dev)

Rust-bindings to [Superluminal Performance](https://superluminal.eu/) profiler API for Windows for adding user events to captures.

## How to use

In `Cargo.toml` add:

```toml
[dependencies]
superluminal-perf = "0.1.0"
```

Example usage:

```rust
superluminal_perf::begin_event(b"my-event\0");
calc();
superluminal_perf::end_event();

superluminal_perf::begin_event(b"my-event2\0");
calc2();
superluminal_perf::end_event();
```

On non-Windows platforms the events will compile out.

## Contributing

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Note that Superluminal's Performance C API itself, that is crate uses and embeds, is licensed under BSD.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
