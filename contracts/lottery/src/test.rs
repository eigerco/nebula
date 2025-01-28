#![cfg(test)]

extern crate std;

use super::*;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use soroban_sdk::{
    map,
    testutils::{Address as _, AuthorizedInvocation, Events, AuthorizedFunction},
    token, vec, Address, Env, IntoVal, Map, Symbol, Vec, Val,
};

struct TestRandomNumberGenerator(SmallRng);

impl RandomNumberGeneratorTrait for TestRandomNumberGenerator {
    fn new(_: &Env, seed: u64) -> Self {
        TestRandomNumberGenerator(SmallRng::seed_from_u64(seed))
    }

    fn number(&mut self, _: &Env, max_range: u32) -> u32 {
        self.0.gen_range(1..max_range)
    }
}

#[test]
fn admin_is_identified_on_init() {
    let test_scenario = setup_test_scenario();
    let thresholds = map![&test_scenario.env, (5, 30), (4, 15)];

    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &2,
        &5,
        &50,
        &thresholds,
        &10,
    );

    let auths = test_scenario.env.auths();
    assert_auth(
        &auths,
        0,
        test_scenario.client.address.clone(),
        test_scenario.client.address.clone(),
        Symbol::new(&test_scenario.env, "init"),
        (&test_scenario.client.address, &test_scenario.test_token_client.address, 2i128, 5u32, 50u32, thresholds, 10u32).into_val(&test_scenario.env),
    )
}

#[test]
fn lottery_is_properly_created() {
    let test_scenario = setup_test_scenario();
    let thresholds = map![&test_scenario.env, (5, 30), (4, 15)];

    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &2,
        &5,
        &50,
        &thresholds,
        &10,
    );

    let last_event = test_scenario.env.events().all().slice(test_scenario.env.events().all().len() - 1..);
    assert_eq!(
        last_event,
        vec![
            &test_scenario.env,
            (
                test_scenario.contract_id.clone(),
                (Symbol::new(&test_scenario.env, "new_lottery_created"), 1u32).into_val(&test_scenario.env),
                (5u32, 50u32, thresholds, 2i128).into_val(&test_scenario.env)
            )
        ]
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn lottery_cannot_be_initialized_twice() {
    let test_scenario = setup_test_scenario();
    let thresholds = map![&test_scenario.env, (5, 30), (4, 15)];

    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &2,
        &5,
        &50,
        &thresholds,
        &10,
    );
    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &2,
        &5,
        &50,
        &thresholds,
        &10,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn lottery_cannot_be_created_without_initialization() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, LotteryContract);
    let client = LotteryContractClient::new(&env, &contract_id);

    let thresholds = map![&env, (5, 30), (4, 15)];
    client.create_lottery(&2, &5, &50, &thresholds, &10);
}

