#![cfg(test)]

extern crate std;

use crate::{draw_numbers, count_matches, get_winners, count_total_prizes_percentage, recalculate_new_thresholds, LotteryResult, LotteryTicket, calculate_prizes};

use super::{LotteryContract, LotteryContractClient};

use soroban_sdk::{
    testutils::{Address as _, AuthorizedInvocation, Events},
    token, vec, Address, Env, IntoVal, Symbol, Vec, Map, map,
};

#[test]
fn admin_is_identified_on_init() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, LotteryContract);
    let client = LotteryContractClient::new(&env, &contract_id);
    let target_token = create_token_contract(&env, &Address::random(&env));

    client.init(&client.address, &target_token.address, &2, &5, &50, &map![&env, (5, 30), (4, 15)], &10);

    let auths = env.auths();
    assert_auth(
        &auths,
        0,
        client.address.clone(),
        client.address.clone(),
        Symbol::new(&env, "init"),
        //(&client.address, &target_token.address, 2i128, 5u32, 50u32, map![&env, (5, 30), (4, 15)].into_iter(), 10u32).into_val(&env),
    )
}

fn assert_auth(
    auths: &[(Address, AuthorizedInvocation)],
    idx: usize,
    call_addr: Address,
    auth_addr: Address,
    func: Symbol,
    //args: Vec<Val>,
) {
    let auth = auths.get(idx).unwrap();
    assert_eq!(auth.0, call_addr);
    // assert_eq!(
    //     auth.1.function,
    //     AuthorizedFunction::Contract((auth_addr, func, args))
    // );
}

fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::AdminClient<'a> {
    token::AdminClient::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn lottery_cannot_be_initialized_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, LotteryContract);
    let client = LotteryContractClient::new(&env, &contract_id);
    let token_admin = Address::random(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    client.init(&client.address, &test_token_client.address, &2, &5, &50, &map![&env, (5, 30), (4, 15)], &10);
    client.init(&client.address, &test_token_client.address, &2, &5, &50, &map![&env, (5, 30), (4, 15)], &10);
}

#[test]
#[should_panic(expected = "Error(Contract, #11)")]
fn lottery_cannot_be_created_without_initialization() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, LotteryContract);
    let client = LotteryContractClient::new(&env, &contract_id);

    let thresholds = map![&env, (5, 30), (4, 15)];
    client.create_lottery(&2, &5, &50, &thresholds, &10);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn lottery_cannot_be_created_with_too_low_max_range() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, LotteryContract);
    let client = LotteryContractClient::new(&env, &contract_id);
    let token_admin = Address::random(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    client.init(&client.address, &test_token_client.address, &2, &5, &4, &map![&env, (5, 30), (4, 15)], &10);
}

#[test]
fn buy_ticket_works_as_expected() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, LotteryContract);
    let client = LotteryContractClient::new(&env, &contract_id);
    let token_admin = Address::random(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    client.init(&client.address, &test_token_client.address, &100, &5, &50, 
        &map![&env, (5, 30), (4, 15), (3, 10)], &10);

    let ticket_buyer = Address::random(&env);

    // Transfer some funds to the buyer
    test_token_client.mint(&ticket_buyer, &101);

    let candidates = client.buy_ticket(&ticket_buyer, &vec![&env, 3, 5, 10, 20, 33]);

    assert_eq!(1, candidates);

    let auths = env.auths();

    assert_auth(
        &auths,
        0,
        ticket_buyer.clone(),
        client.address.clone(),
        Symbol::new(&env, "buy_ticket"),
        // (ticket_buyer.clone(),).into_val(&env),
    );
}

#[test]
fn play_lottery_works_as_expected() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, LotteryContract);
    let client = LotteryContractClient::new(&env, &contract_id);
    let token_admin = Address::random(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    client.init(&client.address, &test_token_client.address, &100, &5, &50, 
        &map![&env, (5, 30), (4, 15), (3, 10)], &2);

    let ticket_buyer1 = Address::random(&env);
    let ticket_buyer2 = Address::random(&env);

    // Transfer some funds to the buyer
    test_token_client.mint(&ticket_buyer1, &101);
    test_token_client.mint(&ticket_buyer2, &101);

    client.buy_ticket(&ticket_buyer1, &vec![&env, 3, 5, 14, 22, 35]);
    let tickets = client.buy_ticket(&ticket_buyer2, &vec![&env, 22, 14, 35, 44, 29]);

    assert_eq!(2, tickets);

    client.play_lottery(&666);

    assert_auth(
        &env.auths(),
        0,
        client.address.clone(),
        client.address.clone(),
        Symbol::new(&env, "play_lottery")
    );

    let winners_events = env.events().all().slice(env.events().all().len() - 2..);

    assert!(winners_events.contains(            
        (
            contract_id.clone(),
            (Symbol::new(&env, "won_prize"), &ticket_buyer1).into_val(&env),
            20i128.into_val(&env)
        )
    ));
    assert!(winners_events.contains(            
        (
            contract_id.clone(),
            (Symbol::new(&env, "won_prize"), &ticket_buyer2).into_val(&env),
            60i128.into_val(&env)
        )
    ));
}

