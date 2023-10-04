//! Lottery contract
//!
//! This contract provides lottery implementation
//! for the soroban smart contract platform. Admin of the lottery
//! can specify how many numbers will be drawn, from what range
//! (always starting from 1) and also a thresholds of prizes
//! for a given number of correctly selected numbers:
//! e.g. for 5 numbers - 30% of the pool, for 4 numbers - 15% of the pool
//! and so on.
//!
//! Once deployed and initialized, each participant can buy
//! any number of tickets and select the appropriate number of numbers.
//! If all numbers are correctly selected player wins the main prize.
//! In case when not all numbers are correct smaller prizes can be paid out,
//! accordingly to the setup thresholds. It is possible there will be
//! no winners, in which case gathered tokens will be carried over to the next
//! lottery.
//!
//! If there are a lot of winners and won awards are higher than the available
//! lottery pool, the prize thresholds defined during lottery initialization are
//! recalculated and prizes are lowered so that always:
//! sum of prizes <= lottery pool.

#![no_std]

use core::ops::Add;

use soroban_sdk::storage::Persistent;
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, map, panic_with_error, token, vec,
    Address, Env, Map, Symbol, Vec, Bytes
};

/// State of the lottery
#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq)]
enum LotteryState {
    Initialized = 1,
    Active = 2,
    Finished = 3,
}

/// Datakey holds all possible storage keys this
/// contract uses. See https://soroban.stellar.org/docs/getting-started/storing-data .
#[derive(Clone, Copy)]
#[contracttype]
enum DataKey {
    Admin = 1,
    Tickets = 2,
    TicketPrice = 4,
    LotteryNumber = 5,
    LotteryResults = 6,
    NumberOfNumbers = 7,
    MaxRange = 8,
    Thresholds = 9,
    MinPlayersCount = 10,
    Token = 11,
    LotteryState = 12,
}

/// All errors this contract expects.
#[contracterror]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    // The contract should be only initialised once.
    AlreadyInitialized = 1,
    // Participants needs to have enough funds to buy a raffle ticket.
    InsufficientFunds = 2,
    // If not initialized, raffle should not be able to execute actions.
    NotInitialized = 3,
    // In order to play the lottery, at least the min amount of participants should be in.
    MinParticipantsNotSatisfied = 4,
    // Max possible number to select must be at least as high as number of numbers to choose from
    MaxRangeTooLow = 5,
    // Number of numbers to select must be at least 2
    NumberOfNumbersTooLow = 6,
    // There should be at least 1 threshold defined
    NumberOfThresholdsTooLow = 7,
    // Number of selected numbers by players must be exact as number of numbers defined in the lottery initialization
    NotEnoughOrTooManyNumbers = 8,
    // All numbers must be in range (1, max_range)
    InvalidNumbers = 9,
    // Lottery ID should be already stored in DB
    WrongLotteryNumber = 10,
    // At least 1 lottery should be played to have results
    NoLotteryResultsAvailable = 11,
    // Lottery is already active
    AlreadyActive = 12,
    // There is no active lottery at the moment
    NotActive = 13,
    // Sum of thresholds percentages must be below 100
    InvalidThresholds = 14,
    // Ticket price must be above 0
    InvalidTicketPrice = 15
}

/// Helper types for lottery tickets and results
type LotteryTicket = Vec<u32>;
type LotteryResult = Vec<u32>;

#[contract]
pub struct LotteryContract;

