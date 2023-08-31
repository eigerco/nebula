#![cfg(test)]

extern crate std;

use super::{GovernanceContract, GovernanceContractClient};

use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Events},
    token, vec, Address, Env, IntoVal, Symbol, Val, Vec,
};

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

#[test]
fn participant_can_join() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(Some(&Address::random(&env)), GovernanceContract);
    let client = GovernanceContractClient::new(&env, &contract_id);

    let token_admin = Address::random(&env);
    let token_addr = &env.register_stellar_asset_contract(token_admin.clone());
    let token_admin_client = token::AdminClient::new(&env, token_addr);
    let token_client = token::Client::new(&env, token_addr);

    let participant_addr = Address::random(&env);
    // Add funds to client address (as participant)
    token_admin_client.mint(&participant_addr, &1000);

    // Init contract
    let curator = Address::random(&env);
    client.init(&curator, &token_admin_client.address);

    // Join the participant (in this case same as client)
    let initial_stake = 200;
    client.join(&participant_addr, &initial_stake);

    // Ensure we check participant is who says.
    assert_auth(
        &env.auths(),
        0,
        participant_addr.clone(),
        client.address.clone(),
        Symbol::new(&env, "join"),
        (participant_addr.clone(), initial_stake).into_val(&env),
    );

    // After joining, the contract should have received participant funds.
    // The participant should have less funds.
    assert_eq!(200, token_client.balance(&contract_id));
    assert_eq!(800, token_client.balance(&participant_addr));

    // A proper joining event should be published.
    let last_event = env.events().all().last().unwrap();
    assert_eq!(
        vec![&env, last_event],
        vec![
            &env,
            (
                contract_id,
                (Symbol::new(&env, "participant_joined"), participant_addr).into_val(&env),
                200i128.into_val(&env)
            ),
        ]
    )
}

fn assert_auth(
    auths: &[(Address, AuthorizedInvocation)],
    idx: usize,
    call_addr: Address,
    auth_addr: Address,
    func: Symbol,
    args: Vec<Val>,
) {
    let auth = auths.get(idx).unwrap();
    assert_eq!(auth.0, call_addr);
    assert_eq!(
        auth.1.function,
        AuthorizedFunction::Contract((auth_addr, func, args))
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn participant_cant_join_without_enough_funds() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(Some(&Address::random(&env)), GovernanceContract);
    let client = GovernanceContractClient::new(&env, &contract_id);

    let token_admin = Address::random(&env);
    let token_addr = &env.register_stellar_asset_contract(token_admin.clone());
    let token_admin_client = token::AdminClient::new(&env, token_addr);

    let participant_addr = Address::random(&env);
    // Add funds to client address (as participant)
    token_admin_client.mint(&participant_addr, &199);

    // Init contract
    let curator = Address::random(&env);
    client.init(&curator, &token_admin_client.address);

    // Join the participant (in this case same as client). Should not have enough funds. We expect panic.
    client.join(&participant_addr, &200);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn participant_cant_join_with_negative_stake() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(Some(&Address::random(&env)), GovernanceContract);
    let client = GovernanceContractClient::new(&env, &contract_id);

    let token_admin = Address::random(&env);
    let token_addr = &env.register_stellar_asset_contract(token_admin.clone());
    let token_admin_client = token::AdminClient::new(&env, token_addr);

    let participant_addr = Address::random(&env);
    // Add funds to client address (as participant)
    token_admin_client.mint(&participant_addr, &199);

    // Init contract
    let curator = Address::random(&env);
    client.init(&curator, &token_admin_client.address);

    // Join the participant (in this case same as client). Should not have enough funds. We expect panic.
    client.join(&participant_addr, &-1);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn participant_cant_join_with_zero_stake() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(Some(&Address::random(&env)), GovernanceContract);
    let client = GovernanceContractClient::new(&env, &contract_id);

    let token_admin = Address::random(&env);
    let token_addr = &env.register_stellar_asset_contract(token_admin.clone());
    let token_admin_client = token::AdminClient::new(&env, token_addr);

    let participant_addr = Address::random(&env);
    // Add funds to client address (as participant)
    token_admin_client.mint(&participant_addr, &199);

    // Init contract
    let curator = Address::random(&env);
    client.init(&curator, &token_admin_client.address);

    // Join the participant (in this case same as client). Should not have enough funds. We expect panic.
    client.join(&participant_addr, &0);
}