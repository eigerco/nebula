//! Governance contract
//!
//! This contract provides the implementation of
//! a stake controlled DAO that allows participants
//! vote on:
//!
//! * Code upgrades.
//! * Curator changes.
//! * General proposals with arbitrary comments.
//!
//! This contract implements the following layers:
//!
//! * Voting layer. Delegated to the "voting" contract of this catalog,
//! which is called via cross contract calls.
//!
//! * Staking layer. Implemented on this contract. It allows
//! participants to stake and withdraw funds any time.
//!
//! * Governance layer. This contract itself. It manages the membership,
//! like join or leave operations, which implies also funds management
//! via the staking layer.
//!
//! The governance contract maintains an internal balance of all the participant
//! staked funds. The staked balance represents the voting power of each participant
//! at a given moment.
//!
//! Only when a proposal is finally executed by the proposer, the final results of participation
//! are stored in the voting contract storage, as the voting power (staking) can change
//! per each participant during voting as they stake,withdraw or leave the DAO.
//!
//! All participants needs to be "whitelisted" by the curator before they can create or vote proposals.
//!
//! The current voting mechanism requires a minimum participation configured at DAO initial setup
//! in order to consider a proposal "approved". Voting a proposal can only mean a positive vote.

#![no_std]

use participant::{Participant, Repository};
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, storage::Persistent,
    token, Address, BytesN, Env, Map, Symbol,
};

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
    ExecutedProposals = 6,
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
    // Participant cannot be found in storage.
    ParticipantNotFound = 4,
    // Participant needs to be whitelisted before doing certain operations.
    ParticipantNotWhitelisted = 5,
    // This is only triggered due to a programming error, when an expected storage key
    // is not available.
    ExpectedStorageKeyNotFound = 6,
    // Only approved proposals can be executed.
    ProposalNeedsApproval = 7,
    // Only the author can execute proposals.
    OnlyAuthorCanExecuteProposals = 8,
    // Proposals can only be executed once.
    AlreadyExecuted = 9,
}

#[contract]
pub struct GovernanceContract;

#[contractimpl]
impl GovernanceContract {
    /// It initializes the contract with all the needed parameters.
    /// It can only be executed once.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `curator` - The account address that can whitelist participants.
    /// - `token` - The token that accomplishes the token interface and this DAO uses as base currency.
    /// - `voting_period_secs` - The time a created proposal is open for voting.
    /// - `target_approval_rate_bps` - The default max number of participation for new proposals.
    /// - `salt` - A needed salt for generating addresses for the deployed contracts.
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
        storage.set(&DataKey::ExecutedProposals, &Map::<u64, ()>::new(&env));
    }

    /// Participants can join the DAO by invoking this function.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `participant_addr` - The participant address.
    /// - `amount` - The initial amount this user wants to participate with.
    pub fn join(env: Env, participant_addr: Address, amount: i128) -> Result<(), Error> {
        participant_addr.require_auth();

        let storage = env.storage().persistent();
        let mut participant_repo = participant::Repository::new(&storage)?;
        let mut participant = Participant::new(participant_addr.clone());

        Self::stake_funds(&env, &mut participant, amount)?;

        participant_repo.save(participant);

        env.events().publish(
            (Symbol::new(&env, "participant_joined"), participant_addr),
            participant_repo.count(),
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

    /// Participants can increase their staked amounts any time.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `participant_addr` - The participant address.
    /// - `amount` - The initial amount this user wants to participate with.
    pub fn stake(env: Env, participant: Address, amount: i128) -> Result<(), Error> {
        participant.require_auth();

        let storage = env.storage().persistent();

        let mut participant_repo = participant::Repository::new(&storage)?;

        let mut stored_participant = participant_repo.find(participant.clone())?;

        Self::stake_funds(&env, &mut stored_participant, amount)?;

        participant_repo.save(stored_participant);

        Ok(())
    }

    /// Participants can leave anytime, withdrawing all amounts.
    /// Once the leave, they need to be whitelisted again.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `participant_addr` - The participant address.
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

    /// Participants can withdraw their staked amounts any time.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `participant_addr` - The participant address.
    /// - `amount` - The initial amount this user wants to participate with.
    pub fn withdraw(env: Env, participant: Address, amount: i128) -> Result<(), Error> {
        participant.require_auth();

        let storage = env.storage().persistent();
        let mut participant_repo = participant::Repository::new(&storage)?;

        let mut stored_participant = participant_repo.find(participant.clone())?;

        Self::withdraw_funds(&env, &mut stored_participant, amount)?;

        participant_repo.save(stored_participant);

        Ok(())
    }

    /// Only curator can invoke this function for whitelisting a participant.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `participant_addr` - The participant address for whitelisting.
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

    /// Only curator can invoke this function for whitelisting a participant.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `participant` - The proposer who is creating this proposal.
    /// - `id` -  The unique ID of the proposal. This can be taken from external systems.
    /// - `payload` - The ['voting_contract::ProposalPayload'] , that represents a Proposal king + its respective payload
    pub fn new_proposal(
        env: Env,
        participant: Address,
        id: u64,
        payload: voting_contract::ProposalPayload,
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

        voting_client.create_proposal(&participant, &id, &payload);

        Ok(())
    }

    /// Any whitelisted participant can vote on a proposal.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `participant` - The proposer who is creating this proposal.
    /// - `id` -  The unique ID of the proposal.
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

        Ok(())
    }

    /// Only a whitelisted participant, who is the proposer, can execute the given
    /// proposal.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `participant` - The proposer who is executing this proposal.
    /// - `id` -  The unique ID of the proposal.
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

        if Self::is_proposal_executed(&storage, id) {
            return Err(Error::AlreadyExecuted);
        }

        if !voting_client.is_proposal_approved_for_balance(&proposal.id, &whitelisted_balance) {
            return Err(Error::ProposalNeedsApproval);
        }

        match proposal.payload {
            voting_contract::ProposalPayload::Comment(_) => {}
            voting_contract::ProposalPayload::CodeUpgrade(wasm_hash) => {
                env.deployer().update_current_contract_wasm(wasm_hash)
            }
            voting_contract::ProposalPayload::NewCurator(address) => {
                storage.set(&DataKey::Curator, &address);
            }
        }

        Self::mark_proposal_as_executed(&storage, id);

        voting_client.update_proposal_with_balance(&id, &whitelisted_balance);

        env.events().publish(
            (Symbol::new(&env, "proposal_executed"), &participant, id),
            (),
        );

        Ok(())
    }

    fn is_proposal_executed(storage: &Persistent, id: u64) -> bool {
        Self::executed_proposal_storage(storage).contains_key(id)
    }

    fn mark_proposal_as_executed(storage: &Persistent, id: u64) {
        let mut executed_proposal_storage = Self::executed_proposal_storage(storage);
        executed_proposal_storage.set(id, ());
        storage.set(&DataKey::ExecutedProposals, &executed_proposal_storage);
    }

    fn executed_proposal_storage(storage: &Persistent) -> Map<u64, ()> {
        storage
            .get::<_, Map<u64, ()>>(&DataKey::ExecutedProposals)
            .unwrap()
    }
}

#[cfg(test)]
mod test;
