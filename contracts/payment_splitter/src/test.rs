#![cfg(test)]

extern crate std;

use crate::*;
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal, Symbol, Val, Vec,
};

fn setup_test<'a>() -> (Env, PaymentSplitterContractClient<'a>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, PaymentSplitterContract);
    let client: PaymentSplitterContractClient<'_> =
        PaymentSplitterContractClient::new(&env, &contract_id);
    (env, client)
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn cannot_initialize_twice() {
    let (env, client) = setup_test();

    let admin = Address::generate(&env);
    let token = create_token_contract(&env, &admin);
    let recipient_1 = Address::generate(&env);
    let recipient_2 = Address::generate(&env);
    client.init(
        &admin,
        &token.address,
        &Vec::from_slice(&env, &[recipient_1.clone(), recipient_2.clone()]),
    );
    client.init(
        &admin,
        &token.address,
        &Vec::from_slice(&env, &[recipient_1.clone(), recipient_2.clone()]),
    );
}

fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::StellarAssetClient<'a> {
    token::StellarAssetClient::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

#[test]
fn splits_works() {
    let (env, client) = setup_test();
    let token_admin = Address::generate(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    let recipient_1 = Address::generate(&env);
    let recipient_2 = Address::generate(&env);

    client.init(
        &token_admin,
        &test_token_client.address,
        &Vec::from_slice(&env, &[recipient_1.clone(), recipient_2.clone()]),
    );

    // Transfer some funds to the admin
    test_token_client.mint(&token_admin, &100);

    client.split(&50);

    assert_auth(
        &env.auths(),
        0,
        token_admin.clone(),
        client.address.clone(),
        Symbol::new(&env, "split"),
        (50i128,).into_val(&env),
    );

    let token_client = token::Client::new(&env, &test_token_client.address);
    let balance = token_client.balance(&token_admin);
    assert_eq!(balance, 50);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn splits_fails_if_not_enough_money() {
    let (env, client) = setup_test();
    let token_admin = Address::generate(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    let recipient_1 = Address::generate(&env);
    let recipient_2 = Address::generate(&env);

    client.init(
        &token_admin,
        &test_token_client.address,
        &Vec::from_slice(&env, &[recipient_1.clone(), recipient_2.clone()]),
    );

    // Transfer some funds to the admin
    test_token_client.mint(&token_admin, &20);

    client.split(&50);

    assert_auth(
        &env.auths(),
        0,
        token_admin.clone(),
        client.address.clone(),
        Symbol::new(&env, "split"),
        (50i128,).into_val(&env),
    );
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