#[contractimpl]
impl LotteryContract {
    /// It initializes the contract with all the needed parameters.
    /// This function must be invoked byt the administrator just
    /// after the contract deployment. 
    /// It invokes the `create_lottery` function at the end of initialization.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `admin` - Admin account address.
    /// - `token` - The asset contract address we are using for this lottery. See [token interface](https://soroban.stellar.org/docs/reference/interfaces/token-interface).
    /// - `ticket_price` - Unitary ticket price for the current lottery.
    /// - `number_of_numbers` - Number of numbers possible to select by players
    /// - `max_range` - Right boundary of the range players will select numbers from (1, max_range)
    /// - `thresholds` - Thresholds with prizes for correctly selected numbers (specified as percentage of the pool balance)
    /// - `min_players_count` - Minimum number of players needed to play the lottery
    pub fn init(
        env: Env,
        admin: Address,
        token: Address,
        ticket_price: i128,
        number_of_numbers: u32,
        max_range: u32,
        thresholds: Map<u32, u32>,
        min_players_count: u32,
    ) {
        admin.require_auth();
        let storage = env.storage().persistent();

        if storage
            .get::<_, LotteryState>(&DataKey::LotteryState)
            .is_some() {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }
        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Token, &token);
        storage.set(&DataKey::LotteryState, &LotteryState::Initialized);
        Self::create_lottery(
            env,
            ticket_price,
            number_of_numbers,
            max_range,
            thresholds,
            min_players_count,
        );
    }

    /// Creates the new lottery.
    /// This function must be invoked by the administrator.
    /// It can be called each time after previous lottery
    /// has been completed. New lottery can have different specs,
    /// pool balance is carried over from the previous one.
    /// All previously stored tickets are cleared
    ///
    /// # Returns
    /// 
    /// - Lottery number
    /// 
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `ticket_price` - Unitary ticket price for the current lottery.
    /// - `number_of_numbers` - Number of numbers possible to select by players
    /// - `max_range` - Right boundary of the range players will select numbers from (1, max_range)
    /// - `thresholds` - Thresholds with prizes for correctly selected numbers (specified as percentage of the pool balance)
    /// - `min_players_count` - Minimum number of players needed to play the lottery
    pub fn create_lottery(
        env: Env,
        ticket_price: i128,
        number_of_numbers: u32,
        max_range: u32,
        thresholds: Map<u32, u32>,
        min_players_count: u32,
    ) -> u32 {
        let storage = env.storage().persistent();
        if storage
            .get::<_, LotteryState>(&DataKey::LotteryState)
            .is_none() {
            panic_with_error!(&env, Error::NotInitialized);
        }

        let admin = storage.get::<_, Address>(&DataKey::Admin).unwrap();
        admin.require_auth();

        let lottery_state = storage
            .get::<_, LotteryState>(&DataKey::LotteryState)
            .unwrap();
        if lottery_state == LotteryState::Active {
            panic_with_error!(&env, Error::AlreadyActive);
        }

        if max_range < number_of_numbers {
            panic_with_error!(&env, Error::MaxRangeTooLow);
        }

        if number_of_numbers < 2 {
            panic_with_error!(&env, Error::NumberOfNumbersTooLow);
        }

        if thresholds.len() < 1 {
            panic_with_error!(&env, Error::NumberOfThresholdsTooLow);
        }

        if ticket_price <= 0 {
            panic_with_error!(&env, Error::InvalidTicketPrice);
        }

        let sum_of_percentages = thresholds.values()
            .iter()
            .fold(0u32, |acc, percentage| acc.add(percentage));

        if sum_of_percentages < 1 || sum_of_percentages > 100 {
            panic_with_error!(&env, Error::InvalidThresholds);
        }

        for threshold_number in thresholds.keys() {
            if threshold_number < 1 || threshold_number > number_of_numbers {
                panic_with_error!(&env, Error::InvalidThresholds);
            }
        }

        let lottery_number = storage
            .get::<_, u32>(&DataKey::LotteryNumber)
            .unwrap_or_default()
            + 1;

        storage.set(&DataKey::NumberOfNumbers, &number_of_numbers);
        storage.set(&DataKey::MaxRange, &max_range);
        storage.set(&DataKey::Thresholds, &thresholds);
        storage.set(&DataKey::MinPlayersCount, &min_players_count);
        storage.set(&DataKey::TicketPrice, &ticket_price);
        storage.set(
            &DataKey::Tickets,
            &Map::<Address, Vec<LotteryTicket>>::new(&env),
        );
        storage.set(&DataKey::LotteryState, &LotteryState::Active);
        storage.set(&DataKey::LotteryNumber, &lottery_number);

        let topic = (Symbol::new(&env, "new_lottery_created"), lottery_number);
        env.events().publish(topic, (number_of_numbers, max_range, thresholds, ticket_price));

        lottery_number
    }

    /// A 'dummy' method that needs to be called by user before buying the ticket.
    /// This a workaround to this issue https://github.com/eigerco/nebula/issues/41
    /// 
    /// # Arguments
    ///
    /// - `_` - The environment for this contract - not used here
    /// - `by` - The address that is registering.
    pub fn register(_: Env, by: Address) {
        by.require_auth();
    }

    /// Allows any participant with enough funds to buy a ticket.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `by` - The address that is buying the ticket. Its enforced to match with the incoming transaction signatures.
    /// - `ticket` - The selected numbers by the player
    pub fn buy_ticket(env: Env, by: Address, ticket: Vec<u32>) -> Result<u32, Error> {
        by.require_auth();

        let storage = env.storage().persistent();

        lottery_must_be_active(&storage)?;

        let number_of_elements = storage.get::<_, u32>(&DataKey::NumberOfNumbers).unwrap();
        let max_range = storage.get::<_, u32>(&DataKey::MaxRange).unwrap();

        if ticket.len() != number_of_elements {
            return Err(Error::NotEnoughOrTooManyNumbers);
        }

        //each number must be within (1, max_range) range
        for number in ticket.iter() {
            if number <= 0 || number > max_range {
                return Err(Error::InvalidNumbers);
            }
        }

        let price = storage.get::<_, i128>(&DataKey::TicketPrice).unwrap();
        let token = storage.get::<_, Address>(&DataKey::Token).unwrap();
        let token_client = token::Client::new(&env, &token);

        if token_client.balance(&by) <= price {
            return Err(Error::InsufficientFunds);
        }

        token_client.transfer(&by, &env.current_contract_address(), &price);

        let mut tickets = storage
            .get::<_, Map<Address, Vec<LotteryTicket>>>(&DataKey::Tickets)
            .unwrap();

        let mut player_selection = tickets.get(by.clone()).unwrap_or(vec![&env]);
        player_selection.push_back(ticket);
        tickets.set(by, player_selection);

        storage.set(&DataKey::Tickets, &tickets);
        Ok(tickets.values().len())
    }

    /// Returns actual pool balance. Can only be invoked by admin
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    pub fn pool_balance(env: Env) -> Result<i128, Error> {
        let storage = env.storage().persistent();
        let admin = storage.get::<_, Address>(&DataKey::Admin).unwrap();
        admin.require_auth();

        let token = storage.get::<_, Address>(&DataKey::Token).unwrap();
        let token_client = token::Client::new(&env, &token);

        Ok(token_client.balance(&env.current_contract_address()))
    }

    /// Returns results of a given lottery. If no results are available, or wrong lottery number
    /// was given an error is returned
    ///
    /// # Returns
    /// 
    /// - Results of the lottery
    /// 
    /// # Arguments
    ///
    /// - `lottery_number` - Number of the lottery
    pub fn check_lottery_results(env: Env, lottery_number: u32) -> Result<Vec<u32>, Error> {
        let storage = env.storage().persistent();
        
        let lottery_results = storage
            .get::<_, Map<u32, LotteryResult>>(&DataKey::LotteryResults)
            .ok_or(Error::NoLotteryResultsAvailable)?;

        if !lottery_results.contains_key(lottery_number) {
            return Err(Error::WrongLotteryNumber);
        }
        Ok(lottery_results.get(lottery_number).unwrap())
    }

    /// Allows an admin to play the lottery anytime.
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `random_seed` - A seed provided by the admin that will be combined with other environment elements.
    pub fn play_lottery(env: Env, random_seed: u64) -> Result<(), Error> {
        let storage = env.storage().persistent();

        lottery_must_be_active(&storage)?;

        let admin = storage.get::<_, Address>(&DataKey::Admin).unwrap();
        admin.require_auth();

        let token: Address = storage.get::<_, Address>(&DataKey::Token).unwrap();
        let token_client = token::Client::new(&env, &token);

        let tickets = storage
            .get::<_, Map<Address, Vec<LotteryTicket>>>(&DataKey::Tickets)
            .unwrap();

        let min_players_count = storage.get::<_, u32>(&DataKey::MinPlayersCount).unwrap();

        if tickets.keys().len() < min_players_count {
            return Err(Error::MinParticipantsNotSatisfied);
        }

        let pool = token_client.balance(&env.current_contract_address());
        let max_range = storage.get::<_, u32>(&DataKey::MaxRange).unwrap();
        let number_of_elements = storage.get::<_, u32>(&DataKey::NumberOfNumbers).unwrap();
        let mut thresholds = storage
            .get::<_, Map<u32, u32>>(&DataKey::Thresholds)
            .unwrap();

        let drawn_numbers = draw_numbers::<RandomNumberGenerator>(&env, max_range, number_of_elements, random_seed);
        let winners = get_winners(&env, &drawn_numbers, &tickets, &thresholds);
        let prizes = calculate_prizes(&env, &winners, &mut thresholds, pool);
        payout_prizes(&env, &token_client, &prizes);

        // store numbers drawn in this lottery
        let lottery_number = storage.get::<_, u32>(&DataKey::LotteryNumber).unwrap();
        let mut lottery_results = storage
            .get::<_, Map<u32, LotteryResult>>(&DataKey::LotteryResults)
            .unwrap_or(map![&env]);

        lottery_results.set(lottery_number, drawn_numbers);
        storage.set(&DataKey::LotteryResults, &lottery_results);

        storage.set(&DataKey::LotteryState, &LotteryState::Finished);

        // emit events with won prizes
        prizes.iter().for_each(|(address, prize)| {
            let topic = (Symbol::new(&env, "won_prize"), &address);
            env.events().publish(topic, prize);
        });
        Ok(())
    }
}

