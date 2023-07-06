export class CodeGen {
    public generateHeader(author: String, license: String) {
        let header = '';
        if (author !== '') {
            header = `//author: ${author}\n`;
        }
        if (license !== '') {
            header += `//license: ${license}\n`;
        }
        header += '\n#![no_std]\n';
        header += '\nuse soroban_sdk::{contractimpl, Env};\n';

        return header;
    }

    public generateImports(imports: Array<String>) {
        let importsStr = '';
        for (let imp of imports) {
            importsStr += `#[import]\nmod ${imp} {}\n\n`;
        }
        return importsStr;
    }

    public generateContract(name: String, params: Array<String>) {
        let contractCode = '';
        contractCode = `pub struct ${name};\n`;
        contractCode += '\n#[contractimpl]\n';
        contractCode += `impl ${name} {`;
        contractCode += `
    pub fn init(Env env) {
        let contract_id = env.register_contract(None, Lottery::WASM);
        let client = Lottery::Client::new(&env, contract_id);`

        contractCode += `
        client.init(`;
        for (let param of params) {
            contractCode += `${param}, `;
        }
        //remove last comma
        if (params.length !== 0)
            contractCode = contractCode.slice(0, -2);

        contractCode += `);
    }\n`;
        contractCode += '}\n\n';

        return contractCode;
    }
}