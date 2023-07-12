import { Address } from "../contractparams/address"
import { LotteryCodeGen } from "./lotterycodegen";
import { VotingCodeGen } from "./votingcodegen";

export class CodeGen {
  private header = ''
  private imports = ''
  private contractCode = ''

  private toHex(str: string) {
    var result = '';
    for (var i=0; i<str.length; i++) {
      result += str.charCodeAt(i).toString(16);
    }
    return result;
  }

  public setHeader (author: string, license: string) {
    this.header = ''
    if (author !== '') {
      this.header = `//author: ${author}\n`
    }
    if (license !== '') {
      this.header += `//license: ${license}\n`
    }
  }

  public setImports (imports: string[]) {
    this.imports = ''
    for (const imp of imports) {
      this.imports += `#[import]\nmod ${imp} {}\n\n`
    }
  }

  public setContractCode (type: string, name: string, params: any[]) {
    this.contractCode = ''
    if (type === 'Lottery') {
      const lotteryCodeGen = new LotteryCodeGen()
      this.contractCode = lotteryCodeGen.generateCode(name)
    } else if (type === 'Voting') {
      const votingCodeGen = new VotingCodeGen()
      this.contractCode = votingCodeGen.generateCode(name)
    }
//     this.contractCode = `pub struct ${name};\n`
//     this.contractCode += '\n#[contractimpl]\n'
//     this.contractCode += `impl ${name} {`
//     this.contractCode += `
//   pub fn init(Env env) {
//     let contract_id = env.register_contract(None, ${type}::WASM);
//     let client = ${type}::Client::new(&env, contract_id);`


//     if (params.length > 0) {
//       this.contractCode += `
//     client.init(`
//       for (const param of params) {
//         console.log(typeof param)
//         if (param instanceof Address) {
//           this.contractCode += `
//           Address::from_account_id(&env, 0x${this.toHex((<Address> param).value)}), `
//         } else {
//           this.contractCode += `
//           ${param}, `
//         }
//       }
//       // remove last comma
//       if (params.length !== 0) {
//         this.contractCode = this.contractCode.slice(0, -2)
//       }

//       this.contractCode += `);`
//   }
//   this.contractCode += `
//   }
// }\n`
  }

  public generateCode(): string {
    return this.header + '\n' + this.contractCode
  }
}
