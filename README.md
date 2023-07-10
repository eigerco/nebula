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

## Contracts

Contracts crates are stored in the `contracts` folder, grouped in a workspace.

In order to run the tests:

```
$ cd contracts
$ cargo test
```