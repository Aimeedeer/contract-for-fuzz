#![no_std]

use soroban_sdk::{contractimpl, contracttype, Env, Val};

pub mod raw;
pub mod typed;

use raw::RawFuzzInstruction;
use typed::TypedFuzzInstruction;

#[contracttype]
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct FakeVal(pub u64);

impl FakeVal {
    pub fn from_val(v: Val) -> FakeVal {
        FakeVal(v.get_payload())
    }

    pub fn to_val(self) -> Val {
        Val::from_payload(self.0)
    }
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum FuzzInstruction {
    Raw(RawFuzzInstruction),
    Typed(TypedFuzzInstruction),
}

pub struct FuzzContract;

#[contractimpl]
impl FuzzContract {
    pub fn run(env: Env, fuzz_instruction: FuzzInstruction) {
        match fuzz_instruction {
            FuzzInstruction::Raw(instr) => {
                instr.run();
            }
            FuzzInstruction::Typed(instr) => {
                instr.run(&env);
            }
        }
    }
}

// duplicated code from guest.rs to generate syscall defs
mod syscalls {
    #![allow(unused)]

    use soroban_env_common::call_macro_with_all_host_functions;

    // This is a helper macro used only by impl_env_for_guest below. It consumes a
    // token-tree of the form:
    //
    //  {fn $fn_id:ident $args:tt -> $ret:ty}
    //
    // and produces the the corresponding method definition to be used in the
    // Guest implementation of the Env trait (calling through to the corresponding
    // unsafe extern function).
    macro_rules! extern_function_helper {
        {
            $fn_str:literal, $(#[$attr:meta])* fn $fn_id:ident($($arg:ident:$type:ty),*) -> $ret:ty
        }
        =>
        {
            #[cfg_attr(target_family = "wasm", link_name = $fn_str)]
            $(#[$attr])*
            pub(crate) fn $fn_id($($arg:$type),*) -> $ret;
        };
    }

    macro_rules! generate_extern_modules {
        {
            $(
                // This outer pattern matches a single 'mod' block of the token-tree
                // passed from the x-macro to this macro. It is embedded in a `$()*`
                // pattern-repetition matcher so that it will match all provided
                // 'mod' blocks provided.
                $(#[$mod_attr:meta])*
                mod $mod_id:ident $mod_str:literal
                {
                    $(
                        // This inner pattern matches a single function description
                        // inside a 'mod' block in the token-tree passed from the
                        // x-macro to this macro. It is embedded in a `$()*`
                        // pattern-repetition matcher so that it will match all such
                        // descriptions.
                        $(#[$fn_attr:meta])*
                        { $fn_str:literal, fn $fn_id:ident $args:tt -> $ret:ty }
                    )*
                }
            )*
        }

        =>  // The part of the macro above this line is a matcher; below is its expansion.

        {
            // This macro expands to a set of mod items, each declaring all the extern fns
            // available for the `impl Env for Guest` methods above to call through to.
            $(
                // Unlike the other uses of the x-macro that "flatten" the
                // mod-and-fn structure of the matched token-tree, this callback
                // macro's expansion preserves the structure, creating a nested set
                // of mods and fns. There is therefore a mod declaration between the
                // outer and inner `$()*` pattern-repetition expanders.
                $(#[$mod_attr])*
                pub mod $mod_id {
                    #[allow(unused_imports)]
                    use soroban_env_common::{Val, Object, Symbol, MapObject, VecObject, BytesObject, Error};
                    #[allow(unused_imports)]
                    use soroban_env_common::{I128Object, I256Object, I64Object, I64Val, U128Object, U256Object, U32Val, U64Object, U64Val, StorageType, TimepointObject, DurationObject};
                    #[allow(unused_imports)]
                    use soroban_env_common::{Void,AddressObject,SymbolObject,StringObject,Bool};
                    #[link(wasm_import_module = $mod_str)]
                    extern "C" {
                        $(
                            // This invokes the extern_function_helper! macro above
                            // passing only the relevant parts of the declaration
                            // matched by the inner pattern above. It is embedded in
                            // one `$()*` pattern-repetition expander so that it
                            // repeats only for the part of each mod that the
                            // corresponding pattern-repetition matcher.
                            extern_function_helper!{$fn_str, $(#[$fn_attr])* fn $fn_id $args -> $ret}
                        )*
                    }
                }
            )*
        };
    }

    // Here we invoke the x-macro passing generate_extern_modules as its callback macro.
    call_macro_with_all_host_functions! { generate_extern_modules }
}
