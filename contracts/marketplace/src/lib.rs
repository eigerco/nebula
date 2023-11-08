//! # Marketplace Contract
//!
//! The marketplace contract enables the creation and management of listings for NFT assets
//! which are already created and need to be sold.
//!
//! It accepts any token that accomplished the token interface for trading. See https://soroban.stellar.org/docs/reference/interfaces/token-interface
//!
//! See public function contracts documentation for further explanations regarding the available
//! actions.
#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error,
    storage::Persistent,
    token::{self, Client},
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

/// Error enum holds all errors this contract contemplates as part of its
/// business logic.
#[contracterror]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    // The contract should not be initialized twice.
    AlreadyInitialized = 1,
    // Triggered if i.e an asset tried to be submitted with a negative price.
    InvalidAssetPrice = 2,
    // Some party has not enough funds to complete the operation.
    BalanceTooLow = 3,
    // Triggered when a marketplace operation is tried over a non existent asset.
    AssetNotListed = 4,
    // Triggered if any of the functions of this contract are tried.
    NotInitialized = 6,
    // Asset quantities cannot be under zero.
    InvalidQuantity = 7,
}

/// Asset represents a listed asset.
#[contracttype]
#[derive(Clone)]
pub struct Asset {
    // This is an auto increment id provided by the contract in the moment
    // of creation.
    id: u64,
    // The address of the asset token contract.
    asset_address: Address,
    // The owner true of the contract. As the ownership temporary changes to the contract
    // itself, so able to act on behalf of the user, we need a return address in case the
    // original owner wants to recover the asset.
    owner: Address,
    // The price of the asset in the listing.
    price: i128,
    // The quantity of assets to sell on an specific listing.
    quantity: i128,
    // This property is responsible of determining if a buy operation can happen. Actually,
    // used for pausing the listing of an asset.
    listed: bool,
}

type AssetStorage = Map<u64, Asset>;

#[contract]
pub struct MarketplaceContract;

