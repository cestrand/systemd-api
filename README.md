[![Rust](https://github.com/cestrand/systemd-api/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/cestrand/systemd-api/actions/workflows/rust.yml)
![GitHub commits since tagged version](https://img.shields.io/github/commits-since/cestrand/systemd-api/0.1.0)

# systemd-api

A Rust library and daemon that exposes **systemd** functionality via API.

---

## Features

- [ ] Start, stop, and restart systemd units
- [ ] Query unit status and properties

---

## Usage

### Permissions

Interacting with systemd typically requires elevated privileges. You can:

- run your app as root,
- or configure appropriate PolicyKit rules.

### Library
[![Rust](https://github.com/cestrand/systemd-api/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/cestrand/systemd-api/actions/workflows/rust.yml)

```toml
# Cargo.toml
[dependencies]
systemd-api = { path = "." }
```

## Development

This crate uses a small build script (`build.rs`) to generate Rust
bindings for libsystemd at compile time. The script invokes
[bindgen](https://docs.rs/bindgen) on `wrapper.h`, which simply includes
the systemd headers from `/usr/include/systemd`. The generated bindings
are written to the build output directory and compiled as part of the
crate. For more details on build scripts, see the
[Cargo documentation](https://doc.rust-lang.org/cargo/reference/build-scripts.html).

## License

Copyright © 2025 Marcin Kolenda  

SPDX-License-Identifier: Apache-2.0 

Licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0).
