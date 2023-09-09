use soroban_sdk::{storage::Persistent, Address, Map};

use crate::{DataKey, Error, Participant};

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
        self
            .participants_storage
            .get(address)
            .ok_or(Error::ParticipantNotFound)
    }

    pub fn remove(&mut self, address: Address) -> Result<(), Error> {
        self
            .participants_storage
            .remove(address)
            .ok_or(Error::ParticipantNotFound)
    }
}

impl<'a> Drop for Repository<'a> {
    fn drop(&mut self) {
        self.storage.set(&self.data_key, &self.participants_storage)
    }
}
