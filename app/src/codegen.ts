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

    public generateContract(name: String, tokenInterface: boolean, symbol: String = '', decimals: number = 1) {
        let contractCode = '';
        contractCode = `pub struct ${name};\n`;
        contractCode += '\n#[contractimpl]\n';
        contractCode += `impl ${name} {`;
        contractCode += `
    pub fn init(Env env) {;
    }\n`;

        if (tokenInterface) {
            contractCode += this.generateTokenInterface(name, symbol, decimals);
        }
        contractCode += '}\n';

        return contractCode;
    }

    private generateTokenInterface(name: String, symbol: String, decimals: number) {
        let tokenInterfaceCode = '';
        tokenInterfaceCode = `
    fn clawback(
        env: soroban_sdk::Env,
        from: Address,
        amount: i128
    ) {
    }\n`;

        tokenInterfaceCode += `
    fn mint(
        env: soroban_sdk::Env,
        to: Address,
        amount: i128
    ) {
    }\n`;

        tokenInterfaceCode += `
    fn set_authorized(
        env: soroban_sdk::Env,
        id: Address,
        authorized: bool
    ) {
    }\n`;

        tokenInterfaceCode += `
    fn increase_allowance(
        env: soroban_sdk::Env,
        from: Address,
        spender: Address,
        amount: i128
    ) {
    }\n`;

        tokenInterfaceCode += `
    fn decrease_allowance(
        env: soroban_sdk::Env,
        from: Address,
        spender: Address,
        amount: i128
    ) {
    }\n`;

        tokenInterfaceCode += `
    fn transfer(
        env: soroban_sdk::Env,
        from: Address,
        to: Address,
        amount: i128
    ) {
    }\n`;

        tokenInterfaceCode += `
    fn transfer_from(
        env: soroban_sdk::Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128
    ) {
    }\n`;

        tokenInterfaceCode += `
    fn burn(
        env: soroban_sdk::Env,
        from: Address,
        amount: i128
    ) {
    }\n`;

        tokenInterfaceCode += `
    fn burn_from(
        env: soroban_sdk::Env,
        spender: Address,
        from: Address,
        amount: i128
    ) {
    }\n`;

        tokenInterfaceCode += `
    fn balance(env: soroban_sdk::Env, id: Address) -> i128 {
    }\n`;

        tokenInterfaceCode += `
    fn spendable_balance(env: soroban_sdk::Env id: Address) -> i128 {
    }\n`;

        tokenInterfaceCode += `
    fn authorized(env: soroban_sdk::Env, id: Address) -> bool {
    }\n`;

        tokenInterfaceCode += `
    fn decimals(env: soroban_sdk::Env) -> u32 {
    \treturn ${decimals};
    }\n`;

        tokenInterfaceCode += `
    fn name(env: soroban_sdk::Env) -> soroban_sdk::Bytes {
        return "${name}";
    }\n`;

        tokenInterfaceCode += `
    fn symbol(env: soroban_sdk::Env) -> soroban_sdk::Bytes {
        return "${symbol}";
    }\n`;

        return tokenInterfaceCode;
    }
}