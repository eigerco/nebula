#![cfg(test)]

extern crate std;

use crate::{MarketplaceContract, MarketplaceContractClient};
use soroban_sdk::{
    testutils::Address as _,
    token::{self},
    Address, Env,
};

fn setup_test<'a>() -> (Env, MarketplaceContractClient<'a>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, MarketplaceContract);
    let client: MarketplaceContractClient<'_> = MarketplaceContractClient::new(&env, &contract_id);
    (env, client)
}

fn create_token_asset<'a>(e: &Env, admin: &Address) -> token::StellarAssetClient<'a> {
    token::StellarAssetClient::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn cannot_initialize_marketplace_twice() {
    let (env, client) = setup_test();
    let address = Address::random(&env); // Address just for satisfying interfaces.
    client.init(&address, &address);
    client.init(&address, &address);
}

#[test]
fn can_create_listing() {
    let (env, client) = setup_test();
    let admin = Address::random(&env);
    let seller = Address::random(&env);
    let token = Address::random(&env);

    let token = create_token_asset(&env, &token);
    client.init(&token.address, &admin);

    let asset_client = create_token_asset(&env, &seller);

    client.create_listing(&seller, &asset_client.address, &100);
    let listing = client.get_listing(&asset_client.address).unwrap();

    assert_eq!(&listing.listed, &true);
    assert_eq!(&listing.owner, &seller);
    assert_eq!(&asset_client.admin(), &client.address); // Now the contract has the ownership of the asset.
    assert_eq!(&listing.price, &100)
}

#[test]
fn can_create_listing_and_pause() {
    let (env, client) = setup_test();
    let admin = Address::random(&env);
    let seller = Address::random(&env);
    let token = Address::random(&env);

    let token = create_token_asset(&env, &token);
    client.init(&token.address, &admin);

    let asset_client: token::StellarAssetClient<'_> = create_token_asset(&env, &seller);
    client.create_listing(&seller, &asset_client.address, &100);

    client.pause_listing(&seller, &asset_client.address, &100);

    let listing = client.get_listing(&asset_client.address).unwrap();

    assert_eq!(&listing.listed, &false);
    assert_eq!(&listing.owner, &seller);
    assert_eq!(&asset_client.admin(), &client.address); // The contract continues being the admin of the asset (still not sold out).
    assert_eq!(&listing.price, &100)
}

#[test]
fn can_create_listing_and_sell() {
    let (env, client) = setup_test();
    let admin = Address::random(&env);
    let seller = Address::random(&env);
    let buyer = Address::random(&env);
    let token = Address::random(&env);

    let token = create_token_asset(&env, &token);
    client.init(&token.address, &admin);

    let asset_client: token::StellarAssetClient<'_> = create_token_asset(&env, &seller);
    client.create_listing(&seller, &asset_client.address, &100);

    token.mint(&buyer, &400);
    client.buy_listing(&buyer, &asset_client.address, &100);

    let listing = client.get_listing(&asset_client.address).unwrap();

    assert_eq!(&listing.listed, &false);
    assert_eq!(&listing.owner, &buyer);
    assert_eq!(&asset_client.admin(), &buyer); // The admin of the stellar asset changed to the new owner/admin).
    assert_eq!(&listing.price, &100);
    assert_eq!(
        &token::Client::new(&env, &token.address).balance(&seller),
        &100
    );
}

#[test]
fn can_update_a_listing() {
    let (env, client) = setup_test();
    let admin = Address::random(&env);
    let seller = Address::random(&env);
    let token = Address::random(&env);

    let token = create_token_asset(&env, &token);
    client.init(&token.address, &admin);

    let asset_client = create_token_asset(&env, &seller);
    client.create_listing(&seller, &asset_client.address, &100);

    let listing = client.get_listing(&asset_client.address).unwrap();

    assert_eq!(&listing.listed, &true);
    assert_eq!(&listing.owner, &seller);
    assert_eq!(&listing.price, &100);

    client.update_price(&seller, &asset_client.address, &100, &200);

    let listing = client.get_listing(&asset_client.address).unwrap();
    assert_eq!(&listing.listed, &true);
    assert_eq!(&listing.owner, &seller);
    assert_eq!(&listing.price, &200);

    client.pause_listing(&seller, &asset_client.address, &200);

    let listing = client.get_listing(&asset_client.address).unwrap();
    assert_eq!(&listing.listed, &false);
    assert_eq!(&listing.owner, &seller);
    assert_eq!(&listing.price, &200);

    client.unpause_listing(&seller, &asset_client.address, &200, &190);

    let listing = client.get_listing(&asset_client.address).unwrap();
    assert_eq!(&listing.listed, &true);
    assert_eq!(&listing.owner, &seller);
    assert_eq!(&listing.price, &190);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn cannot_sell_when_unlisted() {
    let (env, client) = setup_test();
    let admin = Address::random(&env);
    let seller = Address::random(&env);
    let buyer = Address::random(&env);
    let token = Address::random(&env);

    let token = create_token_asset(&env, &token);
    client.init(&token.address, &admin);

    let asset_client = create_token_asset(&env, &seller);
    client.create_listing(&seller, &asset_client.address, &100);

    client.pause_listing(&seller, &asset_client.address, &100);

    token.mint(&buyer, &400);
    client.buy_listing(&buyer, &asset_client.address, &100);
}

#[test]
fn can_remove_a_listing() {
    let (env, client) = setup_test();
    let admin = Address::random(&env);
    let seller = Address::random(&env);
    let token = Address::random(&env);

    let token = create_token_asset(&env, &token);
    client.init(&token.address, &admin);

    let asset_client = create_token_asset(&env, &seller);
    client.create_listing(&seller, &asset_client.address, &100);
    client.remove_listing(&seller, &asset_client.address, &100);

    let listing = client.get_listing(&asset_client.address);
    assert!(listing.is_none());
    assert_eq!(&seller, &asset_client.admin())
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn cannot_create_negative_listing() {
    let (env, client) = setup_test();
    let admin = Address::random(&env);
    let seller = Address::random(&env);
    let token = Address::random(&env);

    let token = create_token_asset(&env, &token);
    client.init(&token.address, &admin);

    let asset_client = create_token_asset(&env, &seller);

    client.create_listing(&seller, &asset_client.address, &-100);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn cannot_do_negative_update() {
    let (env, client) = setup_test();
    let admin = Address::random(&env);
    let seller = Address::random(&env);
    let token = Address::random(&env);

    let token = create_token_asset(&env, &token);
    client.init(&token.address, &admin);

    let asset_client = create_token_asset(&env, &seller);
    client.create_listing(&seller, &asset_client.address, &100);
    client.update_price(&seller, &asset_client.address, &100, &-100)
}
