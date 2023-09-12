//! Governance contract
//!
//! This contract provides the implementation of
//! a stake controlled DAO that allows participants
//! vote on code upgrades.

#![no_std]

use participant::{Participant, Repository};
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, token, Address, BytesN,
    Env, Map, Symbol,
};
use voting_contract::ProposalType;

#[allow(clippy::too_many_arguments)]
mod voting_contract {
    soroban_sdk::contractimport!(file = "../../target/wasm32-unknown-unknown/release/voting.wasm");
}

mod participant;

/// Datakey holds all possible storage keys this
/// contract uses. See https://soroban.stellar.org/docs/getting-started/storing-data .
#[derive(Clone, Copy)]
#[contracttype]
enum DataKey {
    Initialized = 1,
    Curator = 2,
    Token = 3,
    Participants = 4,
    VotingContractAddress = 5,
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
    // Certain amounts are not valid in some operations.(Like under and/or equal to zero)
    InvalidAmount = 3,
    ParticipantNotFound = 4,
    ParticipantNotWhitelisted = 5,
    ExpectedStorageKeyNotFound = 6,
    ProposalNeedsApproval = 7,
    OnlyAuthorCanExecuteProposals = 8,
}

#[contract]
pub struct GovernanceContract;

#[contractimpl]
impl GovernanceContract {
    pub fn init(
        env: Env,
        curator: Address,
        token: Address,
        voting_period_secs: u64,
        target_approval_rate_bps: u32,
        salt: BytesN<32>,
    ) {
        let storage = env.storage().persistent();

        if storage.has(&DataKey::Initialized) {
            panic_with_error!(&env, Error::AlreadyInitialized)
        }

        // Deploy the voting contract (A dependency of this one)
        let voting_contract_hash = env.deployer().upload_contract_wasm(voting_contract::WASM);
        let deployer = env.deployer().with_current_contract(salt);
        let voting_contract_address = deployer.deploy(voting_contract_hash);
        let voting_client = voting_contract::Client::new(&env, &voting_contract_address);

        // Init the voting contract.
        voting_client.init(
            &env.current_contract_address(),
            &voting_period_secs,
            &target_approval_rate_bps,
            &u128::MAX, // This is a dummy value as participation state will be managed by this contract due to data locality.  It needs to be positive.
            &true,      // Only this contract can do operation on behalf of the participants.
        );

        env.events().publish(
            (Symbol::new(&env, "voting_contract_initialized"),),
            voting_contract_address.clone(),
        );

        storage.set(&DataKey::Initialized, &());
        storage.set(&DataKey::Curator, &curator);
        storage.set(&DataKey::Token, &token);
        storage.set(&DataKey::VotingContractAddress, &voting_contract_address);
        storage.set(
            &DataKey::Participants,
            &Map::<Address, Participant>::new(&env),
        );
    }

    pub fn join(env: Env, participant_addr: Address, amount: i128) -> Result<(), Error> {
        participant_addr.require_auth();

        let storage = env.storage().persistent();
        let mut participant_repo = participant::Repository::new(&storage)?;
        let mut participant = Participant::new(participant_addr.clone());

        Self::stake_funds(&env, &mut participant, amount)?;

        participant_repo.save(participant);

        env.events().publish(
            (Symbol::new(&env, "participant_joined"), participant_addr),
            (),
        );
        Ok(())
    }

    fn stake_funds(env: &Env, participant: &mut Participant, amount: i128) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let storage = env.storage().persistent();
        let token_addr = storage.get::<_, Address>(&DataKey::Token).unwrap();
        let token_client = token::Client::new(env, &token_addr);
        let balance = token_client.balance(participant.address());

        if balance < amount {
            return Err(Error::InsufficientFunds);
        }

        token_client.transfer(
            participant.address(),
            &env.current_contract_address(),
            &amount,
        );

        participant.increase_balance(amount)?;

