//! Voting contract
//!
//! This is a simple version of voting contract that allow
//! admins to create and users to vote on multiple proposals.
//!
//! The implemented time lock, defined when the proposals
//! are in closed state. That is, when a proposal deadline
//! is reached, no other action can be performed on it and
//! the current result will become the final one.
//!
//! Proposals are identified by an unique ID, that might
//! be maintained by external applications.
//!
//! Currently only admin of the contract can create proposals,
//! and anyone can vote on them.
//!
//! Theres is an "only admin" mode that can be activated upon
//! initialization and that will restrict all operations for not
//! admin users.

#![no_std]

use shared::voting::{Error, Proposal, ProposalPayload};
use soroban_sdk::{
    contract, contractimpl, contracttype, panic_with_error, storage::Persistent, Address, Env, Map,
    Symbol,
};

/// Datakey holds all possible storage keys this
/// contract uses. See https://soroban.stellar.org/docs/getting-started/storing-data .
#[contracttype]
#[derive(Clone, Copy)]
enum DataKey {
    AlreadyInitialized = 0,
    Admin = 1,
    VoterList = 2,
    Proposals = 3,
    VotingPeriodSecs = 4,
    TargetApprovalRate = 5,
    TotalVoters = 6,
    AdminMode = 7,
}

#[contract]
pub struct ProposalVotingContract;

