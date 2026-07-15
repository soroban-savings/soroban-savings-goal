#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    Env,
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

#[contracttype]
pub enum DataKey {
    Target,
    Balance,
}

#[contract]
pub struct SavingsGoalContract;

#[contractimpl]
impl SavingsGoalContract {

    pub fn create_goal(env: Env, target: i128) -> Result<(), Error> {
        if target <= 0 {
            return Err(Error::InvalidTarget);
        }

        env.storage().instance().set(&DataKey::Target, &target);
        env.storage().instance().set(&DataKey::Balance, &0i128);

        Ok(())
    }

    pub fn deposit(env: Env, amount: i128) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let balance: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Balance)
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&DataKey::Balance, &(balance + amount));

        Ok(())
    }

    pub fn get_balance(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Balance)
            .unwrap_or(0)
    }

    pub fn get_target(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Target)
            .unwrap_or(0)
    }

    pub fn get_remaining_to_target(env: Env) -> i128 {
        let target: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Target)
            .unwrap_or(0);

        let balance: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Balance)
            .unwrap_or(0);

        target.saturating_sub(balance)
    }

    pub fn withdraw(env: Env) -> Result<i128, Error> {
        let target: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Target)
            .unwrap_or(0);

        let balance: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Balance)
            .unwrap_or(0);

        if balance < target {
            return Err(Error::TargetNotReached);
        }

        env.storage()
            .instance()
            .set(&DataKey::Balance, &0i128);

        Ok(balance)
    }
}

mod test;