        env.events()
            .publish((Symbol::new(env, "stake"), participant.address()), amount);
        Ok(())
    }

    pub fn stake(env: Env, participant: Address, amount: i128) -> Result<(), Error> {
        participant.require_auth();

        let storage = env.storage().persistent();

        let mut participant_repo = participant::Repository::new(&storage)?;

        let mut stored_participant = participant_repo.find(participant.clone())?;

        Self::stake_funds(&env, &mut stored_participant, amount)?;

        participant_repo.save(stored_participant);

        Ok(())
    }

    pub fn leave(env: Env, participant: Address) -> Result<(), Error> {
        participant.require_auth();

        let storage = env.storage().persistent();
        let mut participant_repo = participant::Repository::new(&storage)?;

        let mut stored_participant = participant_repo.find(participant.clone())?;

        let amount = stored_participant.balance();

        Self::withdraw_funds(&env, &mut stored_participant, amount)?;

        participant_repo.remove(participant.clone())?;

        env.events()
            .publish((Symbol::new(&env, "participant_left"), &participant), ());

        Ok(())
    }

    fn withdraw_funds(env: &Env, participant: &mut Participant, amount: i128) -> Result<(), Error> {
        let storage = env.storage().persistent();
        let token_addr = storage.get::<_, Address>(&DataKey::Token).unwrap();
        let token_client = token::Client::new(env, &token_addr);

        participant.decrease_balance(amount)?;

        token_client.transfer(
            &env.current_contract_address(),
            participant.address(),
            &amount,
        );

        env.events().publish(
            (Symbol::new(env, "withdraw"), participant.address()),
            amount,
        );

        Ok(())
    }

    pub fn withdraw(env: Env, participant: Address, amount: i128) -> Result<(), Error> {
        participant.require_auth();

        let storage = env.storage().persistent();
        let mut participant_repo = participant::Repository::new(&storage)?;

        let mut stored_participant = participant_repo.find(participant.clone())?;

        Self::withdraw_funds(&env, &mut stored_participant, amount)?;

        participant_repo.save(stored_participant);

        Ok(())
    }

    pub fn whitelist(env: Env, participant: Address) -> Result<(), Error> {
        let storage = env.storage().persistent();
        let curator = storage.get::<_, Address>(&DataKey::Curator).unwrap();
        curator.require_auth();

        let mut participant_repo = participant::Repository::new(&storage)?;

        let mut stored_participant = participant_repo.find(participant.clone())?;

        stored_participant.whitelist();

        participant_repo.save(stored_participant);

        env.events()
            .publish((Symbol::new(&env, "participant_whitelisted"),), participant);

        Ok(())
    }

    pub fn new_proposal(
        env: Env,
        participant: Address,
        id: u64,
        kind: ProposalType,
        new_contract_hash: BytesN<32>,
    ) -> Result<(), Error> {
        participant.require_auth();

        let storage = env.storage().persistent();
        let mut participant_repo = participant::Repository::new(&storage)?;

        let stored_participant = participant_repo.find(participant.clone())?;

        if !stored_participant.is_whitelisted() {
            return Err(Error::ParticipantNotWhitelisted);
        }

        let voting_address = storage
            .get::<_, Address>(&DataKey::VotingContractAddress)
            .unwrap();

        let voting_client = voting_contract::Client::new(&env, &voting_address);

        voting_client.create_proposal(&participant, &id, &kind, &new_contract_hash);

        env.events()
            .publish((Symbol::new(&env, "new_proposal"), &participant, kind), id);

        Ok(())
    }

    pub fn vote(env: Env, participant: Address, id: u64) -> Result<(), Error> {
        participant.require_auth();

        let storage = env.storage().persistent();
        let mut participant_repo = participant::Repository::new(&storage)?;

        let stored_participant = participant_repo.find(participant.clone())?;

        if !stored_participant.is_whitelisted() {
            return Err(Error::ParticipantNotWhitelisted);
        }

        let voting_address = storage
            .get::<_, Address>(&DataKey::VotingContractAddress)
            .unwrap();

        let voting_client = voting_contract::Client::new(&env, &voting_address);

        voting_client.vote(&participant, &id);

        env.events()
            .publish((Symbol::new(&env, "proposal_voted"), &participant, id), ());

        Ok(())
    }

    pub fn execute_proposal(env: Env, participant: Address, id: u64) -> Result<(), Error> {
        participant.require_auth();

        let storage = env.storage().persistent();
        let mut participant_repo = Repository::new(&storage)?;

        let stored_participant = participant_repo.find(participant.clone())?;

        if !stored_participant.is_whitelisted() {
            return Err(Error::ParticipantNotWhitelisted);
        }

        let voting_address = storage
            .get::<_, Address>(&DataKey::VotingContractAddress)
            .unwrap();

        let voting_client = voting_contract::Client::new(&env, &voting_address);

        let whitelisted_balance = participant_repo.whitelisted_balance(&env);

        let proposal = voting_client.find_proposal(&id);

        if proposal.proposer != participant {
            return Err(Error::OnlyAuthorCanExecuteProposals);
        }

        if !voting_client.is_proposal_approved_for_balance(&proposal.id, &whitelisted_balance) {
            return Err(Error::ProposalNeedsApproval);
        }

        match proposal.kind {
            ProposalType::Standard => {
                // TODO - should we do anything for standard proposal ?
            }
            ProposalType::CodeUpgrade => env
                .deployer()
                .update_current_contract_wasm(proposal.comment),
            ProposalType::CuratorChange => {
                let new_curator = utils::bytes_n32_to_address(&proposal.comment);
                storage.set(&DataKey::Curator, &new_curator);
            }
        }

        voting_client.update_proposal_with_balance(&id, &whitelisted_balance);

        env.events().publish(
            (Symbol::new(&env, "proposal_executed"), &participant, id),
            (),
        );

        Ok(())
    }
}

#[cfg(test)]
mod test;
