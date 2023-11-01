#![cfg(test)]

extern crate std;

use crate::{MarketplaceContract, MarketplaceContractClient};
use soroban_sdk::{
    testutils::Address as _,
    token::{self, Client},
    Address, Env,
};

fn setup_test<'a>() -> (
    Env,
    MarketplaceContractClient<'a>,
    Client<'a>,
    token::StellarAssetClient<'a>,
    Client<'a>,
    token::StellarAssetClient<'a>,
    Address,
    Address,
) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, MarketplaceContract);
    let contract_client: MarketplaceContractClient<'_> =
        MarketplaceContractClient::new(&env, &contract_id);

    let seller = Address::random(&env);
    let buyer = Address::random(&env);

    let token_admin_client = create_token_asset(&env, &Address::random(&env));
    let token_client = token::Client::new(&env, &token_admin_client.address);

    contract_client.init(&token_client.address, &Address::random(&env));
    let asset_admin_client = create_token_asset(&env, &Address::random(&env));
    let asset_client = token::Client::new(&env, &asset_admin_client.address);

    (
        env,
        contract_client,
        token_client,
        token_admin_client,
        asset_client,
        asset_admin_client,
        seller,
        buyer,
    )
}

fn create_token_asset<'a>(e: &Env, admin: &Address) -> token::StellarAssetClient<'a> {
    token::StellarAssetClient::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn cannot_initialize_marketplace_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, MarketplaceContract);
    let client: MarketplaceContractClient<'_> = MarketplaceContractClient::new(&env, &contract_id);

    let address = Address::random(&env); // Address just for satisfying interfaces.
    client.init(&address, &address);
    client.init(&address, &address);
}

#[test]
fn can_create_listing() {
    let (
        env,
        contract_client,
        _token_client,
        _token_admin_client,
        _asset_client,
        asset_admin_client,
        seller,
        _buyer,
    ) = setup_test();

    asset_admin_client.mint(&seller, &2); // Seller has 2 NFTs.

    let id = contract_client.create_listing(&seller, &asset_admin_client.address, &100, &2);
    let listing = contract_client.get_listing(&id).unwrap();

    assert_eq!(&listing.id, &1);
    assert_eq!(&listing.listed, &true);
    assert_eq!(&listing.owner, &seller);

    let asset_client = Client::new(&env, &asset_admin_client.address);
    assert_eq!(asset_client.balance(&contract_client.address), 2); // Now the contract has the ownership of the NFTs.
    assert_eq!(&listing.price, &100);
    assert_eq!(&listing.quantity, &2);
}

#[test]
fn create_listing_increments_id() {
    let (
        _env,
        contract_client,
        _token_client,
        _token_admin_client,
        _asset_client,
        asset_admin_client,
        seller,
        _buyer,
    ) = setup_test();

    asset_admin_client.mint(&seller, &4);

    let id_1 = contract_client.create_listing(&seller, &asset_admin_client.address, &100, &2);
    let id_2 = contract_client.create_listing(&seller, &asset_admin_client.address, &100, &2);

    assert_eq!(1, id_1);
    assert_eq!(2, id_2);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn cannot_create_negative_price_listing() {
    let (
        _env,
        contract_client,
        _token_client,
        _token_admin_client,
        _asset_client,
        asset_admin_client,
        seller,
        _buyer,
    ) = setup_test();

    asset_admin_client.mint(&seller, &2);
    contract_client.create_listing(&seller, &asset_admin_client.address, &-100, &2);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn cannot_create_zero_price_listing() {
    let (
        _env,
        contract_client,
        _token_client,
        _token_admin_client,
        _asset_client,
        asset_admin_client,
        seller,
        _buyer,
    ) = setup_test();

    asset_admin_client.mint(&seller, &2);
    contract_client.create_listing(&seller, &asset_admin_client.address, &0, &2);
}

#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn cannot_create_negative_quantity_listing() {
    let (
        _env,
        contract_client,
        _token_client,
        _token_admin_client,
        _asset_client,
        asset_admin_client,
        seller,
        _buyer,
    ) = setup_test();

    asset_admin_client.mint(&seller, &2);
    contract_client.create_listing(&seller, &asset_admin_client.address, &100, &-1);
}

#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn cannot_create_zero_quantity_listing() {
    let (
        _env,
        contract_client,
        _token_client,
        _token_admin_client,
        _asset_client,
        asset_admin_client,
        seller,
        _buyer,
    ) = setup_test();

    asset_admin_client.mint(&seller, &2);
    contract_client.create_listing(&seller, &asset_admin_client.address, &100, &0);
}

#[test]
fn can_create_listing_and_pause() {
    let (
        env,
        contract_client,
        _token_client,
        _token_admin_client,
        _asset_client,
        asset_admin_client,
        seller,
        _buyer,
    ) = setup_test();

    asset_admin_client.mint(&seller, &2);
    let id = contract_client.create_listing(&seller, &asset_admin_client.address, &100, &2);
    contract_client.pause_listing(&seller, &id);
    let listing = contract_client.get_listing(&id).unwrap();

    assert_eq!(&listing.listed, &false);
    assert_eq!(&listing.owner, &seller);
    let asset_client = Client::new(&env, &asset_admin_client.address);
    assert_eq!(asset_client.balance(&contract_client.address), 2); // Now the contract keeps the ownership of the NFTs.
    assert_eq!(&listing.price, &100);
    assert_eq!(&listing.quantity, &2);
}