#[test]
#[should_panic(expected = "Error(Contract, #12)")]
fn lottery_cannot_be_created_if_it_is_initialized() {
    let test_scenario = setup_test_scenario();
    let thresholds = map![&test_scenario.env, (5, 30), (4, 15)];

    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &2,
        &5,
        &50,
        &thresholds,
        &10,
    );
    test_scenario.client.create_lottery(
        &2,
        &5,
        &50,
        &thresholds,
        &10,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn lottery_cannot_be_created_with_too_low_max_range() {
    let test_scenario = setup_test_scenario();

    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &2,
        &5,
        &4,
        &map![&test_scenario.env, (5, 30), (4, 15)],
        &10,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn lottery_should_have_at_least_2_numbers_to_select() {
    let test_scenario = setup_test_scenario();

    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &2,
        &1,
        &4,
        &map![&test_scenario.env, (5, 30), (4, 15)],
        &10,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #15)")]
fn ticket_price_should_be_above_0() {
    let test_scenario = setup_test_scenario();
    
    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &0,
        &5,
        &50,
        &map![&test_scenario.env, (5, 70), (4, 40)],
        &10,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn lottery_should_have_at_least_1_threshold_defined() {
    let test_scenario = setup_test_scenario();
    
    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &2,
        &5,
        &50,
        &map![&test_scenario.env],
        &10,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #14)")]
fn thresholds_percentage_sum_should_be_below_100() {
    let test_scenario = setup_test_scenario();
    
    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &2,
        &5,
        &50,
        &map![&test_scenario.env, (5, 70), (4, 40)],
        &10,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #14)")]
fn thresholds_percentage_sum_should_be_above_0() {
    let test_scenario = setup_test_scenario();
    
    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &2,
        &5,
        &50,
        &map![&test_scenario.env, (5, 0)],
        &10,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #14)")]
fn threshold_numbers_should_be_below_number_of_numbers() {
    let test_scenario = setup_test_scenario();
    
    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &2,
        &5,
        &50,
        &map![&test_scenario.env, (6, 30), (1, 15)],
        &10,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #14)")]
fn threshold_numbers_should_be_above_1() {
    let test_scenario = setup_test_scenario();
    
    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &2,
        &5,
        &50,
        &map![&test_scenario.env, (5, 30), (0, 15)],
        &10,
    );
}


#[test]
fn users_can_buy_tickets() {
    let test_scenario = setup_test_scenario();

    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &100,
        &5,
        &50,
        &map![&test_scenario.env, (5, 30), (4, 15), (3, 10)],
        &10,
    );

    let ticket_buyer = Address::generate(&test_scenario.env);

    test_scenario.test_token_client.mint(&ticket_buyer, &101);
    let ticket = vec![&test_scenario.env, 3, 5, 10, 20, 33];

    let candidates = test_scenario.client.buy_ticket(&ticket_buyer, &ticket);

    assert_eq!(1, candidates);

    let auths = test_scenario.env.auths();

    assert_auth(
        &auths,
        0,
        ticket_buyer.clone(),
        test_scenario.client.address.clone(),
        Symbol::new(&test_scenario.env, "buy_ticket"),
        (ticket_buyer.clone(), ticket).into_val(&test_scenario.env),
    );

    assert_eq!(1, test_scenario.token_client.balance(&ticket_buyer));
    assert_eq!(100, test_scenario.token_client.balance(&test_scenario.contract_id));
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn tickets_cannot_be_bought_for_not_initialized_lottery() {
    let test_scenario = setup_test_scenario();

    let ticket_buyer = Address::generate(&test_scenario.env);

    test_scenario.test_token_client.mint(&ticket_buyer, &101);
    let ticket = vec![&test_scenario.env, 3, 5, 10, 20, 33];

    test_scenario.client.buy_ticket(&ticket_buyer, &ticket);
}

#[test]
#[should_panic(expected = "Error(Contract, #13)")]
fn tickets_cannot_be_bought_for_finished_lottery() {
    let test_scenario = setup_test_scenario();

    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &100,
        &5,
        &50,
        &map![&test_scenario.env, (5, 30), (4, 15), (3, 10)],
        &2,
    );

    let ticket_buyer1 = Address::generate(&test_scenario.env);
    let ticket_buyer2 = Address::generate(&test_scenario.env);

    test_scenario.test_token_client.mint(&ticket_buyer1, &200);
    test_scenario.test_token_client.mint(&ticket_buyer2, &101);

    test_scenario.client.buy_ticket(&ticket_buyer1, &vec![&test_scenario.env, 3, 5, 14, 22, 35]);
    let tickets = test_scenario.client.buy_ticket(&ticket_buyer2, &vec![&test_scenario.env, 22, 14, 35, 44, 29]);

    assert_eq!(2, tickets);

    test_scenario.client.play_lottery(&666);
    test_scenario.client.buy_ticket(&ticket_buyer1, &vec![&test_scenario.env, 3, 5, 14, 22, 35]);
}

#[test]
#[should_panic(expected = "Error(Contract, #8)")]
fn ticket_should_have_the_same_number_of_numbers_as_defined_in_lottery() {
    let test_scenario = setup_test_scenario();

    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &100,
        &5,
        &50,
        &map![&test_scenario.env, (5, 30), (4, 15), (3, 10)],
        &10,
    );

    let ticket_buyer = Address::generate(&test_scenario.env);

    test_scenario.test_token_client.mint(&ticket_buyer, &101);
    test_scenario.client.buy_ticket(&ticket_buyer, &vec![&test_scenario.env, 3, 5, 10, 20]);
}

#[test]
#[should_panic(expected = "Error(Contract, #9)")]
fn all_ticket_numbers_should_be_in_the_specified_range() {
    let test_scenario = setup_test_scenario();

    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &100,
        &5,
        &50,
        &map![&test_scenario.env, (5, 30), (4, 15), (3, 10)],
        &10,
    );

    let ticket_buyer = Address::generate(&test_scenario.env);

    test_scenario.test_token_client.mint(&ticket_buyer, &101);
    test_scenario.client.buy_ticket(&ticket_buyer, &vec![&test_scenario.env, 1, 5, 10, 20, 51]);
}

#[test]
fn play_lottery_works_as_expected() {
    let test_scenario = setup_test_scenario();

    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &100,
        &5,
        &50,
        &map![&test_scenario.env, (5, 30), (4, 15), (3, 10)],
        &2,
    );

    let ticket_buyer1 = Address::generate(&test_scenario.env);
    let ticket_buyer2 = Address::generate(&test_scenario.env);

    test_scenario.test_token_client.mint(&ticket_buyer1, &101);
    test_scenario.test_token_client.mint(&ticket_buyer2, &101);

    test_scenario.client.buy_ticket(&ticket_buyer1, &vec![&test_scenario.env, 3, 5, 45, 2, 24]);
    let tickets = test_scenario.client.buy_ticket(&ticket_buyer2, &vec![&test_scenario.env, 44, 4, 45, 2, 24]);

    assert_eq!(2, tickets);

    test_scenario.client.play_lottery(&666);

    assert_auth(
        &test_scenario.env.auths(),
        0,
        test_scenario.client.address.clone(),
        test_scenario.client.address.clone(),
        Symbol::new(&test_scenario.env, "play_lottery"),
        (666u64, ).into_val(&test_scenario.env),
    );

    let winners_events = test_scenario.env.events().all().slice(test_scenario.env.events().all().len() - 2..);

    assert!(winners_events.contains((
        test_scenario.contract_id.clone(),
        (Symbol::new(&test_scenario.env, "won_prize"), &ticket_buyer1).into_val(&test_scenario.env),
        20i128.into_val(&test_scenario.env)
    )));
    assert!(winners_events.contains((
        test_scenario.contract_id.clone(),
        (Symbol::new(&test_scenario.env, "won_prize"), &ticket_buyer2).into_val(&test_scenario.env),
        60i128.into_val(&test_scenario.env)
    )));
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn lottery_cannot_be_played_not_initialized() {
    let test_scenario = setup_test_scenario();
    test_scenario.client.play_lottery(&666);
}

#[test]
#[should_panic(expected = "Error(Contract, #13)")]
fn lottery_cannot_be_played_if_finished() {
    let test_scenario = setup_test_scenario();

    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &100,
        &5,
        &50,
        &map![&test_scenario.env, (5, 30), (4, 15), (3, 10)],
        &2,
    );

    let ticket_buyer1 = Address::generate(&test_scenario.env);
    let ticket_buyer2 = Address::generate(&test_scenario.env);

    test_scenario.test_token_client.mint(&ticket_buyer1, &200);
    test_scenario.test_token_client.mint(&ticket_buyer2, &101);

    test_scenario.client.buy_ticket(&ticket_buyer1, &vec![&test_scenario.env, 3, 5, 14, 22, 35]);
    let tickets = test_scenario.client.buy_ticket(&ticket_buyer2, &vec![&test_scenario.env, 22, 14, 35, 44, 29]);

    assert_eq!(2, tickets);

    test_scenario.client.play_lottery(&666);
    test_scenario.client.play_lottery(&666);
}

#[test]
fn play_lottery_with_many_prizes_works_as_expected() {
    let test_scenario = setup_test_scenario();

    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &100,
        &5,
        &50,
        &map![&test_scenario.env, (5, 60), (4, 15), (3, 10)],
        &2,
    );

    let ticket_buyer1 = Address::generate(&test_scenario.env);
    let ticket_buyer2 = Address::generate(&test_scenario.env);

    // Transfer some funds to the buyer
    test_scenario.test_token_client.mint(&ticket_buyer1, &101);
    test_scenario.test_token_client.mint(&ticket_buyer2, &101);

    test_scenario.client.buy_ticket(&ticket_buyer1, &vec![&test_scenario.env, 44, 4, 45, 2, 24]);
    let tickets = test_scenario.client.buy_ticket(&ticket_buyer2, &vec![&test_scenario.env, 44, 4, 45, 2, 24]);

    assert_eq!(2, tickets);

    test_scenario.client.play_lottery(&666);

    assert_auth(
        &test_scenario.env.auths(),
        0,
        test_scenario.client.address.clone(),
        test_scenario.client.address.clone(),
        Symbol::new(&test_scenario.env, "play_lottery"),
        (666u64, ).into_val(&test_scenario.env),
    );

    let winners_events = test_scenario.env.events().all().slice(test_scenario.env.events().all().len() - 2..);

    assert!(winners_events.contains((
        test_scenario.contract_id.clone(),
        (Symbol::new(&test_scenario.env, "won_prize"), &ticket_buyer1).into_val(&test_scenario.env),
        100i128.into_val(&test_scenario.env)
    )));
    assert!(winners_events.contains((
        test_scenario.contract_id.clone(),
        (Symbol::new(&test_scenario.env, "won_prize"), &ticket_buyer2).into_val(&test_scenario.env),
        100i128.into_val(&test_scenario.env)
    )));
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn lottery_cannot_be_played_without_min_participants() {
    let test_scenario = setup_test_scenario();

    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &100,
        &5,
        &50,
        &map![&test_scenario.env, (5, 60), (4, 15), (3, 10)],
        &2,
    );

    let ticket_buyer1 = Address::generate(&test_scenario.env);

    test_scenario.test_token_client.mint(&ticket_buyer1, &101);

    test_scenario.client.buy_ticket(&ticket_buyer1, &vec![&test_scenario.env, 22, 14, 35, 44, 29]);
    test_scenario.client.play_lottery(&666);
}

#[test]
fn correct_lottery_results_are_returned() {
    let test_scenario = setup_test_scenario();

    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &100,
        &5,
        &50,
        &map![&test_scenario.env, (5, 30), (4, 15), (3, 10)],
        &1,
    );

    let ticket_buyer1 = Address::generate(&test_scenario.env);

    test_scenario.test_token_client.mint(&ticket_buyer1, &101);

    test_scenario.client.buy_ticket(&ticket_buyer1, &vec![&test_scenario.env, 3, 5, 14, 22, 35]);

    test_scenario.client.play_lottery(&666);

    let results = test_scenario.client.check_lottery_results(&1);
    assert_eq!(5, results.len());
    assert!(results.contains(44));
    assert!(results.contains(4));
    assert!(results.contains(45));
    assert!(results.contains(2));
    assert!(results.contains(24));
}

#[test]
#[should_panic(expected = "Error(Contract, #10)")]
fn should_not_return_results_for_wrong_lottery_number() {
    let test_scenario = setup_test_scenario();
    
    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &100,
        &5,
        &50,
        &map![&test_scenario.env, (5, 30), (4, 15), (3, 10)],
        &1,
    );

    let ticket_buyer1 = Address::generate(&test_scenario.env);

    test_scenario.test_token_client.mint(&ticket_buyer1, &101);

    test_scenario.client.buy_ticket(&ticket_buyer1, &vec![&test_scenario.env, 3, 5, 14, 22, 35]);

    test_scenario.client.play_lottery(&666);
    test_scenario.client.check_lottery_results(&2);
}

