use crate::{syscalls, FakeVal};
use core::mem;
use soroban_env_common::{StorageType, U32Val};
use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug)]
pub enum RawFuzzInstruction {
    Address(RawModAddress),
    Buf(RawModBuf),
    Call(RawModCall),
    Context(RawModContext),
    Crypto(RawModCrypto),
    Int(RawModInt),
    Ledger(RawModLedger),
    Map(RawModMap),
    Prng(RawModPrng),
    Test,
    Vec(RawModVec),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum RawModAddress {
    AccountPublicKeyToAddress(FakeVal),
    AddressToAccountPublicKey(FakeVal),
    AddressToContractId(FakeVal),
    AuthorizeAsCurrContract(FakeVal),
    ContractIdToAddress(FakeVal),
    RequireAuth(FakeVal),
    RequireAuthForArgs(FakeVal, FakeVal),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum RawModBuf {
    BytesAppend(FakeVal, FakeVal),
    BytesBack(FakeVal),
    BytesCopyFromLinearMemory(FakeVal, u32, u32, u32),
    BytesCopyToLinearMemory(FakeVal, u32, u32, u32),
    BytesDel(FakeVal, u32),
    BytesFront(FakeVal),
    BytesGet(FakeVal, u32),
    BytesInsert(FakeVal, u32, u32),
    BytesLen(FakeVal),
    BytesNew,
    BytesNewFromLinearMemory(u32, u32),
    BytesPop(FakeVal),
    BytesPush(FakeVal, u32),
    BytesPut(FakeVal, u32, u32),
    BytesSlice(FakeVal, u32, u32),
    DeserializeFromBytes(FakeVal),
    SerializeToBytes(FakeVal),
    StringCopyToLinearMemory(FakeVal, u32, u32, u32),
    StringLen(FakeVal),
    StringNewFromLinearMemory(u32, u32),
    SymbolCopyToLinearMemory(FakeVal, u32, u32, u32),
    SymbolIndexInLinearMemory(FakeVal, u32, u32),
    SymbolLen(FakeVal),
    SymbolNewFromLinearMemory(u32, u32),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum RawModCall {
    Call(FakeVal, FakeVal, FakeVal),
    TryCall(FakeVal, FakeVal, FakeVal),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum RawModContext {
    ContractEvent(FakeVal, FakeVal),
    FailWithError(FakeVal),
    GetCurrentCallStack,
    GetCurrentContractAddress,
    GetInvokingContract,
    GetLedgerNetworkId,
    GetLedgerSequence,
    GetLedgerTimestamp,
    GetLedgerVersion,
    GetMaxExpirationLedger,
    LogFromLinearMemory(u32, u32, u32, u32),
    ObjCmp(FakeVal, FakeVal),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum RawModCrypto {
    ComputeHashKeccak256(FakeVal),
    ComputeHashSha256(FakeVal),
    RecoverKeyEcdsaSecp256k1(FakeVal, FakeVal, FakeVal),
    VerifySigEd25519(FakeVal, FakeVal, FakeVal),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum RawModInt {
    DurationObjFromU64(u64),
    DurationObjToU64(FakeVal),
    I256Add(FakeVal, FakeVal),
    I256Div(FakeVal, FakeVal),
    I256Mul(FakeVal, FakeVal),
    I256Pow(FakeVal, u32),
    I256Shl(FakeVal, u32),
    I256Shr(FakeVal, u32),
    I256Sub(FakeVal, FakeVal),
    I256ObjFromBeBytes(FakeVal),
    I256ObjToBeBytes(FakeVal),
    ObjFromI64(i64),
    ObjFromI128Pieces(i64, u64),
    ObjFromI256Pieces(i64, u64, u64, u64),
    ObjFromU64(u64),
    ObjFromU128Pieces(u64, u64),
    ObjFromU256Pieces(u64, u64, u64, u64),
    ObjToI64(FakeVal),
    ObjToI128Hi64(FakeVal),
    ObjToI128Lo64(FakeVal),
    ObjToI256HiHi(FakeVal),
    ObjToI256HiLo(FakeVal),
    ObjToI256LoHi(FakeVal),
    ObjToI256LoLo(FakeVal),
    ObjToU64(FakeVal),
    ObjToU128Hi64(FakeVal),
    ObjToU128Lo64(FakeVal),
    ObjToU256HiHi(FakeVal),
    ObjToU256HiLo(FakeVal),
    ObjToU256LoHi(FakeVal),
    ObjToU256LoLo(FakeVal),
    TimepointObjFromU64(u64),
    TimepointObjToU64(FakeVal),
    U256Add(FakeVal, FakeVal),
    U256Div(FakeVal, FakeVal),
    U256Mul(FakeVal, FakeVal),
    U256Pow(FakeVal, u32),
    U256Shl(FakeVal, u32),
    U256Shr(FakeVal, u32),
    U256Sub(FakeVal, FakeVal),
    U256ValFromBeBytes(FakeVal),
    U256ValToBeBytes(FakeVal),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum RawModLedger {
    BumpContractData(FakeVal, u32, u32),
    BumpContractInstanceAndCode(FakeVal, u32, u32),
    BumpCurrentContract(u32, u32), // BumpCurrentContractInstanceAndCode
    CreateAssetContract(FakeVal),
    CreateContract(FakeVal, FakeVal, FakeVal),
    DelContractData(FakeVal),
    GetAssetContractId(FakeVal),
    GetContractData(FakeVal),
    GetContractId(FakeVal, FakeVal),
    HasContractData(FakeVal),
    PutContractData(FakeVal, FakeVal),
    UpdateCurrentContractWasm(FakeVal),
    UploadWasm(FakeVal),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum RawModMap {
    MapDel(FakeVal, FakeVal),
    MapGet(FakeVal, FakeVal),
    MapHas(FakeVal, FakeVal),
    MapKeyByPos(FakeVal, u32),
    MapKeys(FakeVal),
    MapLen(FakeVal),
    MapNew,
    MapNewFromLinearMemory(u32, u32, u32),
    MapPut(FakeVal, FakeVal, FakeVal),
    MapUnpackToLinearMemory(FakeVal, u32, u32, u32),
    MapValByPos(FakeVal, u32),
    MapValues(FakeVal),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum RawModPrng {
    PrngBytesNew(u32),
    PrngReseed(FakeVal),
    PrngU64InInclusiveRange(u64, u64),
    PrngVecShuffle(FakeVal),
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum RawModVec {
    VecAppend(FakeVal, FakeVal),
    VecBack(FakeVal),
    VecBinarySearch(FakeVal, FakeVal),
    VecDel(FakeVal, u32),
    VecFirstIndexOf(FakeVal, FakeVal),
    VecFront(FakeVal),
    VecGet(FakeVal, u32),
    VecInsert(FakeVal, u32, FakeVal),
    VecLastIndexOf(FakeVal, FakeVal),
    VecLen(FakeVal),
    VecNew,
    VecNewFromLinearMemory(u32, u32),
    VecPopBack(FakeVal),
    VecPopFront(FakeVal),
    VecPushBack(FakeVal, FakeVal),
    VecPushFront(FakeVal, FakeVal),
    VecPut(FakeVal, u32, FakeVal),
    VecSlice(FakeVal, u32, u32),
    VecUnpackToLinearMemory(FakeVal, u32, u32),
}

impl RawFuzzInstruction {
    pub fn run(self) {
        let fuzz_instruction = self;
        use RawFuzzInstruction::*;
        match fuzz_instruction {
            Address(v) => match v {
                RawModAddress::AccountPublicKeyToAddress(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::address::account_public_key_to_address(v);
                },
                RawModAddress::AddressToAccountPublicKey(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::address::address_to_account_public_key(v);
                },
                RawModAddress::AddressToContractId(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::address::address_to_contract_id(v);
                },
                RawModAddress::AuthorizeAsCurrContract(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::address::authorize_as_curr_contract(v);
                },
                RawModAddress::ContractIdToAddress(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::address::contract_id_to_address(v);
                },
                RawModAddress::RequireAuth(v_0) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    syscalls::address::require_auth(v_0);
                },
                RawModAddress::RequireAuthForArgs(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::address::require_auth_for_args(v_0, v_1);
                },
            },
            Buf(v) => match v {
                RawModBuf::BytesAppend(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::buf::bytes_append(v_0, v_1);
                },
                RawModBuf::BytesBack(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::buf::bytes_back(v);
                },
                RawModBuf::BytesCopyFromLinearMemory(v_0, v_1, v_2, v_3) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    let v_3 = U32Val::from(v_3);

                    syscalls::buf::bytes_copy_from_linear_memory(v_0, v_1, v_2, v_3);
                },
                RawModBuf::BytesCopyToLinearMemory(v_0, v_1, v_2, v_3) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    let v_3 = U32Val::from(v_3);

                    syscalls::buf::bytes_copy_to_linear_memory(v_0, v_1, v_2, v_3);
                },
                RawModBuf::BytesDel(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);

                    syscalls::buf::bytes_del(v_0, v_1);
                },
                RawModBuf::BytesFront(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::buf::bytes_front(v);
                },
                RawModBuf::BytesGet(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);

                    syscalls::buf::bytes_get(v_0, v_1);
                },
                RawModBuf::BytesInsert(v_0, v_1, v_2) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);

                    syscalls::buf::bytes_insert(v_0, v_1, v_2);
                },
                RawModBuf::BytesLen(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::buf::bytes_len(v);
                },
                RawModBuf::BytesNew => unsafe {
                    syscalls::buf::bytes_new();
                },
                RawModBuf::BytesNewFromLinearMemory(v_0, v_1) => unsafe {
                    let v_0 = U32Val::from(v_0);
                    let v_1 = U32Val::from(v_1);

                    syscalls::buf::bytes_new_from_linear_memory(v_0, v_1);
                },
                RawModBuf::BytesPop(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::buf::bytes_pop(v);
                },
                RawModBuf::BytesPush(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);

                    syscalls::buf::bytes_push(v_0, v_1);
                },
                RawModBuf::BytesPut(v_0, v_1, v_2) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);

                    syscalls::buf::bytes_put(v_0, v_1, v_2);
                },
                RawModBuf::BytesSlice(v_0, v_1, v_2) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);

                    syscalls::buf::bytes_slice(v_0, v_1, v_2);
                },
                RawModBuf::DeserializeFromBytes(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::buf::deserialize_from_bytes(v);
                },
                RawModBuf::SerializeToBytes(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::buf::serialize_to_bytes(v);
                },
                RawModBuf::StringCopyToLinearMemory(v_0, v_1, v_2, v_3) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    let v_3 = U32Val::from(v_3);

                    syscalls::buf::string_copy_to_linear_memory(v_0, v_1, v_2, v_3);
                },
                RawModBuf::StringLen(v_0) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    syscalls::buf::string_len(v_0);
                },
                RawModBuf::StringNewFromLinearMemory(v_0, v_1) => unsafe {
                    let v_0 = U32Val::from(v_0);
                    let v_1 = U32Val::from(v_1);

                    syscalls::buf::string_new_from_linear_memory(v_0, v_1);
                },
                RawModBuf::SymbolCopyToLinearMemory(v_0, v_1, v_2, v_3) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    let v_3 = U32Val::from(v_3);

                    syscalls::buf::symbol_copy_to_linear_memory(v_0, v_1, v_2, v_3);
                },
                RawModBuf::SymbolIndexInLinearMemory(v_0, v_1, v_2) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);

                    syscalls::buf::symbol_index_in_linear_memory(v_0, v_1, v_2);
                },
                RawModBuf::SymbolLen(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::buf::symbol_len(v);
                },
                RawModBuf::SymbolNewFromLinearMemory(v_0, v_1) => unsafe {
                    let v_0 = U32Val::from(v_0);
                    let v_1 = U32Val::from(v_1);

                    syscalls::buf::symbol_new_from_linear_memory(v_0, v_1);
                },
            },
            Call(v) => match v {
                RawModCall::Call(v_0, v_1, v_2) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    let v_2 = mem::transmute(v_2.0);

                    syscalls::call::call(v_0, v_1, v_2);
                },
                RawModCall::TryCall(v_0, v_1, v_2) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    let v_2 = mem::transmute(v_2.0);

                    syscalls::call::call(v_0, v_1, v_2);
                },
            },
            Context(v) => match v {
                RawModContext::ContractEvent(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);

                    syscalls::context::contract_event(v_0, v_1);
                },
                RawModContext::FailWithError(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::context::fail_with_error(v);
                },
                RawModContext::GetCurrentCallStack => unsafe {
                    syscalls::context::get_current_call_stack();
                },
                RawModContext::GetCurrentContractAddress => unsafe {
                    syscalls::context::get_current_contract_address();
                },
                RawModContext::GetInvokingContract => unsafe {
                    syscalls::context::get_invoking_contract();
                },
                RawModContext::GetLedgerNetworkId => unsafe {
                    syscalls::context::get_ledger_network_id();
                },
                RawModContext::GetLedgerSequence => unsafe {
                    syscalls::context::get_ledger_sequence();
                },
                RawModContext::GetLedgerTimestamp => unsafe {
                    syscalls::context::get_ledger_timestamp();
                },
                RawModContext::GetLedgerVersion => unsafe {
                    syscalls::context::get_ledger_version();
                },
                RawModContext::GetMaxExpirationLedger => unsafe {
                    syscalls::context::get_max_expiration_ledger();
                },
                RawModContext::LogFromLinearMemory(v_0, v_1, v_2, v_3) => unsafe {
                    let v_0 = U32Val::from(v_0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    let v_3 = U32Val::from(v_3);
                    syscalls::context::log_from_linear_memory(v_0, v_1, v_2, v_3);
                },
                RawModContext::ObjCmp(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::context::obj_cmp(v_0, v_1);
                },
            },
            Crypto(v) => match v {
                RawModCrypto::ComputeHashKeccak256(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::crypto::compute_hash_keccak256(v);
                },
                RawModCrypto::ComputeHashSha256(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::crypto::compute_hash_sha256(v);
                },
                RawModCrypto::RecoverKeyEcdsaSecp256k1(v_0, v_1, v_2) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    let v_2 = mem::transmute(v_2.0);
                    syscalls::crypto::recover_key_ecdsa_secp256k1(v_0, v_1, v_2);
                },
                RawModCrypto::VerifySigEd25519(v_0, v_1, v_2) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    let v_2 = mem::transmute(v_2.0);
                    syscalls::crypto::verify_sig_ed25519(v_0, v_1, v_2);
                },
            },
            Int(v) => match v {
                RawModInt::DurationObjFromU64(v) => unsafe {
                    syscalls::int::duration_obj_from_u64(v);
                },
                RawModInt::DurationObjToU64(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::duration_obj_to_u64(v);
                },
                RawModInt::I256Add(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::int::i256_add(v_0, v_1);
                },
                RawModInt::I256Div(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::int::i256_div(v_0, v_1);
                },
                RawModInt::I256Mul(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::int::i256_mul(v_0, v_1);
                },
                RawModInt::I256Pow(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    syscalls::int::i256_pow(v_0, v_1);
                },
                RawModInt::I256Shl(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    syscalls::int::i256_shl(v_0, v_1);
                },
                RawModInt::I256Shr(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    syscalls::int::i256_shr(v_0, v_1);
                },
                RawModInt::I256Sub(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::int::i256_sub(v_0, v_1);
                },
                RawModInt::I256ObjFromBeBytes(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::i256_val_from_be_bytes(v);
                },
                RawModInt::I256ObjToBeBytes(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::i256_val_to_be_bytes(v);
                },
                RawModInt::ObjFromI64(v) => unsafe {
                    syscalls::int::obj_from_i64(v);
                },
                RawModInt::ObjFromI128Pieces(v_0, v_1) => unsafe {
                    syscalls::int::obj_from_i128_pieces(v_0, v_1);
                },
                RawModInt::ObjFromI256Pieces(v_0, v_1, v_2, v_3) => unsafe {
                    syscalls::int::obj_from_i256_pieces(v_0, v_1, v_2, v_3);
                },
                RawModInt::ObjFromU64(v) => unsafe {
                    syscalls::int::obj_from_u64(v);
                },
                RawModInt::ObjFromU128Pieces(v_0, v_1) => unsafe {
                    syscalls::int::obj_from_u128_pieces(v_0, v_1);
                },
                RawModInt::ObjFromU256Pieces(v_0, v_1, v_2, v_3) => unsafe {
                    syscalls::int::obj_from_u256_pieces(v_0, v_1, v_2, v_3);
                },
                RawModInt::ObjToI64(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::obj_to_i64(v);
                },
                RawModInt::ObjToI128Hi64(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::obj_to_i128_hi64(v);
                },
                RawModInt::ObjToI128Lo64(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::obj_to_i128_lo64(v);
                },
                RawModInt::ObjToI256HiHi(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::obj_to_i256_hi_hi(v);
                },
                RawModInt::ObjToI256HiLo(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::obj_to_i256_hi_lo(v);
                },
                RawModInt::ObjToI256LoHi(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::obj_to_i256_lo_hi(v);
                },
                RawModInt::ObjToI256LoLo(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::obj_to_i256_lo_lo(v);
                },
                RawModInt::ObjToU64(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::obj_to_u64(v);
                },
                RawModInt::ObjToU128Hi64(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::obj_to_u128_hi64(v);
                },
                RawModInt::ObjToU128Lo64(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::obj_to_u128_lo64(v);
                },
                RawModInt::ObjToU256HiHi(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::obj_to_u256_hi_hi(v);
                },
                RawModInt::ObjToU256HiLo(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::obj_to_u256_hi_lo(v);
                },
                RawModInt::ObjToU256LoHi(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::obj_to_u256_lo_hi(v);
                },
                RawModInt::ObjToU256LoLo(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::obj_to_u256_lo_lo(v);
                },
                RawModInt::TimepointObjFromU64(v) => unsafe {
                    syscalls::int::timepoint_obj_from_u64(v);
                },
                RawModInt::TimepointObjToU64(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::timepoint_obj_to_u64(v);
                },
                RawModInt::U256Add(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::int::u256_add(v_0, v_1);
                },
                RawModInt::U256Div(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::int::u256_div(v_0, v_1);
                },
                RawModInt::U256Mul(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::int::u256_mul(v_0, v_1);
                },
                RawModInt::U256Pow(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    syscalls::int::u256_pow(v_0, v_1);
                },
                RawModInt::U256Shl(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    syscalls::int::u256_shl(v_0, v_1);
                },
                RawModInt::U256Shr(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    syscalls::int::u256_shr(v_0, v_1);
                },
                RawModInt::U256Sub(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::int::u256_sub(v_0, v_1);
                },
                RawModInt::U256ValFromBeBytes(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::u256_val_from_be_bytes(v);
                },
                RawModInt::U256ValToBeBytes(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::int::u256_val_to_be_bytes(v);
                },
            },
            Ledger(v) => match v {
                RawModLedger::BumpContractData(v_0, v_1, v_2) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    syscalls::ledger::bump_contract_data(v_0, StorageType::Temporary, v_1, v_2);
                },
                RawModLedger::BumpContractInstanceAndCode(v_0, v_1, v_2) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    syscalls::ledger::bump_contract_instance_and_code(v_0, v_1, v_2);
                },
                RawModLedger::BumpCurrentContract(v_0, v_1) => unsafe {
                    let v_0 = U32Val::from(v_0);
                    let v_1 = U32Val::from(v_1);
                    syscalls::ledger::bump_current_contract_instance_and_code(v_0, v_1);
                },
                RawModLedger::CreateAssetContract(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::ledger::create_asset_contract(v);
                },
                RawModLedger::CreateContract(v_0, v_1, v_2) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    let v_2 = mem::transmute(v_2.0);
                    syscalls::ledger::create_contract(v_0, v_1, v_2);
                },
                RawModLedger::DelContractData(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::ledger::del_contract_data(v, StorageType::Temporary);
                },
                RawModLedger::GetAssetContractId(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::ledger::get_asset_contract_id(v);
                },
                RawModLedger::GetContractData(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::ledger::get_contract_data(v, StorageType::Temporary);
                },
                RawModLedger::GetContractId(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::ledger::get_contract_id(v_0, v_1);
                },
                RawModLedger::HasContractData(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::ledger::has_contract_data(v, StorageType::Temporary);
                },
                RawModLedger::PutContractData(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::ledger::put_contract_data(v_0, v_1, StorageType::Temporary);
                },
                RawModLedger::UpdateCurrentContractWasm(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::ledger::update_current_contract_wasm(v);
                },
                RawModLedger::UploadWasm(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::ledger::upload_wasm(v);
                },
            },
            Map(v) => match v {
                RawModMap::MapDel(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::map::map_del(v_0, v_1);
                },
                RawModMap::MapGet(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::map::map_get(v_0, v_1);
                },
                RawModMap::MapHas(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::map::map_has(v_0, v_1);
                },
                RawModMap::MapKeyByPos(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    syscalls::map::map_key_by_pos(v_0, v_1);
                },
                RawModMap::MapKeys(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::map::map_keys(v);
                },
                RawModMap::MapLen(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::map::map_len(v);
                },
                RawModMap::MapNew => unsafe {
                    syscalls::map::map_new();
                },
                RawModMap::MapNewFromLinearMemory(v_0, v_1, v_2) => unsafe {
                    let v_0 = U32Val::from(v_0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    syscalls::map::map_new_from_linear_memory(v_0, v_1, v_2);
                },
                RawModMap::MapPut(v_0, v_1, v_2) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    let v_2 = mem::transmute(v_2.0);
                    syscalls::map::map_put(v_0, v_1, v_2);
                },
                RawModMap::MapUnpackToLinearMemory(v_0, v_1, v_2, v_3) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    let v_3 = U32Val::from(v_3);

                    syscalls::map::map_unpack_to_linear_memory(v_0, v_1, v_2, v_3);
                },
                RawModMap::MapValByPos(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    syscalls::map::map_val_by_pos(v_0, v_1);
                },
                RawModMap::MapValues(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::map::map_values(v);
                },
            },
            Prng(v) => match v {
                RawModPrng::PrngBytesNew(v) => unsafe {
                    let v = U32Val::from(v);
                    syscalls::prng::prng_bytes_new(v);
                },
                RawModPrng::PrngReseed(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::prng::prng_reseed(v);
                },
                RawModPrng::PrngU64InInclusiveRange(v_0, v_1) => unsafe {
                    syscalls::prng::prng_u64_in_inclusive_range(v_0, v_1);
                },
                RawModPrng::PrngVecShuffle(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::prng::prng_vec_shuffle(v);
                },
            },
            Test => unsafe {
                syscalls::test::dummy0();
            },
            Vec(v) => match v {
                RawModVec::VecAppend(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::vec::vec_append(v_0, v_1);
                },
                RawModVec::VecBack(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::vec::vec_back(v);
                },
                RawModVec::VecBinarySearch(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::vec::vec_binary_search(v_0, v_1);
                },
                RawModVec::VecDel(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    syscalls::vec::vec_del(v_0, v_1);
                },
                RawModVec::VecFirstIndexOf(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::vec::vec_first_index_of(v_0, v_1);
                },
                RawModVec::VecFront(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::vec::vec_front(v);
                },
                RawModVec::VecGet(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    syscalls::vec::vec_get(v_0, v_1);
                },
                RawModVec::VecInsert(v_0, v_1, v_2) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = mem::transmute(v_2.0);
                    syscalls::vec::vec_insert(v_0, v_1, v_2);
                },
                RawModVec::VecLastIndexOf(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::vec::vec_last_index_of(v_0, v_1);
                },
                RawModVec::VecLen(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::vec::vec_len(v);
                },
                RawModVec::VecNew => unsafe {
                    syscalls::vec::vec_new();
                },
                RawModVec::VecNewFromLinearMemory(v_0, v_1) => unsafe {
                    let v_0 = U32Val::from(v_0);
                    let v_1 = U32Val::from(v_1);
                    syscalls::vec::vec_new_from_linear_memory(v_0, v_1);
                },
                RawModVec::VecPopBack(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::vec::vec_pop_back(v);
                },
                RawModVec::VecPopFront(v) => unsafe {
                    let v = mem::transmute(v.0);
                    syscalls::vec::vec_pop_front(v);
                },
                RawModVec::VecPushBack(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::vec::vec_push_back(v_0, v_1);
                },
                RawModVec::VecPushFront(v_0, v_1) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = mem::transmute(v_1.0);
                    syscalls::vec::vec_push_front(v_0, v_1);
                },
                RawModVec::VecPut(v_0, v_1, v_2) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = mem::transmute(v_2.0);
                    syscalls::vec::vec_put(v_0, v_1, v_2);
                },
                RawModVec::VecSlice(v_0, v_1, v_2) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    syscalls::vec::vec_slice(v_0, v_1, v_2);
                },
                RawModVec::VecUnpackToLinearMemory(v_0, v_1, v_2) => unsafe {
                    let v_0 = mem::transmute(v_0.0);
                    let v_1 = U32Val::from(v_1);
                    let v_2 = U32Val::from(v_2);
                    syscalls::vec::vec_unpack_to_linear_memory(v_0, v_1, v_2);
                },
            },
        }
    }
}