#[test]
fn can_create_listing_and_sell() {
    let (
        _env,
        contract_client,
        token_client,
        token_admin_client,
        asset_client,
        asset_admin_client,
        seller,
        buyer,
    ) = setup_test();

    asset_admin_client.mint(&seller, &2);
    let id = contract_client.create_listing(&seller, &asset_client.address, &100, &2);

    token_admin_client.mint(&buyer, &400);

    contract_client.buy_listing(&buyer, &id, &2);

    let listing = contract_client.get_listing(&id).unwrap();

    assert_eq!(&listing.listed, &false);
    assert_eq!(&listing.owner, &buyer);
    assert_eq!(asset_client.balance(&contract_client.address), 0); // Contract no longer the owner of the NFTS.
    assert_eq!(asset_client.balance(&buyer), 2); // Now the buyer has the ownership of the NFTs.
    assert_eq!(&listing.price, &100);

    assert_eq!(
        &token_client.balance(&seller), // Seller has 200 tokens more.
        &200
    );
    assert_eq!(
        &token_client.balance(&buyer), // Buyer has 200 tokes less.
        &200
    );
}

#[test]
fn can_update_a_listing() {
    let (
        _env,
        contract_client,
        _token_client,
        _token_admin_client,
        _asset_client,
        asset_admin_client,
        seller,
        _buyer,
    ) = setup_test();

    asset_admin_client.mint(&seller, &10);
    let id = contract_client.create_listing(&seller, &asset_admin_client.address, &100, &2);

    let listing = contract_client.get_listing(&id).unwrap();

    assert_eq!(&listing.listed, &true);
    assert_eq!(&listing.owner, &seller);
    assert_eq!(&listing.price, &100);
    assert_eq!(&listing.quantity, &2);

    contract_client.update_price(&seller, &id, &200);

    let listing = contract_client.get_listing(&id).unwrap();
    assert_eq!(&listing.listed, &true);
    assert_eq!(&listing.owner, &seller);
    assert_eq!(&listing.price, &200);

    // TODO: move commented code below to its own test

    // client.pause_listing(&seller, &asset_admin_client.address, &200, &3);

    // let listing = client.get_listing(&asset_admin_client.address).unwrap();
    // assert_eq!(&listing.listed, &false);
    // assert_eq!(&listing.owner, &seller);
    // assert_eq!(&listing.price, &200);

    // client.unpause_listing(&seller, &asset_admin_client.address, &200, &3);

    // let listing = client.get_listing(&asset_admin_client.address).unwrap();
    // assert_eq!(&listing.listed, &true);
    // assert_eq!(&listing.owner, &seller);
    // assert_eq!(&listing.price, &190);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn cannot_sell_when_unlisted() {
    let (
        _env,
        contract_client,
        _token_client,
        token_admin_client,
        _asset_client,
        asset_admin_client,
        seller,
        buyer,
    ) = setup_test();

    asset_admin_client.mint(&seller, &2);
    let id = contract_client.create_listing(&seller, &asset_admin_client.address, &100, &2);
    contract_client.pause_listing(&seller, &id);

    token_admin_client.mint(&buyer, &400);
    contract_client.buy_listing(&buyer, &id, &2);
}

#[test]
fn can_remove_a_listing() {
    let (
        _env,
        contract_client,
        _token_client,
        _token_admin_client,
        _asset_client,
        asset_admin_client,
        seller,
        _buyer,
    ) = setup_test();

    asset_admin_client.mint(&seller, &2);

    let id: u64 = contract_client.create_listing(&seller, &asset_admin_client.address, &100, &2);
    contract_client.remove_listing(&seller, &id);

    let listing = contract_client.get_listing(&id);
    assert!(listing.is_none());
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn cannot_do_negative_update() {
    let (
        _env,
        contract_client,
        _token_client,
        _token_admin_client,
        _asset_client,
        asset_admin_client,
        seller,
        _buyer,
    ) = setup_test();

    asset_admin_client.mint(&seller, &2);

    let id = contract_client.create_listing(&seller, &asset_admin_client.address, &100, &2);
    contract_client.update_price(&seller, &id, &-100)
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn cannot_create_listing_without_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, MarketplaceContract);
    let client: MarketplaceContractClient<'_> = MarketplaceContractClient::new(&env, &contract_id);
    client.create_listing(&Address::random(&env), &Address::random(&env), &1, &1);
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn cannot_buy_listing_without_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, MarketplaceContract);
    let client: MarketplaceContractClient<'_> = MarketplaceContractClient::new(&env, &contract_id);
    client.buy_listing(&Address::random(&env), &1, &1);
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn cannot_get_listing_without_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, MarketplaceContract);
    let client: MarketplaceContractClient<'_> = MarketplaceContractClient::new(&env, &contract_id);
    client.get_listing(&1).unwrap();
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn cannot_pause_listing_without_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, MarketplaceContract);
    let client: MarketplaceContractClient<'_> = MarketplaceContractClient::new(&env, &contract_id);
    client.pause_listing(&Address::random(&env), &1);
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn cannot_unpause_listing_without_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, MarketplaceContract);
    let client: MarketplaceContractClient<'_> = MarketplaceContractClient::new(&env, &contract_id);
    client.unpause_listing(&Address::random(&env), &1);
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn cannot_update_price_listing_without_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, MarketplaceContract);
    let client: MarketplaceContractClient<'_> = MarketplaceContractClient::new(&env, &contract_id);
    client.update_price(&Address::random(&env), &1, &1);
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn cannot_remove_listing_without_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, MarketplaceContract);
    let client: MarketplaceContractClient<'_> = MarketplaceContractClient::new(&env, &contract_id);
    client.remove_listing(&Address::random(&env), &1);
}
