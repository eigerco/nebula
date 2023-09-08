# Publishing contracts
Nebula provides a way of publishing and importing Soroban contracts.
Using OCI registries we provide a consistent way of managing contracts

## Basic

The provided binary can be used to push to any OCI registry eg Github Packages which we use.

```bash
 nebula-publish --module test.wasm --image ghcr.io/eigerco/nebula/contracts/test --username <....> --password <....>
 ```

### Github Actions

Since OCI registries allow one to add a tag, you can hook this to releases and offer your contracts for extension.

```bash
 nebula-publish --module test.wasm --image ghcr.io/eigerco/nebula/contracts/test:tag
 ```

## Learn more
https://www.thorsten-hans.com/distribute-webassembly-modules-as-oci-artifacts/

https://github.com/engineerd/wasm-to-oci