#![no_main]

use fuzzcontract::*;
use libfuzzer_sys::fuzz_target;
use soroban_sdk::arbitrary::arbitrary;
use soroban_sdk::arbitrary::fuzz_catch_panic;
use soroban_sdk::arbitrary::SorobanArbitrary;
use soroban_sdk::testutils::Logs;
use soroban_sdk::{Env, FromVal, Val};

mod fuzzcontract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/contract_for_fuzz.wasm"
    );
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum RawFuzzInstructionPrototype {
    Address(RawModAddressPrototype),
    Buf(RawModBufPrototype),
    Call(RawModCallPrototype),
    Context(RawModContextPrototype),
    Crypto(RawModCryptoPrototype),
    Int(RawModIntPrototype),
    Ledger(RawModLedgerPrototype),
    Map(RawModMapPrototype),
    Prng(RawModPrngPrototype),
    Test,
    Vec(RawModVecPrototype),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum RawModAddressPrototype {
    AccountPublicKeyToAddress(<Val as SorobanArbitrary>::Prototype),
    AddressToAccountPublicKey(<Val as SorobanArbitrary>::Prototype),
    AddressToContractId(<Val as SorobanArbitrary>::Prototype),
    AuthorizeAsCurrContract(<Val as SorobanArbitrary>::Prototype),
    ContractIdToAddress(<Val as SorobanArbitrary>::Prototype),
    RequireAuth(<Val as SorobanArbitrary>::Prototype),
    RequireAuthForArgs(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum RawModBufPrototype {
    BytesAppend(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    BytesBack(<Val as SorobanArbitrary>::Prototype),
    BytesCopyFromLinearMemory(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesCopyToLinearMemory(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesDel(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesFront(<Val as SorobanArbitrary>::Prototype),
    BytesGet(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesInsert(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesLen(<Val as SorobanArbitrary>::Prototype),
    BytesNew,
    BytesNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesPop(<Val as SorobanArbitrary>::Prototype),
    BytesPush(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesPut(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BytesSlice(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    DeserializeFromBytes(<Val as SorobanArbitrary>::Prototype),
    SerializeToBytes(<Val as SorobanArbitrary>::Prototype),
    StringCopyToLinearMemory(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    StringLen(<Val as SorobanArbitrary>::Prototype),
    StringNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    SymbolCopyToLinearMemory(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    SymbolIndexInLinearMemory(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    SymbolLen(<Val as SorobanArbitrary>::Prototype),
    SymbolNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum RawModCallPrototype {
    Call(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    TryCall(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum RawModContextPrototype {
    ContractEvent(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    FailWithError(<Val as SorobanArbitrary>::Prototype),
    GetCurrentCallStack,
    GetCurrentContractAddress,
    GetInvokingContract,
    GetLedgerNetworkId,
    GetLedgerSequence,
    GetLedgerTimestamp,
    GetLedgerVersion,
    GetMaxExpirationLedger,
    LogFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    ObjCmp(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum RawModCryptoPrototype {
    ComputeHashKeccak256(<Val as SorobanArbitrary>::Prototype),
    ComputeHashSha256(<Val as SorobanArbitrary>::Prototype),
    RecoverKeyEcdsaSecp256k1(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    VerifySigEd25519(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum RawModIntPrototype {
    DurationObjFromU64(<u64 as SorobanArbitrary>::Prototype),
    DurationObjToU64(<Val as SorobanArbitrary>::Prototype),
    I256Add(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    I256Div(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    I256Mul(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    I256Pow(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    I256Shl(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    I256Shr(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    I256Sub(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    I256ObjFromBeBytes(<Val as SorobanArbitrary>::Prototype),
    I256ObjToBeBytes(<Val as SorobanArbitrary>::Prototype),
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
    ObjToI64(<Val as SorobanArbitrary>::Prototype),
    ObjToI128Hi64(<Val as SorobanArbitrary>::Prototype),
    ObjToI128Lo64(<Val as SorobanArbitrary>::Prototype),
    ObjToI256HiHi(<Val as SorobanArbitrary>::Prototype),
    ObjToI256HiLo(<Val as SorobanArbitrary>::Prototype),
    ObjToI256LoHi(<Val as SorobanArbitrary>::Prototype),
    ObjToI256LoLo(<Val as SorobanArbitrary>::Prototype),
    ObjToU64(<Val as SorobanArbitrary>::Prototype),
    ObjToU128Hi64(<Val as SorobanArbitrary>::Prototype),
    ObjToU128Lo64(<Val as SorobanArbitrary>::Prototype),
    ObjToU256HiHi(<Val as SorobanArbitrary>::Prototype),
    ObjToU256HiLo(<Val as SorobanArbitrary>::Prototype),
    ObjToU256LoHi(<Val as SorobanArbitrary>::Prototype),
    ObjToU256LoLo(<Val as SorobanArbitrary>::Prototype),
    TimepointObjFromU64(<u64 as SorobanArbitrary>::Prototype),
    TimepointObjToU64(<Val as SorobanArbitrary>::Prototype),
    U256Add(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    U256Div(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    U256Mul(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    U256Pow(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    U256Shl(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    U256Shr(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    U256Sub(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    U256ValFromBeBytes(<Val as SorobanArbitrary>::Prototype),
    U256ValToBeBytes(<Val as SorobanArbitrary>::Prototype),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum RawModLedgerPrototype {
    BumpContractData(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BumpContractInstanceAndCode(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    BumpCurrentContract(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    CreateAssetContract(<Val as SorobanArbitrary>::Prototype),
    CreateContract(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    DelContractData(<Val as SorobanArbitrary>::Prototype),
    GetAssetContractId(<Val as SorobanArbitrary>::Prototype),
    GetContractData(<Val as SorobanArbitrary>::Prototype),
    GetContractId(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    HasContractData(<Val as SorobanArbitrary>::Prototype),
    PutContractData(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    UpdateCurrentContractWasm(<Val as SorobanArbitrary>::Prototype),
    UploadWasm(<Val as SorobanArbitrary>::Prototype),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum RawModMapPrototype {
    MapDel(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    MapGet(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    MapHas(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    MapKeyByPos(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    MapKeys(<Val as SorobanArbitrary>::Prototype),
    MapLen(<Val as SorobanArbitrary>::Prototype),
    MapNew,
    MapNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    MapPut(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    MapUnpackToLinearMemory(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    MapValByPos(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    MapValues(<Val as SorobanArbitrary>::Prototype),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum RawModPrngPrototype {
    PrngBytesNew(u32),
    PrngReseed(<Val as SorobanArbitrary>::Prototype),
    PrngU64InInclusiveRange(u64, u64),
    PrngVecShuffle(<Val as SorobanArbitrary>::Prototype),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum RawModVecPrototype {
    VecAppend(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    VecBack(<Val as SorobanArbitrary>::Prototype),
    VecBinarySearch(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    VecDel(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    VecFirstIndexOf(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    VecFront(<Val as SorobanArbitrary>::Prototype),
    VecGet(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    VecInsert(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    VecLastIndexOf(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    VecLen(<Val as SorobanArbitrary>::Prototype),
    VecNew,
    VecNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    VecPopBack(<Val as SorobanArbitrary>::Prototype),
    VecPopFront(<Val as SorobanArbitrary>::Prototype),
    VecPushBack(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    VecPushFront(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    VecPut(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    VecSlice(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    VecUnpackToLinearMemory(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
}

impl RawFuzzInstructionPrototype {
    fn to_guest(&self, env: &Env) -> RawFuzzInstruction {
        match self {
            RawFuzzInstructionPrototype::Address(v) => match v {
                RawModAddressPrototype::AccountPublicKeyToAddress(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Address(RawModAddress::AccountPublicKeyToAddress(FakeVal(
                        v.get_payload(),
                    )))
                }
                RawModAddressPrototype::AddressToAccountPublicKey(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Address(RawModAddress::AddressToAccountPublicKey(FakeVal(
                        v.get_payload(),
                    )))
                }
                RawModAddressPrototype::AddressToContractId(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Address(RawModAddress::AddressToContractId(FakeVal(
                        v.get_payload(),
                    )))
                }
                RawModAddressPrototype::AuthorizeAsCurrContract(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Address(RawModAddress::AuthorizeAsCurrContract(FakeVal(
                        v.get_payload(),
                    )))
                }
                RawModAddressPrototype::ContractIdToAddress(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Address(RawModAddress::ContractIdToAddress(FakeVal(
                        v.get_payload(),
                    )))
                }
                RawModAddressPrototype::RequireAuth(v_0) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Address(RawModAddress::RequireAuth(FakeVal(
                        v_0.get_payload(),
                    )))
                }
                RawModAddressPrototype::RequireAuthForArgs(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Address(RawModAddress::RequireAuthForArgs(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
            },
            RawFuzzInstructionPrototype::Buf(v) => match v {
                RawModBufPrototype::BytesAppend(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Buf(RawModBuf::BytesAppend(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModBufPrototype::BytesBack(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Buf(RawModBuf::BytesBack(FakeVal(v.get_payload())))
                }
                RawModBufPrototype::BytesCopyFromLinearMemory(v_0, v_1, v_2, v_3) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Buf(RawModBuf::BytesCopyFromLinearMemory(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                        *v_2,
                        *v_3,
                    ))
                }
                RawModBufPrototype::BytesCopyToLinearMemory(v_0, v_1, v_2, v_3) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Buf(RawModBuf::BytesCopyToLinearMemory(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                        *v_2,
                        *v_3,
                    ))
                }
                RawModBufPrototype::BytesDel(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Buf(RawModBuf::BytesDel(FakeVal(v_0.get_payload()), *v_1))
                }
                RawModBufPrototype::BytesFront(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Buf(RawModBuf::BytesFront(FakeVal(v.get_payload())))
                }
                RawModBufPrototype::BytesGet(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Buf(RawModBuf::BytesGet(FakeVal(v_0.get_payload()), *v_1))
                }
                RawModBufPrototype::BytesInsert(v_0, v_1, v_2) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Buf(RawModBuf::BytesInsert(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                        *v_2,
                    ))
                }
                RawModBufPrototype::BytesLen(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Buf(RawModBuf::BytesLen(FakeVal(v.get_payload())))
                }
                RawModBufPrototype::BytesNew => RawFuzzInstruction::Buf(RawModBuf::BytesNew),
                RawModBufPrototype::BytesNewFromLinearMemory(v_0, v_1) => {
                    RawFuzzInstruction::Buf(RawModBuf::BytesNewFromLinearMemory(*v_0, *v_1))
                }
                RawModBufPrototype::BytesPop(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Buf(RawModBuf::BytesPop(FakeVal(v.get_payload())))
                }
                RawModBufPrototype::BytesPush(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Buf(RawModBuf::BytesPush(FakeVal(v_0.get_payload()), *v_1))
                }
                RawModBufPrototype::BytesPut(v_0, v_1, v_2) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Buf(RawModBuf::BytesPut(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                        *v_2,
                    ))
                }
                RawModBufPrototype::BytesSlice(v_0, v_1, v_2) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Buf(RawModBuf::BytesSlice(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                        *v_2,
                    ))
                }
                RawModBufPrototype::DeserializeFromBytes(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Buf(RawModBuf::DeserializeFromBytes(FakeVal(
                        v.get_payload(),
                    )))
                }
                RawModBufPrototype::SerializeToBytes(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Buf(RawModBuf::SerializeToBytes(FakeVal(v.get_payload())))
                }
                RawModBufPrototype::StringCopyToLinearMemory(v_0, v_1, v_2, v_3) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Buf(RawModBuf::StringCopyToLinearMemory(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                        *v_2,
                        *v_3,
                    ))
                }
                RawModBufPrototype::StringLen(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Buf(RawModBuf::StringLen(FakeVal(v.get_payload())))
                }
                RawModBufPrototype::StringNewFromLinearMemory(v_0, v_1) => {
                    RawFuzzInstruction::Buf(RawModBuf::StringNewFromLinearMemory(*v_0, *v_1))
                }
                RawModBufPrototype::SymbolCopyToLinearMemory(v_0, v_1, v_2, v_3) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Buf(RawModBuf::SymbolCopyToLinearMemory(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                        *v_2,
                        *v_3,
                    ))
                }
                RawModBufPrototype::SymbolIndexInLinearMemory(v_0, v_1, v_2) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Buf(RawModBuf::SymbolIndexInLinearMemory(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                        *v_2,
                    ))
                }
                RawModBufPrototype::SymbolLen(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Buf(RawModBuf::SymbolLen(FakeVal(v.get_payload())))
                }
                RawModBufPrototype::SymbolNewFromLinearMemory(v_0, v_1) => {
                    RawFuzzInstruction::Buf(RawModBuf::SymbolNewFromLinearMemory(*v_0, *v_1))
                }
            },
            RawFuzzInstructionPrototype::Call(v) => match v {
                RawModCallPrototype::Call(v_0, v_1, v_2) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    let v_2 = Val::from_val(env, v_2);
                    RawFuzzInstruction::Call(RawModCall::Call(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                        FakeVal(v_2.get_payload()),
                    ))
                }
                RawModCallPrototype::TryCall(v_0, v_1, v_2) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    let v_2 = Val::from_val(env, v_2);
                    RawFuzzInstruction::Call(RawModCall::TryCall(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                        FakeVal(v_2.get_payload()),
                    ))
                }
            },
            RawFuzzInstructionPrototype::Context(v) => match v {
                RawModContextPrototype::ContractEvent(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Context(RawModContext::ContractEvent(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModContextPrototype::FailWithError(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Context(RawModContext::FailWithError(FakeVal(
                        v.get_payload(),
                    )))
                }
                RawModContextPrototype::GetCurrentCallStack => {
                    RawFuzzInstruction::Context(RawModContext::GetCurrentCallStack)
                }
                RawModContextPrototype::GetCurrentContractAddress => {
                    RawFuzzInstruction::Context(RawModContext::GetCurrentContractAddress)
                }
                RawModContextPrototype::GetInvokingContract => {
                    RawFuzzInstruction::Context(RawModContext::GetInvokingContract)
                }
                RawModContextPrototype::GetLedgerNetworkId => {
                    RawFuzzInstruction::Context(RawModContext::GetLedgerNetworkId)
                }
                RawModContextPrototype::GetLedgerSequence => {
                    RawFuzzInstruction::Context(RawModContext::GetLedgerSequence)
                }
                RawModContextPrototype::GetLedgerTimestamp => {
                    RawFuzzInstruction::Context(RawModContext::GetLedgerTimestamp)
                }
                RawModContextPrototype::GetLedgerVersion => {
                    RawFuzzInstruction::Context(RawModContext::GetLedgerVersion)
                }
                RawModContextPrototype::GetMaxExpirationLedger => {
                    RawFuzzInstruction::Context(RawModContext::GetMaxExpirationLedger)
                }
                RawModContextPrototype::LogFromLinearMemory(v_0, v_1, v_2, v_3) => {
                    RawFuzzInstruction::Context(RawModContext::LogFromLinearMemory(
                        *v_0, *v_1, *v_2, *v_3,
                    ))
                }
                RawModContextPrototype::ObjCmp(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Context(RawModContext::ObjCmp(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
            },
            RawFuzzInstructionPrototype::Crypto(v) => match v {
                RawModCryptoPrototype::ComputeHashKeccak256(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Crypto(RawModCrypto::ComputeHashKeccak256(FakeVal(
                        v.get_payload(),
                    )))
                }
                RawModCryptoPrototype::ComputeHashSha256(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Crypto(RawModCrypto::ComputeHashSha256(FakeVal(
                        v.get_payload(),
                    )))
                }
                RawModCryptoPrototype::RecoverKeyEcdsaSecp256k1(v_0, v_1, v_2) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    let v_2 = Val::from_val(env, v_2);
                    RawFuzzInstruction::Crypto(RawModCrypto::RecoverKeyEcdsaSecp256k1(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                        FakeVal(v_2.get_payload()),
                    ))
                }
                RawModCryptoPrototype::VerifySigEd25519(v_0, v_1, v_2) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    let v_2 = Val::from_val(env, v_2);
                    RawFuzzInstruction::Crypto(RawModCrypto::VerifySigEd25519(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                        FakeVal(v_2.get_payload()),
                    ))
                }
            },
            RawFuzzInstructionPrototype::Int(v) => match v {
                RawModIntPrototype::DurationObjFromU64(v) => {
                    RawFuzzInstruction::Int(RawModInt::DurationObjFromU64(*v))
                }
                RawModIntPrototype::DurationObjToU64(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::DurationObjToU64(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::I256Add(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Int(RawModInt::I256Add(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModIntPrototype::I256Div(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Int(RawModInt::I256Div(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModIntPrototype::I256Mul(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Int(RawModInt::I256Mul(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModIntPrototype::I256Pow(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Int(RawModInt::I256Pow(FakeVal(v_0.get_payload()), *v_1))
                }
                RawModIntPrototype::I256Shl(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Int(RawModInt::I256Shl(FakeVal(v_0.get_payload()), *v_1))
                }
                RawModIntPrototype::I256Shr(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Int(RawModInt::I256Shr(FakeVal(v_0.get_payload()), *v_1))
                }
                RawModIntPrototype::I256Sub(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Int(RawModInt::I256Sub(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModIntPrototype::I256ObjFromBeBytes(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::I256ObjFromBeBytes(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::I256ObjToBeBytes(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::I256ObjToBeBytes(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::ObjFromI64(v) => {
                    RawFuzzInstruction::Int(RawModInt::ObjFromI64(*v))
                }
                RawModIntPrototype::ObjFromI128Pieces(v_0, v_1) => {
                    RawFuzzInstruction::Int(RawModInt::ObjFromI128Pieces(*v_0, *v_1))
                }
                RawModIntPrototype::ObjFromI256Pieces(v_0, v_1, v_2, v_3) => {
                    RawFuzzInstruction::Int(RawModInt::ObjFromI256Pieces(*v_0, *v_1, *v_2, *v_3))
                }
                RawModIntPrototype::ObjFromU64(v) => {
                    RawFuzzInstruction::Int(RawModInt::ObjFromU64(*v))
                }
                RawModIntPrototype::ObjFromU128Pieces(v_0, v_1) => {
                    RawFuzzInstruction::Int(RawModInt::ObjFromU128Pieces(*v_0, *v_1))
                }
                RawModIntPrototype::ObjFromU256Pieces(v_0, v_1, v_2, v_3) => {
                    RawFuzzInstruction::Int(RawModInt::ObjFromU256Pieces(*v_0, *v_1, *v_2, *v_3))
                }
                RawModIntPrototype::ObjToI64(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::ObjToI64(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::ObjToI128Hi64(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::ObjToI128Hi64(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::ObjToI128Lo64(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::ObjToI128Lo64(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::ObjToI256HiHi(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::ObjToI256HiHi(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::ObjToI256HiLo(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::ObjToI256HiLo(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::ObjToI256LoHi(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::ObjToI256LoHi(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::ObjToI256LoLo(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::ObjToI256LoLo(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::ObjToU64(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::ObjToU64(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::ObjToU128Hi64(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::ObjToU128Hi64(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::ObjToU128Lo64(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::ObjToU128Lo64(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::ObjToU256HiHi(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::ObjToU256HiHi(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::ObjToU256HiLo(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::ObjToU256HiLo(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::ObjToU256LoHi(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::ObjToU256LoHi(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::ObjToU256LoLo(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::ObjToU256LoLo(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::TimepointObjFromU64(v) => {
                    RawFuzzInstruction::Int(RawModInt::TimepointObjFromU64(*v))
                }
                RawModIntPrototype::TimepointObjToU64(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::TimepointObjToU64(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::U256Add(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Int(RawModInt::U256Add(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModIntPrototype::U256Div(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Int(RawModInt::U256Div(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModIntPrototype::U256Mul(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Int(RawModInt::U256Mul(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModIntPrototype::U256Pow(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Int(RawModInt::U256Pow(FakeVal(v_0.get_payload()), *v_1))
                }
                RawModIntPrototype::U256Shl(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Int(RawModInt::U256Shl(FakeVal(v_0.get_payload()), *v_1))
                }
                RawModIntPrototype::U256Shr(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Int(RawModInt::U256Shr(FakeVal(v_0.get_payload()), *v_1))
                }
                RawModIntPrototype::U256Sub(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Int(RawModInt::U256Sub(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModIntPrototype::U256ValFromBeBytes(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::U256ValFromBeBytes(FakeVal(v.get_payload())))
                }
                RawModIntPrototype::U256ValToBeBytes(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Int(RawModInt::U256ValToBeBytes(FakeVal(v.get_payload())))
                }
            },
            RawFuzzInstructionPrototype::Ledger(v) => match v {
                RawModLedgerPrototype::BumpContractData(v_0, v_1, v_2) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Ledger(RawModLedger::BumpContractData(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                        *v_2,
                    ))
                }
                RawModLedgerPrototype::BumpContractInstanceAndCode(v_0, v_1, v_2) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Ledger(RawModLedger::BumpContractInstanceAndCode(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                        *v_2,
                    ))
                }
                RawModLedgerPrototype::BumpCurrentContract(v_0, v_1) => {
                    RawFuzzInstruction::Ledger(RawModLedger::BumpCurrentContract(*v_0, *v_1))
                }
                RawModLedgerPrototype::CreateAssetContract(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Ledger(RawModLedger::CreateAssetContract(FakeVal(
                        v.get_payload(),
                    )))
                }
                RawModLedgerPrototype::CreateContract(v_0, v_1, v_2) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    let v_2 = Val::from_val(env, v_2);
                    RawFuzzInstruction::Ledger(RawModLedger::CreateContract(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                        FakeVal(v_2.get_payload()),
                    ))
                }
                RawModLedgerPrototype::DelContractData(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Ledger(RawModLedger::DelContractData(FakeVal(
                        v.get_payload(),
                    )))
                }
                RawModLedgerPrototype::GetAssetContractId(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Ledger(RawModLedger::GetAssetContractId(FakeVal(
                        v.get_payload(),
                    )))
                }
                RawModLedgerPrototype::GetContractData(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Ledger(RawModLedger::GetContractData(FakeVal(
                        v.get_payload(),
                    )))
                }
                RawModLedgerPrototype::GetContractId(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Ledger(RawModLedger::GetContractId(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModLedgerPrototype::HasContractData(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Ledger(RawModLedger::HasContractData(FakeVal(
                        v.get_payload(),
                    )))
                }
                RawModLedgerPrototype::PutContractData(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Ledger(RawModLedger::PutContractData(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModLedgerPrototype::UpdateCurrentContractWasm(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Ledger(RawModLedger::UpdateCurrentContractWasm(FakeVal(
                        v.get_payload(),
                    )))
                }
                RawModLedgerPrototype::UploadWasm(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Ledger(RawModLedger::UploadWasm(FakeVal(v.get_payload())))
                }
            },
            RawFuzzInstructionPrototype::Map(v) => match v {
                RawModMapPrototype::MapDel(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Map(RawModMap::MapDel(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModMapPrototype::MapGet(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Map(RawModMap::MapGet(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModMapPrototype::MapHas(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Map(RawModMap::MapHas(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModMapPrototype::MapKeyByPos(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Map(RawModMap::MapKeyByPos(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                    ))
                }
                RawModMapPrototype::MapKeys(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Map(RawModMap::MapKeys(FakeVal(v.get_payload())))
                }
                RawModMapPrototype::MapLen(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Map(RawModMap::MapLen(FakeVal(v.get_payload())))
                }
                RawModMapPrototype::MapNew => RawFuzzInstruction::Map(RawModMap::MapNew),
                RawModMapPrototype::MapNewFromLinearMemory(v_0, v_1, v_2) => {
                    RawFuzzInstruction::Map(RawModMap::MapNewFromLinearMemory(*v_0, *v_1, *v_2))
                }
                RawModMapPrototype::MapPut(v_0, v_1, v_2) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    let v_2 = Val::from_val(env, v_2);
                    RawFuzzInstruction::Map(RawModMap::MapPut(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                        FakeVal(v_2.get_payload()),
                    ))
                }
                RawModMapPrototype::MapUnpackToLinearMemory(v_0, v_1, v_2, v_3) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Map(RawModMap::MapUnpackToLinearMemory(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                        *v_2,
                        *v_3,
                    ))
                }
                RawModMapPrototype::MapValByPos(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Map(RawModMap::MapValByPos(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                    ))
                }
                RawModMapPrototype::MapValues(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Map(RawModMap::MapValues(FakeVal(v.get_payload())))
                }
            },
            RawFuzzInstructionPrototype::Prng(v) => match v {
                RawModPrngPrototype::PrngBytesNew(v) => {
                    RawFuzzInstruction::Prng(RawModPrng::PrngBytesNew(*v))
                }
                RawModPrngPrototype::PrngReseed(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Prng(RawModPrng::PrngReseed(FakeVal(v.get_payload())))
                }
                RawModPrngPrototype::PrngU64InInclusiveRange(v_0, v_1) => {
                    RawFuzzInstruction::Prng(RawModPrng::PrngU64InInclusiveRange(*v_0, *v_1))
                }
                RawModPrngPrototype::PrngVecShuffle(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Prng(RawModPrng::PrngVecShuffle(FakeVal(v.get_payload())))
                }
            },
            RawFuzzInstructionPrototype::Test => RawFuzzInstruction::Test,
            RawFuzzInstructionPrototype::Vec(v) => match v {
                RawModVecPrototype::VecAppend(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Vec(RawModVec::VecAppend(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModVecPrototype::VecBack(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Vec(RawModVec::VecBack(FakeVal(v.get_payload())))
                }
                RawModVecPrototype::VecBinarySearch(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Vec(RawModVec::VecBinarySearch(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModVecPrototype::VecDel(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Vec(RawModVec::VecDel(FakeVal(v_0.get_payload()), *v_1))
                }
                RawModVecPrototype::VecFirstIndexOf(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Vec(RawModVec::VecFirstIndexOf(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModVecPrototype::VecFront(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Vec(RawModVec::VecFront(FakeVal(v.get_payload())))
                }
                RawModVecPrototype::VecGet(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Vec(RawModVec::VecGet(FakeVal(v_0.get_payload()), *v_1))
                }
                RawModVecPrototype::VecInsert(v_0, v_1, v_2) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_2 = Val::from_val(env, v_2);
                    RawFuzzInstruction::Vec(RawModVec::VecInsert(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                        FakeVal(v_2.get_payload()),
                    ))
                }
                RawModVecPrototype::VecLastIndexOf(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Vec(RawModVec::VecLastIndexOf(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModVecPrototype::VecLen(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Vec(RawModVec::VecLen(FakeVal(v.get_payload())))
                }
                RawModVecPrototype::VecNew => RawFuzzInstruction::Vec(RawModVec::VecNew),
                RawModVecPrototype::VecNewFromLinearMemory(v_0, v_1) => {
                    RawFuzzInstruction::Vec(RawModVec::VecNewFromLinearMemory(*v_0, *v_1))
                }
                RawModVecPrototype::VecPopBack(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Vec(RawModVec::VecPopBack(FakeVal(v.get_payload())))
                }
                RawModVecPrototype::VecPopFront(v) => {
                    let v = Val::from_val(env, v);
                    RawFuzzInstruction::Vec(RawModVec::VecPopFront(FakeVal(v.get_payload())))
                }
                RawModVecPrototype::VecPushBack(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Vec(RawModVec::VecPushBack(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModVecPrototype::VecPushFront(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    RawFuzzInstruction::Vec(RawModVec::VecPushFront(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                RawModVecPrototype::VecPut(v_0, v_1, v_2) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_2 = Val::from_val(env, v_2);
                    RawFuzzInstruction::Vec(RawModVec::VecPut(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                        FakeVal(v_2.get_payload()),
                    ))
                }
                RawModVecPrototype::VecSlice(v_0, v_1, v_2) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Vec(RawModVec::VecSlice(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                        *v_2,
                    ))
                }
                RawModVecPrototype::VecUnpackToLinearMemory(v_0, v_1, v_2) => {
                    let v_0 = Val::from_val(env, v_0);
                    RawFuzzInstruction::Vec(RawModVec::VecUnpackToLinearMemory(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                        *v_2,
                    ))
                }
            },
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
        if !env.logs().all().is_empty() {
            env.logs().print();
        }
        panic!("host panicked: {panic_r:?}");
    }
});
