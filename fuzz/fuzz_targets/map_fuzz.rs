#![no_main]

use fuzzcontract::*;
use libfuzzer_sys::fuzz_target;
use soroban_sdk::arbitrary::arbitrary;
use soroban_sdk::arbitrary::fuzz_catch_panic;
use soroban_sdk::arbitrary::SorobanArbitrary;
use soroban_sdk::testutils::Logger;
use soroban_sdk::{Env, FromVal, Map, Val};

mod fuzzcontract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/contract_for_fuzz.wasm"
    );
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub struct TestCases {
    map: <Map<u64, u64> as SorobanArbitrary>::Prototype,
    tests: [TypedModMapPrototype; 10],
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum TypedModMapPrototype {
    MapDel(<Val as SorobanArbitrary>::Prototype),
    MapGet(<Val as SorobanArbitrary>::Prototype),
    MapHas(<Val as SorobanArbitrary>::Prototype),
    MapKeys,
    MapLen,
    MapMaxKey,
    MapMinKey,
    MapNew,
    MapNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    MapNextKey(<Val as SorobanArbitrary>::Prototype),
    MapPrevKey(<Val as SorobanArbitrary>::Prototype),
    MapPut(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    MapUnpackToLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    MapValues,
}

impl TypedModMapPrototype {
    fn to_guest(&self, env: &Env, map: &Map<Val, Val>) -> TypedFuzzInstruction {
        match self {
            TypedModMapPrototype::MapDel(v) => {
                let v = Val::from_val(env, v);
                TypedFuzzInstruction::Map(TypedModMap::MapDel(
                    map.clone(),
                    FakeVal(v.get_payload()),
                ))
            }
            TypedModMapPrototype::MapGet(v) => {
                let v = Val::from_val(env, v);
                TypedFuzzInstruction::Map(TypedModMap::MapGet(
                    map.clone(),
                    FakeVal(v.get_payload()),
                ))
            }
            TypedModMapPrototype::MapHas(v) => {
                let v = Val::from_val(env, v);
                TypedFuzzInstruction::Map(TypedModMap::MapHas(
                    map.clone(),
                    FakeVal(v.get_payload()),
                ))
            }
            TypedModMapPrototype::MapKeys => {
                TypedFuzzInstruction::Map(TypedModMap::MapKeys(map.clone()))
            }
            TypedModMapPrototype::MapLen => {
                TypedFuzzInstruction::Map(TypedModMap::MapLen(map.clone()))
            }
            TypedModMapPrototype::MapMaxKey => {
                TypedFuzzInstruction::Map(TypedModMap::MapMaxKey(map.clone()))
            }
            TypedModMapPrototype::MapMinKey => {
                TypedFuzzInstruction::Map(TypedModMap::MapMinKey(map.clone()))
            }
            TypedModMapPrototype::MapNew => TypedFuzzInstruction::Map(TypedModMap::MapNew),
            TypedModMapPrototype::MapNewFromLinearMemory(v_0, v_1, v_2) => {
                TypedFuzzInstruction::Map(TypedModMap::MapNewFromLinearMemory(*v_0, *v_1, *v_2))
            }
            TypedModMapPrototype::MapNextKey(v) => {
                let v = Val::from_val(env, v);
                TypedFuzzInstruction::Map(TypedModMap::MapNextKey(
                    map.clone(),
                    FakeVal(v.get_payload()),
                ))
            }
            TypedModMapPrototype::MapPrevKey(v) => {
                let v = Val::from_val(env, v);
                TypedFuzzInstruction::Map(TypedModMap::MapPrevKey(
                    map.clone(),
                    FakeVal(v.get_payload()),
                ))
            }
            TypedModMapPrototype::MapPut(v_0, v_1) => {
                let v_0 = Val::from_val(env, v_0);
                let v_1 = Val::from_val(env, v_1);
                TypedFuzzInstruction::Map(TypedModMap::MapPut(
                    map.clone(),
                    FakeVal(v_0.get_payload()),
                    FakeVal(v_1.get_payload()),
                ))
            }
            TypedModMapPrototype::MapUnpackToLinearMemory(v_0, v_1, v_2) => {
                TypedFuzzInstruction::Map(TypedModMap::MapUnpackToLinearMemory(
                    map.clone(),
                    *v_0,
                    *v_1,
                    *v_2,
                ))
            }
            TypedModMapPrototype::MapValues => {
                TypedFuzzInstruction::Map(TypedModMap::MapValues(map.clone()))
            }
        }
    }
}

fuzz_target!(|input: TestCases| {
    let env = Env::default();
    env.budget().reset_unlimited();

    let contract_id = env.register_contract_wasm(None, fuzzcontract::WASM);

    let client = fuzzcontract::Client::new(&env, &contract_id);

    let map = Map::<u64, u64>::from_val(&env, &input.map);
    let map = Map::<Val, Val>::from_val(&env, map.as_val());

    for test in input.tests {
        let fuzz_instruction = test.to_guest(&env, &map);
        let fuzz_instruction = FuzzInstruction::Typed(fuzz_instruction);

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
