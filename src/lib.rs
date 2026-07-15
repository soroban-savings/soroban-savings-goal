#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    Address, Env, Symbol,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    InvalidTarget = 1,
    InvalidAmount = 2,
    TargetNotReached = 3,
    GoalAlreadyExists = 4,
    GoalNotFound = 5,
    Unauthorized = 6,
}

#[derive(Clone)]
#[contracttype]
pub struct GoalData {
    pub target: i128,
    pub balance: i128,
    pub created_at: u64,
}

#[contracttype]
pub enum DataKey {
    Goal(Address, Symbol),
}

#[contract]
pub struct SavingsGoalContract;

#[contractimpl]
impl SavingsGoalContract {

    pub fn create_goal(
        env: Env,
        owner: Address,
        goal_name: Symbol,
        target: i128,
    ) -> Result<(), Error> {
        owner.require_auth();

        if target <= 0 {
            return Err(Error::InvalidTarget);
        }

        let key = DataKey::Goal(owner.clone(), goal_name.clone());

        if env.storage().instance().has(&key) {
            return Err(Error::GoalAlreadyExists);
        }

        let goal = GoalData {
            target,
            balance: 0,
            created_at: env.ledger().timestamp(),
        };

        env.storage().instance().set(&key, &goal);

        Ok(())
    }

    pub fn deposit(
        env: Env,
        owner: Address,
        goal_name: Symbol,
        amount: i128,
    ) -> Result<(), Error> {
        owner.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let key = DataKey::Goal(owner.clone(), goal_name.clone());

        let mut goal: GoalData = env
            .storage()
            .instance()
            .get(&key)
            .ok_or(Error::GoalNotFound)?;

        goal.balance += amount;
        env.storage().instance().set(&key, &goal);

        Ok(())
    }

    pub fn get_balance(env: Env, owner: Address, goal_name: Symbol) -> Result<i128, Error> {
        let key = DataKey::Goal(owner, goal_name);
        let goal: GoalData = env.storage().instance().get(&key).ok_or(Error::GoalNotFound)?;
        Ok(goal.balance)
    }

    pub fn get_target(env: Env, owner: Address, goal_name: Symbol) -> Result<i128, Error> {
        let key = DataKey::Goal(owner, goal_name);
        let goal: GoalData = env.storage().instance().get(&key).ok_or(Error::GoalNotFound)?;
        Ok(goal.target)
    }

    pub fn get_remaining_to_target(
        env: Env,
        owner: Address,
        goal_name: Symbol,
    ) -> Result<i128, Error> {
        let key = DataKey::Goal(owner, goal_name);
        let goal: GoalData = env.storage().instance().get(&key).ok_or(Error::GoalNotFound)?;
        Ok(goal.target.saturating_sub(goal.balance))
    }

    pub fn withdraw(env: Env, owner: Address, goal_name: Symbol) -> Result<i128, Error> {
        owner.require_auth();

        let key = DataKey::Goal(owner.clone(), goal_name.clone());

        let goal: GoalData = env
            .storage()
            .instance()
            .get(&key)
            .ok_or(Error::GoalNotFound)?;

        if goal.balance < goal.target {
            return Err(Error::TargetNotReached);
        }

        let withdrawn = goal.balance;

        env.storage().instance().remove(&key);

        Ok(withdrawn)
    }
}

mod test;