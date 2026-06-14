#![cfg(test)]

use soroban_sdk::Env;

use crate::{
    SavingsGoalContract,
    SavingsGoalContractClient,
};

#[test]
fn test_goal_flow() {

    let env = Env::default();

    let contract_id = env.register(SavingsGoalContract, ());

    let client =
        SavingsGoalContractClient::new(
            &env,
            &contract_id,
        );

    client.create_goal(&1000);

    client.deposit(&400);
    client.deposit(&600);

    assert_eq!(
        client.get_balance(),
        1000
    );

    assert_eq!(
        client.get_target(),
        1000
    );

    let withdrawn =
        client.withdraw();

    assert_eq!(
        withdrawn,
        1000
    );

    assert_eq!(
        client.get_balance(),
        0
    );
}