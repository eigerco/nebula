# Nebula Importer

A simple utility that allows developers to import Soroban contracts into other contracts. These contracts would need to be published to an OCI registry.
The current cross contract solution offered by Soroban only works on local files.
This extends on that and allows remote import of contracts

## Getting Started

The importer hooks to the build process of rustc.

### Add a build dependency

```toml
[build-dependencies]
nebula = { git = "https://github.com/eigerco/nebula" }
```

### Create a `build.rs`

```rust
fn main() {
    nebula::build::sync_all_contracts();
}
```

### Add import metadata in `Cargo.toml`

```toml
[package.metadata.nebula.imports]
token = "ghcr.io/eigerco/nebula/contracts/token"
voting = "ghcr.io/eigerco/nebula/contracts/voting:latest"
```

### Use the contracts

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
