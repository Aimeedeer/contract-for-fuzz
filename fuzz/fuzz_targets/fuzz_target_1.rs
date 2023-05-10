#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::{Env, FromVal};
use crate::fuzzcontract::FuzzInstruction;
use soroban_sdk::arbitrary::SorobanArbitrary;

mod fuzzcontract {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/contract_for_fuzz.wasm");
}

fuzz_target!(|input: <FuzzInstruction as SorobanArbitrary>::Prototype| {
    let env = Env::default();
    let budget = env.budget().0;
    
    let contract_id = env.register_contract_wasm(None, fuzzcontract::WASM);
    
    let client = fuzzcontract::Client::new(&env, &contract_id);

    let fuzz_instruction = FuzzInstruction::from_val(&env, &input);

    client.run(&fuzz_instruction)
});
