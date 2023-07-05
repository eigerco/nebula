
    #![no_std]
use soroban_sdk::{contractimpl, vec, Env, Symbol, Vec};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, receiver: Symbol) -> Vec<Symbol> {
        vec![&env, Symbol::short("Hello"), receiver]
    }
}
    