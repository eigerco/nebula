# Nebula - A Soroban contract library

<p align="center">
<img src="app/public/logo.png" alt="nebula-logo" width="300"/>
</p>

Nebula is a tool for easing development of [Soroban](https://soroban.stellar.org/docs) smart contracts. By providing pre-built templates, new components and complex contracts that developers can 
use with 0 setup we supercharge the growth of the community and the Soroban ecosystem. Its integrated with a user interface and a set of pre-coded, community audited smart contracts. 
It aims to play in a cohesive way with the existing [Stellar](https://stellar.org/) ecosystem.

## Status of this project

Progress on this project is currently just a demonstration of the [initial](https://github.com/eigerco/nebula/milestone/1) part of the first milestone, which aims to show the essence and 
interoperability of the tools that are planned to be built:

* The [Nebula's UI](#wizard-ui) wizard.
* The [smart contracts](#contracts).

⚠️ Disclaimer: Currently, production usage is discouraged.

## Wizard UI

Currently a live, latest version of the wizard can be found [here](https://nebula.eiger.co/).

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
The .next folder will contain the files ready for static serving.

## Contracts

Nebula provides a set of common contracts for speeding up the deployment of certain common operations. 

Contracts are located [here](contracts/) and intended to be used by the [UI wizard](#wizard-ui). They use a trait based 
approach for maximizing the extension/customization of the contracts.

After [installing rust](https://www.rust-lang.org/tools/install), tests can be run by just:

```bash
$ cd contracts
$ cargo test
```
### Tests in Futurenet

* [Initial raffle contract](https://github.com/eigerco/nebula/issues/5#issuecomment-1644065962)
* [Initial voting contract](https://github.com/eigerco/nebula/issues/5#issuecomment-1645208546)

## About [Eiger](https://www.eiger.co)

We are engineers. We contribute to various ecosystems by building low level implementations and core components. We built these Soroban contracts early in its development to help newcomers to the Stellar ecosystem.
Contact us at hello@eiger.co
Follow us on [X/Twitter](https://x.com/eiger_co)
