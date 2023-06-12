#![no_main]

use fuzzcontract::*;
use libfuzzer_sys::fuzz_target;
use soroban_sdk::arbitrary::arbitrary;
use soroban_sdk::arbitrary::fuzz_catch_panic;
use soroban_sdk::arbitrary::SorobanArbitrary;
use soroban_sdk::testutils::Logger;
use soroban_sdk::{Env, FromVal, RawVal,Vec};
use std::vec::Vec as RustVec;

mod fuzzcontract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/contract_for_fuzz.wasm"
    );
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub struct TestCases {
    v_0: <Vec<RawVal> as SorobanArbitrary>::Prototype,
    v_1: <Vec<RawVal> as SorobanArbitrary>::Prototype,
    tests: [TypedModVecPrototype; 10],
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum TypedModVecPrototype {
    VecAppend,
    VecBack,
    VecBinarySearch(<RawVal as SorobanArbitrary>::Prototype),
    VecDel(<u32 as SorobanArbitrary>::Prototype),
    VecFirstIndexOf(<RawVal as SorobanArbitrary>::Prototype),
    VecFront,
    VecGet(<u32 as SorobanArbitrary>::Prototype),
    VecInsert(
        <u32 as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),
    VecLastIndexOf(<RawVal as SorobanArbitrary>::Prototype),
    VecLen,
    VecNew(<RawVal as SorobanArbitrary>::Prototype),
    VecNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    VecPopBack,
    VecPopFront,
    VecPushBack(<RawVal as SorobanArbitrary>::Prototype),
    VecPushFront(<RawVal as SorobanArbitrary>::Prototype),
    VecPut(
        <u32 as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),
    VecSlice(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    VecUnpackToLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
}

impl TestCases {
    fn to_guest(&self, env: &Env) -> RustVec<TypedFuzzInstruction> {
        let instructions = self.tests
            .iter()
            .map(|test| match test {
                TypedModVecPrototype::VecAppend => {
                    let v_0 = Vec::<RawVal>::from_val(env, &self.v_0);
                    let v_1 = Vec::<RawVal>::from_val(env, &self.v_1);
                    TypedFuzzInstruction::Vec(TypedModVec::VecAppend(v_0, v_1))
                }
                TypedModVecPrototype::VecBack => {
                    let v = Vec::<RawVal>::from_val(env, &self.v_0);
                    TypedFuzzInstruction::Vec(TypedModVec::VecBack(v))
                }
                TypedModVecPrototype::VecBinarySearch(v) => {
                    let v_0 = Vec::<RawVal>::from_val(env, &self.v_0);
                    let v_1 = RawVal::from_val(env, v);
                    TypedFuzzInstruction::Vec(TypedModVec::VecBinarySearch(
                        v_0,
                        FakeRawVal(v_1.get_payload()),
                    ))
                }
                TypedModVecPrototype::VecDel(v) => {
                    let v_0 = Vec::<RawVal>::from_val(env, &self.v_0);
                    TypedFuzzInstruction::Vec(TypedModVec::VecDel(v_0, *v))
                }
               TypedModVecPrototype::VecFirstIndexOf(v) => {
                    let v_0 = Vec::<RawVal>::from_val(env, &self.v_0);
                    let v_1 = RawVal::from_val(env, v);
                    TypedFuzzInstruction::Vec(TypedModVec::VecFirstIndexOf(
                        v_0,
                        FakeRawVal(v_1.get_payload()),
                    ))
                }
                TypedModVecPrototype::VecFront => {
                    let v = Vec::<RawVal>::from_val(env, &self.v_0);
                    TypedFuzzInstruction::Vec(TypedModVec::VecFront(v))
                }
                TypedModVecPrototype::VecGet(v) => {
                    let v_0 = Vec::<RawVal>::from_val(env, &self.v_0);
                    TypedFuzzInstruction::Vec(TypedModVec::VecGet(v_0, *v))
                }
                TypedModVecPrototype::VecInsert(v_1, v_2) => {
                    let v_0 = Vec::<RawVal>::from_val(env, &self.v_0);
                    let v_2 = RawVal::from_val(env, v_2);
                    TypedFuzzInstruction::Vec(TypedModVec::VecInsert(
                        v_0,
                        *v_1,
                        FakeRawVal(v_2.get_payload()),
                    ))
                }
                TypedModVecPrototype::VecLastIndexOf(v) => {
                    let v_0 = Vec::<RawVal>::from_val(env, &self.v_0);
                    let v_1 = RawVal::from_val(env, v);
                    TypedFuzzInstruction::Vec(TypedModVec::VecLastIndexOf(
                        v_0,
                        FakeRawVal(v_1.get_payload()),
                    ))
                }
                TypedModVecPrototype::VecLen => {
                    let v = Vec::<RawVal>::from_val(env, &self.v_0);
                    TypedFuzzInstruction::Vec(TypedModVec::VecLen(v))
                }
                TypedModVecPrototype::VecNew(v) => {
                    let v = RawVal::from_val(env, v);
                    TypedFuzzInstruction::Vec(TypedModVec::VecNew(FakeRawVal(v.get_payload())))
                }
                TypedModVecPrototype::VecNewFromLinearMemory(v_0, v_1) => {
                    TypedFuzzInstruction::Vec(TypedModVec::VecNewFromLinearMemory(*v_0, *v_1))
                }
                TypedModVecPrototype::VecPopBack => {
                    let v = Vec::<RawVal>::from_val(env, &self.v_0);
                    TypedFuzzInstruction::Vec(TypedModVec::VecPopBack(v))
                }
                TypedModVecPrototype::VecPopFront => {
                    let v = Vec::<RawVal>::from_val(env, &self.v_0);
                    TypedFuzzInstruction::Vec(TypedModVec::VecPopFront(v))
                }
                TypedModVecPrototype::VecPushBack(v) => {
                    let v_0 = Vec::<RawVal>::from_val(env, &self.v_0);
                    let v_1 = RawVal::from_val(env, v);
                    TypedFuzzInstruction::Vec(TypedModVec::VecPushBack(
                        v_0,
                        FakeRawVal(v_1.get_payload()),
                    ))
                }
                TypedModVecPrototype::VecPushFront(v) => {
                    let v_0 = Vec::<RawVal>::from_val(env, &self.v_0);
                    let v_1 = RawVal::from_val(env, v);
                    TypedFuzzInstruction::Vec(TypedModVec::VecPushFront(
                        v_0,
                        FakeRawVal(v_1.get_payload()),
                    ))
                }
                TypedModVecPrototype::VecPut(v_1, v_2) => {
                    let v_0 = Vec::<RawVal>::from_val(env, &self.v_0);
                    let v_2 = RawVal::from_val(env, v_2);
                    TypedFuzzInstruction::Vec(TypedModVec::VecPut(
                        v_0,
                        *v_1,
                        FakeRawVal(v_2.get_payload()),
                    ))
                }
                TypedModVecPrototype::VecSlice(v_1, v_2) => {
                    let v_0 = Vec::<RawVal>::from_val(env, &self.v_0);
                    TypedFuzzInstruction::Vec(TypedModVec::VecSlice(v_0, *v_1, *v_2))
                }
                TypedModVecPrototype::VecUnpackToLinearMemory(v_1, v_2) => {
                    let v_0 = Vec::<RawVal>::from_val(env, &self.v_0);
                    TypedFuzzInstruction::Vec(TypedModVec::VecUnpackToLinearMemory(v_0, *v_1, *v_2))
                }
            })
            .collect();

        instructions
    }
}

        
fuzz_target!(|input: TestCases| {
    let env = Env::default();

    let contract_id = env.register_contract_wasm(None, fuzzcontract::WASM);

    let client = fuzzcontract::Client::new(&env, &contract_id);

    let fuzz_instructions = input.to_guest(&env);
    for instr in fuzz_instructions {
        let fuzz_instruction = FuzzInstruction::Typed(instr);

        // Returning an error is ok; panicking is not.
        let panic_r = fuzz_catch_panic(|| {
            let _call_r = client.try_run(&fuzz_instruction);
        });

        if panic_r.is_err() {
            if !env.logger().all().is_empty() {
                env.logger().print();
            }
            panic!("host panicked: {panic_r:?}");
        }
    }
});