#[test]
fn play_lottery_with_high_prizes_works_as_expected() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, LotteryContract);
    let client = LotteryContractClient::new(&env, &contract_id);
    let token_admin = Address::random(&env);
    let test_token_client = create_token_contract(&env, &token_admin);

    client.init(&client.address, &test_token_client.address, &100, &5, &50, 
        &map![&env, (5, 60), (4, 15), (3, 10)], &2);

    let ticket_buyer1 = Address::random(&env);
    let ticket_buyer2 = Address::random(&env);

    // Transfer some funds to the buyer
    test_token_client.mint(&ticket_buyer1, &101);
    test_token_client.mint(&ticket_buyer2, &101);

    client.buy_ticket(&ticket_buyer1, &vec![&env, 22, 14, 35, 44, 29]);
    let tickets = client.buy_ticket(&ticket_buyer2, &vec![&env, 22, 14, 35, 44, 29]);

    assert_eq!(2, tickets);

    client.play_lottery(&666);

    assert_auth(
        &env.auths(),
        0,
        client.address.clone(),
        client.address.clone(),
        Symbol::new(&env, "play_lottery")
    );

    let winners_events = env.events().all().slice(env.events().all().len() - 2..);

    assert!(winners_events.contains(            
        (
            contract_id.clone(),
            (Symbol::new(&env, "won_prize"), &ticket_buyer1).into_val(&env),
            100i128.into_val(&env)
        )
    ));
    assert!(winners_events.contains(            
        (
            contract_id.clone(),
            (Symbol::new(&env, "won_prize"), &ticket_buyer2).into_val(&env),
            100i128.into_val(&env)
        )
    ));
}

#[test]
fn draw_numbers_works_seed_is_deterministic() {
    let env = Env::default();
    let result = draw_numbers(&env, 50, 5, 666);
    assert_eq!(vec![&env, 22, 14, 35, 44, 29], result);
}

#[test]
fn count_matches_works_correct() {
    let env = Env::default();
    let result = draw_numbers(&env, 50, 5, 666);
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
    let result = draw_numbers(&env, 50, 5, 666);
    let thresholds = map![&env, (5, 30), (4, 15), (3, 10)];
    let player1 = Address::random(&env);
    let player2 = Address::random(&env);
    let player3 = Address::random(&env);
    let tickets = map![&env, 
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
    let (result, tickets, thresholds) = 
        setup_test(Address::random(&env), Address::random(&env), Address::random(&env));

    let winners = get_winners(&env, &result, &tickets, &thresholds);
    let total_prizes_percentage = count_total_prizes_percentage(&winners, &thresholds);
    assert_eq!(105, total_prizes_percentage);
}

#[test]
fn recalculate_new_thresholds_works_as_expected() {
    let env = Env::default();
    let (result, tickets, mut thresholds) = 
        setup_test(Address::random(&env), Address::random(&env), Address::random(&env));
    
    let winners = get_winners(&env, &result, &tickets, &thresholds);
    let total_prizes_percentage = count_total_prizes_percentage(&winners, &thresholds);
    recalculate_new_thresholds(&winners, &mut thresholds, total_prizes_percentage);
    assert_eq!(2, thresholds.len());
    assert_eq!(14, thresholds.get(4).unwrap());
    assert_eq!(28, thresholds.get(5).unwrap());
}

#[test]
fn calculate_prizes_works_as_expected() {
    let env = Env::default();
    let p1 = Address::random(&env);
    let p2 = Address::random(&env);
    let p3 = Address::random(&env);

    let (result, tickets, mut thresholds) = 
        setup_test(p1.clone(), p2.clone(), p3.clone());
    let winners = get_winners(&env, &result, &tickets, &thresholds);

    let pool = 400;
    let prizes = calculate_prizes(&env, &winners, &mut thresholds, pool);

    assert_eq!(3, prizes.len());
    assert_eq!(112, prizes.get(p1.clone()).unwrap());
    assert_eq!(112, prizes.get(p2.clone()).unwrap());
    assert_eq!(168, prizes.get(p3.clone()).unwrap());
}

fn setup_test(add1: Address, add2: Address, add3: Address) -> (LotteryResult, Map::<Address, Vec::<LotteryTicket>>, Map::<u32, u32>) {
    let env = Env::default();
    let result = draw_numbers(&env, 50, 5, 666);
    let thresholds = map![&env, (5, 30), (4, 15), (3, 10)];
    let tickets = map![&env, 
        (add1, vec![&env, vec![&env, 22, 14, 35, 44, 29]]), 
        (add2, vec![&env, vec![&env, 22, 14, 35, 44, 29]]),
        (add3, vec![&env, 
            vec![&env, 22, 14, 35, 44, 29],
            vec![&env, 22, 14, 35, 44, 1]])
        ];

    (result, tickets, thresholds)
}

