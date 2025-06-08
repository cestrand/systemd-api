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

```toml
# Cargo.toml
[dependencies]
systemd-api = { path = "." }
```

## License

Copyright © 2025 Marcin Kolenda  

SPDX-License-Identifier: Apache-2.0 

Licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0).
