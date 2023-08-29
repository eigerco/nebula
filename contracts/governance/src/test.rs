#![cfg(test)]

extern crate std;

use super::{GovernanceContract, GovernanceContractClient};

use soroban_sdk::Env;

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn cannot_be_initialized_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, GovernanceContract);
    let client = GovernanceContractClient::new(&env, &contract_id);

    client.init(&client.address);
    client.init(&client.address);
}
