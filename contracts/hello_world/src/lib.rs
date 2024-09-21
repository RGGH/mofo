#![no_std]
use soroban_sdk::{contract, contractimpl,Map,Symbol, vec,symbol_short, Env, String, Vec};
//use soroban_sdk::symbol_short;

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        let mut scores: Map<Symbol,u32> = Map::new(&env);
        scores.set(symbol_short!("Alice"),10u32.into());

        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

mod test;
