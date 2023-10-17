#![cfg(test)]

use crate::{
    Error, Proposal, ProposalPayload, ProposalVotingContract, ProposalVotingContractClient,
};
use rstest::rstest;
use soroban_sdk::{
    testutils::{
        Address as _, AuthorizedFunction, AuthorizedInvocation, BytesN as _, Events, Ledger,
    },
    vec, Address, BytesN, Env, IntoVal, Map, Symbol, Val, Vec,
};

#[test]
fn proposal_creation() {
    let (env, client, _admin) = setup_test();

    env.mock_all_auths();

    let id = 1001u64;
    let comment = BytesN::random(&env);
    let payload = ProposalPayload::Comment(comment.clone());

    client.create_custom_proposal(&id, &payload, &client.address, &3600, &50_00, &100);

    assert_auth(
        &env.auths(),
        0,
        client.address.clone(),
        client.address.clone(),
        Symbol::new(&env, "create_custom_proposal"),
        (
            id,
            payload.clone(),
            client.address.clone(),
            3600u64,
            50_00u32,
            100_u128,
        )
            .into_val(&env),
    );

    let last_event = env.events().all().last().unwrap();
    assert_eq!(
        vec![&env, last_event],
        vec![
            &env,
            (
                client.address.clone(),
                (
                    Symbol::new(&env, "proposal_created"),
                    id,
                    payload,
                    client.address
                )
                    .into_val(&env),
                ().into_val(&env)
            ),
        ]
    )
}

fn assert_auth(
    auths: &[(Address, AuthorizedInvocation)],
    idx: usize,
    auth_call: Address,
    call_addr: Address,
    func: Symbol,
    args: Vec<Val>,
) {
    let auth = auths.get(idx).unwrap();
    assert_eq!(auth.0, auth_call);
    assert_eq!(
        auth.1.function,
        AuthorizedFunction::Contract((call_addr, func, args))
    );
}

