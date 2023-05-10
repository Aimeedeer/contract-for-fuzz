#![no_std]

use soroban_sdk::{contractimpl, contracttype};

#[contracttype]
pub enum FuzzInstruction {
    Test1,
}

pub struct FuzzContract;

#[contractimpl]
impl FuzzContract {
    pub fn run(fuzz_instruction: FuzzInstruction) -> () {
        ()
    }
}
