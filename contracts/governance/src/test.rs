#![cfg(test)]

extern crate std;

use super::{voting_contract, GovernanceContract, GovernanceContractClient};

use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, BytesN as _, Events},
    token::{self, AdminClient, Client},
    vec, Address, BytesN, Env, IntoVal, Symbol, Val, Vec,
};

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn cannot_be_initialized_twice() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    sc.contract_client.init(
        &sc.contract_client.address,
        &sc.token_client.address,
        &sc.voting_contract_id,
    );
    sc.contract_client.init(
        &sc.contract_client.address,
        &sc.token_client.address,
        &sc.voting_contract_id,
    );
}

fn setup_scenario<'a>() -> Scenario<'a> {
    let env = Env::default();

    let voting_contract_id =
        env.register_contract_wasm(Some(&Address::random(&env)), voting_contract::WASM);
    let voting_contract_client = voting_contract::Client::new(&env, &voting_contract_id);

    let contract_id = env.register_contract(Some(&Address::random(&env)), GovernanceContract);
    let contract_client = GovernanceContractClient::new(&env, &contract_id);

    let token_admin = Address::random(&env);
    let token_addr = env.register_stellar_asset_contract(token_admin.clone());
    let token_admin_client = token::AdminClient::new(&env, &token_addr);
    let token_client = token::Client::new(&env, &token_addr);

    Scenario {
        env,
        voting_contract_id,
        voting_contract_client,
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
    voting_contract_id: Address,
    voting_contract_client: voting_contract::Client<'a>,
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
    sc.contract_client.init(
        &curator,
        &sc.token_admin_client.address,
        &sc.voting_contract_id,
    );

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

    // A proper joining event should be published and a stake one.
    let last_events = sc
        .env
        .events()
        .all()
        .slice(sc.env.events().all().len() - 2..);
    assert_eq!(
        last_events,
        vec![
            &sc.env,
            (
                sc.contract_id.clone(),
                (Symbol::new(&sc.env, "stake"), participant_addr.clone()).into_val(&sc.env),
                200i128.into_val(&sc.env)
            ),
            (
                sc.contract_id,
                (Symbol::new(&sc.env, "participant_joined"), participant_addr).into_val(&sc.env),
                ().into_val(&sc.env)
            ),
        ]
    )
}

fn assert_auth(
    auths: &[(Address, AuthorizedInvocation)],
    idx: usize,
    auth_addr: Address,
    call_addr: Address,
    func: Symbol,
    args: Vec<Val>,
) {
    let auth = auths.get(idx).unwrap();
    assert_eq!(auth.0, auth_addr);
    assert_eq!(
        auth.1.function,
        AuthorizedFunction::Contract((call_addr, func, args))
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn participant_cant_join_without_enough_funds() {
    let sc = setup_scenario();
    sc.env.mock_all_auths();

    let participant_addr = Address::random(&sc.env);
    sc.token_admin_client.mint(&participant_addr, &199);

    sc.contract_client.init(
        &Address::random(&sc.env),
        &sc.token_admin_client.address,
        &sc.voting_contract_id,
    );

    sc.contract_client.join(&participant_addr, &200);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn participant_cant_join_with_negative_stake() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    let participant_addr = Address::random(&sc.env);
    sc.token_admin_client.mint(&participant_addr, &199);

    sc.contract_client.init(
        &Address::random(&sc.env),
        &sc.token_admin_client.address,
        &sc.voting_contract_id,
    );

    sc.contract_client.join(&participant_addr, &-1);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn participant_cant_join_with_zero_stake() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    let participant_addr = Address::random(&sc.env);
    sc.token_admin_client.mint(&participant_addr, &199);

    sc.contract_client.init(
        &Address::random(&sc.env),
        &sc.token_admin_client.address,
        &sc.voting_contract_id,
    );

    sc.contract_client.join(&participant_addr, &0);
}

#[test]
fn participant_can_leave_withdrawing_all_funds() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    let participant_addr = Address::random(&sc.env);
    // Add funds to client address (as participant)
    sc.token_admin_client.mint(&participant_addr, &1000);

    // Init contract
    let curator = Address::random(&sc.env);
    sc.contract_client.init(
        &curator,
        &sc.token_admin_client.address,
        &sc.voting_contract_id,
    );

    // Join the participant (in this case same as client)
    let initial_stake = 200;
    sc.contract_client.join(&participant_addr, &initial_stake);

    sc.contract_client.leave(&participant_addr);

    // Ensure we check participant is who says.
    assert_auth(
        &sc.env.auths(),
        0,
        participant_addr.clone(),
        sc.contract_client.address.clone(),
        Symbol::new(&sc.env, "leave"),
        (participant_addr.clone(),).into_val(&sc.env),
    );

    // After withdrawing, all balances return to initial status.
    assert_eq!(0, sc.token_client.balance(&sc.contract_id));
    assert_eq!(1000, sc.token_client.balance(&participant_addr));

    // A proper withdrawal and participant left events should be published.
    let last_events = sc
        .env
        .events()
        .all()
        .slice(sc.env.events().all().len() - 2..);
    assert_eq!(
        last_events,
        vec![
            &sc.env,
            (
                sc.contract_id.clone(),
                (Symbol::new(&sc.env, "withdraw"), participant_addr.clone()).into_val(&sc.env),
                200i128.into_val(&sc.env)
            ),
            (
                sc.contract_id.clone(),
                (Symbol::new(&sc.env, "participant_left"), participant_addr).into_val(&sc.env),
                ().into_val(&sc.env)
            ),
        ]
    )
}

#[test]
fn participant_can_withdraw_partial_funds() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    let participant_addr = Address::random(&sc.env);
    // Add funds to client address (as participant)
    sc.token_admin_client.mint(&participant_addr, &1000);

    // Init contract
    sc.contract_client.init(
        &Address::random(&sc.env),
        &sc.token_admin_client.address,
        &sc.voting_contract_id,
    );

    sc.contract_client.join(&participant_addr, &200);

    sc.contract_client.withdraw(&participant_addr, &100);

    // Ensure we check participant is who says.
    assert_auth(
        &sc.env.auths(),
        0,
        participant_addr.clone(),
        sc.contract_client.address.clone(),
        Symbol::new(&sc.env, "withdraw"),
        (participant_addr.clone(), 100i128).into_val(&sc.env),
    );

    assert_eq!(100, sc.token_client.balance(&sc.contract_id));
    assert_eq!(900, sc.token_client.balance(&participant_addr));

    // A proper withdrawal and participant left events should be published.
    let last_event = sc.env.events().all().last().unwrap();
    assert_eq!(
        vec![&sc.env, last_event],
        vec![
            &sc.env,
            (
                sc.contract_id.clone(),
                (Symbol::new(&sc.env, "withdraw"), participant_addr.clone()).into_val(&sc.env),
                100i128.into_val(&sc.env)
            ),
        ]
    )
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn participant_cannot_withdraw_more_partial_funds_than_it_has() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    let participant_addr = Address::random(&sc.env);
    // Add funds to client address (as participant)
    sc.token_admin_client.mint(&participant_addr, &1000);

    sc.contract_client.init(
        &Address::random(&sc.env),
        &sc.token_admin_client.address,
        &sc.voting_contract_id,
    );

    sc.contract_client.join(&participant_addr, &200);

    sc.contract_client.withdraw(&participant_addr, &201);
}

#[test]
fn participant_can_deposit_extra_funds() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    let participant_addr = Address::random(&sc.env);
    // Add funds to client address (as participant)
    sc.token_admin_client.mint(&participant_addr, &1000);

    // Init contract
    sc.contract_client.init(
        &Address::random(&sc.env),
        &sc.token_admin_client.address,
        &sc.voting_contract_id,
    );

    sc.contract_client.join(&participant_addr, &200);

    sc.contract_client.stake(&participant_addr, &100);

    // Ensure we check participant is who says.
    assert_auth(
        &sc.env.auths(),
        0,
        participant_addr.clone(),
        sc.contract_client.address.clone(),
        Symbol::new(&sc.env, "stake"),
        (participant_addr.clone(), 100i128).into_val(&sc.env),
    );

    assert_eq!(300, sc.token_client.balance(&sc.contract_id));
    assert_eq!(700, sc.token_client.balance(&participant_addr));

    // A proper stake event should be published.
    let last_event = sc.env.events().all().last().unwrap();
    assert_eq!(
        vec![&sc.env, last_event],
        vec![
            &sc.env,
            (
                sc.contract_id.clone(),
                (Symbol::new(&sc.env, "stake"), participant_addr.clone()).into_val(&sc.env),
                100i128.into_val(&sc.env)
            ),
        ]
    )
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn non_existent_participant_cannot_stake() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    sc.contract_client.init(
        &Address::random(&sc.env),
        &sc.token_admin_client.address,
        &sc.voting_contract_id,
    );
    sc.contract_client.withdraw(&Address::random(&sc.env), &1);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn non_existent_participant_cannot_leave() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    sc.contract_client.init(
        &Address::random(&sc.env),
        &sc.token_admin_client.address,
        &sc.voting_contract_id,
    );
    sc.contract_client.leave(&Address::random(&sc.env));
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn non_existent_participant_cannot_withdraw() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    sc.contract_client.init(
        &Address::random(&sc.env),
        &sc.token_admin_client.address,
        &sc.voting_contract_id,
    );
    sc.contract_client.leave(&Address::random(&sc.env));
}