#[test]
#[should_panic(expected = "Error(Contract, #11)")]
fn should_not_return_results_for_not_played_lottery() {
    let test_scenario = setup_test_scenario();
    
    test_scenario.client.init(
        &test_scenario.client.address,
        &test_scenario.test_token_client.address,
        &100,
        &5,
        &50,
        &map![&test_scenario.env, (5, 30), (4, 15), (3, 10)],
        &1,
    );

    let ticket_buyer1 = Address::generate(&test_scenario.env);

    test_scenario.test_token_client.mint(&ticket_buyer1, &101);

    test_scenario.client.buy_ticket(&ticket_buyer1, &vec![&test_scenario.env, 3, 5, 14, 22, 35]);

    test_scenario.client.check_lottery_results(&1);
}

#[test]
fn draw_numbers_works_seed_is_deterministic() {
    let env = Env::default();
    let result = draw_numbers::<TestRandomNumberGenerator>(&env, 50, 5, 666);
    assert_eq!(vec![&env, 22, 14, 35, 44, 29], result);
}

#[test]
fn count_matches_counts_correctly() {
    let env = Env::default();
    let result = draw_numbers::<TestRandomNumberGenerator>(&env, 50, 5, 666);
    let mut matches = count_matches(&result, &vec![&env, 22, 14, 35, 44, 29]);
    assert_eq!(5, matches);
    matches = count_matches(&result, &vec![&env, 22, 14, 1, 2, 3]);
    assert_eq!(2, matches);
    matches = count_matches(&result, &vec![&env, 1, 2, 3, 4, 5]);
    assert_eq!(0, matches);
}

