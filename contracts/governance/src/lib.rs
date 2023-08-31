//! Governance contract
//!
//! This contract provides the implementation of
//! a stake controlled DAO that allows participants
//! vote on code upgrades.

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, token, Address, Env,
    Map, Symbol,
};

/// Datakey holds all possible storage keys this
/// contract uses. See https://soroban.stellar.org/docs/getting-started/storing-data .
#[derive(Clone, Copy)]
#[contracttype]
enum DataKey {
    Initialized = 1,
    Curator = 2,
    Token = 3,
    Participants = 4,
}

/// All the expected errors this contract expects.
/// This error codes will appear as output in the transaction
/// receipt.
#[contracterror]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
enum Error {
    /// The contract should be only initialized once.
    AlreadyInitialized = 1,
    // Must have funds for the operation.
    InsufficientFunds = 2,
    // Amounts cannot be negative in some operations.
    UnderZeroAmount = 3,
}

#[contract]
pub struct GovernanceContract;

#[contractimpl]
impl GovernanceContract {
    pub fn init(env: Env, curator: Address, token: Address) {
        let storage = env.storage().persistent();

        if storage.has(&DataKey::Initialized) {
            panic_with_error!(&env, Error::AlreadyInitialized)
        }

        storage.set(&DataKey::Initialized, &());
        storage.set(&DataKey::Curator, &curator);
        storage.set(&DataKey::Token, &token);
        storage.set(
            &DataKey::Participants,
            &Map::<Address, Participant>::new(&env),
        );
    }

    pub fn join(env: Env, participant_addr: Address, initial_stake: i128) {
        participant_addr.require_auth();

        if initial_stake <= 0 {
            panic_with_error!(&env, Error::UnderZeroAmount);
        }

        let storage = env.storage().persistent();
        let token_addr = storage.get::<_, Address>(&DataKey::Token).unwrap();
        let token_client = token::Client::new(&env, &token_addr);
        let balance = token_client.balance(&participant_addr);

        if balance < initial_stake {
            panic_with_error!(&env, Error::InsufficientFunds)
        }

        token_client.transfer(
            &participant_addr,
            &env.current_contract_address(),
            &initial_stake,
        );

        let mut participants = storage
            .get::<_, Map<Address, Participant>>(&DataKey::Participants)
            .unwrap();

        participants.set(
            participant_addr.clone(),
            Participant::new(participant_addr.clone(), initial_stake),
        );

        storage.set(&DataKey::Participants, &participants);

        env.events().publish(
            (Symbol::new(&env, "participant_joined"), participant_addr),
            initial_stake,
        );
    }
}

#[contracttype]
struct Participant {
    address: Address,
    whitelisted: bool,
    current_balance: i128,
}

impl Participant {
    pub fn new(address: Address, initial_stake: i128) -> Self {
        Participant {
            address,
            whitelisted: false,
            current_balance: initial_stake,
        }
    }
}

#[cfg(test)]
mod test;
