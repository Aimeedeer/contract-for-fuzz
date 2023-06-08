#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::{
    Env, FromVal, RawVal,
};
use fuzzcontract::*;
use soroban_sdk::arbitrary::SorobanArbitrary;
use soroban_sdk::arbitrary::arbitrary;
use soroban_sdk::arbitrary::fuzz_catch_panic;
use soroban_sdk::testutils::Logger;

mod fuzzcontract {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/contract_for_fuzz.wasm");
}

#[derive(Clone, Debug)]
#[derive(arbitrary::Arbitrary)]
pub enum RawFuzzInstructionPrototype {
    // mod address
    AccountPublicKeyToAddress(<RawVal as SorobanArbitrary>::Prototype),
    AddressToAccountPublicKey(<RawVal as SorobanArbitrary>::Prototype),
    AddressToContractId(<RawVal as SorobanArbitrary>::Prototype),
    ContractIdToAddress(<RawVal as SorobanArbitrary>::Prototype),
    RequireAuth(<RawVal as SorobanArbitrary>::Prototype),
    RequireAuthForArgs(<RawVal as SorobanArbitrary>::Prototype, <RawVal as SorobanArbitrary>::Prototype),

    // mod buf
    BytesAppend(<RawVal as SorobanArbitrary>::Prototype, <RawVal as SorobanArbitrary>::Prototype),
    BytesBack(<RawVal as SorobanArbitrary>::Prototype),
    BytesCopyFromLinearMemory(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesCopyToLinearMemory(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesDel(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesFront(<RawVal as SorobanArbitrary>::Prototype),
    BytesGet(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesInsert(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesLen(<RawVal as SorobanArbitrary>::Prototype),
    BytesNew,
    BytesNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesPop(<RawVal as SorobanArbitrary>::Prototype),
    BytesPush(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesPut(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesSlice(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    DeserializeFromBytes(<RawVal as SorobanArbitrary>::Prototype),
    SerializeToBytes(<RawVal as SorobanArbitrary>::Prototype),
    StringCopyToLinearMemory(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    StringLen(
        <RawVal as SorobanArbitrary>::Prototype,
    ),
    StringNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    SymbolCopyToLinearMemory(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    SymbolIndexInLinearMemory(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    SymbolLen(<RawVal as SorobanArbitrary>::Prototype),
    SymbolNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),

    // mod call
    Call(
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),
    TryCall(
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),

    // mod context
    ContractEvent(
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),
    FailWithError(<RawVal as SorobanArbitrary>::Prototype),
    GetCurrentCallStack,
    GetCurrentContractAddress,
    GetCurrentContractId,
    GetInvokingContract,
    GetLedgerNetworkId,
    GetLedgerSequence,
    GetLedgerTimestamp,
    GetLedgerVersion,
    LogFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    ObjCmp(
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),

    // mod crypto
    ComputeHashSha256(<RawVal as SorobanArbitrary>::Prototype),
    VerifySigEd25519(
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),

    // mod int
    ObjFromI64(<i64 as SorobanArbitrary>::Prototype),
    ObjFromI128Pieces(
        <i64 as SorobanArbitrary>::Prototype,
        <u64 as SorobanArbitrary>::Prototype,
    ),
    ObjFromI256Pieces(
        <i64 as SorobanArbitrary>::Prototype,
        <u64 as SorobanArbitrary>::Prototype,
        <u64 as SorobanArbitrary>::Prototype,
        <u64 as SorobanArbitrary>::Prototype,
    ),
    ObjFromU64(<u64 as SorobanArbitrary>::Prototype),
    ObjFromU128Pieces(
        <u64 as SorobanArbitrary>::Prototype,
        <u64 as SorobanArbitrary>::Prototype,
    ),
    ObjFromU256Pieces(
        <u64 as SorobanArbitrary>::Prototype,
        <u64 as SorobanArbitrary>::Prototype,
        <u64 as SorobanArbitrary>::Prototype,
        <u64 as SorobanArbitrary>::Prototype,
    ),
    ObjToI64(<RawVal as SorobanArbitrary>::Prototype),
    ObjToI128Hi64(<RawVal as SorobanArbitrary>::Prototype),
    ObjToI128Lo64(<RawVal as SorobanArbitrary>::Prototype),
    ObjToI256HiHi(<RawVal as SorobanArbitrary>::Prototype),
    ObjToI256HiLo(<RawVal as SorobanArbitrary>::Prototype),
    ObjToI256LoHi(<RawVal as SorobanArbitrary>::Prototype),
    ObjToI256LoLo(<RawVal as SorobanArbitrary>::Prototype),
    ObjToU64(<RawVal as SorobanArbitrary>::Prototype),
    ObjToU128Hi64(<RawVal as SorobanArbitrary>::Prototype),
    ObjToU128Lo64(<RawVal as SorobanArbitrary>::Prototype),
    ObjToU256HiHi(<RawVal as SorobanArbitrary>::Prototype),
    ObjToU256HiLo(<RawVal as SorobanArbitrary>::Prototype),
    ObjToU256LoHi(<RawVal as SorobanArbitrary>::Prototype),
    ObjToU256LoLo(<RawVal as SorobanArbitrary>::Prototype),

    // mod ledgeer
    CreateAssetContract(<RawVal as SorobanArbitrary>::Prototype),
    CreateContract(
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),
    DelContractData(<RawVal as SorobanArbitrary>::Prototype),
    GetContractData(<RawVal as SorobanArbitrary>::Prototype),
    HasContractData(<RawVal as SorobanArbitrary>::Prototype),
    PutContractData(
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),
    UpdateCurrentContractWasm(<RawVal as SorobanArbitrary>::Prototype),
    UploadWasm(<RawVal as SorobanArbitrary>::Prototype),

    // mod map
    MapDel(<RawVal as SorobanArbitrary>::Prototype, <RawVal as SorobanArbitrary>::Prototype),
    MapGet(<RawVal as SorobanArbitrary>::Prototype, <RawVal as SorobanArbitrary>::Prototype),
    MapHas(<RawVal as SorobanArbitrary>::Prototype, <RawVal as SorobanArbitrary>::Prototype),
    MapKeys(<RawVal as SorobanArbitrary>::Prototype),
    MapLen(<RawVal as SorobanArbitrary>::Prototype),
    MapMaxKey(<RawVal as SorobanArbitrary>::Prototype),
    MapMinKey(<RawVal as SorobanArbitrary>::Prototype),
    MapNew,
    MapNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    MapNextKey(<RawVal as SorobanArbitrary>::Prototype, <RawVal as SorobanArbitrary>::Prototype),
    MapPrevKey(<RawVal as SorobanArbitrary>::Prototype, <RawVal as SorobanArbitrary>::Prototype),
    MapPut(
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),
    MapUnpackToLinearMemory(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    MapValues(<RawVal as SorobanArbitrary>::Prototype),
    
    // mod prng
    PrngBytesNew(u32),
    PrngReseed(<RawVal as SorobanArbitrary>::Prototype),
    PrngU64InInclusiveRange(u64, u64),
    PrngVecShuffle(<RawVal as SorobanArbitrary>::Prototype),
    
    // mod test
    Test,

    // mod vec
    VecAppend(
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),
    VecBack(<RawVal as SorobanArbitrary>::Prototype),
    VecBinarySearch(
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),
    VecDel(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    VecFirstIndexOf(
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),
    VecFront(<RawVal as SorobanArbitrary>::Prototype),
    VecGet(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    VecInsert(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),
    VecLastIndexOf(
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),
    VecLen(<RawVal as SorobanArbitrary>::Prototype),
    VecNew(<RawVal as SorobanArbitrary>::Prototype),
    VecNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    VecPopBack(<RawVal as SorobanArbitrary>::Prototype),
    VecPopFront(<RawVal as SorobanArbitrary>::Prototype),
    VecPushBack(
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),
    VecPushFront(
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),
    VecPut(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),
    VecSlice(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    VecUnpackToLinearMemory(
        <RawVal as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
}

impl RawFuzzInstructionPrototype {
    fn to_guest(&self, env: &Env) -> RawFuzzInstruction {
        match self {
            RawFuzzInstructionPrototype::AccountPublicKeyToAddress(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Address(RawModAddress::AccountPublicKeyToAddress(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::AddressToAccountPublicKey(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Address(RawModAddress::AddressToAccountPublicKey(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::AddressToContractId(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Address(RawModAddress::AddressToContractId(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::ContractIdToAddress(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Address(RawModAddress::ContractIdToAddress(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::RequireAuth(v_0) => {
                let v_0 = RawVal::from_val(env, v_0);
                RawFuzzInstruction::Address(RawModAddress::RequireAuth(FakeRawVal(v_0.get_payload())))
            }
            RawFuzzInstructionPrototype::RequireAuthForArgs(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                RawFuzzInstruction::Address(RawModAddress::RequireAuthForArgs(FakeRawVal(v_0.get_payload()), FakeRawVal(v_1.get_payload())))
            }
            RawFuzzInstructionPrototype::BytesAppend(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                RawFuzzInstruction::Buf(RawModBuf::BytesAppend(FakeRawVal(v_0.get_payload()), FakeRawVal(v_1.get_payload())))
            }
            RawFuzzInstructionPrototype::BytesBack(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Buf(RawModBuf::BytesBack(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::BytesCopyFromLinearMemory(v_0, v_1, v_2, v_3) => {
                let v_0 = RawVal::from_val(env, v_0);
                RawFuzzInstruction::Buf(RawModBuf::BytesCopyFromLinearMemory(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                    *v_2,
                    *v_3,
                ))
            }
            RawFuzzInstructionPrototype::BytesCopyToLinearMemory(v_0, v_1, v_2, v_3) => {
                let v_0 = RawVal::from_val(env, v_0);
                RawFuzzInstruction::Buf(RawModBuf::BytesCopyToLinearMemory(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                    *v_2,
                    *v_3,
                ))
            }
            RawFuzzInstructionPrototype::BytesDel(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                RawFuzzInstruction::Buf(RawModBuf::BytesDel(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                ))
            }
            RawFuzzInstructionPrototype::BytesFront(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Buf(RawModBuf::BytesFront(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::BytesGet(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                RawFuzzInstruction::Buf(RawModBuf::BytesGet(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                ))
            }
            RawFuzzInstructionPrototype::BytesInsert(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                RawFuzzInstruction::Buf(RawModBuf::BytesInsert(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                    *v_2,
                ))
            }
            RawFuzzInstructionPrototype::BytesLen(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Buf(RawModBuf::BytesLen(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::BytesNew => {
                RawFuzzInstruction::Buf(RawModBuf::BytesNew)
            }
            RawFuzzInstructionPrototype::BytesNewFromLinearMemory(v_0, v_1) => {
                RawFuzzInstruction::Buf(RawModBuf::BytesNewFromLinearMemory(*v_0, *v_1))
            }
            RawFuzzInstructionPrototype::BytesPop(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Buf(RawModBuf::BytesPop(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::BytesPush(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                RawFuzzInstruction::Buf(RawModBuf::BytesPush(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                ))
            }
            RawFuzzInstructionPrototype::BytesPut(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                RawFuzzInstruction::Buf(RawModBuf::BytesPut(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                    *v_2,
                ))
            }
            RawFuzzInstructionPrototype::BytesSlice(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                RawFuzzInstruction::Buf(RawModBuf::BytesSlice(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                    *v_2,
                ))
            }
            RawFuzzInstructionPrototype::DeserializeFromBytes(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Buf(RawModBuf::DeserializeFromBytes(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::SerializeToBytes(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Buf(RawModBuf::SerializeToBytes(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::StringCopyToLinearMemory(v_0, v_1, v_2, v_3) => {
                let v_0 = RawVal::from_val(env, v_0);
                RawFuzzInstruction::Buf(RawModBuf::StringCopyToLinearMemory(FakeRawVal(v_0.get_payload()), *v_1, *v_2, *v_3))
            }
            RawFuzzInstructionPrototype::StringLen(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Buf(RawModBuf::StringLen(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::StringNewFromLinearMemory(v_0, v_1) => {
                RawFuzzInstruction::Buf(RawModBuf::StringNewFromLinearMemory(*v_0, *v_1))
            }
            RawFuzzInstructionPrototype::SymbolCopyToLinearMemory(v_0, v_1, v_2, v_3) => {
                let v_0 = RawVal::from_val(env, v_0);
                RawFuzzInstruction::Buf(RawModBuf::SymbolCopyToLinearMemory(FakeRawVal(v_0.get_payload()), *v_1, *v_2, *v_3))
            }
            RawFuzzInstructionPrototype::SymbolIndexInLinearMemory(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                RawFuzzInstruction::Buf(RawModBuf::SymbolIndexInLinearMemory(FakeRawVal(v_0.get_payload()), *v_1, *v_2))
            }
            RawFuzzInstructionPrototype::SymbolLen(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Buf(RawModBuf::SymbolLen(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::SymbolNewFromLinearMemory(v_0, v_1) => {
                RawFuzzInstruction::Buf(RawModBuf::SymbolNewFromLinearMemory(*v_0, *v_1))
            }
            RawFuzzInstructionPrototype::Call(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                let v_2 = RawVal::from_val(env, v_2);
                RawFuzzInstruction::Call(RawModCall::Call(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                    FakeRawVal(v_2.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::TryCall(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                let v_2 = RawVal::from_val(env, v_2);
                RawFuzzInstruction::Call(RawModCall::TryCall(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                    FakeRawVal(v_2.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::ContractEvent(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                RawFuzzInstruction::Context(RawModContext::ContractEvent(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::FailWithError(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Context(RawModContext::FailWithError(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::GetCurrentCallStack => {
                RawFuzzInstruction::Context(RawModContext::GetCurrentCallStack)
            }
            RawFuzzInstructionPrototype::GetCurrentContractAddress => {
                RawFuzzInstruction::Context(RawModContext::GetCurrentContractAddress)
            }
            RawFuzzInstructionPrototype::GetCurrentContractId => {
                RawFuzzInstruction::Context(RawModContext::GetCurrentContractId)
            }
            RawFuzzInstructionPrototype::GetInvokingContract => {
                RawFuzzInstruction::Context(RawModContext::GetInvokingContract)
            }
            RawFuzzInstructionPrototype::GetLedgerNetworkId => {
                RawFuzzInstruction::Context(RawModContext::GetLedgerNetworkId)
            }
            RawFuzzInstructionPrototype::GetLedgerSequence => {
                RawFuzzInstruction::Context(RawModContext::GetLedgerSequence)
            }
            RawFuzzInstructionPrototype::GetLedgerTimestamp => {
                RawFuzzInstruction::Context(RawModContext::GetLedgerTimestamp)
            }            
            RawFuzzInstructionPrototype::GetLedgerVersion => {
                RawFuzzInstruction::Context(RawModContext::GetLedgerVersion)
            }
            RawFuzzInstructionPrototype::LogFromLinearMemory(v_0, v_1, v_2, v_3) => {
                RawFuzzInstruction::Context(RawModContext::LogFromLinearMemory(*v_0, *v_1, *v_2, *v_3))
            }
            RawFuzzInstructionPrototype::ObjCmp(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                RawFuzzInstruction::Context(RawModContext::ObjCmp(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }            
            RawFuzzInstructionPrototype::ComputeHashSha256(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Crypto(RawModCrypto::ComputeHashSha256(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::VerifySigEd25519(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                let v_2 = RawVal::from_val(env, v_2);
                RawFuzzInstruction::Crypto(RawModCrypto::VerifySigEd25519(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                    FakeRawVal(v_2.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::ObjFromI64(v) => {
                RawFuzzInstruction::Int(RawModInt::ObjFromI64(*v))
            }
            RawFuzzInstructionPrototype::ObjFromI128Pieces(v_0, v_1) => {
                RawFuzzInstruction::Int(RawModInt::ObjFromI128Pieces(*v_0, *v_1))
            }
            RawFuzzInstructionPrototype::ObjFromI256Pieces(v_0, v_1, v_2, v_3) => {
                RawFuzzInstruction::Int(RawModInt::ObjFromI256Pieces(*v_0, *v_1, *v_2, *v_3))
            }
            RawFuzzInstructionPrototype::ObjFromU64(v) => {
                RawFuzzInstruction::Int(RawModInt::ObjFromU64(*v))
            }
            RawFuzzInstructionPrototype::ObjFromU128Pieces(v_0, v_1) => {
                RawFuzzInstruction::Int(RawModInt::ObjFromU128Pieces(*v_0, *v_1))
            }
            RawFuzzInstructionPrototype::ObjFromU256Pieces(v_0, v_1, v_2, v_3) => {
                RawFuzzInstruction::Int(RawModInt::ObjFromU256Pieces(*v_0, *v_1, *v_2, *v_3))
            }
            RawFuzzInstructionPrototype::ObjToI64(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Int(RawModInt::ObjToI64(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::ObjToI128Hi64(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Int(RawModInt::ObjToI128Hi64(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::ObjToI128Lo64(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Int(RawModInt::ObjToI128Lo64(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::ObjToI256HiHi(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Int(RawModInt::ObjToI256HiHi(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::ObjToI256HiLo(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Int(RawModInt::ObjToI256HiLo(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::ObjToI256LoHi(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Int(RawModInt::ObjToI256LoHi(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::ObjToI256LoLo(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Int(RawModInt::ObjToI256LoLo(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::ObjToU64(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Int(RawModInt::ObjToU64(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::ObjToU128Hi64(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Int(RawModInt::ObjToU128Hi64(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::ObjToU128Lo64(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Int(RawModInt::ObjToU128Lo64(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::ObjToU256HiHi(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Int(RawModInt::ObjToU256HiHi(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::ObjToU256HiLo(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Int(RawModInt::ObjToU256HiLo(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::ObjToU256LoHi(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Int(RawModInt::ObjToU256LoHi(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::ObjToU256LoLo(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Int(RawModInt::ObjToU256LoLo(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::CreateAssetContract(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Ledger(RawModLedger::CreateAssetContract(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::CreateContract(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                let v_2 = RawVal::from_val(env, v_2);
                RawFuzzInstruction::Ledger(RawModLedger::CreateContract(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                    FakeRawVal(v_2.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::DelContractData(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Ledger(RawModLedger::DelContractData(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::GetContractData(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Ledger(RawModLedger::GetContractData(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::HasContractData(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Ledger(RawModLedger::HasContractData(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::PutContractData(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                RawFuzzInstruction::Ledger(RawModLedger::PutContractData(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::UpdateCurrentContractWasm(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Ledger(RawModLedger::UpdateCurrentContractWasm(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::UploadWasm(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Ledger(RawModLedger::UploadWasm(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::MapDel(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                RawFuzzInstruction::Map(RawModMap::MapDel(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::MapGet(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                RawFuzzInstruction::Map(RawModMap::MapGet(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::MapHas(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                RawFuzzInstruction::Map(RawModMap::MapHas(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::MapKeys(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Map(RawModMap::MapKeys(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::MapLen(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Map(RawModMap::MapLen(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::MapMaxKey(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Map(RawModMap::MapMaxKey(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::MapMinKey(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Map(RawModMap::MapMinKey(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::MapNew => {
                RawFuzzInstruction::Map(RawModMap::MapNew)
            }
            RawFuzzInstructionPrototype::MapNewFromLinearMemory(v_0, v_1, v_2) => {
                RawFuzzInstruction::Map(RawModMap::MapNewFromLinearMemory(*v_0, *v_1, *v_2))
            }
            RawFuzzInstructionPrototype::MapNextKey(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                RawFuzzInstruction::Map(RawModMap::MapNextKey(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::MapPrevKey(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                RawFuzzInstruction::Map(RawModMap::MapPrevKey(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::MapPut(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                let v_2 = RawVal::from_val(env, v_2);
                RawFuzzInstruction::Map(RawModMap::MapPut(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                    FakeRawVal(v_2.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::MapUnpackToLinearMemory(v_0, v_1, v_2, v_3) => {
                let v_0 = RawVal::from_val(env, v_0);
                RawFuzzInstruction::Map(RawModMap::MapUnpackToLinearMemory(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                    *v_2,
                    *v_3,
                ))
            }
            RawFuzzInstructionPrototype::MapValues(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Map(RawModMap::MapValues(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::PrngBytesNew(v) => {
                RawFuzzInstruction::Prng(RawModPrng::PrngBytesNew(*v))
            }
            RawFuzzInstructionPrototype::PrngReseed(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Prng(RawModPrng::PrngReseed(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::PrngU64InInclusiveRange(v_0, v_1) => {
                RawFuzzInstruction::Prng(RawModPrng::PrngU64InInclusiveRange(*v_0, *v_1))
            }
            RawFuzzInstructionPrototype::PrngVecShuffle(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Prng(RawModPrng::PrngVecShuffle(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::Test => {
                RawFuzzInstruction::Test
            }
            RawFuzzInstructionPrototype::VecAppend(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                RawFuzzInstruction::Vec(RawModVec::VecAppend(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::VecBack(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Vec(RawModVec::VecBack(FakeRawVal(v.get_payload())))
            }
            RawFuzzInstructionPrototype::VecBinarySearch(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                RawFuzzInstruction::Vec(RawModVec::VecBinarySearch(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::VecDel(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                RawFuzzInstruction::Vec(RawModVec::VecDel(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                ))
            }
            RawFuzzInstructionPrototype::VecFirstIndexOf(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                RawFuzzInstruction::Vec(RawModVec::VecFirstIndexOf(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::VecFront(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Vec(RawModVec::VecFront(
                    FakeRawVal(v.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::VecGet(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                RawFuzzInstruction::Vec(RawModVec::VecGet(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                ))
            }
            RawFuzzInstructionPrototype::VecInsert(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_2 = RawVal::from_val(env, v_2);
                RawFuzzInstruction::Vec(RawModVec::VecInsert(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                    FakeRawVal(v_2.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::VecLastIndexOf(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                RawFuzzInstruction::Vec(RawModVec::VecLastIndexOf(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::VecLen(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Vec(RawModVec::VecLen(
                    FakeRawVal(v.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::VecNew(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Vec(RawModVec::VecNew(
                    FakeRawVal(v.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::VecNewFromLinearMemory(v_0, v_1) => {
                RawFuzzInstruction::Vec(RawModVec::VecNewFromLinearMemory(*v_0, *v_1))
            }
            RawFuzzInstructionPrototype::VecPopBack(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Vec(RawModVec::VecPopBack(
                    FakeRawVal(v.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::VecPopFront(v) => {
                let v = RawVal::from_val(env, v);
                RawFuzzInstruction::Vec(RawModVec::VecPopFront(
                    FakeRawVal(v.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::VecPushBack(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                RawFuzzInstruction::Vec(RawModVec::VecPushBack(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::VecPushFront(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                RawFuzzInstruction::Vec(RawModVec::VecPushFront(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::VecPut(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_2 = RawVal::from_val(env, v_2);
                RawFuzzInstruction::Vec(RawModVec::VecPut(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                    FakeRawVal(v_2.get_payload()),
                ))
            }
            RawFuzzInstructionPrototype::VecSlice(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                RawFuzzInstruction::Vec(RawModVec::VecSlice(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                    *v_2,
                ))
            }
            RawFuzzInstructionPrototype::VecUnpackToLinearMemory(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                RawFuzzInstruction::Vec(RawModVec::VecUnpackToLinearMemory(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                    *v_2,
                ))
            }
        }
    }
}

fuzz_target!(|input: RawFuzzInstructionPrototype| {
    let env = Env::default();
    
    let contract_id = env.register_contract_wasm(None, fuzzcontract::WASM);
    
    let client = fuzzcontract::Client::new(&env, &contract_id);

    let fuzz_instruction = input.to_guest(&env);
    let fuzz_instruction = FuzzInstruction::Raw(fuzz_instruction);

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
});