#[test]
fn get_winners_return_correct_winners() {
    let env = Env::default();
    let result = draw_numbers::<TestRandomNumberGenerator>(&env, 50, 5, 666);
    let thresholds = map![&env, (5, 30), (4, 15), (3, 10)];
    let player1 = Address::generate(&env);
    let player2 = Address::generate(&env);
    let player3 = Address::generate(&env);
    let tickets = map![
        &env,
        (player1.clone(), vec![&env, vec![&env, 22, 14, 35, 44, 29]]),
        (player2.clone(), vec![&env, vec![&env, 22, 14, 1, 2, 3]]),
        (player3.clone(), vec![&env, vec![&env, 22, 14, 35, 2, 3]])
    ];

    let winners = get_winners(&env, &result, &tickets, &thresholds);
    assert_eq!(2, winners.keys().len());
    assert_eq!(1, winners.get(5).unwrap().len());
    assert_eq!(1, winners.get(3).unwrap().len());
    assert_eq!(player1, winners.get(5).unwrap().get(0).unwrap());
    assert_eq!(player3, winners.get(3).unwrap().get(0).unwrap());
}

#[test]
fn count_total_prizes_percentage_counts_correctly() {
    let env = Env::default();
    let (result, tickets, thresholds) = setup_additional_test_data(
        &env,
        Address::generate(&env),
        Address::generate(&env),
        Address::generate(&env),
    );

    let winners = get_winners(&env, &result, &tickets, &thresholds);
    let total_prizes_percentage = count_total_prizes_percentage(&winners, &thresholds);
    assert_eq!(105, total_prizes_percentage);
}

