//! Voting contract
//!
//! This is a simple version of voting contract that allows
//!  creating simple proposals.
//!
//! The implemented time lock, defined when the proposal
//! is in closed state. That is, when a proposal deadline
//! is reached, no other action can be performed on it and
//! the current result will become the final one.
//!
//! Proposal is identified by the contract ID, that might
//! be associated in external applications.

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
    AlreadyVoted = 4,
    Overflow = 5,
    VotingClosed = 6,
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
    Proposal = 1,
}

#[contract]
pub struct ProposalVotingContract;

#[contractimpl]
impl ProposalVotingContract {
    /// It initializes the contract with all the needed parameters.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `voting_period_secs` - The number of seconds for the proposal lifetime.
    /// - `target_approval_rate_bps` - The required approval rate in basic points.
    /// - `total_voters` - The max number of voters.
    pub fn init(
        env: Env,
        voting_period_secs: u64,
        target_approval_rate_bps: u32,
        total_voters: u32,
    ) -> Result<(), Error> {
        let storage = env.storage().persistent();

        if storage
            .get::<_, bool>(&DataKey::AlreadyInitialized)
            .is_some()
        {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }

        storage.set(&DataKey::AlreadyInitialized, &true);
        Self::create_proposal(
            env,
            voting_period_secs,
            target_approval_rate_bps,
            total_voters,
        )
    }

    fn create_proposal(
        env: Env,
        voting_period_secs: u64,
        target_approval_rate_bps: u32,
        total_voters: u32,
    ) -> Result<(), Error> {
        let storage = env.storage().persistent();

        storage.set(
            &DataKey::Proposal,
            &Proposal {
                voting_end_time: env
                    .ledger()
                    .timestamp()
                    .checked_add(voting_period_secs)
                    .unwrap(),
                target_approval_rate_bps,
                votes: 0,
                voters: Map::<Address, bool>::new(&env),
                total_voters,
            },
        );
        Ok(())
    }

    /// Positively votes this proposal contract.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `voter` - The voter address, which should match with transaction signatures.
    pub fn vote(env: Env, voter: Address) -> Result<(), Error> {
        voter.require_auth();

        let storage = env.storage().persistent();

        let mut proposal = storage
            .get::<_, Proposal>(&DataKey::Proposal)
            .ok_or(Error::KeyExpected)?;

        proposal.vote(env.ledger().timestamp(), voter.clone())?;
        let updated_approval_rate = proposal.approval_rate_bps();

        storage.set(&DataKey::Proposal, &proposal);

        env.events().publish(
            (Symbol::new(&env, "proposal_voted"), voter.clone()),
            updated_approval_rate,
        );
        Ok(())
    }
}

/// Proposal represent a proposal in the voting system
/// and enforces all the invariants.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Proposal {
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
    /// Positively votes this proposal.
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
