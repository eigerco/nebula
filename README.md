# nebula
Nebula is a tool for easing development of Soroban smart contracts

## Frontend
The frontend contains some basic starter code.

To run:
```
git clone https://github.com/eigerco/nebula
cd nebula/app
npm install
npm run dev
```

To build for deployment: 
```
npm run build
```
The dist folder will contain the files ready for static serving

## Playground

The playground allows frontend interface to compile a contract.
The output is a downloadable wasm.

To run:

```
cd playground
cargo run
```

## Sample contract
This is the crate that is compiled. The user is only allowed to add a `src/lib.rs` from the frontend.
To increase initial compilation speed, please make sure to compile once:
```
cd sample-contract
cargo build --target wasm32-unknown-unknown --release
```