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

    let client = SavingsGoalContractClient::new(&env, &contract_id);

    client.create_goal(&1000);

    client.deposit(&400);
    client.deposit(&600);

    assert_eq!(client.get_balance(), 1000);
    assert_eq!(client.get_target(), 1000);
    assert_eq!(client.get_remaining_to_target(), 0);

    let withdrawn = client.withdraw();

    assert_eq!(withdrawn, 1000);
    assert_eq!(client.get_balance(), 0);
}

#[test]
fn test_rejects_non_positive_values() {
    let env = Env::default();
    let contract_id = env.register(SavingsGoalContract, ());
    let client = SavingsGoalContractClient::new(&env, &contract_id);

    let create_result = std::panic::catch_unwind(|| client.create_goal(&0));
    assert!(create_result.is_err(), "zero target should be rejected");

    let deposit_result = std::panic::catch_unwind(|| client.deposit(&0));
    assert!(deposit_result.is_err(), "zero deposit should be rejected");
}

#[test]
fn test_increase_target_flow() {
    let env = Env::default();
    let contract_id = env.register(SavingsGoalContract, ());
    let client = SavingsGoalContractClient::new(&env, &contract_id);

    client.create_goal(&1000);
    client.increase_target(&500);

    assert_eq!(client.get_target(), 1500);

    client.deposit(&1500);
    assert_eq!(client.get_balance(), 1500);

    let withdrawn = client.withdraw();
    assert_eq!(withdrawn, 1500);
}