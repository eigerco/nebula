// [0] string
//         The asset distributor seed. i.e: SBUW3DVYLKLY5ZUJD5PL2ZHOFWJSVWGJA47F6FLO66UUFZLUUA2JVU5U
// [1] string
//         The asset issuer seed. i.e: SDR4C2CKNCVK4DWMTNI2IXFJ6BE3A6J3WVNCGR6Q3SCMJDTSVHMJGC6U
// [2] string
//         Other possible receivers seeds of the asset. Comma separated list of seeds. i.e: SBUW...,SBUW3..
const args = process.argv.slice(2);

const StellarSdk = require("stellar-sdk");
const server = new StellarSdk.Server("https://horizon-testnet.stellar.org");
// // Keys for accounts to issue and receive the new asset
const issuerKeyPair = StellarSdk.Keypair.fromSecret(args[0]);
const distributorKeyPair = StellarSdk.Keypair.fromSecret(args[1]);
const otherReceiversTrustLines = (args[2] || "").split(",");
const NFT_CODE = "EigerNFT";

const signingPairs = [issuerKeyPair, distributorKeyPair];
// Create an object to represent the new asset
const eigerNft = new StellarSdk.Asset(NFT_CODE, issuerKeyPair.publicKey());
async function main() {
  const fee = await server.fetchBaseFee();
  server
    .loadAccount(issuerKeyPair.publicKey())
    .then(function (receiver) {
      let transaction = new StellarSdk.TransactionBuilder(receiver, {
        networkPassphrase: StellarSdk.Networks.TESTNET,
        fee,
      })
        .addOperation(
          StellarSdk.Operation.manageData({
            name: "nftsource",
            value: "https://www.eiger.co",
            source: issuerKeyPair.publicKey(),
          })
        )
        .addOperation(
          StellarSdk.Operation.setOptions({
            masterWeight: 0,
            source: issuerKeyPair.publicKey(),
          })
        )
        .addOperation(
          StellarSdk.Operation.changeTrust({
            asset: eigerNft,
            limit: "0.0000001",
            source: distributorKeyPair.publicKey(),
          })
        )
        .addOperation(
          StellarSdk.Operation.payment({
            destination: distributorKeyPair.publicKey(),
            asset: eigerNft,
            amount: "0.0000001",
            source: issuerKeyPair.publicKey(),
          })
        );
      for (const line of otherReceiversTrustLines) {
        const curKeyPair = StellarSdk.Keypair.fromSecret(line);
        transaction = transaction.addOperation(
          StellarSdk.Operation.changeTrust({
            source: curKeyPair.publicKey(),
            asset: eigerNft,
          })
        );
        signingPairs.push(curKeyPair);
      }
      transaction = transaction
        // // setTimeout is required for a transaction
        .setTimeout(30)
        .build();
      for (const pair of signingPairs) {
        transaction.sign(pair);
      }

      return submitTransaction(transaction);
    })
    .catch(console.error);
}

async function submitTransaction(transaction) {
  try {
    await server.submitTransaction(transaction);
    console.log(`${NFT_CODE}:${issuerKeyPair.publicKey()}`);
  } catch (error) {
    throw error;
  }
}

main();