/// Ensures lottery is initialized and not finished.
/// If not, error is returned.
///
/// # Arguments
///
/// - `storage` - Contracts data storage
fn  lottery_must_be_active(storage: &Persistent) -> Result<(), Error> {
    let lottery_state_opt = storage
        .get::<_, LotteryState>(&DataKey::LotteryState);

    if lottery_state_opt.is_none() {
        return Err(Error::NotInitialized);
    }

    if lottery_state_opt.unwrap() != LotteryState::Active {
        return Err(Error::NotActive);
    }
    Ok(())
}

/// Calculates the winners of a lottery. There can be several winners
/// with different prizes according to number of correctly selected numbers and
/// specified thresholds.
///
/// # Returns
///
/// A map where number of selected numbers is a key, and
/// vector of addresses that properly selected this number of numbers is a value
///
/// # Arguments
///
/// - `env` - The environment for this contract.
/// - `drawn_numbers` - An array containing numbers selected in this lottery,
/// - `tickets` - Tickets with players selection
/// - `thresholds` - Defined thresholds with prizes
fn get_winners(
    env: &Env,
    drawn_numbers: &Vec<u32>,
    tickets: &Map<Address, Vec<LotteryTicket>>,
    thresholds: &Map<u32, u32>,
) -> Map<u32, Vec<Address>> {
    let mut winners = Map::<u32, Vec<Address>>::new(&env);

    tickets
        .iter()
        .for_each(|(ticket_address, tickets)|
            tickets
                .iter()
                .map(|ticket| count_matches(drawn_numbers, &ticket))
                .filter(|count, | thresholds.contains_key(*count))
                .for_each(|count| {
                    let mut addresses = winners.get(count).unwrap_or(Vec::<Address>::new(&env));
                    addresses.push_back(ticket_address.clone());
                    winners.set(count, addresses);
                })
        );
    winners
}

