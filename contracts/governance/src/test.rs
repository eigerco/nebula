#![cfg(test)]

extern crate std;

use super::{GovernanceContract, GovernanceContractClient};

use soroban_sdk::{testutils::Address as _, token, Address, Env};

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn cannot_be_initialized_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, GovernanceContract);
    let client = GovernanceContractClient::new(&env, &contract_id);
    let token_admin = Address::random(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    client.init(&client.address, &test_token_client.address);
    client.init(&client.address, &test_token_client.address);
}

fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::AdminClient<'a> {
    token::AdminClient::new(e, &e.register_stellar_asset_contract(admin.clone()))
}
