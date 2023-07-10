#![cfg(test)]

use super::{LotteryContract, LotteryContractClient};
use soroban_sdk::{Env, IntoVal, Symbol};

#[test]
fn admin_is_identified_on_init() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, LotteryContract);
    let client = LotteryContractClient::new(&env, &contract_id);
    client.initialize(&client.address, &2, &100);

    assert_eq!(
        env.auths(),
        [(
            client.address.clone(),
            client.address.clone(),
            Symbol::new(&env, "initialize"),
            (client.address.clone(), 2u32, 100u32).into_val(&env)
        )]
    )
}
