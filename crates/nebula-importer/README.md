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
    // TODO: Can we just use OUT_DIR?
    nebula_importer::sync_contracts("Contracts.toml", "./contracts").expect("Could not sync contracts");
}
```

### Create the dependency file eg `Contracts.toml` with imported contracts

```toml
[imports]
token = "ghcr.io/eigerco/nebula/contracts/token"
voting = "ghcr.io/eigerco/nebula/contracts/voting:latest"
```

## Read more
https://www.thorsten-hans.com/distribute-webassembly-modules-as-oci-artifacts/
https://github.com/engineerd/wasm-to-oci