#[contractimpl]
impl ProposalVotingContract {
    /// It initializes the contract with all the needed parameters.
    /// This function must be invoked by the administrator just
    /// after the contract deployment.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `admin` - The address that can create proposals.
    /// - `voting_period_secs` - The default number of seconds of proposals lifetime for new proposals.
    /// - `target_approval_rate_bps` - The default required approval rate in basic points for new proposals.
    /// - `participation` - The default max number of participation for new proposals.
    /// - `admin_mode` - Certain functions like `voting` are open for anyone who wants to invoke them. This
    /// doable, but if we are using this contract as dependency of another contract and we are interested in
    /// restricting all operations to be only performed by the Admin address (see admin params), this should
    /// be set to true.
    pub fn init(
        env: Env,
        admin: Address,
        voting_period_secs: u64,
        target_approval_rate_bps: u32,
        participation: u128,
        admin_mode: bool,
    ) {
        let storage = env.storage().persistent();

        if storage
            .get::<_, ()>(&DataKey::AlreadyInitialized)
            .is_some()
        {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }

        if voting_period_secs == 0 {
            panic_with_error!(&env, Error::InvalidVotingPeriod);
        }

        if target_approval_rate_bps == 0 {
            panic_with_error!(&env, Error::InvalidTargetApprovalRate);
        }

        if participation == 0 {
            panic_with_error!(&env, Error::NotEnoughParticipants);
        }

        storage.set(&DataKey::AlreadyInitialized, &());
        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Proposals, &Map::<u64, Proposal>::new(&env));
        // Todo, to better study if this parameters would be better as hardcoded values, due to fees. See https://soroban.stellar.org/docs/fundamentals-and-concepts/fees-and-metering#resource-fee .
        storage.set(&DataKey::VotingPeriodSecs, &voting_period_secs);
        storage.set(&DataKey::TargetApprovalRate, &target_approval_rate_bps);
        storage.set(&DataKey::TotalVoters, &participation);
        storage.set(&DataKey::AdminMode, &admin_mode);
    }

    /// Check admin mode is an internal function that will ensure
    /// Admin is required depending on a dynamic configuration
    /// that can be configured in the init function.
    fn check_admin_mode(storage: &Persistent) {
        let require_admin_mode = storage
            .get::<_, bool>(&DataKey::AdminMode)
            .ok_or(Error::KeyExpected)
            .unwrap();

        if require_admin_mode {
            storage
                .get::<_, Address>(&DataKey::Admin)
                .unwrap()
                .require_auth();
        }
    }

    /// Creates a new proposal with the default parameters.
    pub fn create_proposal(
        env: Env,
        proposer: Address,
        id: u64,
        payload: ProposalPayload,
    ) -> Result<(), Error> {
        let storage = env.storage().persistent();
        let voting_period_secs = storage.get::<_, u64>(&DataKey::VotingPeriodSecs).unwrap();
        let target_approval_rate_bps = storage.get(&DataKey::TargetApprovalRate).unwrap();
        let total_participation = storage.get::<_, u128>(&DataKey::TotalVoters).unwrap();

        Self::create_custom_proposal(
            env,
            id,
            payload,
            proposer,
            voting_period_secs,
            target_approval_rate_bps,
            total_participation,
        )
    }

    /// Creates a custom proposal by specifying all the available
    /// parameters.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `id` - The unique identifier of the proposal.
    /// - `payload` - An ADT, representing the kind of the proposal plus its payload. See ['ProposalPayload'].
    /// - `voting_period_secs` - The number of seconds of proposals lifetime.
    /// - `target_approval_rate_bps` - The required approval rate in basic points. i.e for a 50%, 5000 should be passed.
    /// - `total_participation` - The max number of participation (can be votes, staked amounts ...). This will be taken into account for calculating the approval rate.
    #[allow(clippy::too_many_arguments)]
    pub fn create_custom_proposal(
        env: Env,
        id: u64,
        payload: ProposalPayload,
        proposer: Address,
        voting_period_secs: u64,
        target_approval_rate_bps: u32,
        total_participation: u128,
    ) -> Result<(), Error> {
        proposer.require_auth();

        let storage = env.storage().persistent();

        storage
            .get::<_, Address>(&DataKey::Admin)
            .ok_or(Error::KeyExpected)?
            .require_auth();

        if id == 0 {
            return Err(Error::NotValidID);
        }

        if voting_period_secs == 0 {
            return Err(Error::InvalidVotingPeriod);
        }

        if target_approval_rate_bps == 0 {
            return Err(Error::InvalidTargetApprovalRate);
        }

        if total_participation == 0 {
            return Err(Error::NotEnoughParticipants);
        }

        let mut proposal_storage = storage
            .get::<_, Map<u64, Proposal>>(&DataKey::Proposals)
            .ok_or(Error::KeyExpected)?;

        if proposal_storage.contains_key(id) {
            return Err(Error::DuplicatedEntity);
        }

        proposal_storage.set(
            id,
            Proposal {
                id,
                payload: payload.clone(),
                proposer: proposer.clone(),
                voting_end_time: env
                    .ledger()
                    .timestamp()
                    .checked_add(voting_period_secs)
                    .unwrap(),
                target_approval_rate_bps,
                participation: 0,
                voters: Map::<Address, bool>::new(&env),
                total_participation,
            },
        );
        storage.set(&DataKey::Proposals, &proposal_storage);

        env.events().publish(
            (Symbol::new(&env, "proposal_created"), id, payload, proposer),
            (),
        );
        Ok(())
    }

    /// Positively votes a specific proposal.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `voter` - The voter address, which should match with transaction signatures.
    /// - `id` - The unique identifier of the proposal.
    pub fn vote(env: Env, voter: Address, id: u64) -> Result<(), Error> {
        voter.require_auth();

        let storage = env.storage().persistent();

        Self::check_admin_mode(&storage);

        let mut proposal_storage = storage
            .get::<_, Map<u64, Proposal>>(&DataKey::Proposals)
            .ok_or(Error::KeyExpected)?;

        let mut proposal = proposal_storage.get(id).ok_or(Error::NotFound)?;

        proposal.vote(env.ledger().timestamp(), voter.clone(), 1)?;
        let updated_approval_rate = proposal.approval_rate_bps();
        proposal_storage.set(id, proposal);

        storage.set(&DataKey::Proposals, &proposal_storage);

        env.events().publish(
            (Symbol::new(&env, "proposal_voted"), id, voter),
            updated_approval_rate,
        );
        Ok(())
    }

    pub fn find_proposal(env: Env, id: u64) -> Result<Proposal, Error> {
        let storage = env.storage().persistent();

        Self::check_admin_mode(&storage);

        let proposal_storage = storage
            .get::<_, Map<u64, Proposal>>(&DataKey::Proposals)
            .ok_or(Error::KeyExpected)?;

        proposal_storage.get(id).ok_or(Error::NotFound)
    }   

    pub fn update_proposal(
        env: Env,
        new_proposal: Proposal,
    ) -> Result<(), Error> {
        let storage = env.storage().persistent();

        storage
            .get::<_, Address>(&DataKey::Admin)
            .ok_or(Error::KeyExpected)?
            .require_auth();

        let mut proposal_storage = storage
            .get::<_, Map<u64, Proposal>>(&DataKey::Proposals)
            .ok_or(Error::KeyExpected)?;

        let old_proposal = proposal_storage.get(new_proposal.id).ok_or(Error::NotFound)?;

        proposal_storage.set(old_proposal.id, new_proposal);

        storage.set(&DataKey::Proposals, &proposal_storage);
        Ok(())
    }
}

#[cfg(test)]
mod test;
