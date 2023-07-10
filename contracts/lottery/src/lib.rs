#![no_std]

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use soroban_sdk::{
    contracterror, contractimpl, contracttype, token, Address, Env, Map, Symbol, Vec,
};

#[derive(Clone, Copy)]
#[contracttype]
enum DataKey {
    Admin = 1,
    Candidates = 2,
    WinnerCount = 3,
    TicketPrice = 4,
    Token = 5,
    AlreadyPlayed = 6,
}

#[contracterror]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InsufficientFunds = 1,
    AlreadyPlayed = 2,
    MinParticipantsNotSatisfied = 3,
}

pub struct LotteryContract;

#[contractimpl]
impl LotteryContract {
    pub fn initialize(
        env: Env,
        admin: Address,
        token: Address,
        winners_count: u32,
        ticket_price: i128,
    ) {
        admin.require_auth();
        let storage = env.storage();
        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Token, &token);
        storage.set(&DataKey::WinnerCount, &winners_count);
        storage.set(&DataKey::TicketPrice, &ticket_price);
        storage.set(&DataKey::Candidates, &Vec::<Address>::new(&env));
        storage.set(&DataKey::AlreadyPlayed, &false);
    }

    pub fn buy_ticket(env: Env, by: Address) -> Result<u32, Error> {
        by.require_auth();

        let storage = env.storage();
        let price: i128 = storage.get(&DataKey::TicketPrice).unwrap().unwrap();
        let token: Address = storage.get(&DataKey::Token).unwrap().unwrap();
        let token_client = token::Client::new(&env, &token);

        if token_client.balance(&by) <= price {
            return Err(Error::InsufficientFunds);
        }

        token_client.transfer(&by, &env.current_contract_address(), &price);

        let mut candidates: Vec<Address> = storage.get(&DataKey::Candidates).unwrap().unwrap();
        candidates.push_back(by);
        storage.set(&DataKey::Candidates, &candidates);
        Ok(candidates.len())
    }

    pub fn play_raffle(env: Env, random_seed: u64) -> Result<(), Error> {
        let storage = env.storage();

        let admin: Address = storage.get(&DataKey::Admin).unwrap().unwrap();
        admin.require_auth();

        if storage.get(&DataKey::AlreadyPlayed).unwrap().unwrap() {
            return Err(Error::AlreadyPlayed);
        }

        let token: Address = storage.get(&DataKey::Token).unwrap().unwrap();

        let token_client = token::Client::new(&env, &token);

        let candidates: Vec<Address> = storage.get(&DataKey::Candidates).unwrap().unwrap();

        if candidates.is_empty() {
            return Err(Error::MinParticipantsNotSatisfied);
        }

        let winners_count: u32 = storage.get(&DataKey::WinnerCount).unwrap().unwrap();
        let players = candidates.len();

        // Calculate the winners
        let winners_idx = calculate_winners(
            &env,
            winners_count,
            players,
            random_seed.checked_add(env.ledger().timestamp()).unwrap(), // TODO, this needs to be more investigated, as it could be very deterministic.
        );

        // Pay the winners
        let balance = token_client.balance(&env.current_contract_address());
        let payout = balance / i128::from(winners_count);

        for winner in winners_idx {
            let candidate = candidates.get(winner.unwrap()).unwrap().unwrap();
            token_client.transfer(&env.current_contract_address(), &candidate, &payout);
            let topics = (Symbol::short("winner"), candidate);
            env.events().publish(topics, payout);
        }
        storage.set(&DataKey::AlreadyPlayed, &true);
        Ok(())
    }
}

fn calculate_winners(
    env: &Env,
    winners_count: u32,
    candidates_len: u32,
    random_seed: u64,
) -> Vec<u32> {
    let mut winners = Map::new(env);
    let mut rand = SmallRng::seed_from_u64(random_seed);

    for _ in 0..winners_count {
        let winner = rand.gen_range(0..candidates_len);
        if winners.contains_key(winner) {
            continue;
        }
        winners.set(winner, 1);
    }
    winners.keys()
}

mod test;
