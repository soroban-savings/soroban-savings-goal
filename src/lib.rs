#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    Address, Env,
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
    Owner,
    Target,
    Balance,
}

#[contract]
pub struct SavingsGoalContract;

#[contractimpl]
impl SavingsGoalContract {

    pub fn create_goal(env: Env, owner: Address, target: i128) -> Result<(), Error> {
        owner.require_auth();

        if target <= 0 {
            return Err(Error::InvalidTarget);
        }

        if env.storage().instance().has(&DataKey::Owner) {
            return Err(Error::GoalAlreadyExists);
        }

        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::Target, &target);
        env.storage().instance().set(&DataKey::Balance, &0i128);

        Ok(())
    }

    pub fn deposit(env: Env, amount: i128) -> Result<(), Error> {
        let owner: Address = env
            .storage()
            .instance()
            .get(&DataKey::Owner)
            .ok_or(Error::GoalNotFound)?;

        owner.require_auth();

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
        let owner: Address = env
            .storage()
            .instance()
            .get(&DataKey::Owner)
            .ok_or(Error::GoalNotFound)?;

        owner.require_auth();

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