#[test]
fn curator_can_whitelist_participant() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    let curator = &Address::random(&sc.env);
    let participant = &Address::random(&sc.env);
    sc.token_admin_client.mint(participant, &1000);

    sc.contract_client.init(
        curator,
        &sc.token_admin_client.address,
        &sc.voting_contract_id,
    );

    sc.contract_client.join(participant, &200);
    sc.contract_client.whitelist(participant);

    assert_auth(
        &sc.env.auths(),
        0,
        curator.clone(),
        sc.contract_client.address.clone(),
        Symbol::new(&sc.env, "whitelist"),
        (participant.clone(),).into_val(&sc.env),
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn not_existent_participant_cannot_create_proposals() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    sc.contract_client.init(
        &Address::random(&sc.env),
        &sc.token_admin_client.address,
        &sc.voting_contract_id,
    );

    let participant = &Address::random(&sc.env);
    let hash = BytesN::random(&sc.env);

    sc.contract_client
        .propose_code_upgrade(participant, &1, &hash);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn not_whitelisted_participant_cannot_create_proposals() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    let participant = &Address::random(&sc.env);
    sc.token_admin_client.mint(participant, &1000);

    sc.contract_client.init(
        &Address::random(&sc.env),
        &sc.token_admin_client.address,
        &sc.voting_contract_id,
    );

    sc.contract_client.join(participant, &200);
    let hash = BytesN::random(&sc.env);
    sc.contract_client
        .propose_code_upgrade(participant, &1, &hash);
}

