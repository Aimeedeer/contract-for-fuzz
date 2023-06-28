#![no_main]

use fuzzcontract::*;
use libfuzzer_sys::fuzz_target;
use soroban_sdk::arbitrary::arbitrary;
use soroban_sdk::arbitrary::fuzz_catch_panic;
use soroban_sdk::arbitrary::SorobanArbitrary;
use soroban_sdk::testutils::Logs;
use soroban_sdk::{Address, Bytes, Vec};
use soroban_sdk::{Env, FromVal, IntoVal, Map, String, Symbol, Val};

mod fuzzcontract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/contract_for_fuzz.wasm"
    );
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum TypedFuzzInstructionPrototype {
    Address(TypedModAddressPrototype),
    Buf(TypedModBufPrototype),
    Call(TypedModCallPrototype),
    Context(TypedModContextPrototype),
    Crypto(TypedModCryptoPrototype),
    Int(TypedModIntPrototype),
    Ledger(TypedModLedgerPrototype),
    Map(TypedModMapPrototype),
    Prng(TypedModPrngPrototype),
    Test,
    Vec(TypedModVecPrototype),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum TypedModAddressPrototype {
    AccountPublicKeyToAddress(<Bytes as SorobanArbitrary>::Prototype),
    AddressToAccountPublicKey(<Address as SorobanArbitrary>::Prototype),
    AddressToContractId(<Address as SorobanArbitrary>::Prototype),
    AuthorizeAsCurrContract(<Vec<Val> as SorobanArbitrary>::Prototype),
    ContractIdToAddress(<Bytes as SorobanArbitrary>::Prototype),
    RequireAuth(<Address as SorobanArbitrary>::Prototype),
    RequireAuthForArgs(
        <Address as SorobanArbitrary>::Prototype,
        <Vec<Val> as SorobanArbitrary>::Prototype,
    ),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum TypedModBufPrototype {
    BytesAppend(
        <Bytes as SorobanArbitrary>::Prototype,
        <Bytes as SorobanArbitrary>::Prototype,
    ),
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
    SerializeToBytes(<Val as SorobanArbitrary>::Prototype),
    StringCopyToLinearMemory(
        <String as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    StringLen(<String as SorobanArbitrary>::Prototype),
    StringNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    SymbolCopyToLinearMemory(
        <Symbol as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    SymbolIndexInLinearMemory(
        <Symbol as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    SymbolLen(<Symbol as SorobanArbitrary>::Prototype),
    SymbolNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum TypedModCallPrototype {
    Call(
        <Address as SorobanArbitrary>::Prototype,
        <Symbol as SorobanArbitrary>::Prototype,
        <Vec<Val> as SorobanArbitrary>::Prototype,
    ),
    TryCall(
        <Address as SorobanArbitrary>::Prototype,
        <Symbol as SorobanArbitrary>::Prototype,
        <Vec<Val> as SorobanArbitrary>::Prototype,
    ),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum TypedModContextPrototype {
    ContractEvent(
        <Vec<Val> as SorobanArbitrary>::Prototype,
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
pub enum TypedModCryptoPrototype {
    ComputeHashKeccak256(<Bytes as SorobanArbitrary>::Prototype),
    ComputeHashSha256(<Bytes as SorobanArbitrary>::Prototype),
    RecoverKeyEcdsaSecp256k1(
        <Bytes as SorobanArbitrary>::Prototype,
        <Bytes as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    VerifySigEd25519(
        <Bytes as SorobanArbitrary>::Prototype,
        <Bytes as SorobanArbitrary>::Prototype,
        <Bytes as SorobanArbitrary>::Prototype,
    ),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum TypedModIntPrototype {
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
    I256ObjFromBeBytes(<Bytes as SorobanArbitrary>::Prototype),
    I256ObjToBeBytes(<Val as SorobanArbitrary>::Prototype),
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
    ObjToI64(<i64 as SorobanArbitrary>::Prototype),
    ObjToI128Hi64(<i128 as SorobanArbitrary>::Prototype),
    ObjToI128Lo64(<i128 as SorobanArbitrary>::Prototype),
    ObjToI256HiHi(<Val as SorobanArbitrary>::Prototype),
    ObjToI256HiLo(<Val as SorobanArbitrary>::Prototype),
    ObjToI256LoHi(<Val as SorobanArbitrary>::Prototype),
    ObjToI256LoLo(<Val as SorobanArbitrary>::Prototype),
    ObjToU64(<u64 as SorobanArbitrary>::Prototype),
    ObjToU128Hi64(<u128 as SorobanArbitrary>::Prototype),
    ObjToU128Lo64(<u128 as SorobanArbitrary>::Prototype),
    ObjToU256HiHi(<Val as SorobanArbitrary>::Prototype),
    ObjToU256HiLo(<Val as SorobanArbitrary>::Prototype),
    ObjToU256LoHi(<Val as SorobanArbitrary>::Prototype),
    ObjToU256LoLo(<Val as SorobanArbitrary>::Prototype),
    TimepointObjFromU64(<u64 as SorobanArbitrary>::Prototype),
    TimepointObjToU64(<Val as SorobanArbitrary>::Prototype),
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
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum TypedModLedgerPrototype {
    BumpContractData(
        <Val as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    CreateAssetContract(<Bytes as SorobanArbitrary>::Prototype),
    CreateContract(
        <Address as SorobanArbitrary>::Prototype,
        <Bytes as SorobanArbitrary>::Prototype,
        <Bytes as SorobanArbitrary>::Prototype,
    ),
    DelContractData(<Val as SorobanArbitrary>::Prototype),
    GetAssetContractId(<Bytes as SorobanArbitrary>::Prototype),
    GetContractData(<Val as SorobanArbitrary>::Prototype),
    GetContractId(
        <Address as SorobanArbitrary>::Prototype,
        <Bytes as SorobanArbitrary>::Prototype,
    ),
    HasContractData(<Val as SorobanArbitrary>::Prototype),
    PutContractData(
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    UpdateCurrentContractWasm(<Bytes as SorobanArbitrary>::Prototype),
    UploadWasm(<Bytes as SorobanArbitrary>::Prototype),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum TypedModMapPrototype {
    MapDel(
        <Map<Val, Val> as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    MapGet(
        <Map<Val, Val> as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    MapHas(
        <Map<Val, Val> as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    MapKeys(<Map<Val, Val> as SorobanArbitrary>::Prototype),
    MapLen(<Map<Val, Val> as SorobanArbitrary>::Prototype),
    MapMaxKey(<Map<Val, Val> as SorobanArbitrary>::Prototype),
    MapMinKey(<Map<Val, Val> as SorobanArbitrary>::Prototype),
    MapNew,
    MapNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    MapNextKey(
        <Map<Val, Val> as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    MapPrevKey(
        <Map<Val, Val> as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    MapPut(
        <Map<Val, Val> as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    MapUnpackToLinearMemory(
        <Map<Val, Val> as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    MapValues(<Map<Val, Val> as SorobanArbitrary>::Prototype),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum TypedModPrngPrototype {
    PrngBytesNew(u32),
    PrngReseed(<Bytes as SorobanArbitrary>::Prototype),
    PrngU64InInclusiveRange(u64, u64),
    PrngVecShuffle(<Vec<Val> as SorobanArbitrary>::Prototype),
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum TypedModVecPrototype {
    VecAppend(
        <Vec<Val> as SorobanArbitrary>::Prototype,
        <Vec<Val> as SorobanArbitrary>::Prototype,
    ),
    VecBack(<Vec<Val> as SorobanArbitrary>::Prototype),
    VecBinarySearch(
        <Vec<Val> as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    VecDel(
        <Vec<Val> as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    VecFirstIndexOf(
        <Vec<Val> as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    VecFront(<Vec<Val> as SorobanArbitrary>::Prototype),
    VecGet(
        <Vec<Val> as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    VecInsert(
        <Vec<Val> as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    VecLastIndexOf(
        <Vec<Val> as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    VecLen(<Vec<Val> as SorobanArbitrary>::Prototype),
    VecNew(<Val as SorobanArbitrary>::Prototype),
    VecNewFromLinearMemory(
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    VecPopBack(<Vec<Val> as SorobanArbitrary>::Prototype),
    VecPopFront(<Vec<Val> as SorobanArbitrary>::Prototype),
    VecPushBack(
        <Vec<Val> as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    VecPushFront(
        <Vec<Val> as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    VecPut(
        <Vec<Val> as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <Val as SorobanArbitrary>::Prototype,
    ),
    VecSlice(
        <Vec<Val> as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
    VecUnpackToLinearMemory(
        <Vec<Val> as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
        <u32 as SorobanArbitrary>::Prototype,
    ),
}

impl TypedFuzzInstructionPrototype {
    fn to_guest(&self, env: &Env) -> TypedFuzzInstruction {
        match self {
            TypedFuzzInstructionPrototype::Address(v) => match v {
                TypedModAddressPrototype::AccountPublicKeyToAddress(v) => {
                    let v = Bytes::from_val(env, v);
                    TypedFuzzInstruction::Address(TypedModAddress::AccountPublicKeyToAddress(v))
                }
                TypedModAddressPrototype::AddressToAccountPublicKey(v) => {
                    let v = Address::from_val(env, v);
                    TypedFuzzInstruction::Address(TypedModAddress::AddressToAccountPublicKey(v))
                }
                TypedModAddressPrototype::AddressToContractId(v) => {
                    let v = Address::from_val(env, v);
                    TypedFuzzInstruction::Address(TypedModAddress::AddressToContractId(v))
                }
                TypedModAddressPrototype::AuthorizeAsCurrContract(v) => {
                    let v = Vec::<Val>::from_val(env, v);
                    TypedFuzzInstruction::Address(TypedModAddress::AuthorizeAsCurrContract(v))
                }
                TypedModAddressPrototype::ContractIdToAddress(v) => {
                    let v = Bytes::from_val(env, v);
                    TypedFuzzInstruction::Address(TypedModAddress::ContractIdToAddress(v))
                }
                TypedModAddressPrototype::RequireAuth(v) => {
                    let v = Address::from_val(env, v);
                    TypedFuzzInstruction::Address(TypedModAddress::RequireAuth(v))
                }
                TypedModAddressPrototype::RequireAuthForArgs(v_0, v_1) => {
                    let v_0 = Address::from_val(env, v_0);
                    let v_1 = Vec::<Val>::from_val(env, v_1);
                    TypedFuzzInstruction::Address(TypedModAddress::RequireAuthForArgs(v_0, v_1))
                }
            },
            TypedFuzzInstructionPrototype::Buf(v) => match v {
                TypedModBufPrototype::BytesAppend(v_0, v_1) => TypedFuzzInstruction::Buf(
                    TypedModBuf::BytesAppend(v_0.into_val(env), v_1.into_val(env)),
                ),
                TypedModBufPrototype::BytesBack(v) => {
                    TypedFuzzInstruction::Buf(TypedModBuf::BytesBack(v.into_val(env)))
                }
                TypedModBufPrototype::BytesCopyFromLinearMemory(v_0, v_1, v_2, v_3) => {
                    let v_0 = Bytes::from_val(env, v_0);
                    TypedFuzzInstruction::Buf(TypedModBuf::BytesCopyFromLinearMemory(
                        v_0, *v_1, *v_2, *v_3,
                    ))
                }
                TypedModBufPrototype::BytesCopyToLinearMemory(v_0, v_1, v_2, v_3) => {
                    let v_0 = Bytes::from_val(env, v_0);
                    TypedFuzzInstruction::Buf(TypedModBuf::BytesCopyToLinearMemory(
                        v_0, *v_1, *v_2, *v_3,
                    ))
                }
                TypedModBufPrototype::BytesDel(v_0, v_1) => {
                    let v_0 = Bytes::from_val(env, v_0);
                    TypedFuzzInstruction::Buf(TypedModBuf::BytesDel(v_0, *v_1))
                }
                TypedModBufPrototype::BytesFront(v) => {
                    let v = Bytes::from_val(env, v);
                    TypedFuzzInstruction::Buf(TypedModBuf::BytesFront(v))
                }
                TypedModBufPrototype::BytesGet(v_0, v_1) => {
                    let v_0 = Bytes::from_val(env, v_0);
                    TypedFuzzInstruction::Buf(TypedModBuf::BytesGet(v_0, *v_1))
                }
                TypedModBufPrototype::BytesInsert(v_0, v_1, v_2) => {
                    let v_0 = Bytes::from_val(env, v_0);
                    TypedFuzzInstruction::Buf(TypedModBuf::BytesInsert(v_0, *v_1, *v_2))
                }
                TypedModBufPrototype::BytesLen(v) => {
                    let v = Bytes::from_val(env, v);
                    TypedFuzzInstruction::Buf(TypedModBuf::BytesLen(v))
                }
                TypedModBufPrototype::BytesNew => TypedFuzzInstruction::Buf(TypedModBuf::BytesNew),
                TypedModBufPrototype::BytesNewFromLinearMemory(v_0, v_1) => {
                    TypedFuzzInstruction::Buf(TypedModBuf::BytesNewFromLinearMemory(*v_0, *v_1))
                }
                TypedModBufPrototype::BytesPop(v) => {
                    let v = Bytes::from_val(env, v);
                    TypedFuzzInstruction::Buf(TypedModBuf::BytesPop(v))
                }
                TypedModBufPrototype::BytesPush(v_0, v_1) => {
                    let v_0 = Bytes::from_val(env, v_0);
                    TypedFuzzInstruction::Buf(TypedModBuf::BytesPush(v_0, *v_1))
                }
                TypedModBufPrototype::BytesPut(v_0, v_1, v_2) => {
                    let v_0 = Bytes::from_val(env, v_0);
                    TypedFuzzInstruction::Buf(TypedModBuf::BytesPut(v_0, *v_1, *v_2))
                }
                TypedModBufPrototype::BytesSlice(v_0, v_1, v_2) => {
                    let v_0 = Bytes::from_val(env, v_0);
                    TypedFuzzInstruction::Buf(TypedModBuf::BytesSlice(v_0, *v_1, *v_2))
                }
                TypedModBufPrototype::DeserializeFromBytes(v) => {
                    let v = Bytes::from_val(env, v);
                    TypedFuzzInstruction::Buf(TypedModBuf::DeserializeFromBytes(v))
                }
                TypedModBufPrototype::SerializeToBytes(v) => {
                    let v = Val::from_val(env, v);
                    TypedFuzzInstruction::Buf(TypedModBuf::SerializeToBytes(FakeVal(
                        v.get_payload(),
                    )))
                }
                TypedModBufPrototype::StringCopyToLinearMemory(v_0, v_1, v_2, v_3) => {
                    let v_0 = String::from_val(env, v_0);
                    TypedFuzzInstruction::Buf(TypedModBuf::StringCopyToLinearMemory(
                        v_0, *v_1, *v_2, *v_3,
                    ))
                }
                TypedModBufPrototype::StringLen(v) => {
                    let v = String::from_val(env, v);
                    TypedFuzzInstruction::Buf(TypedModBuf::StringLen(v))
                }
                TypedModBufPrototype::StringNewFromLinearMemory(v_0, v_1) => {
                    TypedFuzzInstruction::Buf(TypedModBuf::StringNewFromLinearMemory(*v_0, *v_1))
                }
                TypedModBufPrototype::SymbolCopyToLinearMemory(v_0, v_1, v_2, v_3) => {
                    let v_0 = Symbol::from_val(env, v_0);
                    TypedFuzzInstruction::Buf(TypedModBuf::SymbolCopyToLinearMemory(
                        v_0, *v_1, *v_2, *v_3,
                    ))
                }
                TypedModBufPrototype::SymbolIndexInLinearMemory(v_0, v_1, v_2) => {
                    let v_0 = Symbol::from_val(env, v_0);
                    TypedFuzzInstruction::Buf(TypedModBuf::SymbolIndexInLinearMemory(
                        v_0, *v_1, *v_2,
                    ))
                }
                TypedModBufPrototype::SymbolLen(v) => {
                    let v = Symbol::from_val(env, v);
                    TypedFuzzInstruction::Buf(TypedModBuf::SymbolLen(v))
                }
                TypedModBufPrototype::SymbolNewFromLinearMemory(v_0, v_1) => {
                    TypedFuzzInstruction::Buf(TypedModBuf::SymbolNewFromLinearMemory(*v_0, *v_1))
                }
            },
            TypedFuzzInstructionPrototype::Call(v) => match v {
                TypedModCallPrototype::Call(v_0, v_1, v_2) => {
                    let v_0 = Address::from_val(env, v_0);
                    let v_1 = Symbol::from_val(env, v_1);
                    let v_2 = Vec::<Val>::from_val(env, v_2);
                    TypedFuzzInstruction::Call(TypedModCall::Call(v_0, v_1, v_2))
                }
                TypedModCallPrototype::TryCall(v_0, v_1, v_2) => {
                    let v_0 = Address::from_val(env, v_0);
                    let v_1 = Symbol::from_val(env, v_1);
                    let v_2 = Vec::<Val>::from_val(env, v_2);
                    TypedFuzzInstruction::Call(TypedModCall::TryCall(v_0, v_1, v_2))
                }
            },
            TypedFuzzInstructionPrototype::Context(v) => match v {
                TypedModContextPrototype::ContractEvent(v_0, v_1) => {
                    let v_0 = Vec::<Val>::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    TypedFuzzInstruction::Context(TypedModContext::ContractEvent(
                        v_0,
                        FakeVal(v_1.get_payload()),
                    ))
                }
                TypedModContextPrototype::FailWithError(v) => {
                    let v = Val::from_val(env, v);
                    TypedFuzzInstruction::Context(TypedModContext::FailWithError(FakeVal(
                        v.get_payload(),
                    )))
                }
                TypedModContextPrototype::GetCurrentCallStack => {
                    TypedFuzzInstruction::Context(TypedModContext::GetCurrentCallStack)
                }
                TypedModContextPrototype::GetCurrentContractAddress => {
                    TypedFuzzInstruction::Context(TypedModContext::GetCurrentContractAddress)
                }
                TypedModContextPrototype::GetInvokingContract => {
                    TypedFuzzInstruction::Context(TypedModContext::GetInvokingContract)
                }
                TypedModContextPrototype::GetLedgerNetworkId => {
                    TypedFuzzInstruction::Context(TypedModContext::GetLedgerNetworkId)
                }
                TypedModContextPrototype::GetLedgerSequence => {
                    TypedFuzzInstruction::Context(TypedModContext::GetLedgerSequence)
                }
                TypedModContextPrototype::GetLedgerTimestamp => {
                    TypedFuzzInstruction::Context(TypedModContext::GetLedgerTimestamp)
                }
                TypedModContextPrototype::GetLedgerVersion => {
                    TypedFuzzInstruction::Context(TypedModContext::GetLedgerVersion)
                }
                TypedModContextPrototype::LogFromLinearMemory(v_0, v_1, v_2, v_3) => {
                    TypedFuzzInstruction::Context(TypedModContext::LogFromLinearMemory(
                        *v_0, *v_1, *v_2, *v_3,
                    ))
                }
                TypedModContextPrototype::ObjCmp(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    TypedFuzzInstruction::Context(TypedModContext::ObjCmp(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
            },
            TypedFuzzInstructionPrototype::Crypto(v) => match v {
                TypedModCryptoPrototype::ComputeHashKeccak256(v) => {
                    let v = Bytes::from_val(env, v);
                    TypedFuzzInstruction::Crypto(TypedModCrypto::ComputeHashKeccak256(v))
                }
                TypedModCryptoPrototype::ComputeHashSha256(v) => {
                    let v = Bytes::from_val(env, v);
                    TypedFuzzInstruction::Crypto(TypedModCrypto::ComputeHashSha256(v))
                }
                TypedModCryptoPrototype::RecoverKeyEcdsaSecp256k1(v_0, v_1, v_2) => {
                    let v_0 = Bytes::from_val(env, v_0);
                    let v_1 = Bytes::from_val(env, v_1);
                    TypedFuzzInstruction::Crypto(TypedModCrypto::RecoverKeyEcdsaSecp256k1(
                        v_0, v_1, *v_2,
                    ))
                }
                TypedModCryptoPrototype::VerifySigEd25519(v_0, v_1, v_2) => {
                    let v_0 = Bytes::from_val(env, v_0);
                    let v_1 = Bytes::from_val(env, v_1);
                    let v_2 = Bytes::from_val(env, v_2);
                    TypedFuzzInstruction::Crypto(TypedModCrypto::VerifySigEd25519(v_0, v_1, v_2))
                }
            },
            TypedFuzzInstructionPrototype::Int(v) => match v {
                TypedModIntPrototype::DurationObjFromU64(v) => {
                    TypedFuzzInstruction::Int(TypedModInt::DurationObjFromU64(*v))
                }
                TypedModIntPrototype::DurationObjToU64(v) => {
                    let v = Val::from_val(env, v);
                    TypedFuzzInstruction::Int(TypedModInt::DurationObjToU64(FakeVal(
                        v.get_payload(),
                    )))
                }
                TypedModIntPrototype::I256Add(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    TypedFuzzInstruction::Int(TypedModInt::I256Add(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                TypedModIntPrototype::I256Div(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    TypedFuzzInstruction::Int(TypedModInt::I256Div(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                TypedModIntPrototype::I256Mul(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    TypedFuzzInstruction::Int(TypedModInt::I256Mul(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                TypedModIntPrototype::I256ObjFromBeBytes(v) => {
                    let v = Bytes::from_val(env, v);
                    TypedFuzzInstruction::Int(TypedModInt::I256ObjFromBeBytes(v))
                }
                TypedModIntPrototype::I256ObjToBeBytes(v) => {
                    let v = Val::from_val(env, v);
                    TypedFuzzInstruction::Int(TypedModInt::I256ObjToBeBytes(FakeVal(
                        v.get_payload(),
                    )))
                }
                TypedModIntPrototype::I256Pow(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    TypedFuzzInstruction::Int(TypedModInt::I256Pow(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                    ))
                }
                TypedModIntPrototype::I256Shl(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    TypedFuzzInstruction::Int(TypedModInt::I256Shl(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                    ))
                }
                TypedModIntPrototype::I256Shr(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    TypedFuzzInstruction::Int(TypedModInt::I256Shr(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                    ))
                }
                TypedModIntPrototype::I256Sub(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    TypedFuzzInstruction::Int(TypedModInt::I256Sub(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
                TypedModIntPrototype::ObjFromI64(v) => {
                    TypedFuzzInstruction::Int(TypedModInt::ObjFromI64(*v))
                }
                TypedModIntPrototype::ObjFromI128Pieces(v_0, v_1) => {
                    TypedFuzzInstruction::Int(TypedModInt::ObjFromI128Pieces(*v_0, *v_1))
                }
                TypedModIntPrototype::ObjFromI256Pieces(v_0, v_1, v_2, v_3) => {
                    TypedFuzzInstruction::Int(TypedModInt::ObjFromI256Pieces(
                        *v_0, *v_1, *v_2, *v_3,
                    ))
                }
                TypedModIntPrototype::ObjFromU64(v) => {
                    TypedFuzzInstruction::Int(TypedModInt::ObjFromU64(*v))
                }
                TypedModIntPrototype::ObjFromU128Pieces(v_0, v_1) => {
                    TypedFuzzInstruction::Int(TypedModInt::ObjFromU128Pieces(*v_0, *v_1))
                }
                TypedModIntPrototype::ObjFromU256Pieces(v_0, v_1, v_2, v_3) => {
                    TypedFuzzInstruction::Int(TypedModInt::ObjFromU256Pieces(
                        *v_0, *v_1, *v_2, *v_3,
                    ))
                }
                TypedModIntPrototype::ObjToI64(v) => {
                    TypedFuzzInstruction::Int(TypedModInt::ObjToI64(*v))
                }
                TypedModIntPrototype::ObjToI128Hi64(v) => {
                    TypedFuzzInstruction::Int(TypedModInt::ObjToI128Hi64(*v))
                }
                TypedModIntPrototype::ObjToI128Lo64(v) => {
                    TypedFuzzInstruction::Int(TypedModInt::ObjToI128Lo64(*v))
                }
                TypedModIntPrototype::ObjToI256HiHi(v) => {
                    let v = Val::from_val(env, v);
                    TypedFuzzInstruction::Int(TypedModInt::ObjToI256HiHi(FakeVal(v.get_payload())))
                }
                TypedModIntPrototype::ObjToI256HiLo(v) => {
                    let v = Val::from_val(env, v);
                    TypedFuzzInstruction::Int(TypedModInt::ObjToI256HiLo(FakeVal(v.get_payload())))
                }
                TypedModIntPrototype::ObjToI256LoHi(v) => {
                    let v = Val::from_val(env, v);
                    TypedFuzzInstruction::Int(TypedModInt::ObjToI256LoHi(FakeVal(v.get_payload())))
                }
                TypedModIntPrototype::ObjToI256LoLo(v) => {
                    let v = Val::from_val(env, v);
                    TypedFuzzInstruction::Int(TypedModInt::ObjToI256LoLo(FakeVal(v.get_payload())))
                }
                TypedModIntPrototype::ObjToU64(v) => {
                    TypedFuzzInstruction::Int(TypedModInt::ObjToU64(*v))
                }
                TypedModIntPrototype::ObjToU128Hi64(v) => {
                    TypedFuzzInstruction::Int(TypedModInt::ObjToU128Hi64(*v))
                }
                TypedModIntPrototype::ObjToU128Lo64(v) => {
                    TypedFuzzInstruction::Int(TypedModInt::ObjToU128Lo64(*v))
                }
                TypedModIntPrototype::ObjToU256HiHi(v) => {
                    let v = Val::from_val(env, v);
                    TypedFuzzInstruction::Int(TypedModInt::ObjToU256HiHi(FakeVal(v.get_payload())))
                }
                TypedModIntPrototype::ObjToU256HiLo(v) => {
                    let v = Val::from_val(env, v);
                    TypedFuzzInstruction::Int(TypedModInt::ObjToU256HiLo(FakeVal(v.get_payload())))
                }
                TypedModIntPrototype::ObjToU256LoHi(v) => {
                    let v = Val::from_val(env, v);
                    TypedFuzzInstruction::Int(TypedModInt::ObjToU256LoHi(FakeVal(v.get_payload())))
                }
                TypedModIntPrototype::ObjToU256LoLo(v) => {
                    let v = Val::from_val(env, v);
                    TypedFuzzInstruction::Int(TypedModInt::ObjToU256LoLo(FakeVal(v.get_payload())))
                }
                TypedModIntPrototype::TimepointObjFromU64(v) => {
                    TypedFuzzInstruction::Int(TypedModInt::TimepointObjFromU64(*v))
                }
                TypedModIntPrototype::TimepointObjToU64(v) => {
                    let v = Val::from_val(env, v);
                    TypedFuzzInstruction::Int(TypedModInt::TimepointObjToU64(FakeVal(
                        v.get_payload(),
                    )))
                }
                TypedModIntPrototype::U256Pow(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    TypedFuzzInstruction::Int(TypedModInt::U256Pow(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                    ))
                }
                TypedModIntPrototype::U256Shl(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    TypedFuzzInstruction::Int(TypedModInt::U256Shl(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                    ))
                }
                TypedModIntPrototype::U256Shr(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    TypedFuzzInstruction::Int(TypedModInt::U256Shr(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                    ))
                }
                TypedModIntPrototype::U256Sub(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    TypedFuzzInstruction::Int(TypedModInt::U256Sub(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                    ))
                }
            },
            TypedFuzzInstructionPrototype::Ledger(v) => match v {
                TypedModLedgerPrototype::BumpContractData(v_0, v_1) => {
                    let v_0 = Val::from_val(env, v_0);
                    TypedFuzzInstruction::Ledger(TypedModLedger::BumpContractData(
                        FakeVal(v_0.get_payload()),
                        *v_1,
                    ))
                }
                TypedModLedgerPrototype::CreateAssetContract(v) => {
                    let v = Bytes::from_val(env, v);
                    TypedFuzzInstruction::Ledger(TypedModLedger::CreateAssetContract(v))
                }
                TypedModLedgerPrototype::CreateContract(v_0, v_1, v_2) => {
                    let v_0 = Address::from_val(env, v_0);
                    let v_1 = Bytes::from_val(env, v_1);
                    let v_2 = Bytes::from_val(env, v_2);
                    TypedFuzzInstruction::Ledger(TypedModLedger::CreateContract(v_0, v_1, v_2))
                }
                TypedModLedgerPrototype::DelContractData(v) => {
                    let v = Val::from_val(env, v);
                    TypedFuzzInstruction::Ledger(TypedModLedger::DelContractData(FakeVal(
                        v.get_payload(),
                    )))
                }
                TypedModLedgerPrototype::GetAssetContractId(v) => {
                    let v = Bytes::from_val(env, v);
                    TypedFuzzInstruction::Ledger(TypedModLedger::GetAssetContractId(v))
                }
                TypedModLedgerPrototype::GetContractData(v) => {
                    let v = Val::from_val(env, v);
                    TypedFuzzInstruction::Ledger(TypedModLedger::GetContractData(FakeVal(
                        v.get_payload(),
                    )))
                }
                TypedModLedgerPrototype::GetContractId(v_0, v_1) => {
                    let v_0 = Address::from_val(env, v_0);
                    let v_1 = Bytes::from_val(env, v_1);
                    TypedFuzzInstruction::Ledger(TypedModLedger::GetContractId(v_0, v_1))
                }
                TypedModLedgerPrototype::HasContractData(v) => {
                    let v = Val::from_val(env, v);
                    TypedFuzzInstruction::Ledger(TypedModLedger::HasContractData(FakeVal(
                        v.get_payload(),
                    )))
                }
                TypedModLedgerPrototype::PutContractData(v_0, v_1, v_2) => {
                    let v_0 = Val::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    let v_2 = Val::from_val(env, v_2);
                    TypedFuzzInstruction::Ledger(TypedModLedger::PutContractData(
                        FakeVal(v_0.get_payload()),
                        FakeVal(v_1.get_payload()),
                        FakeVal(v_2.get_payload()),
                    ))
                }
                TypedModLedgerPrototype::UpdateCurrentContractWasm(v) => {
                    let v = Bytes::from_val(env, v);
                    TypedFuzzInstruction::Ledger(TypedModLedger::UpdateCurrentContractWasm(v))
                }
                TypedModLedgerPrototype::UploadWasm(v) => {
                    let v = Bytes::from_val(env, v);
                    TypedFuzzInstruction::Ledger(TypedModLedger::UploadWasm(v))
                }
            },
            TypedFuzzInstructionPrototype::Map(v) => match v {
                TypedModMapPrototype::MapDel(v_0, v_1) => {
                    let v_0 = Map::<Val, Val>::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    TypedFuzzInstruction::Map(TypedModMap::MapDel(v_0, FakeVal(v_1.get_payload())))
                }
                TypedModMapPrototype::MapGet(v_0, v_1) => {
                    let v_0 = Map::<Val, Val>::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    TypedFuzzInstruction::Map(TypedModMap::MapGet(v_0, FakeVal(v_1.get_payload())))
                }
                TypedModMapPrototype::MapHas(v_0, v_1) => {
                    let v_0 = Map::<Val, Val>::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    TypedFuzzInstruction::Map(TypedModMap::MapHas(v_0, FakeVal(v_1.get_payload())))
                }
                TypedModMapPrototype::MapKeys(v) => {
                    let v = Map::<Val, Val>::from_val(env, v);
                    TypedFuzzInstruction::Map(TypedModMap::MapKeys(v))
                }
                TypedModMapPrototype::MapLen(v) => {
                    let v = Map::<Val, Val>::from_val(env, v);
                    TypedFuzzInstruction::Map(TypedModMap::MapLen(v))
                }
                TypedModMapPrototype::MapMaxKey(v) => {
                    let v = Map::<Val, Val>::from_val(env, v);
                    TypedFuzzInstruction::Map(TypedModMap::MapMaxKey(v))
                }
                TypedModMapPrototype::MapMinKey(v) => {
                    let v = Map::<Val, Val>::from_val(env, v);
                    TypedFuzzInstruction::Map(TypedModMap::MapMinKey(v))
                }
                TypedModMapPrototype::MapNew => TypedFuzzInstruction::Map(TypedModMap::MapNew),
                TypedModMapPrototype::MapNewFromLinearMemory(v_0, v_1, v_2) => {
                    TypedFuzzInstruction::Map(TypedModMap::MapNewFromLinearMemory(*v_0, *v_1, *v_2))
                }
                TypedModMapPrototype::MapNextKey(v_0, v_1) => {
                    let v_0 = Map::<Val, Val>::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    TypedFuzzInstruction::Map(TypedModMap::MapNextKey(
                        v_0,
                        FakeVal(v_1.get_payload()),
                    ))
                }
                TypedModMapPrototype::MapPrevKey(v_0, v_1) => {
                    let v_0 = Map::<Val, Val>::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    TypedFuzzInstruction::Map(TypedModMap::MapPrevKey(
                        v_0,
                        FakeVal(v_1.get_payload()),
                    ))
                }
                TypedModMapPrototype::MapPut(v_0, v_1, v_2) => {
                    let v_0 = Map::<Val, Val>::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    let v_2 = Val::from_val(env, v_2);
                    TypedFuzzInstruction::Map(TypedModMap::MapPut(
                        v_0,
                        FakeVal(v_1.get_payload()),
                        FakeVal(v_2.get_payload()),
                    ))
                }
                TypedModMapPrototype::MapUnpackToLinearMemory(v_0, v_1, v_2, v_3) => {
                    let v_0 = Map::<Val, Val>::from_val(env, v_0);
                    TypedFuzzInstruction::Map(TypedModMap::MapUnpackToLinearMemory(
                        v_0, *v_1, *v_2, *v_3,
                    ))
                }
                TypedModMapPrototype::MapValues(v) => {
                    let v = Map::<Val, Val>::from_val(env, v);
                    TypedFuzzInstruction::Map(TypedModMap::MapValues(v))
                }
            },
            TypedFuzzInstructionPrototype::Prng(v) => match v {
                TypedModPrngPrototype::PrngBytesNew(v) => {
                    TypedFuzzInstruction::Prng(TypedModPrng::PrngBytesNew(*v))
                }
                TypedModPrngPrototype::PrngReseed(v) => {
                    let v = Bytes::from_val(env, v);
                    TypedFuzzInstruction::Prng(TypedModPrng::PrngReseed(v))
                }
                TypedModPrngPrototype::PrngU64InInclusiveRange(v_0, v_1) => {
                    TypedFuzzInstruction::Prng(TypedModPrng::PrngU64InInclusiveRange(*v_0, *v_1))
                }
                TypedModPrngPrototype::PrngVecShuffle(v) => {
                    let v = Vec::<Val>::from_val(env, v);
                    TypedFuzzInstruction::Prng(TypedModPrng::PrngVecShuffle(v))
                }
            },
            TypedFuzzInstructionPrototype::Test => TypedFuzzInstruction::Test,
            TypedFuzzInstructionPrototype::Vec(v) => match v {
                TypedModVecPrototype::VecAppend(v_0, v_1) => {
                    let v_0 = Vec::<Val>::from_val(env, v_0);
                    let v_1 = Vec::<Val>::from_val(env, v_1);
                    TypedFuzzInstruction::Vec(TypedModVec::VecAppend(v_0, v_1))
                }
                TypedModVecPrototype::VecBack(v) => {
                    let v = Vec::<Val>::from_val(env, v);
                    TypedFuzzInstruction::Vec(TypedModVec::VecBack(v))
                }
                TypedModVecPrototype::VecBinarySearch(v_0, v_1) => {
                    let v_0 = Vec::<Val>::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    TypedFuzzInstruction::Vec(TypedModVec::VecBinarySearch(
                        v_0,
                        FakeVal(v_1.get_payload()),
                    ))
                }
                TypedModVecPrototype::VecDel(v_0, v_1) => {
                    let v_0 = Vec::<Val>::from_val(env, v_0);
                    TypedFuzzInstruction::Vec(TypedModVec::VecDel(v_0, *v_1))
                }
                TypedModVecPrototype::VecFirstIndexOf(v_0, v_1) => {
                    let v_0 = Vec::<Val>::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    TypedFuzzInstruction::Vec(TypedModVec::VecFirstIndexOf(
                        v_0,
                        FakeVal(v_1.get_payload()),
                    ))
                }
                TypedModVecPrototype::VecFront(v) => {
                    let v = Vec::<Val>::from_val(env, v);
                    TypedFuzzInstruction::Vec(TypedModVec::VecFront(v))
                }
                TypedModVecPrototype::VecGet(v_0, v_1) => {
                    let v_0 = Vec::<Val>::from_val(env, v_0);
                    TypedFuzzInstruction::Vec(TypedModVec::VecGet(v_0, *v_1))
                }
                TypedModVecPrototype::VecInsert(v_0, v_1, v_2) => {
                    let v_0 = Vec::<Val>::from_val(env, v_0);
                    let v_2 = Val::from_val(env, v_2);
                    TypedFuzzInstruction::Vec(TypedModVec::VecInsert(
                        v_0,
                        *v_1,
                        FakeVal(v_2.get_payload()),
                    ))
                }
                TypedModVecPrototype::VecLastIndexOf(v_0, v_1) => {
                    let v_0 = Vec::<Val>::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    TypedFuzzInstruction::Vec(TypedModVec::VecLastIndexOf(
                        v_0,
                        FakeVal(v_1.get_payload()),
                    ))
                }
                TypedModVecPrototype::VecLen(v) => {
                    let v = Vec::<Val>::from_val(env, v);
                    TypedFuzzInstruction::Vec(TypedModVec::VecLen(v))
                }
                TypedModVecPrototype::VecNew(v) => {
                    let v = Val::from_val(env, v);
                    TypedFuzzInstruction::Vec(TypedModVec::VecNew(FakeVal(v.get_payload())))
                }
                TypedModVecPrototype::VecNewFromLinearMemory(v_0, v_1) => {
                    TypedFuzzInstruction::Vec(TypedModVec::VecNewFromLinearMemory(*v_0, *v_1))
                }
                TypedModVecPrototype::VecPopBack(v) => {
                    let v = Vec::<Val>::from_val(env, v);
                    TypedFuzzInstruction::Vec(TypedModVec::VecPopBack(v))
                }
                TypedModVecPrototype::VecPopFront(v) => {
                    let v = Vec::<Val>::from_val(env, v);
                    TypedFuzzInstruction::Vec(TypedModVec::VecPopFront(v))
                }
                TypedModVecPrototype::VecPushBack(v_0, v_1) => {
                    let v_0 = Vec::<Val>::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    TypedFuzzInstruction::Vec(TypedModVec::VecPushBack(
                        v_0,
                        FakeVal(v_1.get_payload()),
                    ))
                }
                TypedModVecPrototype::VecPushFront(v_0, v_1) => {
                    let v_0 = Vec::<Val>::from_val(env, v_0);
                    let v_1 = Val::from_val(env, v_1);
                    TypedFuzzInstruction::Vec(TypedModVec::VecPushFront(
                        v_0,
                        FakeVal(v_1.get_payload()),
                    ))
                }
                TypedModVecPrototype::VecPut(v_0, v_1, v_2) => {
                    let v_0 = Vec::<Val>::from_val(env, v_0);
                    let v_2 = Val::from_val(env, v_2);
                    TypedFuzzInstruction::Vec(TypedModVec::VecPut(
                        v_0,
                        *v_1,
                        FakeVal(v_2.get_payload()),
                    ))
                }
                TypedModVecPrototype::VecSlice(v_0, v_1, v_2) => {
                    let v_0 = Vec::<Val>::from_val(env, v_0);
                    TypedFuzzInstruction::Vec(TypedModVec::VecSlice(v_0, *v_1, *v_2))
                }
                TypedModVecPrototype::VecUnpackToLinearMemory(v_0, v_1, v_2) => {
                    let v_0 = Vec::<Val>::from_val(env, v_0);
                    TypedFuzzInstruction::Vec(TypedModVec::VecUnpackToLinearMemory(v_0, *v_1, *v_2))
                }
            },
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
        if !env.logs().all().is_empty() {
            env.logs().print();
        }
        panic!("host panicked: {panic_r:?}");
    }
});
