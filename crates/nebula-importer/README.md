# Nebula Importer

A simple utility that allows a developer to import soroban contracts.

## Getting Started

The importer hooks to the build process of rustc

### Import the library

```toml
[build-dependencies]
nebula-importer = { git = "https://github.com/eigerco/nebula" }
```

### Create a `build.rs`

```rs
fn main() {
    nebula_importer::sync_all_contracts();
}
```

### Create the dependency metadata in `Cargo.toml` with imported contracts

```toml
[package.metadata.nebula.imports]
token = "ghcr.io/eigerco/nebula/contracts/token"
voting = "ghcr.io/eigerco/nebula/contracts/voting:latest"
```

### Use the contracts in your lib.rs

```rust
mod contracts {
    include!(concat!(env!("OUT_DIR"), "/nebula_importer.rs"));
}

fn main() {
    let client = contracts::voting::Client::new();
    // .....
}
```

## Read more
https://www.thorsten-hans.com/distribute-webassembly-modules-as-oci-artifacts/
https://github.com/engineerd/wasm-to-oci
