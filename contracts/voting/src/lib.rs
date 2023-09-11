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
    contract, contracterror, contractimpl, contracttype, panic_with_error, Address, BytesN,
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

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum ProposalType {
    Standard = 1,
    CodeUpgrade = 2,
    CuratorChange = 3,
}

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
        participation: u128,
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
        storage.set(&DataKey::TotalVoters, &participation);
    }

    /// Creates a new proposal with the default parameters.
    pub fn create_proposal(
        env: Env,
        proposer: Address,
        id: u64,
        kind: ProposalType,
        comment: BytesN<32>,
    ) -> Result<(), Error> {
        let storage = env.storage().persistent();
        let voting_period_secs = storage.get::<_, u64>(&DataKey::VotingPeriodSecs).unwrap();
        let target_approval_rate_bps = storage.get(&DataKey::TargetApprovalRate).unwrap();
        let total_participation = storage.get::<_, u128>(&DataKey::TotalVoters).unwrap();

        Self::create_custom_proposal(
            env,
            id,
            kind,
            proposer,
            comment,
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
    /// - `comment` - Comment has enough size for a wasm contract hash. It could also be a string.
    /// - `voting_period_secs` - The number of seconds of proposals lifetime.
    /// - `target_approval_rate_bps` - The required approval rate in basic points. i.e for a 50%, 5000 should be passed.
    /// - `total_participation` - The max number of participation (can be votes, staked amounts ...). This will be taken into account for calculating the approval rate.
    #[allow(clippy::too_many_arguments)]
    pub fn create_custom_proposal(
        env: Env,
        id: u64,
        kind: ProposalType,
        proposer: Address,
        comment: BytesN<32>,
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
                kind,
                proposer,
                comment,
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

        proposal.vote(env.ledger().timestamp(), voter, 1)?;
        let updated_approval_rate = proposal.approval_rate_bps();
        proposal_storage.set(id, proposal);

        storage.set(&DataKey::Proposals, &proposal_storage);

        env.events().publish(
            (Symbol::new(&env, "proposal_voted"), id),
            updated_approval_rate,
        );
        Ok(())
    }

    pub fn find_proposal(env: Env, id: u64) -> Result<Proposal, Error> {
        let storage = env.storage().persistent();

        let proposal_storage = storage
            .get::<_, Map<u64, Proposal>>(&DataKey::Proposals)
            .ok_or(Error::KeyExpected)?;

        proposal_storage.get(id).ok_or(Error::NotFound)
    }

    pub fn update_proposal_with_balance(
        env: Env,
        id: u64,
        balance: Map<Address, i128>,
    ) -> Result<(), Error> {
        let storage = env.storage().persistent();

        storage
            .get::<_, Address>(&DataKey::Admin)
            .ok_or(Error::KeyExpected)?
            .require_auth();

        let mut proposal_storage = storage
            .get::<_, Map<u64, Proposal>>(&DataKey::Proposals)
            .ok_or(Error::KeyExpected)?;

        let mut proposal = proposal_storage.get(id).ok_or(Error::NotFound)?;

        proposal.set_participation_from_balance(&balance)?;

        proposal_storage.set(id, proposal);

        storage.set(&DataKey::Proposals, &proposal_storage);
        Ok(())
    }
}

/// Proposal represent a proposal in th voting system
/// and enforces all the invariants.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Proposal {
    id: u64,
    // Allows external systems to discriminate among type of proposal. This probably
    // goes in hand with the `comment` field.
    kind: ProposalType,
    // The address this proposal is created from.
    proposer: Address,
    // Comment has enough size for a wasm contract hash. It could also be a string.
    comment: BytesN<32>,
    // Unix time in seconds. Voting ends at this time.
    voting_end_time: u64,
    // Number of votes accumulated.
    participation: u128,
    // Target approval rate in basic points. i.e 10,43% would be 1043.
    target_approval_rate_bps: u32,
    // The expected, maximum participation.
    total_participation: u128,
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
    /// - `weight` - The amount of participation for this vote.
    pub fn vote(&mut self, current_time: u64, voter: Address, weight: u128) -> Result<(), Error> {
        if self.is_closed(current_time) {
            return Err(Error::VotingClosed);
        }

        if self.voters.get(voter.clone()).is_some() {
            return Err(Error::AlreadyVoted);
        }

        self.participation = self
            .participation
            .checked_add(weight)
            .ok_or(Error::Overflow)?;
        self.voters.set(voter, true);
        Ok(())
    }

    pub fn is_closed(&self, current_time: u64) -> bool {
        current_time >= self.voting_end_time || self.participation == self.total_participation
    }

    /// It provides a calculation of the approval rate by using fixed point integer arithmetic of
    /// 2 positions. It returns the basic points, which would need to be divided by 100
    /// in order to get the original approval percentage. i.e if this function returns 1043 bps,
    /// the equivalent percentage would be 10,43% .
    pub fn approval_rate_bps(&self) -> Result<u32, Error> {
        if self.participation == 0 {
            return Ok(0);
        }
        Ok(u32::try_from(
            self.participation
                .checked_mul(10_000)
                .ok_or(Error::Overflow)?
                .checked_div(self.total_participation)
                .ok_or(Error::Overflow)?,
        )
        .unwrap())
    }

    pub fn is_approved(&self) -> bool {
        self.approval_rate_bps().unwrap() >= self.target_approval_rate_bps
    }

    pub fn get_comment(&self) -> &BytesN<32> {
        &self.comment
    }

    pub fn get_kind(&self) -> ProposalType {
        self.kind
    }

    /// It provides a way to update the current proposal participation
    /// data from a provided balance in which is assumed there are no negative balances.
    ///
    /// All the current proposal voters addresses, must be present in the provided balance.
    /// If not, it will return with an error on the first not found address.
    ///
    /// After calling this function, all quorum calculations will use the calculated data.
    pub fn set_participation_from_balance(
        &mut self,
        balance: &Map<Address, i128>,
    ) -> Result<(), Error> {
        self.participation = self
            .voters
            .iter()
            .try_fold(0u128, |acc, (address, _)| {
                let stake = balance.get(address)?;
                acc.checked_add(stake as u128)
            })
            .ok_or(Error::NotFound)?;

        self.total_participation = balance
            .values()
            .iter()
            .try_fold(0u128, |acc, balance| acc.checked_add(balance as u128))
            .ok_or(Error::Overflow)?;
        Ok(())
    }
}

#[cfg(test)]
mod test;
