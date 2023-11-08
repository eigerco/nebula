var StellarSdk = require("stellar-sdk");
var server = new StellarSdk.Server("https://horizon-testnet.stellar.org");

// Keys for accounts to issue and receive the new asset
var issuingKeys = StellarSdk.Keypair.fromSecret(
  "SCZANGBA5YHTNYVVV4C3U252E2B6P6F5T3U6MM63WBSBZATAQI3EBTQ4",
);
var receivingKeys = StellarSdk.Keypair.fromSecret(
  "SDSAVCRE5JRAI7UFAVLE5IMIZRD6N6WOJUWKY4GFN34LOBEEUS4W2T2D",
);

// Create an object to represent the new asset
var astroDollar = new StellarSdk.Asset("AstroDollar", issuingKeys.publicKey());

// First, the receiving account must trust the asset
server
  .loadAccount(receivingKeys.publicKey())
  .then(function (receiver) {
    var transaction = new StellarSdk.TransactionBuilder(receiver, {
      fee: 100,
      networkPassphrase: StellarSdk.Networks.TESTNET,
    })
      // The `changeTrust` operation creates (or alters) a trustline
      // The `limit` parameter below is optional
      .addOperation(
        StellarSdk.Operation.changeTrust({
          asset: astroDollar,
          limit: "1000",
        }),
      )
      // setTimeout is required for a transaction
      .setTimeout(100)
      .build();
    transaction.sign(receivingKeys);
    return server.submitTransaction(transaction);
  })
  .then(console.log)

  // Second, the issuing account actually sends a payment using the asset
  .then(function () {
    return server.loadAccount(issuingKeys.publicKey());
  })
  .then(function (issuer) {
    var transaction = new StellarSdk.TransactionBuilder(issuer, {
      fee: 100,
      networkPassphrase: StellarSdk.Networks.TESTNET,
    })
      .addOperation(
        StellarSdk.Operation.payment({
          destination: receivingKeys.publicKey(),
          asset: astroDollar,
          amount: "10",
        }),
      )
      // setTimeout is required for a transaction
      .setTimeout(100)
      .build();
    transaction.sign(issuingKeys);
    return server.submitTransaction(transaction);
  })
  .then(console.log)
  .catch(function (error) {
    console.error("Error!", error);
  });