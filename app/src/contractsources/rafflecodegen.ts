export class RaffleCodeGen {
  public generateCode(name: string) {
    return `#![no_std]
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, token, Address, Env,
    Map, Symbol, Vec,
};

#[derive(Clone, Copy)]
#[contracttype]
enum DataKey {
    Admin = 1,
    Candidates = 2,
    MaxWinnerCount = 3,
    TicketPrice = 4,
    Token = 5,
    AlreadyInitialized = 6,
    AlreadyPlayed = 7,
}

#[contracterror]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    InsufficientFunds = 2,
    AlreadyPlayed = 3,
    MinParticipantsNotSatisfied = 4,
    InvalidMaxWinners = 5,
    MinimumTicketPrice = 6,
    NotInitialized = 7,
}

#[contract]
pub struct ${name};

#[contractimpl]
impl RaffleTrait for ${name} {
    pub fn init(
        env: Env,
        admin: Address,
        token: Address,
        max_winners_count: u32,
        ticket_price: i128,
    ) {
        admin.require_auth();
        let storage = env.storage().persistent();

        if max_winners_count == 0 {
            panic_with_error!(&env, Error::InvalidMaxWinners);
        }

        if ticket_price <= 1 {
            panic_with_error!(&env, Error::MinimumTicketPrice);
        }

        if storage
            .get::<_, bool>(&DataKey::AlreadyInitialized)
            .is_some()
        {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }

        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Token, &token);
        storage.set(&DataKey::MaxWinnerCount, &max_winners_count);
        storage.set(&DataKey::TicketPrice, &ticket_price);
        storage.set(&DataKey::Candidates, &Vec::<Address>::new(&env));
        storage.set(&DataKey::AlreadyPlayed, &false);
        storage.set(&DataKey::AlreadyInitialized, &true);
    }

    pub fn buy_ticket(env: Env, by: Address) -> Result<u32, Error> {
        by.require_auth();

        let storage = env.storage().persistent();

        if !storage
            .get::<_, bool>(&DataKey::AlreadyInitialized)
            .is_some()
        {
            return Err(Error::NotInitialized);
        }

        let price = storage.get::<_, i128>(&DataKey::TicketPrice).unwrap();
        let token = storage.get::<_, Address>(&DataKey::Token).unwrap();
        let token_client = token::Client::new(&env, &token);

        if token_client.balance(&by) <= price {
            return Err(Error::InsufficientFunds);
        }

        token_client.transfer(&by, &env.current_contract_address(), &price);

        let mut candidates = storage
            .get::<_, Vec<Address>>(&DataKey::Candidates)
            .unwrap();
        candidates.push_back(by);
        storage.set(&DataKey::Candidates, &candidates);
        Ok(candidates.len())
    }

    pub fn play_raffle(env: Env, random_seed: u64) -> Result<(), Error> {
        let storage = env.storage().persistent();

        if !storage
            .get::<_, bool>(&DataKey::AlreadyInitialized)
            .is_some()
        {
            return Err(Error::NotInitialized);
        }

        let admin = storage.get::<_, Address>(&DataKey::Admin).unwrap();
        admin.require_auth();

        if storage.get::<_, bool>(&DataKey::AlreadyPlayed).unwrap() {
            return Err(Error::AlreadyPlayed);
        }

        let token: Address = storage.get::<_, Address>(&DataKey::Token).unwrap();

        let token_client = token::Client::new(&env, &token);

        let candidates = storage
            .get::<_, Vec<Address>>(&DataKey::Candidates)
            .unwrap();

        if candidates.is_empty() {
            return Err(Error::MinParticipantsNotSatisfied);
        }

        let max_winners_count = storage.get::<_, u32>(&DataKey::MaxWinnerCount).unwrap();
        let players = candidates.len();

        // Calculate the winners
        let winners_idx = calculate_winners(
            &env,
            max_winners_count,
            players,
            random_seed.checked_add(env.ledger().timestamp()).unwrap(), // TODO, this needs to be more investigated, as it could be very deterministic.
        );

        // Pay the winners
        let balance = token_client.balance(&env.current_contract_address());
        let payout = balance / i128::from(max_winners_count);

        for winner in winners_idx {
            let candidate = candidates.get(winner).unwrap();
            token_client.transfer(&env.current_contract_address(), &candidate, &payout);
            let topics = (Symbol::new(&env, "winner"), candidate);
            env.events().publish(topics, payout);
        }
        storage.set(&DataKey::AlreadyPlayed, &true);
        Ok(())
    }
}

/// It calculates the winners of a raffle in a best effort way, avoiding
/// duplicate winners.
///
/// # Arguments
///
/// - 'env' - The environment for this contract.
/// - 'max_winners_count' - The maximum number of winners. Collisions from the random generator are currently solved by omission,
/// so this should be read as "the max number of" but not the "exact number of".
/// - 'candidates_len' - The number of participants on this raffle.
/// - 'random_seed' - The random seed for the number generator. Currently it determines the output of the generator across calls.
fn calculate_winners(
    env: &Env,
    max_winners_count: u32,
    candidates_len: u32,
    random_seed: u64,
) -> Vec<u32> {
    let mut winners = Map::new(env);
    let mut rand = SmallRng::seed_from_u64(random_seed);

    for _ in 0..max_winners_count {
        let winner = rand.gen_range(0..candidates_len);
        if winners.contains_key(winner) {
            continue;
        }
        winners.set(winner, true);
    }
    winners.keys()
}
    `
  }

  generateInvokeCommand(name: string, params: any[]) {
    const admin: string = params[0]
    const token: string = params[1]
    const maxWinnersCount: number = params[2]
    const ticketPrice: number = params[3]

    return `soroban contract invoke \\
--wasm ${name}.wasm \\
--id 1 \\
-- \\
init \\
    --admin ${admin} \\
    --token ${token} \\
    --max_winners_count ${maxWinnersCount} \\
    --ticket_price ${ticketPrice}`
  }

  getInvokes(commandId: any): object {
    return {
      lenses: [
        {
          range: {
            startLineNumber: 42,
            startColumn: 1,
            endLineNumber: 43,
            endColumn: 1,
          },
          id: 'invoke',
          command: {
            id: commandId('init', `--admin \\
        --token \\
        --max_winners_count 1 \\
        --ticket_price 1`),
            title: 'invoke',
          }
        }, {
          range: {
            startLineNumber: 76,
            startColumn: 1,
            endLineNumber: 77,
            endColumn: 1,
          },
          id: 'invoke',
          command: {
            id: commandId('buy_ticket', '--by'),
            title: 'invoke',
          }
        }, {
          range: {
            startLineNumber: 106,
            startColumn: 1,
            endLineNumber: 107,
            endColumn: 1,
          },
          id: 'invoke',
          command: {
            id: commandId('play_raffle', '--random_seed'),
            title: 'invoke',
          }
        }
      ]
    }
  }
}
