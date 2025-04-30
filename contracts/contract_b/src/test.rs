#![cfg(test)]

use crate::{ContractB, ContractBClient};
use contract_a::ContractA;
use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();

    // Register contract A using the imported WASM.
    let contract_a_id = env.register(ContractA, ());

    // Register contract B defined in this crate.
    let contract_b_id = env.register(ContractB, ());

    // Create a client for calling contract B.
    let client = ContractBClient::new(&env, &contract_b_id);

    // Invoke contract B via its client. Contract B will invoke contract A.
    let sum = client.add_with(&contract_a_id, &5, &7);
    assert_eq!(sum, 12);
}
