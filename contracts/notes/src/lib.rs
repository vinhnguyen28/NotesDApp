#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn set(env: Env, value: u32) {
        let key = Symbol::new(&env, "count");
        env.storage().persistent().set(&key, &value);
    }

    pub fn get(env: Env) -> u32 {
        let key = Symbol::new(&env, "count");
        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or(0)
    }

    pub fn increment(env: Env) -> u32 {
        let key = Symbol::new(&env, "count");
        let value: u32 = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(0);

        let new_value = value + 1;

        env.storage()
            .persistent()
            .set(&key, &new_value);

        new_value
    }
}