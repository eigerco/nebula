# NFT script generator

This [Go](https://go.dev/doc/install) script facilitates the creation of an NFT token asset in the stellar network for testing purposes.
It does it by:

1. Creating issuer account.
2. Creating distribution account.
3. Stablish trustline  issuer <=> distribution
4. Transfer NFT from issuer => distribution
5. Stablish other trustlines for the accounts that are going to interact with the created NFT.

This script is committed in the repository.

## Usage example

```bash
$ ./asset_test_gen -help

-d string
        The asset distributor seed. i.e: SBUW3DVYLKLY5ZUJD5PL2ZHOFWJSVWGJA47F6FLO66UUFZLUUA2JVU5U
  -i string
        The asset issuer seed. i.e: SDR4C2CKNCVK4DWMTNI2IXFJ6BE3A6J3WVNCGR6Q3SCMJDTSVHMJGC6U
  -r string
        Other possible receivers seeds of the asset. Comma separated list of seeds. i.e: SBUW...,SBUW3..
```

## Why Go ?  

At the time of this write theres no other easier compiled language available for the stellar SDK.

## More information/helpful resources

* https://developers.stellar.org/docs/issuing-assets/how-to-issue-an-asset
* https://www.reddit.com/r/Stellar/comments/m9mklm/stepbystep_creating_an_nft_on_stellar_network/
* https://medium.com/stellar-community/best-practices-for-creating-nfts-on-stellar-5c91e53e9eb9
* https://discordapp.com/channels/897514728459468821/1169987533011173467/1169987533011173467