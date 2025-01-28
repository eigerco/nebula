import { Address, TransactionBuilder, xdr } from "@stellar/stellar-sdk";
import * as StellarSdk from "@stellar/stellar-sdk";
import { StellarDefaults } from "./invoke";

export interface ContractDeploy extends StellarDefaults {
  wasm: Buffer;
  source: StellarSdk.Account;
}

export function deployContract(deploy: ContractDeploy, fee = "100") {
  const { networkPassphrase, source, wasm, auth } = deploy;
  return new TransactionBuilder(source, { fee })
    .setNetworkPassphrase(networkPassphrase)
    .setTimeout(StellarSdk.TimeoutInfinite)
    .addOperation(
      StellarSdk.Operation.invokeHostFunction({
        func: xdr.HostFunction.hostFunctionTypeCreateContract(
          new xdr.CreateContractArgs({
            contractIdPreimage,
            executable: xdr.ContractExecutable.contractExecutableWasm(wasm),
          })
        ),
        auth: auth ?? [],
      })
    )
    .build();
}
