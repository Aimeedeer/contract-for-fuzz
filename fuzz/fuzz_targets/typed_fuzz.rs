#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::{
    Env, FromVal, RawVal, IntoVal, Symbol, Status, String, Map
};
use soroban_sdk::{Address, Bytes, Vec};
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
pub enum TypedFuzzInstructionPrototype {
    AccountPublicKeyToAddress(<Bytes as SorobanArbitrary>::Prototype),
    AddressToAccountPublicKey(<Address as SorobanArbitrary>::Prototype),
    AddressToContractId(<Address as SorobanArbitrary>::Prototype),
    ContractIdToAddress(<Bytes as SorobanArbitrary>::Prototype),
    RequireAuth(<Address as SorobanArbitrary>::Prototype),
    RequireAuthForArgs(<Address as SorobanArbitrary>::Prototype, <Vec<RawVal> as SorobanArbitrary>::Prototype),

    BytesAppend(<Bytes as SorobanArbitrary>::Prototype, <Bytes as SorobanArbitrary>::Prototype),
    BytesBack(<Bytes as SorobanArbitrary>::Prototype),
    BytesCopyFromLinearMemory(
        <Bytes as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesCopyToLinearMemory(
        <Bytes as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesDel(
        <Bytes as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesFront(<Bytes as SorobanArbitrary>::Prototype),
    BytesGet(
        <Bytes as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesInsert(
        <Bytes as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesLen(<Bytes as SorobanArbitrary>::Prototype),
    BytesNew,
    BytesNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesPop(<Bytes as SorobanArbitrary>::Prototype),
    BytesPush(
        <Bytes as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesPut(
        <Bytes as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesSlice(
        <Bytes as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    DeserializeFromBytes(<Bytes as SorobanArbitrary>::Prototype),
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

    Call(
        <Bytes as SorobanArbitrary>::Prototype,
        <Symbol as SorobanArbitrary>::Prototype,
        <Vec<RawVal> as SorobanArbitrary>::Prototype,
    ),
    TryCall(
        <Bytes as SorobanArbitrary>::Prototype,
        <Symbol as SorobanArbitrary>::Prototype,
        <Vec<RawVal> as SorobanArbitrary>::Prototype,
    ),

    ContractEvent(
        <Vec<RawVal> as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),
//    FailWithStatus(<Status as SorobanArbitrary>::Prototype),
    GetCurrentCallStack,
    GetCurrentContractAddress,
    GetCurrentContractId,
    GetInvokingContract,
    GetLedgerNetworkId,
    GetLedgerSequence,
    GetLedgerTimestamp,
    GetLedgerVersion,
    LogFmtValues(
        <String as SorobanArbitrary>::Prototype,
        <Vec<RawVal> as SorobanArbitrary>::Prototype,
    ),
    LogValue(<RawVal as SorobanArbitrary>::Prototype),
    ObjCmp(
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),

    ComputeHashSha256(<Bytes as SorobanArbitrary>::Prototype),
    VerifySigEd25519(
        <Bytes as SorobanArbitrary>::Prototype,
        <Bytes as SorobanArbitrary>::Prototype,
        <Bytes as SorobanArbitrary>::Prototype,
    ),

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

    CreateContractFromContract(
        <Bytes as SorobanArbitrary>::Prototype,
        <Bytes as SorobanArbitrary>::Prototype,
    ),
    DelContractData(<RawVal as SorobanArbitrary>::Prototype),
    GetContractData(<RawVal as SorobanArbitrary>::Prototype),
    HasContractData(<RawVal as SorobanArbitrary>::Prototype),
    PutContractData(<RawVal as SorobanArbitrary>::Prototype, <RawVal as SorobanArbitrary>::Prototype),
    UpdateCurrentContractWasm(<Bytes as SorobanArbitrary>::Prototype),

    MapDel(<Map<RawVal, RawVal> as SorobanArbitrary>::Prototype, <RawVal as SorobanArbitrary>::Prototype),
    MapGet(<Map<RawVal, RawVal> as SorobanArbitrary>::Prototype, <RawVal as SorobanArbitrary>::Prototype),
    MapHas(<Map<RawVal, RawVal> as SorobanArbitrary>::Prototype, <RawVal as SorobanArbitrary>::Prototype),
    MapKeys(<Map<RawVal, RawVal> as SorobanArbitrary>::Prototype),
    MapLen(<Map<RawVal, RawVal> as SorobanArbitrary>::Prototype),
    MapMaxKey(<Map<RawVal, RawVal> as SorobanArbitrary>::Prototype),
    MapMinKey(<Map<RawVal, RawVal> as SorobanArbitrary>::Prototype),
    MapNew,
    MapNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    MapNextKey(<Map<RawVal, RawVal> as SorobanArbitrary>::Prototype, <RawVal as SorobanArbitrary>::Prototype),
    MapPrevKey(<Map<RawVal, RawVal> as SorobanArbitrary>::Prototype, <RawVal as SorobanArbitrary>::Prototype),
    MapPut(
        <Map<RawVal, RawVal> as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
        <RawVal as SorobanArbitrary>::Prototype,
    ),
    MapUnpackToLinearMemory(
        <Map<RawVal, RawVal> as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    MapValues(<Map<RawVal, RawVal> as SorobanArbitrary>::Prototype),

    Test,

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

impl TypedFuzzInstructionPrototype {
    fn to_guest(&self, env: &Env) -> TypedFuzzInstruction {
        match self {
            TypedFuzzInstructionPrototype::AccountPublicKeyToAddress(v) => {
                let v = Bytes::from_val(env, v);
                TypedFuzzInstruction::Address(TypedModAddress::AccountPublicKeyToAddress(v))
            }
            TypedFuzzInstructionPrototype::AddressToAccountPublicKey(v) => {
                let v = Address::from_val(env, v);
                TypedFuzzInstruction::Address(TypedModAddress::AddressToAccountPublicKey(v))
            }
            TypedFuzzInstructionPrototype::AddressToContractId(v) => {
                let v = Address::from_val(env, v);
                TypedFuzzInstruction::Address(TypedModAddress::AddressToContractId(v))
            }
            TypedFuzzInstructionPrototype::ContractIdToAddress(v) => {
                let v = Bytes::from_val(env, v);
                TypedFuzzInstruction::Address(TypedModAddress::ContractIdToAddress(v))
            }
            TypedFuzzInstructionPrototype::RequireAuth(v) => {
                let v = Address::from_val(env, v);
                TypedFuzzInstruction::Address(TypedModAddress::RequireAuth(v))
            }
            TypedFuzzInstructionPrototype::RequireAuthForArgs(v_0, v_1) => {
                let v_0 = Address::from_val(env, v_0);
                let v_1 = Vec::<RawVal>::from_val(env, v_1);
                TypedFuzzInstruction::Address(TypedModAddress::RequireAuthForArgs(v_0, v_1))
            }
            TypedFuzzInstructionPrototype::BytesAppend(v_0, v_1) => {
                TypedFuzzInstruction::Buf(TypedModBuf::BytesAppend(v_0.into_val(env), v_1.into_val(env)))
            }
            TypedFuzzInstructionPrototype::BytesBack(v) => {
                TypedFuzzInstruction::Buf(TypedModBuf::BytesBack(v.into_val(env)))
            }
            TypedFuzzInstructionPrototype::BytesCopyFromLinearMemory(v_0, v_1, v_2, v_3) => {
                let v_0 = Bytes::from_val(env, v_0);
                TypedFuzzInstruction::Buf(TypedModBuf::BytesCopyFromLinearMemory(
                    v_0,
                    *v_1,
                    *v_2,
                    *v_3,
                ))
            }
            TypedFuzzInstructionPrototype::BytesCopyToLinearMemory(v_0, v_1, v_2, v_3) => {
                let v_0 = Bytes::from_val(env, v_0);
                TypedFuzzInstruction::Buf(TypedModBuf::BytesCopyToLinearMemory(
                    v_0,
                    *v_1,
                    *v_2,
                    *v_3,
                ))
            }
            TypedFuzzInstructionPrototype::BytesDel(v_0, v_1) => {
                let v_0 = Bytes::from_val(env, v_0);
                TypedFuzzInstruction::Buf(TypedModBuf::BytesDel(
                    v_0,
                    *v_1,
                ))
            }
            TypedFuzzInstructionPrototype::BytesFront(v) => {
                let v = Bytes::from_val(env, v);
                TypedFuzzInstruction::Buf(TypedModBuf::BytesFront(v))
            }
            TypedFuzzInstructionPrototype::BytesGet(v_0, v_1) => {
                let v_0 = Bytes::from_val(env, v_0);
                TypedFuzzInstruction::Buf(TypedModBuf::BytesGet(
                    v_0,
                    *v_1,
                ))
            }
            TypedFuzzInstructionPrototype::BytesInsert(v_0, v_1, v_2) => {
                let v_0 = Bytes::from_val(env, v_0);
                TypedFuzzInstruction::Buf(TypedModBuf::BytesInsert(
                    v_0,
                    *v_1,
                    *v_2,
                ))
            }
            TypedFuzzInstructionPrototype::BytesLen(v) => {
                let v = Bytes::from_val(env, v);
                TypedFuzzInstruction::Buf(TypedModBuf::BytesLen(v))
            }
            TypedFuzzInstructionPrototype::BytesNew => {
                TypedFuzzInstruction::Buf(TypedModBuf::BytesNew)
            }
            TypedFuzzInstructionPrototype::BytesNewFromLinearMemory(v_0, v_1) => {
                TypedFuzzInstruction::Buf(TypedModBuf::BytesNewFromLinearMemory(*v_0, *v_1))
            }
            TypedFuzzInstructionPrototype::BytesPop(v) => {
                let v = Bytes::from_val(env, v);
                TypedFuzzInstruction::Buf(TypedModBuf::BytesPop(v))
            }
            TypedFuzzInstructionPrototype::BytesPush(v_0, v_1) => {
                let v_0 = Bytes::from_val(env, v_0);
                TypedFuzzInstruction::Buf(TypedModBuf::BytesPush(
                    v_0,
                    *v_1,
                ))
            }
            TypedFuzzInstructionPrototype::BytesPut(v_0, v_1, v_2) => {
                let v_0 = Bytes::from_val(env, v_0);
                TypedFuzzInstruction::Buf(TypedModBuf::BytesPut(
                    v_0,
                    *v_1,
                    *v_2,
                ))
            }
            TypedFuzzInstructionPrototype::BytesSlice(v_0, v_1, v_2) => {
                let v_0 = Bytes::from_val(env, v_0);
                TypedFuzzInstruction::Buf(TypedModBuf::BytesSlice(
                    v_0,
                    *v_1,
                    *v_2,
                ))
            }
            TypedFuzzInstructionPrototype::DeserializeFromBytes(v) => {
                let v = Bytes::from_val(env, v);
                TypedFuzzInstruction::Buf(TypedModBuf::DeserializeFromBytes(v))
            }
            TypedFuzzInstructionPrototype::SerializeToBytes(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Buf(TypedModBuf::SerializeToBytes(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::StringCopyToLinearMemory(v_0, v_1, v_2, v_3) => {
                let v_0 = RawVal::from_val(env, v_0);
                TypedFuzzInstruction::Buf(TypedModBuf::StringCopyToLinearMemory(FakeRawVal(v_0.get_payload()), *v_1, *v_2, *v_3))
            }
            TypedFuzzInstructionPrototype::StringLen(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Buf(TypedModBuf::StringLen(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::StringNewFromLinearMemory(v_0, v_1) => {
                TypedFuzzInstruction::Buf(TypedModBuf::StringNewFromLinearMemory(*v_0, *v_1))
            }
            TypedFuzzInstructionPrototype::SymbolCopyToLinearMemory(v_0, v_1, v_2, v_3) => {
                let v_0 = RawVal::from_val(env, v_0);
                TypedFuzzInstruction::Buf(TypedModBuf::SymbolCopyToLinearMemory(FakeRawVal(v_0.get_payload()), *v_1, *v_2, *v_3))
            }
            TypedFuzzInstructionPrototype::SymbolIndexInLinearMemory(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                TypedFuzzInstruction::Buf(TypedModBuf::SymbolIndexInLinearMemory(FakeRawVal(v_0.get_payload()), *v_1, *v_2))
            }
            TypedFuzzInstructionPrototype::SymbolLen(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Buf(TypedModBuf::SymbolLen(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::SymbolNewFromLinearMemory(v_0, v_1) => {
                TypedFuzzInstruction::Buf(TypedModBuf::SymbolNewFromLinearMemory(*v_0, *v_1))
            }
            TypedFuzzInstructionPrototype::Call(v_0, v_1, v_2) => {
                let v_0 = Bytes::from_val(env, v_0);
                let v_1 = Symbol::from_val(env, v_1);
                let v_2 = Vec::<RawVal>::from_val(env, v_2);
                TypedFuzzInstruction::Call(TypedModCall::Call(v_0, v_1, v_2))
            }
            TypedFuzzInstructionPrototype::TryCall(v_0, v_1, v_2) => {
                let v_0 = Bytes::from_val(env, v_0);
                let v_1 = Symbol::from_val(env, v_1);
                let v_2 = Vec::<RawVal>::from_val(env, v_2);
                TypedFuzzInstruction::Call(TypedModCall::TryCall(v_0, v_1, v_2))
            }
            TypedFuzzInstructionPrototype::ContractEvent(v_0, v_1) => {
                let v_0 = Vec::<RawVal>::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                TypedFuzzInstruction::Context(TypedModContext::ContractEvent(v_0, FakeRawVal(v_1.get_payload())))
            }
/*            TypedFuzzInstructionPrototype::FailWithStatus(v) => {
                let v = Status::from_val(env, v);
                TypedFuzzInstruction::Context(TypedModContext::FailWithStatus(v))
            }*/
            TypedFuzzInstructionPrototype::GetCurrentCallStack => {
                TypedFuzzInstruction::Context(TypedModContext::GetCurrentCallStack)
            }
            TypedFuzzInstructionPrototype::GetCurrentContractAddress => {
                TypedFuzzInstruction::Context(TypedModContext::GetCurrentContractAddress)
            }
            TypedFuzzInstructionPrototype::GetCurrentContractId => {
                TypedFuzzInstruction::Context(TypedModContext::GetCurrentContractId)
            }
            TypedFuzzInstructionPrototype::GetInvokingContract => {
                TypedFuzzInstruction::Context(TypedModContext::GetInvokingContract)
            }
            TypedFuzzInstructionPrototype::GetLedgerNetworkId => {
                TypedFuzzInstruction::Context(TypedModContext::GetLedgerNetworkId)
            }
            TypedFuzzInstructionPrototype::GetLedgerSequence => {
                TypedFuzzInstruction::Context(TypedModContext::GetLedgerSequence)
            }
            TypedFuzzInstructionPrototype::GetLedgerTimestamp => {
                TypedFuzzInstruction::Context(TypedModContext::GetLedgerTimestamp)
            }            
            TypedFuzzInstructionPrototype::GetLedgerVersion => {
                TypedFuzzInstruction::Context(TypedModContext::GetLedgerVersion)
            }
            TypedFuzzInstructionPrototype::LogFmtValues(v_0, v_1) => {
                let v_0 = String::from_val(env, v_0);
                let v_1 = Vec::<RawVal>::from_val(env, v_1);
                TypedFuzzInstruction::Context(TypedModContext::LogFmtValues(v_0, v_1))
            }
            TypedFuzzInstructionPrototype::LogValue(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Context(TypedModContext::LogValue(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::ObjCmp(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                TypedFuzzInstruction::Context(TypedModContext::ObjCmp(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }            
            TypedFuzzInstructionPrototype::ComputeHashSha256(v) => {
                let v = Bytes::from_val(env, v);
                TypedFuzzInstruction::Crypto(TypedModCrypto::ComputeHashSha256(v))
            }
            TypedFuzzInstructionPrototype::VerifySigEd25519(v_0, v_1, v_2) => {
                let v_0 = Bytes::from_val(env, v_0);
                let v_1 = Bytes::from_val(env, v_1);
                let v_2 = Bytes::from_val(env, v_2);
                TypedFuzzInstruction::Crypto(TypedModCrypto::VerifySigEd25519(v_0, v_1, v_2))
            }
            TypedFuzzInstructionPrototype::ObjFromI64(v) => {
                TypedFuzzInstruction::Int(TypedModInt::ObjFromI64(*v))
            }
            TypedFuzzInstructionPrototype::ObjFromI128Pieces(v_0, v_1) => {
                TypedFuzzInstruction::Int(TypedModInt::ObjFromI128Pieces(*v_0, *v_1))
            }
            TypedFuzzInstructionPrototype::ObjFromI256Pieces(v_0, v_1, v_2, v_3) => {
                TypedFuzzInstruction::Int(TypedModInt::ObjFromI256Pieces(*v_0, *v_1, *v_2, *v_3))
            }
            TypedFuzzInstructionPrototype::ObjFromU64(v) => {
                TypedFuzzInstruction::Int(TypedModInt::ObjFromU64(*v))
            }
            TypedFuzzInstructionPrototype::ObjFromU128Pieces(v_0, v_1) => {
                TypedFuzzInstruction::Int(TypedModInt::ObjFromU128Pieces(*v_0, *v_1))
            }
            TypedFuzzInstructionPrototype::ObjFromU256Pieces(v_0, v_1, v_2, v_3) => {
                TypedFuzzInstruction::Int(TypedModInt::ObjFromU256Pieces(*v_0, *v_1, *v_2, *v_3))
            }
            TypedFuzzInstructionPrototype::ObjToI64(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Int(TypedModInt::ObjToI64(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::ObjToI128Hi64(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Int(TypedModInt::ObjToI128Hi64(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::ObjToI128Lo64(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Int(TypedModInt::ObjToI128Lo64(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::ObjToI256HiHi(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Int(TypedModInt::ObjToI256HiHi(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::ObjToI256HiLo(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Int(TypedModInt::ObjToI256HiLo(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::ObjToI256LoHi(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Int(TypedModInt::ObjToI256LoHi(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::ObjToI256LoLo(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Int(TypedModInt::ObjToI256LoLo(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::ObjToU64(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Int(TypedModInt::ObjToU64(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::ObjToU128Hi64(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Int(TypedModInt::ObjToU128Hi64(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::ObjToU128Lo64(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Int(TypedModInt::ObjToU128Lo64(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::ObjToU256HiHi(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Int(TypedModInt::ObjToU256HiHi(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::ObjToU256HiLo(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Int(TypedModInt::ObjToU256HiLo(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::ObjToU256LoHi(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Int(TypedModInt::ObjToU256LoHi(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::ObjToU256LoLo(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Int(TypedModInt::ObjToU256LoLo(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::CreateContractFromContract(v_0, v_1) => {
                let v_0 = Bytes::from_val(env, v_0);
                let v_1 = Bytes::from_val(env, v_1);
                TypedFuzzInstruction::Ledger(TypedModLedger::CreateContractFromContract(v_0, v_1))
            }
            TypedFuzzInstructionPrototype::DelContractData(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Ledger(TypedModLedger::DelContractData(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::GetContractData(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Ledger(TypedModLedger::GetContractData(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::HasContractData(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Ledger(TypedModLedger::HasContractData(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::PutContractData(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                TypedFuzzInstruction::Ledger(TypedModLedger::PutContractData(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::UpdateCurrentContractWasm(v) => {
                let v = Bytes::from_val(env, v);
                TypedFuzzInstruction::Ledger(TypedModLedger::UpdateCurrentContractWasm(v))
            }
            TypedFuzzInstructionPrototype::MapDel(v_0, v_1) => {
                let v_0 = Map::<RawVal, RawVal>::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                TypedFuzzInstruction::Map(TypedModMap::MapDel(
                    v_0,
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::MapGet(v_0, v_1) => {
                let v_0 = Map::<RawVal, RawVal>::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                TypedFuzzInstruction::Map(TypedModMap::MapGet(
                    v_0,
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::MapHas(v_0, v_1) => {
                let v_0 = Map::<RawVal, RawVal>::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                TypedFuzzInstruction::Map(TypedModMap::MapHas(
                    v_0,
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::MapKeys(v) => {
                let v = Map::<RawVal, RawVal>::from_val(env, v);
                TypedFuzzInstruction::Map(TypedModMap::MapKeys(v))
            }
            TypedFuzzInstructionPrototype::MapLen(v) => {
                let v = Map::<RawVal, RawVal>::from_val(env, v);
                TypedFuzzInstruction::Map(TypedModMap::MapLen(v))
            }
            TypedFuzzInstructionPrototype::MapMaxKey(v) => {
                let v = Map::<RawVal, RawVal>::from_val(env, v);
                TypedFuzzInstruction::Map(TypedModMap::MapMaxKey(v))
            }
            TypedFuzzInstructionPrototype::MapMinKey(v) => {
                let v = Map::<RawVal, RawVal>::from_val(env, v);
                TypedFuzzInstruction::Map(TypedModMap::MapMinKey(v))
            }
            TypedFuzzInstructionPrototype::MapNew => {
                TypedFuzzInstruction::Map(TypedModMap::MapNew)
            }
            TypedFuzzInstructionPrototype::MapNewFromLinearMemory(v_0, v_1, v_2) => {
                TypedFuzzInstruction::Map(TypedModMap::MapNewFromLinearMemory(*v_0, *v_1, *v_2))
            }
            TypedFuzzInstructionPrototype::MapNextKey(v_0, v_1) => {
                let v_0 = Map::<RawVal, RawVal>::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                TypedFuzzInstruction::Map(TypedModMap::MapNextKey(
                    v_0,
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::MapPrevKey(v_0, v_1) => {
                let v_0 = Map::<RawVal, RawVal>::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                TypedFuzzInstruction::Map(TypedModMap::MapPrevKey(
                    v_0,
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::MapPut(v_0, v_1, v_2) => {
                let v_0 = Map::<RawVal, RawVal>::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                let v_2 = RawVal::from_val(env, v_2);
                TypedFuzzInstruction::Map(TypedModMap::MapPut(
                    v_0,
                    FakeRawVal(v_1.get_payload()),
                    FakeRawVal(v_2.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::MapUnpackToLinearMemory(v_0, v_1, v_2, v_3) => {
                let v_0 = Map::<RawVal, RawVal>::from_val(env, v_0);
                TypedFuzzInstruction::Map(TypedModMap::MapUnpackToLinearMemory(
                    v_0,
                    *v_1,
                    *v_2,
                    *v_3,
                ))
            }
            TypedFuzzInstructionPrototype::MapValues(v) => {
                let v = Map::<RawVal, RawVal>::from_val(env, v);
                TypedFuzzInstruction::Map(TypedModMap::MapValues(v))
            }
            TypedFuzzInstructionPrototype::Test => {
                TypedFuzzInstruction::Test
            }
            TypedFuzzInstructionPrototype::VecAppend(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                TypedFuzzInstruction::Vec(TypedModVec::VecAppend(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::VecBack(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Vec(TypedModVec::VecBack(FakeRawVal(v.get_payload())))
            }
            TypedFuzzInstructionPrototype::VecBinarySearch(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                TypedFuzzInstruction::Vec(TypedModVec::VecBinarySearch(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::VecDel(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                TypedFuzzInstruction::Vec(TypedModVec::VecDel(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                ))
            }
            TypedFuzzInstructionPrototype::VecFirstIndexOf(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                TypedFuzzInstruction::Vec(TypedModVec::VecFirstIndexOf(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::VecFront(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Vec(TypedModVec::VecFront(
                    FakeRawVal(v.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::VecGet(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                TypedFuzzInstruction::Vec(TypedModVec::VecGet(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                ))
            }
            TypedFuzzInstructionPrototype::VecInsert(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_2 = RawVal::from_val(env, v_2);
                TypedFuzzInstruction::Vec(TypedModVec::VecInsert(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                    FakeRawVal(v_2.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::VecLastIndexOf(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                TypedFuzzInstruction::Vec(TypedModVec::VecLastIndexOf(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::VecLen(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Vec(TypedModVec::VecLen(
                    FakeRawVal(v.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::VecNew(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Vec(TypedModVec::VecNew(
                    FakeRawVal(v.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::VecNewFromLinearMemory(v_0, v_1) => {
                TypedFuzzInstruction::Vec(TypedModVec::VecNewFromLinearMemory(*v_0, *v_1))
            }
            TypedFuzzInstructionPrototype::VecPopBack(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Vec(TypedModVec::VecPopBack(
                    FakeRawVal(v.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::VecPopFront(v) => {
                let v = RawVal::from_val(env, v);
                TypedFuzzInstruction::Vec(TypedModVec::VecPopFront(
                    FakeRawVal(v.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::VecPushBack(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                TypedFuzzInstruction::Vec(TypedModVec::VecPushBack(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::VecPushFront(v_0, v_1) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_1 = RawVal::from_val(env, v_1);
                TypedFuzzInstruction::Vec(TypedModVec::VecPushFront(
                    FakeRawVal(v_0.get_payload()),
                    FakeRawVal(v_1.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::VecPut(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                let v_2 = RawVal::from_val(env, v_2);
                TypedFuzzInstruction::Vec(TypedModVec::VecPut(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                    FakeRawVal(v_2.get_payload()),
                ))
            }
            TypedFuzzInstructionPrototype::VecSlice(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                TypedFuzzInstruction::Vec(TypedModVec::VecSlice(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                    *v_2,
                ))
            }
            TypedFuzzInstructionPrototype::VecUnpackToLinearMemory(v_0, v_1, v_2) => {
                let v_0 = RawVal::from_val(env, v_0);
                TypedFuzzInstruction::Vec(TypedModVec::VecUnpackToLinearMemory(
                    FakeRawVal(v_0.get_payload()),
                    *v_1,
                    *v_2,
                ))
            }
        }
    }
}

fuzz_target!(|input: TypedFuzzInstructionPrototype| {
    let env = Env::default();
    
    let contract_id = env.register_contract_wasm(None, fuzzcontract::WASM);
    
    let client = fuzzcontract::Client::new(&env, &contract_id);

    let fuzz_instruction = input.to_guest(&env);
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
});
