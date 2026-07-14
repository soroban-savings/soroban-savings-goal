#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Symbol
};

#[contracttype]
pub enum DataKey {
    Target,
    Balance,
}

#[contract]
pub struct SavingsGoalContract;

#[contractimpl]
impl SavingsGoalContract {

    pub fn create_goal(env: Env, target: i128) {
        if target <= 0 {
            panic!("target must be positive");
        }

        env.storage().instance().set(&DataKey::Target, &target);
        env.storage().instance().set(&DataKey::Balance, &0i128);
    }

    pub fn deposit(env: Env, amount: i128) {
        if amount <= 0 {
            panic!("deposit amount must be positive");
        }

        let balance: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Balance)
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&DataKey::Balance, &(balance + amount));
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

    pub fn withdraw(env: Env) -> i128 {

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
            panic!("Target not reached");
        }

        env.storage()
            .instance()
            .set(&DataKey::Balance, &0i128);

        balance
    }
}
mod test;