#[contractimpl]
impl MarketplaceContract {
    /// It initializes the contract with all the needed parameters.
    /// This function must be invoked by the administrator just
    /// after the contract deployment. No other actions can be
    /// performed before this one.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `token` - The address of the token the contract his contract will use as trading pair. (i.e NFT for XLM)
    /// - `admin` - The address that can create proposals.
    pub fn init(env: Env, token: Address, admin: Address) {
        admin.require_auth();
        let storage = env.storage().persistent();

        if storage.get::<_, ()>(&DataKey::Initialized).is_some() {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }

        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Token, &token);
        storage.set(&DataKey::Initialized, &());
        storage.set(&DataKey::Assets, &Map::<u64, Asset>::new(&env));
        storage.set(&DataKey::LastID, &1u64);
    }

    /// This is a workaround for an under investigation bug. See https://github.com/eigerco/nebula/issues/41.
    pub fn register(_: Env, trader: Address) {
        trader.require_auth();
    }

    /// It gets a listing based on an asset id
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `id` - Id of the asset, previously facilitated by the ['create_listing'] operation.
    pub fn get_listing(env: Env, id: u64) -> Option<Asset> {
        let storage = env.storage().persistent();
        Self::must_be_initialized(&env, &storage);
        let assets: AssetStorage = storage.get(&DataKey::Assets).unwrap();
        assets.get(id)
    }

    /// Allow sellers to list assets. After this operation, the ownership of the assets will
    /// pass to this contract.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `seller` - The address of the account that wants to sell. Authorization will be enforced.
    /// - `asset` - The address of the NFT to be listed. It should accomplish the token interface. See https://soroban.stellar.org/docs/reference/interfaces/token-interface .
    /// - `price` - The price of the listing, in the trading token specified in the ['init'] function.
    /// - `quantity` - The amount of the same NFT that is being sold.
    pub fn create_listing(
        env: Env,
        seller: Address,
        asset: Address,
        price: i128,
        quantity: i128,
    ) -> u64 {
        seller.require_auth();
        if price <= 0 {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        if quantity <= 0 {
            panic_with_error!(&env, Error::InvalidQuantity);
        }
        let storage = env.storage().persistent();

        Self::must_be_initialized(&env, &storage);

        let mut assets: AssetStorage = storage.get(&DataKey::Assets).unwrap();
        let id = Self::current_id(&storage);
        assets.set(
            id,
            Asset {
                id,
                asset_address: asset.clone(),
                owner: seller.clone(),
                price,
                quantity,
                listed: true,
            },
        );
        storage.set(&DataKey::Assets, &assets);

        let asset_client = Client::new(&env, &asset);

        if asset_client.balance(&seller) < quantity {
            panic_with_error!(&env, Error::BalanceTooLow);
        }

        asset_client.transfer(&seller, &env.current_contract_address(), &quantity);

        let topics = (Symbol::new(&env, "create_listing"), (seller));
        env.events().publish(topics, id);

        id
    }

    fn must_be_initialized(env: &Env, storage: &Persistent) {
        if storage.get::<_, ()>(&DataKey::Initialized).is_none() {
            panic_with_error!(&env, Error::NotInitialized);
        }
    }

    fn current_id(storage: &Persistent) -> u64 {
        let id: u64 = storage.get(&DataKey::LastID).unwrap();
        storage.set(&DataKey::LastID, &id.checked_add(1).unwrap());
        id
    }

    /// Enable buyers to purchase any of the listed assets.
    /// They must not be "paused" or an error will be raised.
    /// When this operation completes, the asset will be removed
    /// from the listing.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `buyer` - Address of the account that is buying the current offer.
    /// - `id` - The id of the offer to be bought.
    pub fn buy_listing(env: Env, buyer: Address, id: u64) {
        buyer.require_auth();
        let storage = env.storage().persistent();

        Self::must_be_initialized(&env, &storage);

        let token = storage.get(&DataKey::Token).unwrap();
        let mut assets: AssetStorage = storage.get(&DataKey::Assets).unwrap();
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
        if token.balance(&buyer) < price * quantity {
            panic_with_error!(&env, Error::BalanceTooLow);
        }
        token.transfer(&buyer, &seller, &(price * quantity));
        assets.remove(id);
        storage.set(&DataKey::Assets, &assets);

        let asset_client = Client::new(&env, &asset_address);
        asset_client.transfer(&env.current_contract_address(), &buyer, &quantity);

        let topics = (Symbol::new(&env, "buy_listing"), (buyer));
        env.events().publish(topics, id);
    }

    /// Permit sellers to update the price of a listed asset
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `id` - The id of the listed asset to be updated.
    /// - `new_price` - The new, updated price.
    pub fn update_price(env: Env, id: u64, new_price: i128) {
        if new_price <= 0 {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }

        let storage = env.storage().persistent();

        Self::must_be_initialized(&env, &storage);

        let mut assets: AssetStorage = storage.get(&DataKey::Assets).unwrap();
        let Asset {
            id,
            asset_address,
            owner: seller,
            quantity,
            listed,
            ..
        } = assets.get(id).unwrap();

        seller.require_auth();

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
        let topics = (Symbol::new(&env, "update_price"), (seller));
        env.events().publish(topics, id);
    }

    /// Allow sellers to pause a listing. No buy operation can be performed on this listing from this point,
    /// unless ['unpause_listing'] is called.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `id` - The id of the listed asset to be paused.
    pub fn pause_listing(env: Env, id: u64) {
        let storage = env.storage().persistent();

        Self::must_be_initialized(&env, &storage);

        let mut assets: AssetStorage = storage.get(&DataKey::Assets).unwrap();
        let Asset {
            asset_address,
            owner,
            price,
            quantity,
            ..
        } = assets.get(id).unwrap();

        owner.require_auth();

        assets.set(
            id,
            Asset {
                id,
                asset_address,
                owner: owner.clone(),
                price,
                quantity,
                listed: false,
            },
        );
        storage.set(&DataKey::Assets, &assets);
        let topics = (Symbol::new(&env, "pause_listing"), (owner));
        env.events().publish(topics, id);
    }

    /// Allow sellers to un-pause a previously paused listing. Buy operations are enabled again on this listing from this point.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `id` - The id of the listed asset to be unpaused.
    pub fn unpause_listing(env: Env, id: u64) {
        let storage = env.storage().persistent();

        Self::must_be_initialized(&env, &storage);

        let mut assets: AssetStorage = storage.get(&DataKey::Assets).unwrap();
        let Asset {
            id,
            asset_address,
            owner,
            price,
            quantity,
            ..
        } = assets.get(id).unwrap();

        owner.require_auth();

        assets.set(
            id,
            Asset {
                id,
                asset_address,
                owner: owner.clone(),
                price,
                quantity,
                listed: true,
            },
        );
        storage.set(&DataKey::Assets, &assets);
        let topics = (Symbol::new(&env, "unpause_listing"), (owner));
        env.events().publish(topics, id);
    }

    /// Allow sellers to completely remove a listing. This operation cannot be undone and
    /// will return all the balances to the respective owners (sellers).
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `id` - The id of the listed asset to be removed.
    pub fn remove_listing(env: Env, id: u64) {
        let storage = env.storage().persistent();

        Self::must_be_initialized(&env, &storage);

        let mut assets: AssetStorage = storage.get(&DataKey::Assets).unwrap();
        let Asset { owner, .. } = assets.get(id).unwrap();

        owner.require_auth();

        assets.remove(id).unwrap();
        storage.set(&DataKey::Assets, &assets);

        let topics = (Symbol::new(&env, "remove_listing"), (owner));
        env.events().publish(topics, id);
    }
}

#[cfg(test)]
mod test;
