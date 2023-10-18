//! # Marketplace Contract
//!
//! The marketplace contract enables the creation and management of listings for various assets.
//! Users can buy, update, pause, and remove listings. This contract also supports a fee or commission for transactions.
#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error,
    token::{self},
    Address, Env, Map, Symbol,
};

#[derive(Clone, Copy)]
#[contracttype]
enum DataKey {
    Admin = 1,
    Initialized = 2,
    Assets = 3,
    Token = 4,
}

#[contracterror]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAssetPrice = 2,
    BalanceTooLow = 3,
    AssetNotListed = 4,
    InvalidAuth = 5,
    NotInitialized = 6,
}

#[contracttype]
#[derive(Clone)]
pub struct Asset {
    owner: Address,
    price: i128,
    listed: bool,
}

#[contract]
pub struct MarketplaceContract;

#[contractimpl]
impl MarketplaceContract {
    /// Initialize the contract with the admin's address.
    pub fn init(env: Env, token: Address, admin: Address) {
        admin.require_auth();
        let storage = env.storage().persistent();

        if storage.get::<_, ()>(&DataKey::Initialized).is_some() {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }

        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Token, &token);
        storage.set(&DataKey::Initialized, &());
        let assets: Map<Address, Asset> = storage.get(&DataKey::Assets).unwrap_or(Map::new(&env));
        storage.set(&DataKey::Assets, &assets);
    }

    pub fn get_listing(env: Env, asset: Address) -> Option<Asset> {
        let storage = env.storage().persistent();
        let assets: Map<Address, Asset> = storage.get(&DataKey::Assets).unwrap();
        assets.get(asset)
    }

    /// Allow sellers to list assets by specifying the seller's address, the asset's address, and the asset's price.
    pub fn create_listing(env: Env, seller: Address, asset: Address, price: i128) {
        if price <= 0 {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        seller.require_auth();
        let storage = env.storage().persistent();
        if storage.get::<_, ()>(&DataKey::Initialized).is_none() {
            panic_with_error!(&env, Error::NotInitialized);
        }
        let mut assets: Map<Address, Asset> = storage.get(&DataKey::Assets).unwrap();
        assets.set(
            asset.clone(),
            Asset {
                owner: seller.clone(),
                price,
                listed: true,
            },
        );
        storage.set(&DataKey::Assets, &assets);
        let topics = (Symbol::new(&env, "create_listing"), (seller));
        env.events().publish(topics, asset);
    }

    /// Enable buyers to purchase assets by providing the buyer's address, the asset's address, and the agreed-upon price.
    pub fn buy_listing(env: Env, buyer: Address, asset: Address, price: i128) {
        if price <= 0 {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        buyer.require_auth();
        let storage = env.storage().persistent();
        if storage.get::<_, ()>(&DataKey::Initialized).is_none() {
            panic_with_error!(&env, Error::NotInitialized);
        }
        let token = storage.get(&DataKey::Token).unwrap();
        let mut assets: Map<Address, Asset> = storage.get(&DataKey::Assets).unwrap();
        let Asset {
            owner: seller,
            price: set_price,
            listed,
        } = assets.get(asset.clone()).unwrap();
        if !listed {
            panic_with_error!(&env, Error::AssetNotListed);
        }
        if price != set_price {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        let token = token::Client::new(&env, &token);
        if token.balance(&buyer) < price {
            panic_with_error!(&env, Error::BalanceTooLow);
        }
        token.transfer(&buyer, &seller, &price);
        assets.set(
            asset.clone(),
            Asset {
                owner: buyer.clone(),
                price,
                listed: false,
            },
        );
        storage.set(&DataKey::Assets, &assets);
        let topics = (Symbol::new(&env, "buy_listing"), (buyer));
        env.events().publish(topics, asset);
    }

    /// Permit sellers to update the price of a listed asset,
    /// ensuring they provide the correct seller and asset addresses, as well as the old and new prices.
    pub fn update_price(
        env: Env,
        seller: Address,
        asset: Address,
        old_price: i128,
        new_price: i128,
    ) {
        if new_price <= 0 {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        seller.require_auth();
        let storage = env.storage().persistent();
        if storage.get::<_, ()>(&DataKey::Initialized).is_none() {
            panic_with_error!(&env, Error::NotInitialized);
        }
        let mut assets: Map<Address, Asset> = storage.get(&DataKey::Assets).unwrap();
        let Asset {
            owner: set_seller,
            price: set_price,
            listed,
        } = assets.get(asset.clone()).unwrap();
        if set_seller != seller {
            panic_with_error!(&env, Error::InvalidAuth);
        }

        if old_price != set_price {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        assets.set(
            asset.clone(),
            Asset {
                owner: seller.clone(),
                price: new_price,
                listed,
            },
        );
        storage.set(&DataKey::Assets, &assets);
        let topics = (Symbol::new(&env, "update_price"), (seller));
        env.events().publish(topics, asset);
    }

    /// Allow sellers to pause a listing by specifying their address, the asset's address,
    /// and the price at which it was listed.
    pub fn pause_listing(env: Env, seller: Address, asset: Address, price: i128) {
        if price <= 0 {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        seller.require_auth();
        let storage = env.storage().persistent();
        if storage.get::<_, ()>(&DataKey::Initialized).is_none() {
            panic_with_error!(&env, Error::NotInitialized);
        }
        let mut assets: Map<Address, Asset> = storage.get(&DataKey::Assets).unwrap();
        let Asset {
            owner: set_seller,
            price: set_price,
            ..
        } = assets.get(asset.clone()).unwrap();
        if set_seller != seller {
            panic_with_error!(&env, Error::InvalidAuth);
        }
        if price != set_price {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        assets.set(
            asset.clone(),
            Asset {
                owner: seller.clone(),
                price,
                listed: false,
            },
        );
        storage.set(&DataKey::Assets, &assets);
        let topics = (Symbol::new(&env, "pause_listing"), (seller));
        env.events().publish(topics, asset);
    }

    /// Allow sellers to un-pause a listing by specifying their address, the asset's address,
    /// and the price at which it is listed.
    pub fn unpause_listing(
        env: Env,
        seller: Address,
        asset: Address,
        old_price: i128,
        new_price: i128,
    ) {
        if old_price <= 0 || new_price <= 0 {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        seller.require_auth();
        let storage = env.storage().persistent();
        if storage.get::<_, ()>(&DataKey::Initialized).is_none() {
            panic_with_error!(&env, Error::NotInitialized);
        }
        let mut assets: Map<Address, Asset> = storage.get(&DataKey::Assets).unwrap();
        let Asset {
            owner: set_seller,
            price: set_price,
            ..
        } = assets.get(asset.clone()).unwrap();
        if set_seller != seller {
            panic_with_error!(&env, Error::InvalidAuth);
        }
        if old_price != set_price {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        assets.set(
            asset.clone(),
            Asset {
                owner: seller.clone(),
                price: new_price,
                listed: true,
            },
        );
        storage.set(&DataKey::Assets, &assets);
        let topics = (Symbol::new(&env, "unpause_listing"), (seller));
        env.events().publish(topics, asset);
    }
    /// Allow sellers to completely remove a listing by specifying their address, the asset's address,
    /// and the price at which it was listed.
    pub fn remove_listing(env: Env, seller: Address, asset: Address, price: i128) {
        if price <= 0 {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        seller.require_auth();
        let storage = env.storage().persistent();
        if storage.get::<_, ()>(&DataKey::Initialized).is_none() {
            panic_with_error!(&env, Error::NotInitialized);
        }
        let mut assets: Map<Address, Asset> = storage.get(&DataKey::Assets).unwrap();
        let Asset {
            owner: set_seller,
            price: set_price,
            ..
        } = assets.get(asset.clone()).unwrap();
        if set_seller != seller {
            panic_with_error!(&env, Error::InvalidAuth);
        }
        if price != set_price {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        assets.remove(asset.clone()).unwrap();
        storage.set(&DataKey::Assets, &assets);
        let topics = (Symbol::new(&env, "remove_listing"), (seller));
        env.events().publish(topics, asset);
    }
}

#[cfg(test)]
mod test;