#[test]
fn whitelisted_participant_can_create_code_upgrade_proposals() {
    let sc = setup_scenario();

    sc.env.mock_all_auths();

    let participant = Address::random(&sc.env);
    sc.token_admin_client.mint(&participant, &1000);

    sc.contract_client.init(
        &sc.contract_client.address,
        &sc.token_admin_client.address,
        &sc.voting_contract_id,
    );

    sc.voting_contract_client
        .init(&sc.contract_id, &864000, &50_000, &0);

    sc.contract_client.join(&participant, &200);
    sc.contract_client.whitelist(&participant);

    let new_contract_hash = BytesN::random(&sc.env);
    sc.contract_client
        .propose_code_upgrade(&participant, &1, &new_contract_hash);

    assert_auth(
        &sc.env.auths(),
        0,
        participant.clone(),
        sc.contract_client.address.clone(),
        Symbol::new(&sc.env, "propose_code_upgrade"),
        (participant.clone(), 1u64, new_contract_hash).into_val(&sc.env),
    );

    let last_event = sc.env.events().all().last().unwrap();
    assert_eq!(
        vec![&sc.env, last_event],
        vec![
            &sc.env,
            (
                sc.contract_id.clone(),
                (Symbol::new(&sc.env, "new_proposal"), participant.clone()).into_val(&sc.env),
                1u64.into_val(&sc.env)
            ),
        ]
    )
}
