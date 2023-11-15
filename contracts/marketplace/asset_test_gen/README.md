# Stellar Asset Generator

1. Creating issuer account.
2. Creating distribution account.
3. Stablish trustline  issuer <=> distribution
4. Transfer NFT from issuer => distribution
5. Stablish other trustlines for the accounts that are going to interact with the created NFT.

This script is committed in the repository.

## Usage example

```bash
$ node asset_test_gen.js [asset distributor seed] [asset issuer seed] [ossible receivers]
        The asset distributor seed. i.e: SBUW3DVYLKLY5ZUJD5PL2ZHOFWJSVWGJA47F6FLO66UUFZLUUA2JVU5U
        The asset issuer seed. i.e: SDR4C2CKNCVK4DWMTNI2IXFJ6BE3A6J3WVNCGR6Q3SCMJDTSVHMJGC6U
        Other possible receivers seeds of the asset. Comma separated list of seeds. i.e: SBUW...,SBUW3..
```
