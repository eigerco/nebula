# Nebula Importer

A simple utility that allows a developer to import soroban contracts. These contracts would need to be published to an OCI registry.

## Getting Started

The importer hooks to the build process of rustc

### Add a build dependency

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

### Create the dependency metadata in `Cargo.toml` with contracts you want to import

```toml
[package.metadata.nebula.imports]
token = "ghcr.io/eigerco/nebula/contracts/token"
voting = "ghcr.io/eigerco/nebula/contracts/voting:latest"
```

### Use the contracts in your lib.rs

```rust
mod contracts {
    include!(concat!(env!("OUT_DIR"), "/nebula_imports.rs"));
}

fn main() {
    let client = contracts::voting::Client::new();
    // .....
}
```

## Read more

https://www.thorsten-hans.com/distribute-webassembly-modules-as-oci-artifacts/
https://github.com/engineerd/wasm-to-oci