fn setup_test<'a>() -> (Env, ProposalVotingContractClient<'a>, Address) {
    let env = Env::default();
    let contract_id = env.register_contract(None, ProposalVotingContract);
    let client = ProposalVotingContractClient::new(&env, &contract_id);
    let admin = Address::random(&env);
    client.init(&admin, &3600, &50_00, &1000, &false);

    (env, client, admin)
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn cannot_initialize_voting_twice() {
    let (env, client, admin) = setup_test();
    env.mock_all_auths();
    client.init(&admin, &3600, &50_00, &1000, &false);
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn cannot_create_same_id_proposals() {
    let (env, client, _) = setup_test();
    env.mock_all_auths();
    let comment = BytesN::random(&env);

    let id = 1001u64;
    client.create_custom_proposal(
        &id,
        &ProposalPayload::Comment(comment.clone()),
        &client.address,
        &3600,
        &50_00,
        &2,
    );
    client.create_custom_proposal(
        &id,
        &ProposalPayload::Comment(comment),
        &client.address,
        &3600,
        &50_00,
        &2,
    );
}

#[test]
#[should_panic(expected = "Error(Auth, InvalidAction)")]
fn only_admin_can_create_proposals() {
    let (env, client, _) = setup_test();

    let comment = BytesN::random(&env);
    client.create_custom_proposal(
        &1,
        &ProposalPayload::Comment(comment),
        &client.address,
        &3600,
        &50_00,
        &2,
    );
}

#[test]
fn voter_can_vote_proposals() {
    let (env, client, _) = setup_test();
    env.mock_all_auths();

    let id = 12;
    let comment = BytesN::random(&env);

    client.create_custom_proposal(
        &id,
        &ProposalPayload::Comment(comment),
        &client.address,
        &3600,
        &50_00,
        &2,
    );
    client.vote(&client.address, &id);

    let last_event = env.events().all().last().unwrap();
    assert_eq!(
        vec![&env, last_event],
        vec![
            &env,
            (
                client.address.clone(),
                (Symbol::new(&env, "proposal_voted"), id, client.address).into_val(&env),
                5000u32.into_val(&env)
            ),
        ]
    )
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn voter_cannot_vote_a_proposal_twice() {
    let (env, client, _) = setup_test();
    env.mock_all_auths();
    let prd_id = 12;

    let comment = BytesN::random(&env);

    client.create_custom_proposal(
        &prd_id,
        &ProposalPayload::Comment(comment),
        &client.address,
        &3600,
        &50_00,
        &2,
    );
    client.vote(&client.address, &prd_id);
    client.vote(&client.address, &prd_id); // Double voting here. Expected panic.
}

#[test]
fn cannot_vote_if_voting_time_exceeded() {
    let (mut env, _, _) = setup_test();

    let comment = BytesN::random(&env);
    let proposer = Address::random(&env);

    let mut proposal = Proposal {
        id: 1,
        payload: ProposalPayload::Comment(comment),
        proposer,
        voting_end_time: env.ledger().timestamp() + 3600,
        participation: 0,
        voters: Map::<Address, bool>::new(&env),
        target_approval_rate_bps: 50_00,
        total_participation: 2,
    };

    advance_ledger_time_in(3600, &mut env);

    let result = proposal.vote(env.ledger().timestamp(), Address::random(&env), 1);

    assert_eq!(Err(Error::VotingClosed), result)
}

#[test]
fn cannot_vote_if_total_voters_reached() {
    let (env, _, _) = setup_test();

    let mut voters = Map::<Address, bool>::new(&env);

    voters.set(Address::random(&env), true); // Dummy voters
    voters.set(Address::random(&env), true); // Dummy voters

    let comment = BytesN::random(&env);
    let proposer = Address::random(&env);

    let mut proposal = Proposal {
        id: 1,
        payload: ProposalPayload::Comment(comment),
        proposer,
        voting_end_time: env.ledger().timestamp() + 3600,
        participation: 2,
        voters,
        target_approval_rate_bps: 50_00,
        total_participation: 2,
    };

    let result = proposal.vote(env.ledger().timestamp(), Address::random(&env), 1);
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
    #[case] total_participation: u128,
    #[case] participation: u128,
    #[case] expected: u32,
    #[case] is_approved: bool,
) {
    let (env, _, _) = setup_test();

    let voters = Map::<Address, bool>::new(&env);

    let comment = BytesN::random(&env);
    let proposer = Address::random(&env);

    let proposal = Proposal {
        id: 1,
        payload: ProposalPayload::Comment(comment),
        proposer,
        voting_end_time: env.ledger().timestamp() + 3600,
        participation,
        target_approval_rate_bps: 50_00,
        voters,
        total_participation,
    };

    assert_eq!(Ok(expected), proposal.approval_rate_bps());
    assert!(is_approved == proposal.is_approved());
}

#[test]
#[should_panic(expected = "Error(Contract, #9)")]
fn cannot_create_id0_proposals() {
    let (env, client, _) = setup_test();
    env.mock_all_auths();
    let comment = BytesN::random(&env);

    client.create_custom_proposal(
        &0,
        &ProposalPayload::Comment(comment),
        &client.address,
        &3600,
        &50_00,
        &2,
    );
}

#[test]
fn proposal_comment_is_accessible() {
    let (env, _, _) = setup_test();
    env.mock_all_auths();
    let payload = ProposalPayload::Comment(BytesN::random(&env));
    let proposer = Address::random(&env);

    let proposal = Proposal {
        id: 112,
        payload: payload.clone(),
        proposer,
        voting_end_time: 123123,
        participation: 0,
        target_approval_rate_bps: 0,
        total_participation: 0,
        voters: Map::<Address, bool>::new(&env),
    };

    assert_eq!(payload, proposal.payload().clone());
}

#[test]
fn proposal_total_participation_can_be_set_from_balance() {
    let (env, _, _) = setup_test();
    env.mock_all_auths();

    let mut voters = Map::<Address, bool>::new(&env);

    let voter_1 = Address::random(&env);
    let voter_2 = Address::random(&env);

    voters.set(voter_1.clone(), true); // Only voter_1 votes in favour.

    let mut proposal = Proposal {
        id: 112,
        payload: ProposalPayload::Comment(BytesN::random(&env)),
        proposer: Address::random(&env),
        voting_end_time: 123123,
        target_approval_rate_bps: 5000, // Half the participation is enough to approve.
        // Participation data is in zero values, as it will be calculated from provided balance.
        participation: 0,
        total_participation: 0,
        voters,
    };

    let mut balance = Map::<Address, i128>::new(&env);

    balance.set(voter_1, 1000);
    balance.set(voter_2, 1000);

    proposal.set_participation_from_balance(&balance).unwrap();

    assert_eq!(5000, proposal.approval_rate_bps().unwrap());
    assert!(proposal.is_approved());
}

#[test]
fn set_participation_from_balance_checks_all_local_addresses_exist_in_balance() {
    let (env, _, _) = setup_test();
    env.mock_all_auths();

    let mut voters = Map::<Address, bool>::new(&env);

    let voter_1 = Address::random(&env);
    let voter_2 = Address::random(&env);

    voters.set(voter_1.clone(), true); // Only voter_1 votes in favour.
    voters.set(voter_2.clone(), true); // Only voter_1 votes in favour.

    let mut proposal = Proposal {
        id: 112,
        payload: ProposalPayload::Comment(BytesN::random(&env)),
        proposer: Address::random(&env),
        voting_end_time: 123123,
        target_approval_rate_bps: 5000, // Half the participation is enough to approve.

        // Participation data is in zero values, as it will be calculated from provided balance.
        participation: 0,
        total_participation: 0,
        voters,
    };

    let mut balance = Map::<Address, i128>::new(&env);

    balance.set(voter_1, 1000);
    //balance.set(voter_2, 1000); This should exist, so we expect an error.

    assert_eq!(
        Err(Error::NotFound),
        proposal.set_participation_from_balance(&balance)
    );
}

#[test]
fn proposals_can_be_queried_by_anyone() {
    let (env, client, _) = setup_test();
    env.mock_all_auths();

    client.create_proposal(
        &client.address,
        &1,
        &ProposalPayload::Comment(BytesN::random(&env)),
    );

    let proposal = client.find_proposal(&1);

    assert_eq!(1, proposal.id);
}

#[test]
fn proposals_can_be_updated_only_by_admin() {
    let (env, client, admin) = setup_test();
    env.mock_all_auths();

    client.create_proposal(
        &client.address,
        &1,
        &ProposalPayload::Comment(BytesN::random(&env)),
    );

    let voter_1 = Address::random(&env);
    let voter_2 = Address::random(&env);

    client.vote(&voter_1, &1);
    client.vote(&voter_2, &1);

    let mut proposal = client.find_proposal(&1);

    let mut balance = Map::<Address, i128>::new(&env);
    balance.set(voter_1, 1000);
    balance.set(voter_2, 1000);

    proposal.set_participation_from_balance(&balance).unwrap();

    client.update_proposal(&proposal);

    assert_auth(
        &env.auths(),
        0,
        admin,
        client.address.clone(),
        Symbol::new(&env, "update_proposal"),
        (proposal,).into_val(&env),
    );

    // If we retrieve the proposal again, is updated.
    let stored_proposal = client.find_proposal(&1);
    assert!(stored_proposal.is_approved());
    assert_eq!(10_000, stored_proposal.approval_rate_bps().unwrap());
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn proposals_can_be_updated_only_if_they_exist_first() {
    let (env, client, _) = setup_test();
    env.mock_all_auths();
    let proposal = Proposal {
        id: 112,
        payload: ProposalPayload::Comment(BytesN::random(&env)),
        proposer: Address::random(&env),
        voting_end_time: 123123,
        target_approval_rate_bps: 5000,
        participation: 0,
        total_participation: 0,
        voters: Map::<Address, bool>::new(&env),
    };
    client.update_proposal(&proposal);
}

#[test]
fn non_admin_user_cannot_vote_if_admin_mode_is_on() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, ProposalVotingContract);
    let client = ProposalVotingContractClient::new(&env, &contract_id);
    let admin = Address::random(&env);
    client.init(&admin, &3600, &50_00, &1000, &true);

    let proposer = Address::random(&env);
    client.create_proposal(
        &proposer,
        &1,
        &ProposalPayload::Comment(BytesN::random(&env)),
    );
    client.vote(&client.address, &1);

    assert_auth(
        &env.auths(),
        1, // The second auth should admin
        admin,
        client.address.clone(),
        Symbol::new(&env, "vote"),
        (client.address, 1u64).into_val(&env),
    );
}
