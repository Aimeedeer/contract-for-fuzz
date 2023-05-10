#![no_std]

use soroban_sdk::{contractimpl, contracttype, log, Env, RawVal};

#[contracttype]
#[derive(Clone, Debug)]
pub enum FuzzInstruction {
    LogValue(RawVal),
}

pub struct FuzzContract;

#[contractimpl]
impl FuzzContract {
    pub fn run(env: Env, fuzz_instruction: FuzzInstruction) {
        match fuzz_instruction {
            FuzzInstruction::LogValue(rawval) => log!(&env, "log value: {}", rawval)
        }

    }
}
