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
//! accordingly to the lottery specification. It is possible there will be 
//! no winners, in which case gathered tokens will be carried over to the next
//! lottery. 
//! 
//! If there are a lot of winners and won awards are higher than the available 
//! lottery pool, the prize thresholds defined during lottery initialization are 
//! recalculated and prizes are lowered so that always:
//! sum of prizes <= lottery pool. 

#![no_std]

use rand::rngs::SmallRng;
use rand::{SeedableRng, Rng};

use soroban_sdk::storage::Persistent;
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, token, Address, Env,
    Symbol, Map, Vec, vec, map,
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

/// All the expected errors this contract expects.
/// This error codes will appear as output in the transaction
/// receipt.
#[contracterror]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// The contract should be only initialised once.
    AlreadyInitialized = 1,
    // Participants needs to have enough funds to buy a raffle ticket.
    InsufficientFunds = 2,
    // The raffle can only be executed once.
    AlreadyPlayed = 3,
    // In order to play the raffle, at least the min amount of participants should be in.
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
    // Minimum ticket price.
    MinimumTicketPrice = 10,
    // If not initialized, raffle should not be able to execute actions.
    NotInitialized = 11,
    // Lottery ID should be already stored in DB
    WrongLotteryNumber = 12,
    // At least 1 lottery should be played to have results
    NoLotteryResultsAvailable = 13,
    
    AlreadyActive = 14,
    NotActive = 15
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
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `admin` - The address can play the raffle anytime.
    /// - `token` - The asset contract address we are using for this raffle. See [token interface](https://soroban.stellar.org/docs/reference/interfaces/token-interface).
    /// - `ticket_price` - Unitary ticket price for the current raffle.
    /// - `number_of_elements` - Number of numbers possible to select by players
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
        min_players_count: u32
    ) {
        admin.require_auth();
        let storage = env.storage().persistent();

        if storage
            .get::<_, LotteryState>(&DataKey::LotteryState)
            .is_some() {
            let lottery_state = storage.get::<_, LotteryState>(&DataKey::LotteryState).unwrap();

            if lottery_state == LotteryState::Initialized || 
                lottery_state == LotteryState::Active {
                panic_with_error!(&env, Error::AlreadyInitialized);
            }

            if lottery_state == LotteryState::Finished {
                panic_with_error!(&env, Error::AlreadyPlayed);
            }
        }
        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Token, &token);
        storage.set(&DataKey::LotteryResults, &Map::<u32, LotteryResult>::new(&env));
        storage.set(&DataKey::LotteryState, &LotteryState::Initialized);
        Self::create_lottery(env, ticket_price, number_of_numbers, max_range, thresholds, min_players_count).unwrap();
    }

    /// Creates the new lottery.
    /// This function must be invoked byt the administrator. 
    /// It can be called each time after previous lottery
    /// has been completed. New lottery can have different specs,
    /// pool balance is carried over from the previous one. 
    /// All previously stored tickets are cleared
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    /// - `ticket_price` - Unitary ticket price for the current raffle.
    /// - `number_of_elements` - Number of numbers possible to select by players
    /// - `max_range` - Right boundary of the range players will select numbers from (1, max_range)
    /// - `thresholds` - Thresholds with prizes for correctly selected numbers (specified as percentage of the pool balance)
    /// - `min_players_count` - Minimum number of players needed to play the lottery
    pub fn create_lottery(
        env: Env,
        ticket_price: i128,
        number_of_numbers: u32,
        max_range: u32,
        thresholds: Map<u32, u32>,
        min_players_count: u32
    ) -> Result<(), Error> {
        let storage = env.storage().persistent();
        if storage
            .get::<_, LotteryState>(&DataKey::LotteryState)
            .is_none() {
            return Err(Error::NotInitialized);
        }

        let admin = storage.get::<_, Address>(&DataKey::Admin).unwrap();
        admin.require_auth();

        let lottery_state = storage.get::<_, LotteryState>(&DataKey::LotteryState).unwrap();
        if lottery_state == LotteryState::Active {
            return Err(Error::AlreadyActive);
        }

        if max_range < number_of_numbers {
            return Err(Error::MaxRangeTooLow);
        }

        if number_of_numbers < 2 {
            return Err(Error::NumberOfNumbersTooLow);
        }

        if thresholds.len() < 1 {
            return Err(Error::NumberOfThresholdsTooLow);
        }

        let mut lottery_number = 0;

        if storage.get::<_, u32>(&DataKey::LotteryNumber).is_some() {
            lottery_number = storage.get::<_, u32>(&DataKey::LotteryNumber).unwrap() + 1;
        }

        storage.set(&DataKey::NumberOfNumbers, &number_of_numbers);
        storage.set(&DataKey::MaxRange, &max_range);
        storage.set(&DataKey::Thresholds, &thresholds);
        storage.set(&DataKey::MinPlayersCount, &min_players_count);
        storage.set(&DataKey::TicketPrice, &ticket_price);
        storage.set(&DataKey::Tickets, &Map::<Address, Vec<LotteryTicket>>::new(&env));
        storage.set(&DataKey::LotteryState, &LotteryState::Active);
        storage.set(&DataKey::LotteryNumber, &lottery_number);

        Ok(())
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

        let player_selection_opt = tickets.get(by.clone());
        if player_selection_opt.is_some() {
            let mut player_selection = player_selection_opt.unwrap();
            player_selection.push_back(ticket);
            tickets.set(by, player_selection);
        } else {
            tickets.set(by, vec![&env, ticket]);
        }
        storage.set(&DataKey::Tickets, &tickets);
        Ok(tickets.values().len())
    }

    /// Returns actual pool balance. Can only be invoked by admin
    ///
    /// # Arguments
    ///
    /// - `env` - The environment for this contract.
    pub fn get_pool_balance(env: Env) -> Result<i128, Error> {
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
    /// # Arguments
    /// 
    /// - `lottery_number` - Number of the lottery
    pub fn check_lottery_results(env: Env, lottery_number: u32) -> Result<LotteryResult, Error> {
        let storage = env.storage().persistent();
        let lottery_results_opt = storage
            .get::<_, Map::<u32, LotteryResult>>(&DataKey::LotteryResults);

        if lottery_results_opt.is_none() {
            return Err(Error::NoLotteryResultsAvailable);
        }

        let lottery_results = lottery_results_opt.unwrap();
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
    /// - `random_seed` - A seed provided by the admin, that will be combined with other environment elements. See calculate_winners function.
    pub fn play_lottery(env: Env, random_seed: u64) -> Result<(), Error> {
        let storage = env.storage().persistent();

        lottery_must_be_active(&storage)?;

        let admin = storage.get::<_, Address>(&DataKey::Admin).unwrap();
        admin.require_auth();

        let token: Address = storage.get::<_, Address>(&DataKey::Token).unwrap();
        let token_client = token::Client::new(&env, &token);

        let tickets = storage
            .get::<_, Map::<Address, Vec<LotteryTicket>>>(&DataKey::Tickets)
            .unwrap();

        let min_players_count = storage
            .get::<_, u32>(&DataKey::MinPlayersCount)
            .unwrap();

        if tickets.keys().len() < min_players_count {
            return Err(Error::MinParticipantsNotSatisfied);
        }

        let pool = token_client.balance(&env.current_contract_address());
        let max_range = storage
            .get::<_, u32>(&DataKey::MaxRange)
            .unwrap();
        let number_of_elements = storage
            .get::<_, u32>(&DataKey::NumberOfNumbers)
            .unwrap();
        let mut thresholds = storage
            .get::<_, Map<u32, u32>>(&DataKey::Thresholds)
            .unwrap();

        let drawn_numbers = draw_numbers(&env, max_range, number_of_elements, random_seed);
        let winners = get_winners(&env, &drawn_numbers, &tickets, &thresholds);
        let prizes = calculate_prizes(&env, &winners, &mut thresholds, pool);
        payout_prizes(&env, &token_client, &prizes);

        // store numbers drawn in this lottery
        let lottery_number = storage.get::<_, u32>(&DataKey::LotteryNumber).unwrap();
        let lottery_results_opt = storage
            .get::<_, Map::<u32, LotteryResult>>(&DataKey::LotteryResults);
        if lottery_results_opt.is_some() {
            let mut lottery_results = lottery_results_opt.unwrap();
            lottery_results.set(lottery_number, drawn_numbers);
            storage.set(&DataKey::LotteryResults, &lottery_results);
        }
        else {
            let current_lottery_results = map![&env, (lottery_number, drawn_numbers)];
            storage.set(&DataKey::LotteryResults, &current_lottery_results);
        }

        storage.set(&DataKey::LotteryState, &LotteryState::Finished);

        // emit events with won prizes
        for address in prizes.keys() {
            let topic = (Symbol::new(&env, "won_prize"), &address);
            let prize = prizes.get(address.clone()).unwrap();
            env.events().publish(topic, prize);
        }
        Ok(())
    }
}

/// Ensures lottery is initialized and not finished. 
/// If not error is returned.
/// 
/// # Arguments
///
/// - `storage` - Contracts data storage
fn lottery_must_be_active(storage: &Persistent) -> Result<(), Error> {
    if storage
        .get::<_, LotteryState>(&DataKey::LotteryState)
        .is_none() {
        return Err(Error::NotInitialized);
    }
    let lottery_state = storage.get::<_, LotteryState>(&DataKey::LotteryState).unwrap();
    if lottery_state != LotteryState::Active {
        return Err(Error::NotActive);
    }
    Ok(())
}

/// Calculates the winners of a lottery. There can be several winners 
/// with different prizes according to number of correctly selected numbers and 
/// specified thresholds
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
    thresholds: &Map<u32, u32>
) -> Map<u32, Vec<Address>> {
    let mut winners = Map::<u32, Vec<Address>>::new(&env);

    for ticket_address in tickets.keys() {
        for ticket in tickets.get(ticket_address.clone()).unwrap() {
            let count = count_matches(&drawn_numbers, &ticket);
            if thresholds.contains_key(count) {                
                let mut addresses = if winners.contains_key(count) {
                    winners.get(count).unwrap()
                }
                else {
                    Vec::<Address>::new(&env)
                };
                addresses.push_back(ticket_address.clone());
                winners.set(count, addresses);
            }
        }
    }
    winners
}

