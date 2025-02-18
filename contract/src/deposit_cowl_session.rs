#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");
extern crate alloc;

use casper_contract::{
    contract_api::{
        runtime::{call_versioned_contract, get_named_arg, put_key, revert},
        storage::new_uref,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ContractPackageHash, Key, RuntimeArgs, U256};
use cowl_swap::{
    constants::{
        ARG_AMOUNT, ARG_BALANCE_COWL, ARG_COWL_CEP18_CONTRACT_PACKAGE,
        ARG_COWL_SWAP_CONTRACT_PACKAGE, ARG_RECIPIENT, ENTRY_POINT_BALANCE_COWL,
        ENTRY_POINT_TRANSFER,
    },
    error::SwapError,
};

#[no_mangle]
pub extern "C" fn call() {
    let amount: U256 = get_named_arg(ARG_AMOUNT);
    if amount == U256::zero() {
        revert(SwapError::InvalidAmount);
    }

    let cowl_cep18_contract_package_key: Key = get_named_arg(ARG_COWL_CEP18_CONTRACT_PACKAGE);

    let cowl_cep18_contract_package = ContractPackageHash::from(
        cowl_cep18_contract_package_key
            .into_hash()
            .unwrap_or_revert_with(SwapError::InvalidTokenContractPackage),
    );

    let cowl_swap_contract_package_key: Key = get_named_arg(ARG_COWL_SWAP_CONTRACT_PACKAGE);

    let cowl_swap_contract_package_key_hash = ContractPackageHash::from(
        cowl_swap_contract_package_key
            .into_hash()
            .unwrap_or_revert_with(SwapError::InvalidPackageHash),
    );

    call_versioned_contract::<()>(
        cowl_cep18_contract_package,
        None,
        ENTRY_POINT_TRANSFER,
        runtime_args! {
            ARG_RECIPIENT => cowl_swap_contract_package_key,
            ARG_AMOUNT => amount
        },
    );

    let balance = call_versioned_contract::<U256>(
        cowl_swap_contract_package_key_hash,
        None,
        ENTRY_POINT_BALANCE_COWL,
        runtime_args! {},
    );
    let new_uref = new_uref(balance);
    put_key(ARG_BALANCE_COWL, new_uref.into());
}
