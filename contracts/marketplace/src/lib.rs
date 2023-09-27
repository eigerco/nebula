use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, token, Address, Env, Map,
};

type Seller = Address;

#[derive(Clone, Copy)]
#[contracttype]
enum DataKey {
    Admin = 1,
    Percentage = 2,
    AlreadyInitialized = 3,
    Assets = 4,
    Token = 5,
}

#[contracterror]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidPercentage = 2,
    InvalidAssetPrice = 3,
    BalanceTooLow = 4,
    AssetNotListed = 5,
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
    /// Initialize the contract with the admin's address and a percentage for the admin fee.
    pub fn init(env: Env, token: Address, admin: Address, percentage: i128) {
        admin.require_auth();
        let storage = env.storage().persistent();

        if storage
            .get::<_, bool>(&DataKey::AlreadyInitialized)
            .is_some()
        {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }

        if percentage >= 100 || percentage < 0 {
            panic_with_error!(&env, Error::InvalidPercentage);
        }
        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Token, &token);
        storage.set(&DataKey::Percentage, &percentage);
        storage.set(&DataKey::AlreadyInitialized, &true);
        let assets: Map<Address, Asset> = storage.get(&DataKey::Assets).unwrap_or(Map::new(&env));
        storage.set(&DataKey::Assets, &assets);
    }

    pub fn get_listing(env: Env, asset: Address) -> Option<Asset> {
        let storage = env.storage().persistent();
        let assets: Map<Address, Asset> = storage.get(&DataKey::Assets).unwrap();
        assets.get(asset)
    }

    /// Allow sellers to list assets by specifying the seller's address, the asset's address, and the asset's price.
    pub fn create_listing(env: Env, seller: Seller, asset: Address, price: i128) {
        if price <= 0 {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        seller.require_auth();
        // TODO: Check if asset is owned by seller
        let storage = env.storage().persistent();
        let mut assets = storage.get(&DataKey::Assets).unwrap_or(Map::new(&env));
        assets.set(
            asset,
            Asset {
                owner: seller,
                price,
                listed: true,
            },
        );
        storage.set(&DataKey::Assets, &assets);
    }

    /// Enable buyers to purchase assets by providing the buyer's address, the asset's address, and the agreed-upon price.
    pub fn buy_listing(env: Env, buyer: Address, asset: Address, price: i128) {
        if price <= 0 {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        buyer.require_auth();
        let storage = env.storage().persistent();
        let token = storage.get(&DataKey::Token).unwrap();
        let mut assets: Map<Address, Asset> = storage.get(&DataKey::Assets).unwrap();
        let Asset {
            owner: seller,
            price: set_price,
            listed,
        } = assets.get(asset.clone()).unwrap();
        if listed == false {
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
            asset,
            Asset {
                owner: buyer,
                price,
                listed: false,
            },
        );
        storage.set(&DataKey::Assets, &assets);
    }

    /// Permit sellers to update the price of a listed asset,
    /// ensuring they provide the correct seller and asset addresses, as well as the old and new prices.
    pub fn update_listing(
        env: Env,
        seller: Address,
        asset: Address,
        old_price: i128,
        new_price: i128,
        listed: bool,
    ) {
        if new_price <= 0 {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        seller.require_auth();
        let storage = env.storage().persistent();
        let mut assets: Map<Address, Asset> = storage.get(&DataKey::Assets).unwrap();
        let Asset {
            owner: set_seller,
            price: set_price,
            ..
        } = assets.get(asset.clone()).unwrap();
        assert_eq!(set_seller, seller);

        if old_price != set_price {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        assets.set(
            asset,
            Asset {
                owner: seller,
                price: new_price,
                listed,
            },
        );
        storage.set(&DataKey::Assets, &assets);
    }

    /// Allow sellers to pause a listing by specifying their address, the asset's address,
    /// and the price at which it was listed.
    pub fn pause_listing(env: Env, seller: Address, asset: Address, price: i128) {
        if price <= 0 {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        seller.require_auth();
        let storage = env.storage().persistent();
        let mut assets: Map<Address, Asset> = storage.get(&DataKey::Assets).unwrap();
        let Asset {
            owner: set_seller,
            price: set_price,
            ..
        } = assets.get(asset.clone()).unwrap();
        assert_eq!(set_seller, seller);
        if price != set_price {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        assets.set(
            asset,
            Asset {
                owner: seller,
                price,
                listed: false,
            },
        );
        storage.set(&DataKey::Assets, &assets);
    }

    /// Allow sellers to completely remove a listing by specifying their address, the asset's address,
    /// and the price at which it was listed.
    pub fn remove_listing(env: Env, seller: Address, asset: Address, price: i128) {
        if price <= 0 {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        seller.require_auth();
        let storage = env.storage().persistent();
        let mut assets: Map<Address, Asset> = storage.get(&DataKey::Assets).unwrap();
        let Asset {
            owner: set_seller,
            price: set_price,
            ..
        } = assets.get(asset.clone()).unwrap();
        assert_eq!(set_seller, seller);
        if price != set_price {
            panic_with_error!(&env, Error::InvalidAssetPrice);
        }
        assets.remove(asset).unwrap();
        storage.set(&DataKey::Assets, &assets);
    }
}

#[cfg(test)]
mod test;
