use soroban_sdk::{contracttype, storage::Persistent, Address, Env, Map};

use crate::{DataKey, Error};

#[derive(Debug, Clone)]
#[contracttype]
pub struct Participant {
    address: Address,
    whitelisted: bool,
    current_balance: i128,
}

impl Participant {
    pub fn new(address: Address) -> Self {
        Participant {
            address,
            whitelisted: false,
            current_balance: 0,
        }
    }

    pub fn is_whitelisted(&self) -> bool {
        self.whitelisted
    }

    pub fn whitelist(&mut self) {
        self.whitelisted = true
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn balance(&self) -> i128 {
        self.current_balance
    }

    pub fn increase_balance(&mut self, inc: i128) -> Result<(), Error> {
        if inc <= 0 {
            return Err(Error::InvalidAmount);
        }
        self.current_balance = self.current_balance.checked_add(inc).unwrap();
        Ok(())
    }

    pub fn decrease_balance(&mut self, dec: i128) -> Result<(), Error> {
        if dec <= 0 {
            return Err(Error::InvalidAmount);
        }
        if dec > self.balance() {
            return Err(Error::InsufficientFunds);
        }
        self.current_balance = self.current_balance.checked_sub(dec).unwrap();
        Ok(())
    }
}

/// Participant repository provides a pattern for
/// condensing all storage logic for the participant.
///
/// It consists on adquiring the participants data structure
/// from the Soroban storage, do all needed operations and
/// finally persist them once the object is dropped.
pub struct Repository<'a> {
    storage: &'a Persistent,
    data_key: DataKey,
    participants_storage: Map<Address, Participant>,
}

impl<'a> Repository<'a> {
    pub fn new(storage: &'a Persistent) -> Result<Self, Error> {
        let participants_storage = storage
            .get::<_, Map<Address, Participant>>(&DataKey::Participants)
            .ok_or(Error::AlreadyInitialized)?;

        Ok(Repository {
            storage,
            participants_storage,
            data_key: DataKey::Participants,
        })
    }

    pub fn save(&mut self, participant: Participant) {
        self.participants_storage
            .set(participant.address.clone(), participant);
    }

    pub fn find(&mut self, address: Address) -> Result<Participant, Error> {
        self.participants_storage
            .get(address)
            .ok_or(Error::ParticipantNotFound)
    }

    pub fn remove(&mut self, address: Address) -> Result<(), Error> {
        self.participants_storage
            .remove(address)
            .ok_or(Error::ParticipantNotFound)
    }

    pub fn whitelisted_balance(&self, env: &Env) -> Map<Address, i128> {
        self.participants_storage.iter().fold(
            Map::<Address, i128>::new(env),
            |mut whitelisted_balance, (addr, participant)| {
                if participant.is_whitelisted() {
                    whitelisted_balance.set(addr, participant.balance())
                }
                whitelisted_balance
            },
        )
    }
}

impl<'a> Drop for Repository<'a> {
    fn drop(&mut self) {
        self.storage.set(&self.data_key, &self.participants_storage)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn participant_can_only_increase_positive_amounts() {
        let env = Env::default();
        let mut p = Participant::new(Address::random(&env));
        assert_eq!(Err(Error::InvalidAmount), p.increase_balance(0));
        assert_eq!(Err(Error::InvalidAmount), p.increase_balance(-1));
        p.increase_balance(1).unwrap();
    }

    #[test]
    fn participant_can_only_decrease_positive_amounts() {
        let env = Env::default();
        let mut p = Participant::new(Address::random(&env));
        p.increase_balance(5).unwrap();

        assert_eq!(Err(Error::InvalidAmount), p.decrease_balance(0));
        assert_eq!(Err(Error::InvalidAmount), p.decrease_balance(-1));
        p.decrease_balance(1).unwrap();
    }

    #[test]
    fn participant_cannot_decrease_more_than_it_has() {
        let env = Env::default();
        let mut p = Participant::new(Address::random(&env));
        p.increase_balance(1).unwrap();
        assert_eq!(Err(Error::InsufficientFunds), p.decrease_balance(2));
    }

}
