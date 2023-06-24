use crate::{syscalls, FakeVal};
use core::mem;
use soroban_env_common::{
    BytesObject, DurationObject, I128Object, I256Object, I64Object, StorageType, SymbolObject,
    TimepointObject, U128Object, U256Object, U32Val, U64Object,
};
use soroban_sdk::contracttype;
use soroban_sdk::{Address, Bytes, Env, Map, String, Symbol, TryFromVal, Val, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub enum TypedFuzzInstruction {
    Address(TypedModAddress),
    Buf(TypedModBuf),
    Call(TypedModCall),
    Context(TypedModContext),
    Crypto(TypedModCrypto),
    Int(TypedModInt),
    Ledger(TypedModLedger),
    Map(TypedModMap),
    Prng(TypedModPrng),
    Test,
    Vec(TypedModVec),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum TypedModAddress {
    AccountPublicKeyToAddress(Bytes),
    AddressToAccountPublicKey(Address),
    AddressToContractId(Address),
    AuthorizeAsCurrContract(Vec<Val>),
    ContractIdToAddress(Bytes),
    RequireAuth(Address),
    RequireAuthForArgs(Address, Vec<Val>),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum TypedModBuf {
    BytesAppend(Bytes, Bytes),
    BytesBack(Bytes),
    BytesCopyFromLinearMemory(Bytes, u32, u32, u32),
    BytesCopyToLinearMemory(Bytes, u32, u32, u32),
    BytesDel(Bytes, u32),
    BytesFront(Bytes),
    BytesGet(Bytes, u32),
    BytesInsert(Bytes, u32, u32),
    BytesLen(Bytes),
    BytesNew,
    BytesNewFromLinearMemory(u32, u32),
    BytesPop(Bytes),
    BytesPush(Bytes, u32),
    BytesPut(Bytes, u32, u32),
    BytesSlice(Bytes, u32, u32),
    DeserializeFromBytes(Bytes),
    SerializeToBytes(FakeVal),
    StringCopyToLinearMemory(String, u32, u32, u32),
    StringLen(String),
    StringNewFromLinearMemory(u32, u32),
    SymbolCopyToLinearMemory(Symbol, u32, u32, u32),
    SymbolIndexInLinearMemory(Symbol, u32, u32),
    SymbolLen(Symbol),
    SymbolNewFromLinearMemory(u32, u32),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum TypedModCall {
    Call(Address, Symbol, Vec<Val>),
    TryCall(Address, Symbol, Vec<Val>),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum TypedModContext {
    ContractEvent(Vec<Val>, FakeVal),
    FailWithError(FakeVal), // soroban_env_common::Error
    GetCurrentCallStack,
    GetCurrentContractAddress,
    GetCurrentContractId,
    GetInvokingContract,
    GetLedgerNetworkId,
    GetLedgerSequence,
    GetLedgerTimestamp,
    GetLedgerVersion,
    LogFromLinearMemory(u32, u32, u32, u32),
    ObjCmp(FakeVal, FakeVal),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum TypedModCrypto {
    ComputeHashKeccak256(Bytes),
    ComputeHashSha256(Bytes),
    RecoverKeyEcdsaSecp256k1(Bytes, Bytes, u32),
    VerifySigEd25519(Bytes, Bytes, Bytes),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum TypedModInt {
    DurationObjFromU64(u64),
    DurationObjToU64(FakeVal),
    I256Add(FakeVal, FakeVal),
    I256Div(FakeVal, FakeVal),
    I256Mul(FakeVal, FakeVal),
    I256ObjFromBeBytes(Bytes),
    I256ObjToBeBytes(FakeVal),
    I256Pow(FakeVal, u32),
    I256Shl(FakeVal, u32),
    I256Shr(FakeVal, u32),
    I256Sub(FakeVal, FakeVal),
    ObjFromI64(i64),
    ObjFromI128Pieces(i64, u64),
    ObjFromI256Pieces(i64, u64, u64, u64),
    ObjFromU64(u64),
    ObjFromU128Pieces(u64, u64),
    ObjFromU256Pieces(u64, u64, u64, u64),
    ObjToI64(i64),
    ObjToI128Hi64(i128),
    ObjToI128Lo64(i128),
    ObjToI256HiHi(FakeVal),
    ObjToI256HiLo(FakeVal),
    ObjToI256LoHi(FakeVal),
    ObjToI256LoLo(FakeVal),
    ObjToU64(u64),
    ObjToU128Hi64(u128),
    ObjToU128Lo64(u128),
    ObjToU256HiHi(FakeVal),
    ObjToU256HiLo(FakeVal),
    ObjToU256LoHi(FakeVal),
    ObjToU256LoLo(FakeVal),
    TimepointObjFromU64(u64),
    TimepointObjToU64(FakeVal),
    U256Pow(FakeVal, u32),
    U256Shl(FakeVal, u32),
    U256Shr(FakeVal, u32),
    U256Sub(FakeVal, FakeVal),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum TypedModLedger {
    CreateAssetContract(Bytes),
    CreateContract(Address, Bytes, Bytes),
    DelContractData(FakeVal),
    GetContractData(FakeVal),
    HasContractData(FakeVal),
    PutContractData(FakeVal, FakeVal, FakeVal),
    UpdateCurrentContractWasm(Bytes),
    UploadWasm(Bytes),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum TypedModMap {
    MapDel(Map<Val, Val>, FakeVal),
    MapGet(Map<Val, Val>, FakeVal),
    MapHas(Map<Val, Val>, FakeVal),
    MapKeys(Map<Val, Val>),
    MapLen(Map<Val, Val>),
    MapMaxKey(Map<Val, Val>),
    MapMinKey(Map<Val, Val>),
    MapNew,
    MapNewFromLinearMemory(u32, u32, u32),
    MapNextKey(Map<Val, Val>, FakeVal),
    MapPrevKey(Map<Val, Val>, FakeVal),
    MapPut(Map<Val, Val>, FakeVal, FakeVal),
    MapUnpackToLinearMemory(Map<Val, Val>, u32, u32, u32),
    MapValues(Map<Val, Val>),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum TypedModPrng {
    PrngBytesNew(u32),
    PrngReseed(Bytes),
    PrngU64InInclusiveRange(u64, u64),
    PrngVecShuffle(Vec<Val>),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum TypedModVec {
    VecAppend(Vec<Val>, Vec<Val>),
    VecBack(Vec<Val>),
    VecBinarySearch(Vec<Val>, FakeVal),
    VecDel(Vec<Val>, u32),
    VecFirstIndexOf(Vec<Val>, FakeVal),
    VecFront(Vec<Val>),
    VecGet(Vec<Val>, u32),
    VecInsert(Vec<Val>, u32, FakeVal),
    VecLastIndexOf(Vec<Val>, FakeVal),
    VecLen(Vec<Val>),
    VecNew(FakeVal),
    VecNewFromLinearMemory(u32, u32),
    VecPopBack(Vec<Val>),
    VecPopFront(Vec<Val>),
    VecPushBack(Vec<Val>, FakeVal),
    VecPushFront(Vec<Val>, FakeVal),
    VecPut(Vec<Val>, u32, FakeVal),
    VecSlice(Vec<Val>, u32, u32),
    VecUnpackToLinearMemory(Vec<Val>, u32, u32),
}

impl TypedFuzzInstruction {
    pub fn run(self, env: &Env) {
        let fuzz_instruction = self;
        use TypedFuzzInstruction::*;
        match fuzz_instruction {
            Address(v) => match v {
                TypedModAddress::AccountPublicKeyToAddress(v) => unsafe {
                    let v = BytesObject::from(v);
                    syscalls::address::account_public_key_to_address(v);
                },
                TypedModAddress::AddressToAccountPublicKey(v) => unsafe {
                    let v = v.to_object();
                    syscalls::address::address_to_account_public_key(v);
                },
                TypedModAddress::AddressToContractId(v) => unsafe {
                    let v = v.to_object();
                    syscalls::address::address_to_contract_id(v);
                },
                TypedModAddress::AuthorizeAsCurrContract(v) => unsafe {
                    let v = v.to_object();
                    syscalls::address::authorize_as_curr_contract(v);
                },
                TypedModAddress::ContractIdToAddress(v) => unsafe {
                    let v = BytesObject::from(v);
                    syscalls::address::contract_id_to_address(v);
                },
                TypedModAddress::RequireAuth(v) => unsafe {
                    let v = v.to_object();
                    syscalls::address::require_auth(v);
                },
                TypedModAddress::RequireAuthForArgs(v_0, v_1) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = v_1.to_object();
                    syscalls::address::require_auth_for_args(v_0, v_1);
                },
            },
            Buf(v) => match v {
                TypedModBuf::BytesAppend(v_0, v_1) => unsafe {
                    let v_0 = BytesObject::from(v_0);
                    let v_1 = BytesObject::from(v_1);
                    syscalls::buf::bytes_append(v_0, v_1);
                },
                TypedModBuf::BytesBack(v) => unsafe {
                    let v = BytesObject::from(v);
                    syscalls::buf::bytes_back(v);
                },
                TypedModBuf::BytesCopyFromLinearMemory(v_0, v_1, v_2, v_3) => unsafe {
                    let v_0 = BytesObject::from(v_0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    let v_3 = U32Val::from(v_3);

                    syscalls::buf::bytes_copy_from_linear_memory(v_0, v_1, v_2, v_3);
                },
                TypedModBuf::BytesCopyToLinearMemory(v_0, v_1, v_2, v_3) => unsafe {
                    let v_0 = BytesObject::from(v_0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    let v_3 = U32Val::from(v_3);

                    syscalls::buf::bytes_copy_to_linear_memory(v_0, v_1, v_2, v_3);
                },
                TypedModBuf::BytesDel(v_0, v_1) => unsafe {
                    let v_0 = BytesObject::from(v_0);
                    let v_1 = U32Val::from(v_1);

                    syscalls::buf::bytes_del(v_0, v_1);
                },
                TypedModBuf::BytesFront(v) => unsafe {
                    let v = BytesObject::from(v);
                    syscalls::buf::bytes_front(v);
                },
                TypedModBuf::BytesGet(v_0, v_1) => unsafe {
                    let v_0 = BytesObject::from(v_0);
                    let v_1 = U32Val::from(v_1);

                    syscalls::buf::bytes_get(v_0, v_1);
                },
                TypedModBuf::BytesInsert(v_0, v_1, v_2) => unsafe {
                    let v_0 = BytesObject::from(v_0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);

                    syscalls::buf::bytes_insert(v_0, v_1, v_2);
                },
                TypedModBuf::BytesLen(v) => unsafe {
                    let v = BytesObject::from(v);
                    syscalls::buf::bytes_len(v);
                },
                TypedModBuf::BytesNew => unsafe {
                    syscalls::buf::bytes_new();
                },
                TypedModBuf::BytesNewFromLinearMemory(v_0, v_1) => unsafe {
                    let v_0 = U32Val::from(v_0);
                    let v_1 = U32Val::from(v_1);

                    syscalls::buf::bytes_new_from_linear_memory(v_0, v_1);
                },
                TypedModBuf::BytesPop(v) => unsafe {
                    let v = BytesObject::from(v);
                    syscalls::buf::bytes_pop(v);
                },
                TypedModBuf::BytesPush(v_0, v_1) => unsafe {
                    let v_0 = BytesObject::from(v_0);
                    let v_1 = U32Val::from(v_1);

                    syscalls::buf::bytes_push(v_0, v_1);
                },
                TypedModBuf::BytesPut(v_0, v_1, v_2) => unsafe {
                    let v_0 = BytesObject::from(v_0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);

                    syscalls::buf::bytes_put(v_0, v_1, v_2);
                },
                TypedModBuf::BytesSlice(v_0, v_1, v_2) => unsafe {
                    let v_0 = BytesObject::from(v_0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);

                    syscalls::buf::bytes_slice(v_0, v_1, v_2);
                },
                TypedModBuf::DeserializeFromBytes(v) => unsafe {
                    let v = BytesObject::from(v);
                    syscalls::buf::deserialize_from_bytes(v);
                },
                TypedModBuf::SerializeToBytes(v) => unsafe {
                    let v = mem::transmute(v);
                    syscalls::buf::serialize_to_bytes(v);
                },
                TypedModBuf::StringCopyToLinearMemory(v_0, v_1, v_2, v_3) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    let v_3 = U32Val::from(v_3);

                    syscalls::buf::string_copy_to_linear_memory(v_0, v_1, v_2, v_3);
                },
                TypedModBuf::StringLen(v_0) => unsafe {
                    let v_0 = v_0.to_object();
                    syscalls::buf::string_len(v_0);
                },
                TypedModBuf::StringNewFromLinearMemory(v_0, v_1) => unsafe {
                    let v_0 = U32Val::from(v_0);
                    let v_1 = U32Val::from(v_1);

                    syscalls::buf::string_new_from_linear_memory(v_0, v_1);
                },
                TypedModBuf::SymbolCopyToLinearMemory(v_0, v_1, v_2, v_3) => unsafe {
                    let v_0 = v_0.to_val();
                    let v_0 = SymbolObject::try_from(v_0).unwrap();
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    let v_3 = U32Val::from(v_3);

                    syscalls::buf::symbol_copy_to_linear_memory(v_0, v_1, v_2, v_3);
                },
                TypedModBuf::SymbolIndexInLinearMemory(v_0, v_1, v_2) => unsafe {
                    let v_0 = v_0.to_symbol_val();
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    syscalls::buf::symbol_index_in_linear_memory(v_0, v_1, v_2);
                },
                TypedModBuf::SymbolLen(v) => unsafe {
                    let v = v.to_val();
                    let v = SymbolObject::try_from(v).unwrap();
                    syscalls::buf::symbol_len(v);
                },
                TypedModBuf::SymbolNewFromLinearMemory(v_0, v_1) => unsafe {
                    let v_0 = U32Val::from(v_0);
                    let v_1 = U32Val::from(v_1);

                    syscalls::buf::symbol_new_from_linear_memory(v_0, v_1);
                },
            },
            Call(v) => match v {
                TypedModCall::Call(v_0, v_1, v_2) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = v_1.to_symbol_val();
                    let v_2 = v_2.to_object();
                    syscalls::call::call(v_0, v_1, v_2);
                },
                TypedModCall::TryCall(v_0, v_1, v_2) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = v_1.to_symbol_val();
                    let v_2 = v_2.to_object();
                    syscalls::call::try_call(v_0, v_1, v_2);
                },
            },
            Context(v) => match v {
                TypedModContext::ContractEvent(v_0, v_1) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = mem::transmute(v_1.0);

                    syscalls::context::contract_event(v_0, v_1);
                },
                TypedModContext::FailWithError(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::context::fail_with_error(v);
                },
                TypedModContext::GetCurrentCallStack => unsafe {
                    syscalls::context::get_current_call_stack();
                },
                TypedModContext::GetCurrentContractAddress => unsafe {
                    syscalls::context::get_current_contract_address();
                },
                TypedModContext::GetCurrentContractId => unsafe {
                    //                    syscalls::context::get_current_contract_id();
                },
                TypedModContext::GetInvokingContract => unsafe {
                    syscalls::context::get_invoking_contract();
                },
                TypedModContext::GetLedgerNetworkId => unsafe {
                    syscalls::context::get_ledger_network_id();
                },
                TypedModContext::GetLedgerSequence => unsafe {
                    syscalls::context::get_ledger_sequence();
                },
                TypedModContext::GetLedgerTimestamp => unsafe {
                    syscalls::context::get_ledger_timestamp();
                },
                TypedModContext::GetLedgerVersion => unsafe {
                    syscalls::context::get_ledger_version();
                },
                TypedModContext::LogFromLinearMemory(v_0, v_1, v_2, v_3) => unsafe {
                    let v_0 = U32Val::from(v_0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    let v_3 = U32Val::from(v_3);
                    syscalls::context::log_from_linear_memory(v_0, v_1, v_2, v_3);
                },
                TypedModContext::ObjCmp(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::context::obj_cmp(v_0, v_1);
                },
            },
            Crypto(v) => match v {
                TypedModCrypto::ComputeHashKeccak256(v) => unsafe {
                    let v = BytesObject::from(v);
                    syscalls::crypto::compute_hash_keccak256(v);
                },
                TypedModCrypto::ComputeHashSha256(v) => unsafe {
                    let v = BytesObject::from(v);
                    syscalls::crypto::compute_hash_sha256(v);
                },
                TypedModCrypto::RecoverKeyEcdsaSecp256k1(v_0, v_1, v_2) => unsafe {
                    let v_0 = BytesObject::from(v_0);
                    let v_1 = BytesObject::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    syscalls::crypto::recover_key_ecdsa_secp256k1(v_0, v_1, v_2);
                },
                TypedModCrypto::VerifySigEd25519(v_0, v_1, v_2) => unsafe {
                    let v_0 = BytesObject::from(v_0);
                    let v_1 = BytesObject::from(v_1);
                    let v_2 = BytesObject::from(v_2);
                    syscalls::crypto::verify_sig_ed25519(v_0, v_1, v_2);
                },
            },
            Int(v) => match v {
                TypedModInt::DurationObjFromU64(v) => unsafe {
                    syscalls::int::duration_obj_from_u64(v);
                },
                TypedModInt::DurationObjToU64(v) => unsafe {
                    let v = Val::try_from_val(env, &v).unwrap();
                    let v = DurationObject::try_from(&v).unwrap();
                    syscalls::int::duration_obj_to_u64(v);
                },
                TypedModInt::I256Add(v_0, v_1) => unsafe {
                    let v_0 = Val::try_from_val(env, &v_0).unwrap();
                    let v_0 = I256Object::try_from(&v_0).unwrap();
                    let v_1 = Val::try_from_val(env, &v_1).unwrap();
                    let v_1 = I256Object::try_from(&v_1).unwrap();
                    syscalls::int::i256_add(v_0, v_1);
                },
                TypedModInt::I256Div(v_0, v_1) => unsafe {
                    let v_0 = Val::try_from_val(env, &v_0).unwrap();
                    let v_0 = I256Object::try_from(&v_0).unwrap();
                    let v_1 = Val::try_from_val(env, &v_1).unwrap();
                    let v_1 = I256Object::try_from(&v_1).unwrap();
                    syscalls::int::i256_div(v_0, v_1);
                },
                TypedModInt::I256Mul(v_0, v_1) => unsafe {
                    let v_0 = Val::try_from_val(env, &v_0).unwrap();
                    let v_0 = I256Object::try_from(&v_0).unwrap();
                    let v_1 = Val::try_from_val(env, &v_1).unwrap();
                    let v_1 = I256Object::try_from(&v_1).unwrap();
                    syscalls::int::i256_mul(v_0, v_1);
                },
                TypedModInt::I256ObjFromBeBytes(v) => unsafe {
                    let v = BytesObject::from(v);
                    syscalls::int::i256_obj_from_be_bytes(v);
                },
                TypedModInt::I256ObjToBeBytes(v) => unsafe {
                    let v = Val::try_from_val(env, &v).unwrap();
                    let v = I256Object::try_from(&v).unwrap();
                    syscalls::int::i256_obj_to_be_bytes(v);
                },
                TypedModInt::I256Pow(v_0, v_1) => unsafe {
                    let v_0 = Val::try_from_val(env, &v_0).unwrap();
                    let v_0 = I256Object::try_from(&v_0).unwrap();
                    let v_1 = U32Val::from(v_1);
                    syscalls::int::i256_pow(v_0, v_1);
                },
                TypedModInt::I256Shl(v_0, v_1) => unsafe {
                    let v_0 = Val::try_from_val(env, &v_0).unwrap();
                    let v_0 = I256Object::try_from(&v_0).unwrap();
                    let v_1 = U32Val::from(v_1);
                    syscalls::int::i256_shl(v_0, v_1);
                },
                TypedModInt::I256Shr(v_0, v_1) => unsafe {
                    let v_0 = Val::try_from_val(env, &v_0).unwrap();
                    let v_0 = I256Object::try_from(&v_0).unwrap();
                    let v_1 = U32Val::from(v_1);
                    syscalls::int::i256_shr(v_0, v_1);
                },
                TypedModInt::I256Sub(v_0, v_1) => unsafe {
                    let v_0 = Val::try_from_val(env, &v_0).unwrap();
                    let v_0 = I256Object::try_from(&v_0).unwrap();
                    let v_1 = Val::try_from_val(env, &v_1).unwrap();
                    let v_1 = I256Object::try_from(&v_1).unwrap();
                    syscalls::int::i256_sub(v_0, v_1);
                },
                TypedModInt::ObjFromI64(v) => unsafe {
                    syscalls::int::obj_from_i64(v);
                },
                TypedModInt::ObjFromI128Pieces(v_0, v_1) => unsafe {
                    syscalls::int::obj_from_i128_pieces(v_0, v_1);
                },
                TypedModInt::ObjFromI256Pieces(v_0, v_1, v_2, v_3) => unsafe {
                    syscalls::int::obj_from_i256_pieces(v_0, v_1, v_2, v_3);
                },
                TypedModInt::ObjFromU64(v) => unsafe {
                    syscalls::int::obj_from_u64(v);
                },
                TypedModInt::ObjFromU128Pieces(v_0, v_1) => unsafe {
                    syscalls::int::obj_from_u128_pieces(v_0, v_1);
                },
                TypedModInt::ObjFromU256Pieces(v_0, v_1, v_2, v_3) => unsafe {
                    syscalls::int::obj_from_u256_pieces(v_0, v_1, v_2, v_3);
                },
                TypedModInt::ObjToI64(v) => unsafe {
                    let v = Val::try_from_val(env, &v).unwrap();
                    let v = I64Object::try_from(&v).unwrap();
                    syscalls::int::obj_to_i64(v);
                },
                TypedModInt::ObjToI128Hi64(v) => unsafe {
                    let v = Val::try_from_val(env, &v).unwrap();
                    let v = I128Object::try_from(&v).unwrap();
                    syscalls::int::obj_to_i128_hi64(v);
                },
                TypedModInt::ObjToI128Lo64(v) => unsafe {
                    let v = Val::try_from_val(env, &v).unwrap();
                    let v = I128Object::try_from(&v).unwrap();
                    syscalls::int::obj_to_i128_lo64(v);
                },
                TypedModInt::ObjToI256HiHi(v) => unsafe {
                    let v = Val::try_from_val(env, &v).unwrap();
                    let v = I256Object::try_from(&v).unwrap();
                    syscalls::int::obj_to_i256_hi_hi(v);
                },
                TypedModInt::ObjToI256HiLo(v) => unsafe {
                    let v = Val::try_from_val(env, &v).unwrap();
                    let v = I256Object::try_from(&v).unwrap();
                    syscalls::int::obj_to_i256_hi_lo(v);
                },
                TypedModInt::ObjToI256LoHi(v) => unsafe {
                    let v = Val::try_from_val(env, &v).unwrap();
                    let v = I256Object::try_from(&v).unwrap();
                    syscalls::int::obj_to_i256_lo_hi(v);
                },
                TypedModInt::ObjToI256LoLo(v) => unsafe {
                    let v = Val::try_from_val(env, &v).unwrap();
                    let v = I256Object::try_from(&v).unwrap();
                    syscalls::int::obj_to_i256_lo_lo(v);
                },
                TypedModInt::ObjToU64(v) => unsafe {
                    let v = Val::try_from_val(env, &v).unwrap();
                    let v = U64Object::try_from(&v).unwrap();
                    syscalls::int::obj_to_u64(v);
                },
                TypedModInt::ObjToU128Hi64(v) => unsafe {
                    let v = Val::try_from_val(env, &v).unwrap();
                    let v = U128Object::try_from(&v).unwrap();
                    syscalls::int::obj_to_u128_hi64(v);
                },
                TypedModInt::ObjToU128Lo64(v) => unsafe {
                    let v = Val::try_from_val(env, &v).unwrap();
                    let v = U128Object::try_from(&v).unwrap();
                    syscalls::int::obj_to_u128_lo64(v);
                },
                TypedModInt::ObjToU256HiHi(v) => unsafe {
                    let v = Val::try_from_val(env, &v).unwrap();
                    let v = U256Object::try_from(&v).unwrap();
                    syscalls::int::obj_to_u256_hi_hi(v);
                },
                TypedModInt::ObjToU256HiLo(v) => unsafe {
                    let v = Val::try_from_val(env, &v).unwrap();
                    let v = U256Object::try_from(&v).unwrap();
                    syscalls::int::obj_to_u256_hi_lo(v);
                },
                TypedModInt::ObjToU256LoHi(v) => unsafe {
                    let v = Val::try_from_val(env, &v).unwrap();
                    let v = U256Object::try_from(&v).unwrap();
                    syscalls::int::obj_to_u256_lo_hi(v);
                },
                TypedModInt::ObjToU256LoLo(v) => unsafe {
                    let v = Val::try_from_val(env, &v).unwrap();
                    let v = U256Object::try_from(&v).unwrap();
                    syscalls::int::obj_to_u256_lo_lo(v);
                },
                TypedModInt::TimepointObjFromU64(v) => unsafe {
                    syscalls::int::timepoint_obj_from_u64(v);
                },
                TypedModInt::TimepointObjToU64(v) => unsafe {
                    let v = Val::try_from_val(env, &v).unwrap();
                    let v = TimepointObject::try_from(&v).unwrap();
                    syscalls::int::timepoint_obj_to_u64(v);
                },
                TypedModInt::U256Pow(v_0, v_1) => unsafe {
                    let v_0 = Val::try_from_val(env, &v_0).unwrap();
                    let v_0 = U256Object::try_from(&v_0).unwrap();
                    let v_1 = U32Val::from(v_1);
                    syscalls::int::u256_pow(v_0, v_1);
                },
                TypedModInt::U256Shl(v_0, v_1) => unsafe {
                    let v_0 = Val::try_from_val(env, &v_0).unwrap();
                    let v_0 = U256Object::try_from(&v_0).unwrap();
                    let v_1 = U32Val::from(v_1);
                    syscalls::int::u256_shl(v_0, v_1);
                },
                TypedModInt::U256Shr(v_0, v_1) => unsafe {
                    let v_0 = Val::try_from_val(env, &v_0).unwrap();
                    let v_0 = U256Object::try_from(&v_0).unwrap();
                    let v_1 = U32Val::from(v_1);
                    syscalls::int::u256_shr(v_0, v_1);
                },
                TypedModInt::U256Sub(v_0, v_1) => unsafe {
                    let v_0 = Val::try_from_val(env, &v_0).unwrap();
                    let v_0 = U256Object::try_from(&v_0).unwrap();
                    let v_1 = Val::try_from_val(env, &v_1).unwrap();
                    let v_1 = U256Object::try_from(&v_1).unwrap();
                    syscalls::int::u256_sub(v_0, v_1);
                },
            },
            Ledger(v) => match v {
                TypedModLedger::CreateAssetContract(v) => unsafe {
                    let v = v.to_object();
                    syscalls::ledger::create_asset_contract(v);
                },
                TypedModLedger::CreateContract(v_0, v_1, v_2) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = BytesObject::from(v_1);
                    let v_2 = BytesObject::from(v_2);
                    syscalls::ledger::create_contract(v_0, v_1, v_2);
                },
                TypedModLedger::DelContractData(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::ledger::del_contract_data(v, StorageType::Temporary);
                },
                TypedModLedger::GetContractData(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::ledger::get_contract_data(v, StorageType::Temporary);
                },
                TypedModLedger::HasContractData(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::ledger::has_contract_data(v, StorageType::Temporary);
                },
                TypedModLedger::PutContractData(v_0, v_1, v_2) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    let v_2 = mem::transmute(v_2.0);
                    syscalls::ledger::put_contract_data(v_0, v_1, StorageType::Temporary, v_2);
                },
                TypedModLedger::UpdateCurrentContractWasm(v) => unsafe {
                    let v = BytesObject::from(v);
                    syscalls::ledger::update_current_contract_wasm(v);
                },
                TypedModLedger::UploadWasm(v) => unsafe {
                    let v = BytesObject::from(v);
                    syscalls::ledger::upload_wasm(v);
                },
            },
            Map(v) => match v {
                TypedModMap::MapDel(v_0, v_1) => unsafe {
                    // todo: private method
                    let v_0 = v_0.to_object();
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::map::map_del(v_0, v_1);
                },
                TypedModMap::MapGet(v_0, v_1) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::map::map_get(v_0, v_1);
                },
                TypedModMap::MapHas(v_0, v_1) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::map::map_has(v_0, v_1);
                },
                TypedModMap::MapKeys(v) => unsafe {
                    let v = v.to_object();
                    syscalls::map::map_keys(v);
                },
                TypedModMap::MapLen(v) => unsafe {
                    let v = v.to_object();
                    syscalls::map::map_len(v);
                },
                TypedModMap::MapMaxKey(v) => unsafe {
                    let v = v.to_object();
                    syscalls::map::map_max_key(v);
                },
                TypedModMap::MapMinKey(v) => unsafe {
                    let v = v.to_object();
                    syscalls::map::map_min_key(v);
                },
                TypedModMap::MapNew => unsafe {
                    syscalls::map::map_new();
                },
                TypedModMap::MapNewFromLinearMemory(v_0, v_1, v_2) => unsafe {
                    let v_0 = U32Val::from(v_0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    syscalls::map::map_new_from_linear_memory(v_0, v_1, v_2);
                },
                TypedModMap::MapNextKey(v_0, v_1) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::map::map_next_key(v_0, v_1);
                },
                TypedModMap::MapPrevKey(v_0, v_1) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::map::map_prev_key(v_0, v_1);
                },
                TypedModMap::MapPut(v_0, v_1, v_2) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = mem::transmute(v_1.0);
                    let v_2 = mem::transmute(v_2.0);
                    syscalls::map::map_put(v_0, v_1, v_2);
                },
                TypedModMap::MapUnpackToLinearMemory(v_0, v_1, v_2, v_3) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    let v_3 = U32Val::from(v_3);

                    syscalls::map::map_unpack_to_linear_memory(v_0, v_1, v_2, v_3);
                },
                TypedModMap::MapValues(v) => unsafe {
                    let v = v.to_object();
                    syscalls::map::map_values(v);
                },
            },
            Prng(v) => match v {
                TypedModPrng::PrngBytesNew(v) => unsafe {
                    let v = U32Val::from(v);
                    syscalls::prng::prng_bytes_new(v);
                },
                TypedModPrng::PrngReseed(v) => unsafe {
                    let v = v.to_object();
                    syscalls::prng::prng_reseed(v);
                },
                TypedModPrng::PrngU64InInclusiveRange(v_0, v_1) => unsafe {
                    syscalls::prng::prng_u64_in_inclusive_range(v_0, v_1);
                },
                TypedModPrng::PrngVecShuffle(v) => unsafe {
                    let v = v.to_object();
                    syscalls::prng::prng_vec_shuffle(v);
                },
            },
            Test => unsafe {
                syscalls::test::dummy0();
            },
            Vec(v) => match v {
                TypedModVec::VecAppend(v_0, v_1) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = v_1.to_object();
                    syscalls::vec::vec_append(v_0, v_1);
                },
                TypedModVec::VecBack(v) => unsafe {
                    let v = v.to_object();
                    syscalls::vec::vec_back(v);
                },
                TypedModVec::VecBinarySearch(v_0, v_1) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::vec::vec_binary_search(v_0, v_1);
                },
                TypedModVec::VecDel(v_0, v_1) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = U32Val::from(v_1);
                    syscalls::vec::vec_del(v_0, v_1);
                },
                TypedModVec::VecFirstIndexOf(v_0, v_1) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::vec::vec_first_index_of(v_0, v_1);
                },
                TypedModVec::VecFront(v) => unsafe {
                    let v = v.to_object();
                    syscalls::vec::vec_front(v);
                },
                TypedModVec::VecGet(v_0, v_1) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = U32Val::from(v_1);
                    syscalls::vec::vec_get(v_0, v_1);
                },
                TypedModVec::VecInsert(v_0, v_1, v_2) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = U32Val::from(v_1);
                    let v_2 = mem::transmute(v_2.0);
                    syscalls::vec::vec_insert(v_0, v_1, v_2);
                },
                TypedModVec::VecLastIndexOf(v_0, v_1) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::vec::vec_last_index_of(v_0, v_1);
                },
                TypedModVec::VecLen(v) => unsafe {
                    let v = v.to_object();
                    syscalls::vec::vec_len(v);
                },
                TypedModVec::VecNew(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::vec::vec_new(v);
                },
                TypedModVec::VecNewFromLinearMemory(v_0, v_1) => unsafe {
                    let v_0 = U32Val::from(v_0);
                    let v_1 = U32Val::from(v_1);
                    syscalls::vec::vec_new_from_linear_memory(v_0, v_1);
                },
                TypedModVec::VecPopBack(v) => unsafe {
                    let v = v.to_object();
                    syscalls::vec::vec_pop_back(v);
                },
                TypedModVec::VecPopFront(v) => unsafe {
                    let v = v.to_object();
                    syscalls::vec::vec_pop_front(v);
                },
                TypedModVec::VecPushBack(v_0, v_1) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::vec::vec_push_back(v_0, v_1);
                },
                TypedModVec::VecPushFront(v_0, v_1) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::vec::vec_push_front(v_0, v_1);
                },
                TypedModVec::VecPut(v_0, v_1, v_2) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = U32Val::from(v_1);
                    let v_2 = mem::transmute(v_2.0);
                    syscalls::vec::vec_put(v_0, v_1, v_2);
                },
                TypedModVec::VecSlice(v_0, v_1, v_2) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    syscalls::vec::vec_slice(v_0, v_1, v_2);
                },
                TypedModVec::VecUnpackToLinearMemory(v_0, v_1, v_2) => unsafe {
                    let v_0 = v_0.to_object();
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    syscalls::vec::vec_unpack_to_linear_memory(v_0, v_1, v_2);
                },
            },
        }
    }
}
