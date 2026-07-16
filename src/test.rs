#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

use crate::{
    Error,
    SavingsGoalContract,
    SavingsGoalContractClient,
};

#[test]
fn test_goal_flow() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SavingsGoalContract, ());
    let client = SavingsGoalContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let goal_name = Symbol::new(&env, "vacation");

    client.create_goal(&owner, &goal_name, &1000);

    client.deposit(&owner, &goal_name, &400);
    client.deposit(&owner, &goal_name, &600);

    assert_eq!(client.get_balance(&owner, &goal_name), 1000);
    assert_eq!(client.get_target(&owner, &goal_name), 1000);
    assert_eq!(client.get_remaining_to_target(&owner, &goal_name), 0);

    let withdrawn = client.withdraw(&owner, &goal_name);
    assert_eq!(withdrawn, 1000);

    // Goal is removed after withdrawal, so this should now fail.
    let result = client.try_get_balance(&owner, &goal_name);
    assert_eq!(result, Err(Ok(Error::GoalNotFound)));
}

#[test]
fn test_rejects_non_positive_values() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SavingsGoalContract, ());
    let client = SavingsGoalContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let goal_name = Symbol::new(&env, "vacation");

    let create_result = client.try_create_goal(&owner, &goal_name, &0);
    assert_eq!(create_result, Err(Ok(Error::InvalidTarget)));

    client.create_goal(&owner, &goal_name, &1000);

    let deposit_result = client.try_deposit(&owner, &goal_name, &0);
    assert_eq!(deposit_result, Err(Ok(Error::InvalidAmount)));
}

#[test]
fn test_multiple_goals_per_owner_are_independent() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SavingsGoalContract, ());
    let client = SavingsGoalContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let vacation = Symbol::new(&env, "vacation");
    let emergency = Symbol::new(&env, "emergency");

    client.create_goal(&owner, &vacation, &1000);
    client.create_goal(&owner, &emergency, &500);

    client.deposit(&owner, &vacation, &200);
    client.deposit(&owner, &emergency, &500);

    assert_eq!(client.get_balance(&owner, &vacation), 200);
    assert_eq!(client.get_balance(&owner, &emergency), 500);
    assert_eq!(client.get_remaining_to_target(&owner, &vacation), 800);

    let withdrawn = client.withdraw(&owner, &emergency);
    assert_eq!(withdrawn, 500);
}

#[test]
fn test_events_are_emitted_for_goal_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SavingsGoalContract, ());
    let client = SavingsGoalContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let goal_name = Symbol::new(&env, "vacation");

    client.create_goal(&owner, &goal_name, &1000);
    client.deposit(&owner, &goal_name, &1000);
    client.withdraw(&owner, &goal_name);

    // One event per lifecycle action: created, deposited, withdrawn.
    assert_eq!(env.events().all().len(), 3);
}