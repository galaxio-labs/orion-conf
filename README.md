# orion-conf
[![CI](https://github.com/galaxio-labs/orion-conf/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/galaxio-labs/orion-conf/actions)
[![Coverage Status](https://coveralls.io/repos/github/galaxio-labs/orion-conf/badge.svg?branch=main)](https://coveralls.io/github/galaxio-labs/orion-conf?branch=main)
[![crates.io](https://img.shields.io/crates/v/orion-conf.svg)](https://crates.io/crates/orion-conf)
[![Downloads](https://img.shields.io/crates/d/orion-conf.svg)](https://crates.io/crates/orion-conf)
[![docs.rs](https://docs.rs/orion_conf/badge.svg)](https://docs.rs/orion_conf)
[![Dependencies](https://deps.rs/repo/github/galaxio-labs/orion-conf/status.svg)](https://deps.rs/repo/github/galaxio-labs/orion-conf)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Rust 2024 edition](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org)
[![GitHub stars](https://img.shields.io/github/stars/galaxio-labs/orion-conf.svg)](https://github.com/galaxio-labs/orion-conf/stargazers)

Lightweight, feature‑gated config IO for Rust. Enable only the formats you need.

Quick Start

```rust
use orion_conf::{ConfigIO, YamlIO};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct AppCfg { name: String, value: i32 }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // cargo run --features yaml
    let cfg = AppCfg { name: "demo".into(), value: 7 };
    let path = std::path::Path::new("app.yaml");

    cfg.save_conf(path)?;                 // delegates by feature priority (YAML > TOML > JSON > INI)
    let loaded = AppCfg::load_conf(path)?; // same priority for loading

    // Or call the format explicitly
    cfg.save_yaml(path)?;
    let _ = AppCfg::load_yaml(path)?;
    Ok(())
}
```

Features

- Formats (opt‑in): `yaml`, `toml`, `json`, `ini`
- Combined: `formats` (all), `full` (all)
- Default features: empty

Key Traits (0.3+)

- `ConfigIO`: unified read/write with feature‑based priority
- `YamlIO`/`TomlIO`/`JsonIO`/`IniIO`: explicit format IO

Examples

- JSON only: `cargo run --example json_feature_test --features json`
- YAML+JSON priority: `cargo run --example feature_priority_test --features yaml,json`

Migration

- See MIGRATION.md for 0.2.x → 0.3.0 changes and API replacements.
