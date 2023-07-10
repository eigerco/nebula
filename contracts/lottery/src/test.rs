#![cfg(test)]

use super::{LotteryContract, LotteryContractClient};
use soroban_sdk::{testutils::Address as _, token, Address, Env, IntoVal, Symbol};

#[test]
fn admin_is_identified_on_init() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, LotteryContract);
    let client = LotteryContractClient::new(&env, &contract_id);
    let target_token = create_token_contract(&env, &Address::random(&env));

    client.initialize(&client.address, &target_token.address, &2, &100);

    assert_eq!(
        env.auths(),
        [(
            client.address.clone(),
            client.address.clone(),
            Symbol::new(&env, "initialize"),
            (&client.address, &target_token.address, 2u32, 100i128).into_val(&env)
        )]
    )
}

#[test]
fn buy_ticket_works_as_expected() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, LotteryContract);
    let client = LotteryContractClient::new(&env, &contract_id);
    let token_admin = Address::random(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    client.initialize(&client.address, &test_token_client.address, &2, &100);

    let ticket_buyer = Address::random(&env);

    // Transfer some funds to the buyer
    test_token_client.mint(&ticket_buyer, &101);

    let candidates = client.buy_ticket(&ticket_buyer);

    assert_eq!(1, candidates);
    assert_eq!(
        env.auths(),
        [
            (
                ticket_buyer.clone(),
                client.address.clone(),
                Symbol::new(&env, "buy_ticket"),
                (ticket_buyer.clone(),).into_val(&env)
            ),
            (
                ticket_buyer.clone(),
                test_token_client.address.clone(),
                Symbol::new(&env, "transfer"),
                (&ticket_buyer, &contract_id, 100i128).into_val(&env)
            )
        ]
    )
}

#[test]
#[should_panic(expected = "ContractError(1)")]
fn buy_ticket_panics_if_buyer_has_not_enough_funds() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, LotteryContract);
    let client = LotteryContractClient::new(&env, &contract_id);
    let token_admin = Address::random(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    client.initialize(&client.address, &test_token_client.address, &2, &100);

    let ticket_buyer = Address::random(&env);

    // Transfer some funds to the buyer
    test_token_client.mint(&ticket_buyer, &100);

    client.buy_ticket(&ticket_buyer);
}

fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(e, &e.register_stellar_asset_contract(admin.clone()))
}
