#![no_std]
#![doc = include_str!("../README.md")]

use soroban_sdk::contractclient;

/// Interface for the Contract A
#[contractclient(name = "ContractAClient")]
pub trait IContractA {
    /// binary operation
    fn bin_op(x: u32, y: u32) -> u32;
}
