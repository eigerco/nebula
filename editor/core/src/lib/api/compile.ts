import { Address, TransactionBuilder, xdr } from "@stellar/stellar-sdk";
export interface CompileResult {
    wasm: Buffer,
    functionSpec: xdr.ScSpecEntry[]
}

export function compileContract(code: string) {

}