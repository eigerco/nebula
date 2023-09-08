## Nebula publish

A small utility for deploying Soroban contracts to Container registry.
Currently only tested on Github

### Running

```bash
 nebula-publish --module test.wasm --image ghcr.io/eigerco/nebula/contracts/test --username <....> --password <....>
```


https://www.thorsten-hans.com/distribute-webassembly-modules-as-oci-artifacts/
https://github.com/engineerd/wasm-to-oci