use soroban_sdk::{contracterror, contracttype, Address, BytesN, ConversionError, Map};

/// ProposalPayload provides an composite
/// data type that allows using a different
/// proposal content for the different proposal
/// types.
///
/// It composes the type: proposal type + payload,
/// That later can be easily matched in other contracts.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum ProposalPayload {
    // A proposal type for changing the curator.
    NewCurator(Address),
    // A proposal type for doing a code upgrade. Needs the new
    // wasm hash as first parameter.
    CodeUpgrade(BytesN<32>),
    // A plain proposal voting of an arbitrary comment, without further actions.
    Comment(BytesN<32>),
}

/// Proposal represent a proposal in th voting system
/// and enforces all the invariants.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Proposal {
    pub id: u64,
    // Allows external systems to discriminate among type of proposal. This probably
    // goes in hand with the `comment` field.
    pub payload: ProposalPayload,
    // The address this proposal is created from.
    pub proposer: Address,
    // Unix time in seconds. Voting ends at this time.
    pub voting_end_time: u64,
    // Number of votes accumulated.
    pub participation: u128,
    // Target approval rate in basic points. i.e 10,43% would be 1043.
    pub target_approval_rate_bps: u32,
    // The expected, maximum participation.
    pub total_participation: u128,
    // A registry about who already voted.
    pub voters: Map<Address, bool>,
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

    pub fn payload(&self) -> &ProposalPayload {
        &self.payload
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
    InvalidVotingPeriod = 10,
    InvalidTargetApprovalRate = 11,
    NotEnoughParticipants = 12
}

impl From<ConversionError> for Error {
    fn from(_: ConversionError) -> Self {
        Error::Conversion
    }
}
