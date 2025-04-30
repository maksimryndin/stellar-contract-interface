#![no_std]

use interfaces::IContractA;
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct ContractA;

#[contractimpl]
impl ContractA {
    pub fn add(x: u32, y: u32) -> u32 {
        x.checked_add(y).expect("no overflow")
    }
}

#[contractimpl]
impl IContractA for ContractA {
    fn bin_op(x: u32, y: u32) -> u32 {
        Self::add(x, y)
    }
}
