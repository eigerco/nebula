package main

import (
	"flag"
	"fmt"
	"log"
	"strings"

	"github.com/stellar/go/clients/horizonclient"
	"github.com/stellar/go/keypair"
	"github.com/stellar/go/network"
	"github.com/stellar/go/txnbuild"
)

func main() {

	var issuerSeed = flag.String("i", "", "The asset issuer seed. i.e: SDR4C2CKNCVK4DWMTNI2IXFJ6BE3A6J3WVNCGR6Q3SCMJDTSVHMJGC6U")
	var distributorSeed = flag.String("d", "", "The asset distributor seed. i.e: SBUW3DVYLKLY5ZUJD5PL2ZHOFWJSVWGJA47F6FLO66UUFZLUUA2JVU5U")
	var otherReceiversTrustLines = flag.String("r", "", "Other possible receivers seeds of the asset. Comma separated list of seeds. i.e: SBUW...,SBUW3..")

	flag.Parse()

	if *issuerSeed == "" || *distributorSeed == "" || *otherReceiversTrustLines == "" {
		flag.PrintDefaults()
		log.Fatal("params needed.")
	}

	client := horizonclient.DefaultTestNetClient

	// Keys for accounts to issue and distribute the new asset.
	issuerKeyPair, err := keypair.ParseFull(*issuerSeed)
	mustBeOK(err)

	distributorKeyPair, err := keypair.ParseFull(*distributorSeed)
	mustBeOK(err)

	signingPairs := []*keypair.Full{
		issuerKeyPair, distributorKeyPair,
	}

	request := horizonclient.AccountRequest{AccountID: issuerKeyPair.Address()}
	issuerAccount, err := client.AccountDetail(request)
	mustBeOK(err)

	// Create an object to represent the new asset
	nft_code := "EigerNFT"
	nft := txnbuild.CreditAsset{Code: nft_code, Issuer: issuerKeyPair.Address()}

	threshold := txnbuild.Threshold(0)
	tx_ops := []txnbuild.Operation{
		// First, the receiving (distribution) account must trust the asset from the
		// issuer.
		&txnbuild.ChangeTrust{
			Line: txnbuild.ChangeTrustAssetWrapper{
				Asset: nft,
			},
			SourceAccount: distributorKeyPair.Address(),
		},
		// Second, the issuing account actually sends distribution payment using the asset
		// and sets other NFT related operations.
		&txnbuild.Payment{
			Destination:   distributorKeyPair.Address(),
			Asset:         nft,
			Amount:        "0.0000001",
			SourceAccount: issuerKeyPair.Address(),
		},
		&txnbuild.ManageData{
			Name:          "nftsource",
			Value:         []byte("https://www.eiger.co"),
			SourceAccount: issuerKeyPair.Address(),
		},
		&txnbuild.SetOptions{
			MasterWeight:  &threshold,
			SourceAccount: issuerKeyPair.Address(),
		},
	}

	// Create needed trust lines, so other receivers can receive the asset.
	for _, trustLine := range strings.Split(*otherReceiversTrustLines, ",") {

		keyPair, err := keypair.ParseFull(trustLine)
		mustBeOK(err)

		tx_ops = append(tx_ops, &txnbuild.ChangeTrust{
			Line: txnbuild.ChangeTrustAssetWrapper{
				Asset: nft,
			},
			SourceAccount: keyPair.Address(),
		})

		signingPairs = append(signingPairs, keyPair)
	}

	issuerSourceAccount := txnbuild.NewSimpleAccount(issuerAccount.AccountID, issuerAccount.Sequence)
	tx, err := txnbuild.NewTransaction(
		txnbuild.TransactionParams{
			SourceAccount:        &issuerSourceAccount,
			IncrementSequenceNum: true,
			Preconditions: txnbuild.Preconditions{
				TimeBounds: txnbuild.NewInfiniteTimeout(),
			},
			BaseFee:    txnbuild.MinBaseFee,
			Operations: tx_ops,
		},
	)
	mustBeOK(err)

	signedTx, err := tx.Sign(network.TestNetworkPassphrase, signingPairs...)
	mustBeOK(err)

	_, err = client.SubmitTransaction(signedTx)
	mustBeOK(err)

	//fmt.Printf("Transaction ID: %s", resultTx.ID)

	// We output the <nft_code>:<issuer_address> so later we can use this in the cli
	// in order to get the NFT contract address:
	//
	// soroban lab token id --asset EigerNFT:GCGZQXCUN2JLI324QQEHUY5NLAXFPMSFJKCPGAM3UDR56EYT6DTAMVI6 --network testnet
	fmt.Printf("%s:%s", nft_code, issuerKeyPair.Address())
}

func mustBeOK(err error) {
	if err != nil {
		log.Fatal(err)
	}
}
