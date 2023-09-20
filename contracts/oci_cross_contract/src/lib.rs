mod contracts;

use soroban_sdk::{contract, contractimpl, Address, Env};
#[contract]
pub struct ContractB;

#[contractimpl]
impl ContractB {
    pub fn add_with(env: Env, contract: Address) {
        let client = contracts::voting::Client::new(&env, &contract);
        client.init(&contract, &3600, &50_00, &1000);
    }
}
