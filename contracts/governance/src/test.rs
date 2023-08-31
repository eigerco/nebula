#![cfg(test)]

extern crate std;

use super::{GovernanceContract, GovernanceContractClient};

use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Events},
    token::{self, AdminClient, Client},
    vec, Address, Env, IntoVal, Symbol, Val, Vec,
};

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn cannot_be_initialized_twice() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    sc.contract_client
        .init(&sc.contract_client.address, &sc.token_client.address);
    sc.contract_client
        .init(&sc.contract_client.address, &sc.token_client.address);
}

fn setup_scenario<'a>() -> Scenario<'a> {
    let env = Env::default();

    let contract_id = env.register_contract(Some(&Address::random(&env)), GovernanceContract);
    let contract_client = GovernanceContractClient::new(&env, &contract_id);

    let token_admin = Address::random(&env);
    let token_addr = env.register_stellar_asset_contract(token_admin.clone());
    let token_admin_client = token::AdminClient::new(&env, &token_addr);
    let token_client = token::Client::new(&env, &token_addr);

    Scenario {
        env,
        contract_id,
        contract_client,
        token_admin,
        token_addr,
        token_client,
        token_admin_client,
    }
}

#[allow(dead_code)]
struct Scenario<'a> {
    env: Env,
    contract_id: Address,
    contract_client: GovernanceContractClient<'a>,
    token_admin: Address,
    token_addr: Address,
    token_client: Client<'a>,
    token_admin_client: AdminClient<'a>,
}

#[test]
fn participant_can_join() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    let participant_addr = Address::random(&sc.env);
    // Add funds to client address (as participant)
    sc.token_admin_client.mint(&participant_addr, &1000);

    // Init contract
    let curator = Address::random(&sc.env);
    sc.contract_client
        .init(&curator, &sc.token_admin_client.address);

    // Join the participant (in this case same as client)
    let initial_stake = 200;
    sc.contract_client.join(&participant_addr, &initial_stake);

    // Ensure we check participant is who says.
    assert_auth(
        &sc.env.auths(),
        0,
        participant_addr.clone(),
        sc.contract_client.address.clone(),
        Symbol::new(&sc.env, "join"),
        (participant_addr.clone(), initial_stake).into_val(&sc.env),
    );

    // After joining, the contract should have received participant funds.
    // The participant should have less funds.
    assert_eq!(200, sc.token_client.balance(&sc.contract_id));
    assert_eq!(800, sc.token_client.balance(&participant_addr));

    // A proper joining event should be published.
    let last_event = sc.env.events().all().last().unwrap();
    assert_eq!(
        vec![&sc.env, last_event],
        vec![
            &sc.env,
            (
                sc.contract_id,
                (Symbol::new(&sc.env, "participant_joined"), participant_addr).into_val(&sc.env),
                200i128.into_val(&sc.env)
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
    let sc = setup_scenario();
    sc.env.mock_all_auths();

    let participant_addr = Address::random(&sc.env);
    // Add funds to client address (as participant)
    sc.token_admin_client.mint(&participant_addr, &199);

    // Init contract
    sc.contract_client
        .init(&Address::random(&sc.env), &sc.token_admin_client.address);

    // Join the participant (in this case same as client). Should not have enough funds. We expect panic.
    sc.contract_client.join(&participant_addr, &200);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn participant_cant_join_with_negative_stake() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    let participant_addr = Address::random(&sc.env);
    // Add funds to client address (as participant)
    sc.token_admin_client.mint(&participant_addr, &199);

    // Init contract
    sc.contract_client
        .init(&Address::random(&sc.env), &sc.token_admin_client.address);

    // Join the participant (in this case same as client). Should not have enough funds. We expect panic.
    sc.contract_client.join(&participant_addr, &-1);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn participant_cant_join_with_zero_stake() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    let participant_addr = Address::random(&sc.env);
    // Add funds to client address (as participant)
    sc.token_admin_client.mint(&participant_addr, &199);

    // Init contract
    sc.contract_client
        .init(&Address::random(&sc.env), &sc.token_admin_client.address);

    // Join the participant (in this case same as client). Should not have enough funds. We expect panic.
    sc.contract_client.join(&participant_addr, &0);
}