#![no_std]
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use soroban_sdk::{contractimpl, contracttype, Address, Env, Vec};

#[derive(Clone)]
#[contracttype]
enum DataKey {
    Admin,
    Candidates,
    WinnerCount,
    TicketPrice,
}

pub struct LotteryContract;

#[contractimpl]
impl LotteryContract {
    pub fn initialize(env: Env, admin: Address, winners_count: u32, ticket_price: u32) {
        admin.require_auth();
        let storage = env.storage();
        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::WinnerCount, &winners_count);
        storage.set(&DataKey::TicketPrice, &ticket_price);
        storage.set(&DataKey::Candidates, &Vec::<Address>::new(&env));
    }

    pub fn buy_ticket(env: Env, by: Address) {
        let storage = env.storage();
        // let price: u32 = storage.get(&DataKey::TicketPrice).unwrap().unwrap();
        // let admin: Address = storage.get(&DataKey::Admin).unwrap().unwrap();
        // token.transfer(env, admin, by, price);
        let winner_count: u32 = storage.get(&DataKey::WinnerCount).unwrap().unwrap();
        let mut candidates: Vec<Address> = storage.get(&DataKey::Candidates).unwrap().unwrap();
        candidates.push_back(by);
        storage.set(&DataKey::Candidates, &candidates);
        
    }

    pub fn play_raffle(env: Env, admin: Address, random_seed: u64) {
        let storage = env.storage();

        // TODO: Assert admin
        let mut candidates: Vec<Address> = storage.get(&DataKey::Candidates).unwrap().unwrap();

        let winners_count: u32 = storage.get(&DataKey::WinnerCount).unwrap().unwrap();
        let players = candidates.len();

        let mut rand = SmallRng::seed_from_u64(random_seed);
        // TODO: Get balance
        // let balance = token::spendable_balance(&token_id);
        let balance = 999;
        let payout = balance / winners_count;
        let mut winners = Vec::new(&env);
        // Todo already winners cannot win twice
        for i in 0..winners_count {
            let winner = rand.gen_range(0..players);
            winners.push_back(winner);
        }
        for winner in winners {
            let candidate = candidates.get(winner.unwrap()).unwrap().unwrap();
            // TODO: token::transfer(candidate, &payout);
            // let topics = (Symbol::short("winner"), admin, candidate);
            // env.events().publish(topics, amount);
        }
    }
}

mod test;
