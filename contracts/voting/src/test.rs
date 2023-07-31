#![cfg(test)]

extern crate std;

use crate::{Error, Proposal, ProposalVotingContract, ProposalVotingContractClient};
use rstest::rstest;
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Ledger},
    Address, Env, IntoVal, Map, Symbol, Val, Vec,
};

#[test]
fn proposal_creation() {
    let (env, client, admin) = setup_test();

    env.mock_all_auths();

    let id = 1001u64;
    client.create_custom_proposal(&id, &3600, &50_00, &100);

    assert_auth(
        &env.auths(),
        0,
        admin,
        client.address,
        Symbol::new(&env, "create_custom_proposal"),
        (1001u64, 3600u64, 50_00u32, 100u32).into_val(&env),
    )
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

fn setup_test<'a>() -> (Env, ProposalVotingContractClient<'a>, Address) {
    let env = Env::default();
    let contract_id = env.register_contract(None, ProposalVotingContract);
    let client = ProposalVotingContractClient::new(&env, &contract_id);
    let admin = Address::random(&env);
    client.init(&admin, &3600, &50_00, &1000);

    (env, client, admin)
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn cannot_initialize_voting_twice() {
    let (env, client, admin) = setup_test();
    env.mock_all_auths();
    client.init(&admin, &3600, &50_00, &1000);
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn cannot_create_same_id_proposals() {
    let (env, client, _) = setup_test();
    env.mock_all_auths();

    let id = 1001u64;
    client.create_custom_proposal(&id, &3600, &50_00, &2);
    client.create_custom_proposal(&id, &3600, &50_00, &2);
}

#[test]
#[should_panic(expected = "Error(Auth, InvalidAction)")]
fn only_admin_can_create_proposals() {
    let (_, client, _) = setup_test();
    client.create_custom_proposal(&1, &3600, &50_00, &2);
}

#[test]
fn voter_can_vote_proposals() {
    let (env, client, _) = setup_test();
    env.mock_all_auths();

    let id = 12;

    client.create_custom_proposal(&id, &3600, &50_00, &2);
    client.vote(&client.address, &id);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn voter_cannot_vote_a_proposal_twice() {
    let (env, client, _) = setup_test();
    env.mock_all_auths();
    let prd_id = 12;
    client.create_custom_proposal(&prd_id, &3600, &50_00, &2);
    client.vote(&client.address, &prd_id);
    client.vote(&client.address, &prd_id); // Double voting here. Expected panic.
}

#[test]
fn cannot_vote_if_voting_time_exceeded() {
    let (mut env, _, _) = setup_test();

    let mut proposal = Proposal {
        id: 1,
        voting_end_time: env.ledger().timestamp() + 3600,
        votes: 0,
        voters: Map::<Address, bool>::new(&env),
        target_approval_rate_bps: 50_00,
        total_voters: 2,
    };

    advance_ledger_time_in(3600, &mut env);

    let result = proposal.vote(env.ledger().timestamp(), Address::random(&env));

    assert_eq!(Err(Error::VotingClosed), result)
}

#[test]
fn cannot_vote_if_total_voters_reached() {
    let (env, _, _) = setup_test();

    let mut voters = Map::<Address, bool>::new(&env);

    voters.set(Address::random(&env), true); // Dummy voters
    voters.set(Address::random(&env), true); // Dummy voters

    let mut proposal = Proposal {
        id: 1,
        voting_end_time: env.ledger().timestamp() + 3600,
        votes: 2,
        voters,
        target_approval_rate_bps: 50_00,
        total_voters: 2,
    };

    let result = proposal.vote(env.ledger().timestamp(), Address::random(&env));
    assert_eq!(Err(Error::VotingClosed), result)
}

fn advance_ledger_time_in(time: u64, env: &mut Env) {
    let mut ledger_info = env.ledger().get();
    ledger_info.timestamp += time;
    env.ledger().set(ledger_info)
}

#[rstest]
#[case::rate_50(2, 1, 50_00, true)]
#[case::precision_is_captured_in_bps(3, 1, 33_33, false)]
#[case::rate_100(2, 2, 10_000, true)]
#[case::no_votes_no_rate(0, 0, 0, false)]
fn proposal_calculate_approval_rate(
    #[case] total_voters: u32,
    #[case] votes: u32,
    #[case] expected: u32,
    #[case] is_approved: bool,
) {
    let (env, _, _) = setup_test();

    let mut voters = Map::<Address, bool>::new(&env);

    for _ in 0..votes {
        voters.set(Address::random(&env), true); // Dummy voters
    }

    let proposal = Proposal {
        id: 1,
        voting_end_time: env.ledger().timestamp() + 3600,
        votes,
        target_approval_rate_bps: 50_00,
        voters,
        total_voters,
    };

    assert_eq!(Ok(expected), proposal.approval_rate_bps());
    assert!(is_approved == proposal.is_approved());
}

#[test]
#[should_panic(expected = "Error(Contract, #9)")]
fn cannot_create_id0_proposals() {
    let (env, client, _) = setup_test();
    env.mock_all_auths();
    client.create_custom_proposal(&0, &3600, &50_00, &2);
}
