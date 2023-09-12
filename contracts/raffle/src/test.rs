#![cfg(test)]

extern crate std;

use crate::calculate_winners;

use super::{RaffleContract, RaffleContractClient};

use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Events},
    token, vec, Address, Env, IntoVal, Symbol, Val, Vec,
};

#[test]
fn admin_is_identified_on_init() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RaffleContract);
    let client = RaffleContractClient::new(&env, &contract_id);
    let target_token = create_token_contract(&env, &Address::random(&env));

    client.init(&client.address, &target_token.address, &2, &100);

    let auths = env.auths();

    assert_auth(
        &auths,
        0,
        client.address.clone(),
        client.address.clone(),
        Symbol::new(&env, "init"),
        (&client.address, &target_token.address, 2u32, 100i128).into_val(&env),
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

fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::AdminClient<'a> {
    token::AdminClient::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn raffle_cannot_be_initialized_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RaffleContract);
    let client = RaffleContractClient::new(&env, &contract_id);
    let token_admin = Address::random(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    client.init(&client.address, &test_token_client.address, &1, &100);
    client.init(&client.address, &test_token_client.address, &1, &100);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn raffle_cannot_be_initialized_without_winners() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RaffleContract);
    let client = RaffleContractClient::new(&env, &contract_id);
    let token_admin = Address::random(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    client.init(&client.address, &test_token_client.address, &0, &100);
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn raffle_cannot_be_initialized_with_ticket_price_zero() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RaffleContract);
    let client = RaffleContractClient::new(&env, &contract_id);
    let token_admin = Address::random(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    client.init(&client.address, &test_token_client.address, &1, &0);
}

#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn ticket_cannot_be_bought_if_not_initialized() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RaffleContract);
    let client = RaffleContractClient::new(&env, &contract_id);

    client.buy_ticket(&client.address);
}

#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn raffle_cannot_be_played_if_not_initialized() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RaffleContract);
    let client = RaffleContractClient::new(&env, &contract_id);

    client.play_raffle(&1);
}

#[test]
fn buy_ticket_works_as_expected() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RaffleContract);
    let client = RaffleContractClient::new(&env, &contract_id);
    let token_admin = Address::random(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    client.init(&client.address, &test_token_client.address, &2, &100);

    let ticket_buyer = Address::random(&env);

    // Transfer some funds to the buyer
    test_token_client.mint(&ticket_buyer, &101);

    let candidates = client.buy_ticket(&ticket_buyer);

    assert_eq!(1, candidates);

    let auths = env.auths();

    assert_auth(
        &auths,
        0,
        ticket_buyer.clone(),
        client.address.clone(),
        Symbol::new(&env, "buy_ticket"),
        (ticket_buyer.clone(),).into_val(&env),
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn buy_ticket_panics_if_buyer_has_not_enough_funds() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RaffleContract);
    let client = RaffleContractClient::new(&env, &contract_id);
    let token_admin = Address::random(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    client.init(&client.address, &test_token_client.address, &2, &100);

    let ticket_buyer = Address::random(&env);

    // Transfer some funds to the buyer
    test_token_client.mint(&ticket_buyer, &100);

    client.buy_ticket(&ticket_buyer);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn buy_ticket_panics_if_invoked_after_raffle_is_played() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RaffleContract);
    let client = RaffleContractClient::new(&env, &contract_id);
    let token_admin = Address::random(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    client.init(&client.address, &test_token_client.address, &1, &100);

    let ticket_buyer_1 = Address::random(&env);
    let ticket_buyer_2 = Address::random(&env);
    let ticket_buyer_3 = Address::random(&env);

    // Transfer some funds to the buyer
    test_token_client.mint(&ticket_buyer_1, &101);
    test_token_client.mint(&ticket_buyer_2, &101);
    test_token_client.mint(&ticket_buyer_3, &101);

    client.buy_ticket(&ticket_buyer_1);
    client.buy_ticket(&ticket_buyer_2);

    client.play_raffle(&666);
    client.buy_ticket(&ticket_buyer_3);
}

#[test]
fn calculate_winners_works_seed_is_deterministic() {
    let env = Env::default();
    let result = calculate_winners(&env, 2, 12, 666);
    assert_eq!(vec![&env, 5, 7], result);
}

#[test]
fn calculate_winners_can_only_win_once() {
    let env = Env::default();
    let result = calculate_winners(&env, 100, 2, 666);
    assert_eq!(vec![&env, 0, 1], result);
}

#[test]
fn play_raffle_works() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RaffleContract);
    let client = RaffleContractClient::new(&env, &contract_id);
    let token_admin = Address::random(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    client.init(&client.address, &test_token_client.address, &1, &100);

    let ticket_buyer_1 = Address::random(&env);
    let ticket_buyer_2 = Address::random(&env);

    // Transfer some funds to the buyer
    test_token_client.mint(&ticket_buyer_1, &101);
    test_token_client.mint(&ticket_buyer_2, &101);

    client.buy_ticket(&ticket_buyer_1);
    client.buy_ticket(&ticket_buyer_2);

    client.play_raffle(&666);

    assert_auth(
        &env.auths(),
        0,
        client.address.clone(),
        client.address.clone(),
        Symbol::new(&env, "play_raffle"),
        (666u64,).into_val(&env),
    );

    let last_event = env.events().all().slice(env.events().all().len() - 1..);
    assert_eq!(
        last_event,
        vec![
            &env,
            (
                contract_id.clone(),
                (Symbol::new(&env, "winner"), &ticket_buyer_2).into_val(&env),
                200i128.into_val(&env)
            )
        ]
    )
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn play_raffle_cannot_be_invoked_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RaffleContract);
    let client = RaffleContractClient::new(&env, &contract_id);
    let token_admin = Address::random(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    client.init(&client.address, &test_token_client.address, &1, &100);

    let ticket_buyer_1 = Address::random(&env);
    let ticket_buyer_2 = Address::random(&env);

    // Transfer some funds to the buyer
    test_token_client.mint(&ticket_buyer_1, &101);
    test_token_client.mint(&ticket_buyer_2, &101);

    client.buy_ticket(&ticket_buyer_1);
    client.buy_ticket(&ticket_buyer_2);

    client.play_raffle(&666);
    client.play_raffle(&666);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn raffle_cannot_be_played_if_not_enough_participants() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RaffleContract);
    let client = RaffleContractClient::new(&env, &contract_id);
    let token_admin = Address::random(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    client.init(&client.address, &test_token_client.address, &1, &100);

    client.play_raffle(&666);
}