#[test]
fn thresholds_are_properly_recalculated() {
    let env = Env::default();
    let (result, mut tickets, mut thresholds) = setup_additional_test_data(
        &env,
        Address::generate(&env),
        Address::generate(&env),
        Address::generate(&env),
    );

    tickets.set(Address::generate(&env), vec![&env, vec![&env, 22, 14, 35, 44, 29]]);
    tickets.set(Address::generate(&env), vec![&env, vec![&env, 22, 14, 1, 44, 29]]);
    tickets.set(Address::generate(&env), vec![&env, vec![&env, 22, 14, 35, 1, 2]]);
    tickets.set(Address::generate(&env), vec![&env, vec![&env, 22, 14, 3, 1, 2]]);

    // there are:
    // - 4 tickets with 5 hits
    // - 2 ticket with 4
    // - 1 ticket with 3
    // that gives a total of 160% of pool balance in prizes


    let winners = get_winners(&env, &result, &tickets, &thresholds);
    let total_prizes_percentage = count_total_prizes_percentage(&winners, &thresholds);
    recalculate_new_thresholds(&winners, &mut thresholds, total_prizes_percentage);
    assert_eq!(3, thresholds.len());
    assert_eq!(6, thresholds.get(3).unwrap());
    assert_eq!(9, thresholds.get(4).unwrap());
    assert_eq!(18, thresholds.get(5).unwrap());
}