/// Calculates prizes for winning players taking into account specified thresholds.
/// In case when total prizes are higher than available pool balance
/// thresholds are recalculated.
/// 
/// # Returns
/// 
/// A map containing addresses and their won prizes
///
/// # Arguments
///
/// - `env` - The environment for this contract.
/// - `winners` - A map containing winning players with a number of correctly selected numbers
/// - `thresholds` - Defined thresholds with prizes
/// - `pool` - Current pool balance
fn calculate_prizes(
    env: &Env,
    winners: &Map<u32, Vec<Address>>,
    thresholds: &mut Map<u32, u32>,
    pool: i128,
) -> Map<Address, i128> {
    let mut prizes = Map::<Address, i128>::new(&env);

    // sum total percentage of prizes won - if it's bigger than 100%, recalculate new thresholds to be equal to 100%
    let total_prizes_percentage = count_total_prizes_percentage(&winners, &thresholds);
    recalculate_new_thresholds(&winners, thresholds, total_prizes_percentage);

    //that would be nicer and probably faster if FromIter trait was implemented for Map & Vec...
    winners.iter().for_each(|(threshold_number, addresses)| {
        let pool_percentage = thresholds.get(threshold_number).unwrap();
        let prize = (pool * pool_percentage as i128 / 100i128) as i128;
        addresses.iter().for_each(|address| {
            let current_player_prize = prizes.get(address.clone()).unwrap_or_default();
            prizes.set(address, current_player_prize + prize)
        });
    });
    prizes
}

/// Pays out prizes to winning players
///
/// # Arguments
/// 
/// - `env` - The environment for this contract.
/// - `token_client` - A token client used for token transfers,
/// - `prizes` - A map containing winning addresses and their prizes
fn payout_prizes(env: &Env, token_client: &token::Client, prizes: &Map<Address, i128>) {
    prizes.iter().for_each(|(address, prize)| {
        token_client.transfer(&env.current_contract_address(), &address, &prize);
    });
}

