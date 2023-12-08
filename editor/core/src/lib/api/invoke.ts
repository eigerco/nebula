import { Address, TransactionBuilder, xdr } from "@stellar/stellar-sdk";
import * as StellarSdk from "@stellar/stellar-sdk";

export interface ContractInvoke extends StellarDefaults {
  scAddress: xdr.ScAddress;
  source: StellarSdk.Account;
  
}

export interface StellarDefaults {
    auth: xdr.SorobanAuthorizationEntry[];
    networkPassphrase: string;
}

export function singleContractInvokeTransaction(
  contract: ContractInvoke,
  functionName: string,
  args: xdr.ScVal[],
  fee = "100"
) {
  const { source, scAddress, auth, networkPassphrase } = contract;
  return new TransactionBuilder(source, { fee })
    .setNetworkPassphrase(networkPassphrase)
    .setTimeout(StellarSdk.TimeoutInfinite)
    .addOperation(
      StellarSdk.Operation.invokeHostFunction({
        func: xdr.HostFunction.hostFunctionTypeInvokeContract(
          new xdr.InvokeContractArgs({
            contractAddress: scAddress,
            functionName,
            args,
          })
        ),
        auth: auth ?? [],
      })
    )
    .build();
}