#[test]
fn prizes_are_properly_calculated_and_assigned() {
    let env = Env::default();
    let p1 = Address::generate(&env);
    let p2 = Address::generate(&env);
    let p3 = Address::generate(&env);

    let (result, tickets, mut thresholds) = setup_additional_test_data(&env, p1.clone(), p2.clone(), p3.clone());
    let winners = get_winners(&env, &result, &tickets, &thresholds);

    let pool = 400;
    let prizes = calculate_prizes(&env, &winners, &mut thresholds, pool);

    assert_eq!(3, prizes.len());
    assert_eq!(112, prizes.get(p1.clone()).unwrap());
    assert_eq!(112, prizes.get(p2.clone()).unwrap());
    assert_eq!(168, prizes.get(p3.clone()).unwrap());
}

fn assert_auth(
    auths: &[(Address, AuthorizedInvocation)],
    idx: usize,
    call_addr: Address,
    auth_addr: Address,
    func: Symbol,
    args: Vec<Val>,
) {
    let auth = auths.get(idx).unwrap();
    assert_eq!(auth.0, call_addr);
    assert_eq!(
        auth.1.function,
        AuthorizedFunction::Contract((auth_addr, func, args))
    );
}

struct TestScenario<'a> {
    env: Env,
    contract_id: Address,
    client: LotteryContractClient<'a>,
    test_token_client: token::StellarAssetClient<'a>,
    token_client: token::Client<'a>
}

fn setup_test_scenario<'a>() -> TestScenario<'a> {
        
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, LotteryContract);
    let client = LotteryContractClient::new(&env, &contract_id);
    let token_admin = Address::generate(&env);
    let token_addr = env.register_stellar_asset_contract(token_admin.clone());
    let test_token_client = token::StellarAssetClient::new(&env, &token_addr);
    let token_client = token::Client::new(&env, &token_addr);

    TestScenario { 
        env, 
        contract_id, 
        client,
        test_token_client, 
        token_client
    }
}

fn setup_additional_test_data(
    env: &Env,
    add1: Address,
    add2: Address,
    add3: Address,
) -> (
    LotteryResult,
    Map<Address, Vec<LotteryTicket>>,
    Map<u32, u32>,
) {
    let result = draw_test_numbers(env, 50, 5, 666);
    let thresholds = map![&env, (5, 30), (4, 15), (3, 10)];

    let tickets = map![
        &env,
        (add1, vec![&env, vec![&env, 22, 14, 35, 44, 29]]),
        (add2, vec![&env, vec![&env, 22, 14, 35, 44, 29]]),
        (
            add3,
            vec![
                &env,
                vec![&env, 22, 14, 35, 44, 29],
                vec![&env, 22, 14, 35, 44, 1]
            ]
        )
    ];
    (result, tickets, thresholds)
}

fn draw_test_numbers(env: &Env, max_range: u32, number_of_numbers: u32, random_seed: u64) -> Vec<u32> {
    let mut numbers = Vec::new(env);
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
