#![no_std]

use interfaces::ContractAClient;
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct ContractB;

#[contractimpl]
impl ContractB {
    pub fn add_with(env: Env, contract: Address, x: u32, y: u32) -> u32 {
        let client = ContractAClient::new(&env, &contract);
        client.bin_op(&x, &y)
    }
}

mod test;
