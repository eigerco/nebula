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

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, Address,
    ConversionError, Env, Map, Symbol,
};

/// All the expected errors this contract expects.
/// This error codes will appear as output in the transaction
/// receipt.
#[contracterror]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    Conversion = 2,
    KeyExpected = 3,
    NotFound = 4,
    AlreadyVoted = 5,
    DuplicatedEntity = 6,
    Overflow = 7,
    VotingClosed = 8,
    NotValidID = 9,
}

impl From<ConversionError> for Error {
    fn from(_: ConversionError) -> Self {
        Error::Conversion
    }
}

/// Datakey holds all possible storage keys this
/// contract uses. See https://soroban.stellar.org/docs/getting-started/storing-data .
#[contracttype]
#[derive(Clone, Copy)]
pub enum DataKey {
    AlreadyInitialized = 0,
    Admin = 1,
    VoterList = 2,
    Proposals = 3,
    VotingPeriodSecs = 4,
    TargetApprovalRate = 5,
    TotalVoters = 6,
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
    /// - `total_voters` - The default max number of voters for new proposals.
    pub fn init(
        env: Env,
        admin: Address,
        voting_period_secs: u64,
        target_approval_rate_bps: u32,
        total_voters: u32,
    ) {
        let storage = env.storage().persistent();

        if storage
            .get::<_, bool>(&DataKey::AlreadyInitialized)
            .is_some()
        {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }

        storage.set(&DataKey::AlreadyInitialized, &true);
        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Proposals, &Map::<u64, Proposal>::new(&env));
        // Todo, to better study if this parameters would be better as hardcoded values, due to fees. See https://soroban.stellar.org/docs/fundamentals-and-concepts/fees-and-metering#resource-fee .
        storage.set(&DataKey::VotingPeriodSecs, &voting_period_secs);
        storage.set(&DataKey::TargetApprovalRate, &target_approval_rate_bps);
        storage.set(&DataKey::TotalVoters, &total_voters);
    }

    /// Creates a new proposal with the default parameters.
    pub fn create_proposal(env: Env, id: u64) -> Result<(), Error> {
        let storage = env.storage().persistent();
        let voting_period_secs = storage.get::<_, u64>(&DataKey::VotingPeriodSecs).unwrap();
        let target_approval_rate_bps = storage.get(&DataKey::TargetApprovalRate).unwrap();
        let total_voters = storage.get::<_, u32>(&DataKey::TotalVoters).unwrap();

        Self::create_custom_proposal(
            env,
            id,
            voting_period_secs,
            target_approval_rate_bps,
            total_voters,
        )
    }

    /// Creates a custom proposal by specifying all the available
    /// parameters.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `id` - The unique identifier of the proposal.
    /// - `voting_period_secs` - The number of seconds of proposals lifetime.
    /// - `target_approval_rate_bps` - The required approval rate in basic points. i.e for a 50%, 5000 should be passed.
    /// - `total_voters` - The max number of voters. This will be taken into account for calculating the approval rate.
    pub fn create_custom_proposal(
        env: Env,
        id: u64,
        voting_period_secs: u64,
        target_approval_rate_bps: u32,
        total_voters: u32,
    ) -> Result<(), Error> {
        let storage = env.storage().persistent();

        storage
            .get::<_, Address>(&DataKey::Admin)
            .ok_or(Error::KeyExpected)?
            .require_auth();

        if id == 0 {
            return Err(Error::NotValidID);
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
                voting_end_time: env.ledger().timestamp() + voting_period_secs,
                target_approval_rate_bps,
                votes: 0,
                voters: Map::<Address, bool>::new(&env),
                total_voters,
            },
        );
        storage.set(&DataKey::Proposals, &proposal_storage);
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

        let mut proposal_storage = storage
            .get::<_, Map<u64, Proposal>>(&DataKey::Proposals)
            .ok_or(Error::KeyExpected)?;

        let mut proposal = proposal_storage.get(id).ok_or(Error::NotFound)?;

        proposal.vote(env.ledger().timestamp(), voter)?;
        let updated_approval_rate = proposal.approval_rate_bps();
        proposal_storage.set(id, proposal);

        storage.set(&DataKey::Proposals, &proposal_storage);

        env.events().publish(
            (Symbol::new(&env, "proposal_voted"), id),
            updated_approval_rate,
        );
        Ok(())
    }
}

/// Proposal represent a proposal in th voting system
/// and enforces all the invariants.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Proposal {
    id: u64,
    // Unix time in seconds. Voting ends at this time.
    voting_end_time: u64,
    // Number of votes accumulated.
    votes: u32,
    // Target approval rate in basic points. i.e 10,43% would be 1043.
    target_approval_rate_bps: u32,
    // The expected, maximum participation.
    total_voters: u32,
    // A registry about who already voted.
    voters: Map<Address, bool>,
}

impl Proposal {
    /// Positively votes a specific proposal.
    ///
    /// # Arguments
    ///
    /// - `current_time` - The current time. Normally obtained from the environment.
    /// - `voter` - The address of the voter. It will be registered to prevent double voting.
    pub fn vote(&mut self, current_time: u64, voter: Address) -> Result<(), Error> {
        if self.is_closed(current_time) {
            return Err(Error::VotingClosed);
        }

        if self.voters.get(voter.clone()).is_some() {
            return Err(Error::AlreadyVoted);
        }

        self.votes = self.votes.checked_add(1).ok_or(Error::Overflow)?;
        self.voters.set(voter, true);
        Ok(())
    }

    pub fn is_closed(&self, current_time: u64) -> bool {
        current_time >= self.voting_end_time || self.voters.len() == self.total_voters
    }

    /// It provides a calculation of the approval rate by using fixed point integer arithmetic of
    /// 2 positions. It returns the basic points, which would need to be divided by 100
    /// in order to get the original approval percentage. i.e if this function returns 1043 bps,
    /// the equivalent percentage would be 10,43% .
    pub fn approval_rate_bps(&self) -> Result<u32, Error> {
        if self.votes == 0 {
            return Ok(0);
        }
        self.votes
            .checked_mul(10_000)
            .ok_or(Error::Overflow)?
            .checked_div(self.total_voters)
            .ok_or(Error::Overflow)
    }

    pub fn is_approved(&self) -> bool {
        self.approval_rate_bps().unwrap() >= self.target_approval_rate_bps
    }
}

#[cfg(test)]
mod test;