/// Counts total prizes percentage by summing up prizes of all winners
///
/// # Returns
/// 
/// Total prizes percentage
/// 
/// # Arguments
/// 
/// - `winners` - A map containing winning players with a number of correctly selected numbers
/// - `thresholds` - Defined thresholds with prizes
fn count_total_prizes_percentage(
    winners: &Map<u32, Vec<Address>>,
    thresholds: &Map<u32, u32>,
) -> u32 {
    winners
        .iter()
        .fold(0u32, |acc, (threshold_number, _)| {
            let threshold_percentage = thresholds.get(threshold_number).unwrap();
            let winners_count = winners.get(threshold_number).unwrap().len();
            acc.add(threshold_percentage * winners_count)
        })
}

/// Recalculates new thresholds in case total prizes percentage is above 100%
///
/// # Arguments
/// 
/// - `winners` - A map containing winning players with a number of correctly selected numbers
/// - `thresholds` - Defined thresholds with prizes, will be updated
/// in case total_prizes_percentage > 100
/// - `total_prizes_percentage` - Total prizes percentage
fn recalculate_new_thresholds(
    winners: &Map<u32, Vec<Address>>,
    thresholds: &mut Map<u32, u32>,
    total_prizes_percentage: u32,
) {
    if total_prizes_percentage > 100 {
        for threshold_number in thresholds.keys() {
            if winners.contains_key(threshold_number) {
                let winners_count = winners.get(threshold_number).unwrap().len();
                let threshold_percentage = thresholds.get(threshold_number).unwrap();
                let val =
                    winners_count * threshold_percentage * 100 / total_prizes_percentage;
                thresholds.set(threshold_number, val / winners_count);
            } else {
                thresholds.remove(threshold_number);
            }
        }
    }
}

/// Counts properly selected numbers for a given ticket
///
/// # Returns
/// 
/// Counted matches
/// 
/// # Arguments
/// 
/// - `drawn_numbers` - An array containing numbers selected in this lottery,
/// - `player_ticket` - Numbers selected by a player
fn count_matches(drawn_numbers: &Vec<u32>, player_ticket: &LotteryTicket) -> u32 {
    drawn_numbers
        .iter()
        .filter(|x| player_ticket.contains(x))
        .count() as u32
}


struct RandomNumberGenerator;

trait RandomNumberGeneratorTrait {
    fn new(env: &Env, seed: u64) -> Self;
    fn number(&mut self, env: &Env, max_range: u32) -> u32;
}

impl RandomNumberGeneratorTrait for RandomNumberGenerator {
    fn new(env: &Env, seed: u64) -> Self {
        let mut arr = [0u8; 32];
        let seed_bytes = seed.to_be_bytes();

        //there is no concat() for wasm build...
        for i in 24..32 {
            arr[i] = seed_bytes[i-24];
        }
        env.prng().seed(Bytes::from_slice(&env, &arr.as_slice()));
        RandomNumberGenerator{}
    }

    fn number(&mut self, env: &Env, max_range: u32) -> u32 {
        env.prng().u64_in_range(1..=max_range as u64) as u32
    }
}

/// Randomly draw numbers within a given range (1, max_range).
/// Ensures that there are no duplicates
///
/// # Returns
/// 
/// A list of randomly selected number
/// 
/// # Arguments
///
/// - `env` - The environment for this contract.
/// - `max_range` - Right boundary of the range players will select numbers from (1, max_range)
/// - `number_of_numbers` - Number of numbers possible to select by players
/// - `random_seed` - A seed provided by the admin, that will be combined with other environment elements
fn draw_numbers<T: RandomNumberGeneratorTrait>(env: &Env, max_range: u32, number_of_numbers: u32, random_seed: u64) -> Vec<u32> {
    let mut numbers = Vec::new(&env);
    for n in 0..number_of_numbers {
        let new_seed = random_seed + n as u64;
        let mut random_generator = T::new(env, new_seed);
        loop {
            // draw a number so many times until a new unique number is found
            let drawn_number = random_generator.number(env, max_range);
            if !numbers.contains(drawn_number) {
                numbers.push_back(drawn_number);
                break;
            }
        }
    }
    numbers
}

#[cfg(test)]
mod test;
