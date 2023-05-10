#![cfg(test)]

use super::{MyContract, MyContractClient};
use soroban_sdk::{vec, Env, Symbol};

#[test]
fn test_contract() {
    let env = Env::default();
    let contract_id = env.register_contract(None, MyContract);
    let client = MyContractClient::new(&env, &contract_id);

    let words = client.hello(&Symbol::short("Dev"));
    assert_eq!(
        words,
        vec![&env, Symbol::short("Hello"), Symbol::short("Dev"),]
    );
}