/// Calculates prizes for winning players taking into account specified thresholds.
/// In case when total prizes are higher than available pool balance 
/// thresholds are recalculated.
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
    pool: i128
) -> Map<Address, i128> {
    
    let mut prizes = Map::<Address, i128>::new(&env);
    
    // sum total percentage of prizes won - if it's bigger than 100%, recalculate new thresholds to be equal to 100%
    let total_prizes_percentage = count_total_prizes_percentage(&winners, &thresholds);
    recalculate_new_thresholds(winners, thresholds, total_prizes_percentage);    

    for threshold_number in winners.keys() {
        let threshold_value = thresholds.get(threshold_number).unwrap() as f64 / 100.0;
        let prize = (pool as f64 * threshold_value).round() as i128;

        for address in winners.get(threshold_number).unwrap() {
            let current_player_prize = prizes.get(address.clone());
            if current_player_prize.is_none() {
                prizes.set(address, prize);
            } else {
                prizes.set(address, current_player_prize.unwrap() + prize);
            }
        }
    }
    prizes
}

/// Pays out prizes to winning players
/// 
/// # Arguments
/// - `env` - The environment for this contract.
/// - `token_client` - A token client used for token transfers,
/// - `prizes` - A map containing winning adresses and their prizes
fn payout_prizes(
    env: &Env,
    token_client: &token::Client,
    prizes: &Map<Address, i128>
) {
    for player_prize in prizes.keys() {
        let prize = prizes.get(player_prize.clone()).unwrap();
        let address = player_prize;

        token_client.transfer(&env.current_contract_address(), &address, &prize);
    }
}

