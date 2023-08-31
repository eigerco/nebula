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
pub enum Error {
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

    pub fn leave(env: Env, participant: Address) {
        participant.require_auth();

        let storage = env.storage().persistent();

        let mut participants = storage
            .get::<_, Map<Address, Participant>>(&DataKey::Participants)
            .unwrap();

        let mut stored_participant = participants.get(participant.clone()).unwrap();

        let amount = stored_participant.current_balance;

        Self::withdraw_stake(&env, &mut stored_participant, amount).unwrap();

        participants.remove(participant.clone());
        storage.set(&DataKey::Participants, &participants);

        env.events()
            .publish((Symbol::new(&env, "participant_left"), &participant), ());
    }

    fn withdraw_stake(env: &Env, participant: &mut Participant, amount: i128) -> Result<(), Error> {
        if participant.current_balance < amount {
            return Err(Error::InsufficientFunds);
        }

        let storage = env.storage().persistent();
        let token_addr = storage.get::<_, Address>(&DataKey::Token).unwrap();
        let token_client = token::Client::new(env, &token_addr);

        token_client.transfer(
            &env.current_contract_address(),
            &participant.address,
            &amount,
        );

        participant.current_balance -= amount;

        env.events()
            .publish((Symbol::new(env, "withdraw"), &participant.address), amount);

        Ok(())
    }

    pub fn withdraw(env: Env, participant: Address, amount: i128) -> Result<(), Error> {
        participant.require_auth();

        let storage = env.storage().persistent();

        let mut participants = storage
            .get::<_, Map<Address, Participant>>(&DataKey::Participants)
            .unwrap();

        let mut stored_participant = participants.get(participant.clone()).unwrap();

        Self::withdraw_stake(&env, &mut stored_participant, amount)?;

        participants.set(participant.clone(), stored_participant);
        storage.set(&DataKey::Participants, &participants);
        Ok(())
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
