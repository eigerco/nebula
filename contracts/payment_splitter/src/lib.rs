//! Payment splitter contract
//!

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, token, Address,
    ConversionError, Env, Vec,
};

#[contracterror]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    Conversion = 2,
    KeyExpected = 3,
    InvalidAmount = 4,
    NoStakeholders = 5
}

impl From<ConversionError> for Error {
    fn from(_: ConversionError) -> Self {
        Error::Conversion
    }
}

#[contracttype]
#[derive(Clone, Copy)]
pub enum DataKey {
    AlreadyInitialized = 0,
    Admin = 1,
    Token = 2,
    PaymentSplit = 3,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PaymentSplit {
    stakeholders: Vec<Address>,
}

#[contract]
pub struct PaymentSplitterContract;

#[contractimpl]
impl PaymentSplitterContract {
    /// Initialize the contract with a list of stakeholders to split the payments.
    pub fn init(
        env: Env,
        admin: Address,
        token: Address,
        stakeholders: Vec<Address>,
    ) -> Result<(), Error> {
        admin.require_auth();
        if stakeholders.is_empty() {
            panic_with_error!(&env, Error::NoStakeholders);
        }
        let storage = env.storage().persistent();
        if storage
            .get::<_, bool>(&DataKey::AlreadyInitialized)
            .is_some()
        {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }
        storage.set(&DataKey::AlreadyInitialized, &true);
        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Token, &token);
        storage.set(&DataKey::PaymentSplit, &PaymentSplit { stakeholders });
        Ok(())
    }

    /// Split an amount between the saved stakeholders
    pub fn split(env: Env, amount: i128) -> Result<(), Error> {
        if amount == 0 {
            panic_with_error!(&env, Error::InvalidAmount);
        }
        let storage = env.storage().persistent();
        let admin: Address = storage.get(&DataKey::Admin).ok_or(Error::KeyExpected)?;
        let token: Address = storage.get(&DataKey::Token).ok_or(Error::KeyExpected)?;
        admin.require_auth();
        let token = token::Client::new(&env, &token);
        let split = storage
            .get::<_, PaymentSplit>(&DataKey::PaymentSplit)
            .ok_or(Error::KeyExpected)?;
        let balance = token.balance(&admin);
        if amount > balance {
            panic_with_error!(&env, Error::InvalidAmount);
        }
        let payout = amount.checked_div(i128::from(split.stakeholders.len())).unwrap();

        for stakeholder in split.stakeholders {
            token.transfer(&admin, &stakeholder, &payout);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test;
