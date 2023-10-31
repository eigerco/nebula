//! # Marketplace Contract
//!
//! The marketplace contract enables the creation and management of listings for various assets.
//! Users can buy, update, pause, and remove listings. This contract also supports a fee or commission for transactions.
#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error,
    storage::Persistent,
    token::{self, Client, StellarAssetClient},
    Address, Env, Map, Symbol,
};

#[derive(Clone, Copy)]
#[contracttype]
enum DataKey {
    Admin = 1,
    Initialized = 2,
    Assets = 3,
    Token = 4,
    LastID = 5,
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
    id: u64,
    asset_address: Address,
    owner: Address,
    price: i128,
    quantity: i128,
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
        let assets: Map<u64, Asset> = storage.get(&DataKey::Assets).unwrap_or(Map::new(&env));
        storage.set(&DataKey::Assets, &assets);
        storage.set(&DataKey::LastID, &1u64);
    }

    pub fn get_listing(env: Env, id: u64) -> Option<Asset> {
        let storage = env.storage().persistent();
        let assets: Map<u64, Asset> = storage.get(&DataKey::Assets).unwrap();
        assets.get(id)
    }

    /// Allow sellers to list assets by specifying the seller's address, the asset's address, and the asset's price.
    pub fn create_listing(
        env: Env,
        seller: Address,
        asset_address: Address,
        price: i128,
        quantity: i128,
    ) -> u64 {
        if price <= 0 {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        seller.require_auth();
        let storage = env.storage().persistent();
        if storage.get::<_, ()>(&DataKey::Initialized).is_none() {
            panic_with_error!(&env, Error::NotInitialized);
        }

        let mut assets: Map<u64, Asset> = storage.get(&DataKey::Assets).unwrap();
        let id = Self::current_id(&storage);
        assets.set(
            id,
            Asset {
                id,
                asset_address: asset_address.clone(),
                owner: seller.clone(),
                price,
                quantity,
                listed: true,
            },
        );
        storage.set(&DataKey::Assets, &assets);

        let asset_client = Client::new(&env, &asset_address);
        asset_client.transfer(&seller, &env.current_contract_address(), &quantity);

        let topics = (Symbol::new(&env, "create_listing"), (seller));
        env.events().publish(topics, id);

        id
    }

    fn current_id(storage: &Persistent) -> u64 {
        let id: u64 = storage.get(&DataKey::LastID).unwrap();
        storage.set(&DataKey::LastID, &id.checked_add(1).unwrap());
        id
    }

    /// Enable buyers to purchase assets by providing the buyer's address, the asset's address, and the agreed-upon price.
    pub fn buy_listing(env: Env, buyer: Address, id: u64, qty: i128) {
        buyer.require_auth();
        let storage = env.storage().persistent();
        if storage.get::<_, ()>(&DataKey::Initialized).is_none() {
            panic_with_error!(&env, Error::NotInitialized);
        }
        let token = storage.get(&DataKey::Token).unwrap();
        let mut assets: Map<u64, Asset> = storage.get(&DataKey::Assets).unwrap();
        let Asset {
            id,
            asset_address,
            owner: seller,
            price,
            quantity,
            listed,
        } = assets.get(id).unwrap();

        if !listed {
            panic_with_error!(&env, Error::AssetNotListed);
        }

        let token = token::Client::new(&env, &token);
        if token.balance(&buyer) < price * qty {
            panic_with_error!(&env, Error::BalanceTooLow);
        }
        token.transfer(&buyer, &seller, &(price * qty));
        assets.set(
            id,
            Asset {
                id,
                asset_address: asset_address.clone(),
                owner: buyer.clone(),
                price,
                quantity,
                listed: false,
            },
        );
        storage.set(&DataKey::Assets, &assets);

        let asset_client = Client::new(&env, &asset_address);
        asset_client.transfer(&env.current_contract_address(), &buyer, &qty);

        let topics = (Symbol::new(&env, "buy_listing"), (buyer));
        env.events().publish(topics, id);
    }

    /// Permit sellers to update the price of a listed asset,
    /// ensuring they provide the correct seller and asset addresses, as well as the old and new prices.
    pub fn update_price(env: Env, seller: Address, id: u64, new_price: i128) {
        if new_price <= 0 {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        seller.require_auth();
        let storage = env.storage().persistent();
        if storage.get::<_, ()>(&DataKey::Initialized).is_none() {
            panic_with_error!(&env, Error::NotInitialized);
        }
        let mut assets: Map<u64, Asset> = storage.get(&DataKey::Assets).unwrap();
        let Asset {
            id,
            asset_address,
            owner: set_seller,
            price,
            quantity,
            listed,
        } = assets.get(id).unwrap();
        if set_seller != seller {
            panic_with_error!(&env, Error::InvalidAuth);
        }

        assets.set(
            id,
            Asset {
                id,
                asset_address,
                owner: seller.clone(),
                price: new_price,
                quantity,
                listed,
            },
        );
        storage.set(&DataKey::Assets, &assets);
        let topics = (Symbol::new(&env, "update"), (seller));
        env.events().publish(topics, id);
    }

    /// Allow sellers to pause a listing by specifying their address, the asset's address,
    /// and the price at which it was listed.
    pub fn pause_listing(env: Env, seller: Address, id: u64) {
        seller.require_auth();
        let storage = env.storage().persistent();
        if storage.get::<_, ()>(&DataKey::Initialized).is_none() {
            panic_with_error!(&env, Error::NotInitialized);
        }
        let mut assets: Map<u64, Asset> = storage.get(&DataKey::Assets).unwrap();
        let Asset {
            asset_address,
            owner: set_seller,
            price,
            quantity,
            ..
        } = assets.get(id).unwrap();
        if set_seller != seller {
            panic_with_error!(&env, Error::InvalidAuth);
        }

        assets.set(
            id,
            Asset {
                id,
                asset_address,
                owner: seller.clone(),
                price,
                quantity,
                listed: false,
            },
        );
        storage.set(&DataKey::Assets, &assets);
        let topics = (Symbol::new(&env, "pause_listing"), (seller));
        env.events().publish(topics, id);
    }

    /// Allow sellers to un-pause a listing by specifying their address, the asset's address,
    /// and the price at which it is listed.
    pub fn unpause_listing(env: Env, seller: Address, id: u64) {
        seller.require_auth();
        let storage = env.storage().persistent();
        if storage.get::<_, ()>(&DataKey::Initialized).is_none() {
            panic_with_error!(&env, Error::NotInitialized);
        }
        let mut assets: Map<u64, Asset> = storage.get(&DataKey::Assets).unwrap();
        let Asset {
            id,
            asset_address,
            owner: set_seller,
            price,
            quantity,
            ..
        } = assets.get(id).unwrap();
        if set_seller != seller {
            panic_with_error!(&env, Error::InvalidAuth);
        }

        assets.set(
            id,
            Asset {
                id,
                asset_address,
                owner: set_seller,
                price,
                quantity,
                listed: true,
            },
        );
        storage.set(&DataKey::Assets, &assets);
        let topics = (Symbol::new(&env, "unpause_listing"), (seller));
        env.events().publish(topics, id);
    }
    /// Allow sellers to completely remove a listing by specifying their address, the asset's address,
    /// and the price at which it was listed.
    pub fn remove_listing(env: Env, seller: Address, id: u64) {
        seller.require_auth();
        let storage = env.storage().persistent();
        if storage.get::<_, ()>(&DataKey::Initialized).is_none() {
            panic_with_error!(&env, Error::NotInitialized);
        }
        let mut assets: Map<u64, Asset> = storage.get(&DataKey::Assets).unwrap();
        let Asset {
            asset_address,
            owner: set_seller,
            ..
        } = assets.get(id).unwrap();
        if set_seller != seller {
            panic_with_error!(&env, Error::InvalidAuth);
        }

        assets.remove(id).unwrap();
        storage.set(&DataKey::Assets, &assets);

        let asset_client = StellarAssetClient::new(&env, &asset_address);
        asset_client.set_admin(&seller);

        let topics = (Symbol::new(&env, "remove_listing"), (seller));
        env.events().publish(topics, id);
    }
}

#[cfg(test)]
mod test;
