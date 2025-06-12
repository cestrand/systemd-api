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

## License

Copyright © 2025 Marcin Kolenda  

SPDX-License-Identifier: Apache-2.0 

Licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0).