/// Counts total prizes percentage by summing up prizes of all winners
/// 
/// # Arguments
/// - `winners` - A map containing winning players with a number of correctly selected numbers
/// - `thresholds` - Defined thresholds with prizes
fn count_total_prizes_percentage(
    winners: &Map<u32, Vec<Address>>,
    thresholds: &Map<u32, u32>,
) -> u32 {
    let mut count = 0;
    for threshold_number in winners.keys() {
        let threshold_percentage = thresholds.get(threshold_number).unwrap();
        let winners_count = winners.get(threshold_number).unwrap().len(); 
        count += threshold_percentage * winners_count;
    }
    count
}

/// Recalculates new thresholds in case total prizes percentage is above 100%
/// 
/// # Arguments
/// - `winners` - A map containing winning players with a number of correctly selected numbers
/// - `thresholds` - Defined thresholds with prizes, will be updated 
/// in case total_prizes_percentage > 100
/// - `total_prizes_percentage` - Total prizes percentage
fn recalculate_new_thresholds(
    winners: &Map<u32, Vec<Address>>,
    thresholds: &mut Map<u32, u32>,
    total_prizes_percentage: u32
) {
    if total_prizes_percentage > 100 {        
        for threshold_number in thresholds.keys() {
            if (winners.contains_key(threshold_number)) {
                let winners_count = winners.get(threshold_number).unwrap().len() as f32;
                let threshold_precentage = thresholds.get(threshold_number).unwrap() as f32;
                let val = winners_count * threshold_precentage * 100.0 / total_prizes_percentage as f32;
                thresholds.set(threshold_number, (val / winners_count).floor() as u32);
            }
            else {
                thresholds.remove(threshold_number);
            }
        }
    }
}

/// Counts properly selected numbers for a given ticket
/// 
/// # Arguments
/// - `drawn_numbers` - An array containing numbers selected in this lottery,
/// - `player_ticket` - Numbers selected by a player
fn count_matches(
    drawn_numbers: &Vec<u32>,
    player_ticket: &LotteryTicket
) -> u32 {
    let mut count = 0;
    for n in 0..drawn_numbers.len() {
        let number = drawn_numbers.get(n).unwrap();
        if player_ticket.contains(number) {
            count += 1;
        }
    }
    count
}

/// Randomly draw numbers within a given range (1, max_range). 
/// Ensures that there are no duplicates
/// 
/// # Arguments
///
/// - `env` - The environment for this contract.
/// - `max_range` - Right boundary of the range players will select numbers from (1, max_range)
/// - `number_of_numbers` - Number of numbers possible to select by players
/// - `random_seed` - A seed provided by the admin, that will be combined with other environment elements
fn draw_numbers(
    env: &Env,
    max_range: u32,
    number_of_numbers: u32,
    random_seed: u64,
) -> Vec<u32> {

    let mut numbers = Vec::new(&env);
    for n in 0..number_of_numbers {
        let new_seed = random_seed + n as u64;
        let mut rand = SmallRng::seed_from_u64(new_seed);
        loop {
            // draw a number so many times until a new unique number is found
            let drawn_number = rand.gen_range(1..max_range);
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
