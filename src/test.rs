#![cfg(test)]

use soroban_sdk::{testutils::Address as _, testutils::Ledger as _, Address, Env, Symbol};

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

    client.create_goal(&owner, &goal_name, &1000, &None);

    client.deposit(&owner, &goal_name, &400);
    client.deposit(&owner, &goal_name, &600);

    assert_eq!(client.get_balance(&owner, &goal_name), 1000);
    assert_eq!(client.get_target(&owner, &goal_name), 1000);
    assert_eq!(client.get_remaining_to_target(&owner, &goal_name), 0);

    let withdrawn = client.withdraw(&owner, &goal_name);
    assert_eq!(withdrawn, 1000);

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

    let create_result = client.try_create_goal(&owner, &goal_name, &0, &None);
    assert_eq!(create_result, Err(Ok(Error::InvalidTarget)));

    client.create_goal(&owner, &goal_name, &1000, &None);

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

    client.create_goal(&owner, &vacation, &1000, &None);
    client.create_goal(&owner, &emergency, &500, &None);

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

    client.create_goal(&owner, &goal_name, &1000, &None);
    client.deposit(&owner, &goal_name, &1000);
    client.withdraw(&owner, &goal_name);

    assert_eq!(env.events().all().len(), 3);
}

#[test]
fn test_rejects_deadline_in_the_past() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000);

    let contract_id = env.register(SavingsGoalContract, ());
    let client = SavingsGoalContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let goal_name = Symbol::new(&env, "vacation");

    let result = client.try_create_goal(&owner, &goal_name, &1000, &Some(500));
    assert_eq!(result, Err(Ok(Error::InvalidDeadline)));
}

#[test]
fn test_create_goal_with_future_deadline_succeeds() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000);

    let contract_id = env.register(SavingsGoalContract, ());
    let client = SavingsGoalContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let goal_name = Symbol::new(&env, "vacation");

    client.create_goal(&owner, &goal_name, &1000, &Some(2_000));

    assert_eq!(client.get_deadline(&owner, &goal_name), Some(2_000));
}

#[test]
fn test_emergency_withdraw_applies_penalty() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SavingsGoalContract, ());
    let client = SavingsGoalContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let goal_name = Symbol::new(&env, "vacation");

    client.create_goal(&owner, &goal_name, &1000, &None);
    client.deposit(&owner, &goal_name, &300);

    // 10% penalty on a balance of 300 = 30, so payout should be 270.
    let payout = client.emergency_withdraw(&owner, &goal_name);
    assert_eq!(payout, 270);

    let result = client.try_get_balance(&owner, &goal_name);
    assert_eq!(result, Err(Ok(Error::GoalNotFound)));
}

#[test]
fn test_emergency_withdraw_rejects_zero_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SavingsGoalContract, ());
    let client = SavingsGoalContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let goal_name = Symbol::new(&env, "vacation");

    client.create_goal(&owner, &goal_name, &1000, &None);

    let result = client.try_emergency_withdraw(&owner, &goal_name);
    assert_eq!(result, Err(Ok(Error::NothingToWithdraw)));
}