//! Payment splitter contract
//!
//! The payment splitter contract allows you to deploy a contract that sets a group of recipients.
//! The admin can invoke the payment splitting multiple times and split tokens between recipients
//!

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, token, Address,
    ConversionError, Env, Symbol, Vec,
};

#[contracterror]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    Conversion = 2,
    AdminKeyExpected = 3,
    InvalidAmount = 4,
    NoStakeholders = 5,
    NotInitialized = 6,
    TokenKeyExpected = 7,
    PaymentSplitExpected = 8
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
        if storage.get::<_, ()>(&DataKey::AlreadyInitialized).is_some() {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }
        storage.set(&DataKey::AlreadyInitialized, &());
        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Token, &token);
        storage.set(&DataKey::PaymentSplit, &PaymentSplit { stakeholders });
        Ok(())
    }

    /// Split an amount between the saved stakeholders
    pub fn split(env: Env, amount: i128) -> Result<(), Error> {
        if amount <= 0 {
            panic_with_error!(&env, Error::InvalidAmount);
        }
        let storage = env.storage().persistent();
        if storage.get::<_, ()>(&DataKey::AlreadyInitialized).is_none() {
            panic_with_error!(&env, Error::NotInitialized);
        }
        let admin: Address = storage.get(&DataKey::Admin).ok_or(Error::AdminKeyExpected)?;
        let token: Address = storage.get(&DataKey::Token).ok_or(Error::TokenKeyExpected)?;
        admin.require_auth();
        let token = token::Client::new(&env, &token);
        let split = storage
            .get::<_, PaymentSplit>(&DataKey::PaymentSplit)
            .ok_or(Error::PaymentSplitExpected)?;
        let balance = token.balance(&admin);
        if amount > balance {
            panic_with_error!(&env, Error::InvalidAmount);
        }
        let payout = amount
            .checked_div(i128::from(split.stakeholders.len()))
            .unwrap();

        for stakeholder in split.stakeholders {
            token.transfer(&admin, &stakeholder, &payout);
        }
        let topics = (Symbol::new(&env, "split"), &admin);
        env.events().publish(topics, payout);
        Ok(())
    }
}

#[cfg(test)]
mod test;
