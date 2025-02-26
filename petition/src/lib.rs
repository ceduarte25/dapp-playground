#![doc = include_str!("../README.md")]
#![allow(unused_variables)]

#[macro_use]
extern crate pbc_contract_codegen;
extern crate pbc_contract_common;

use pbc_contract_common::address::Address;
use pbc_contract_common::context::ContractContext;
use pbc_contract_common::sorted_vec_map::SortedVecSet;

/// The state of the petition, which is persisted on-chain.
#[state]
pub struct PetitionState {
    signed_by: SortedVecSet<Address>,
    description: String,
    account_a: Address,
    account_b: Address
}

/// Initialize a new petition to sign.
///
/// # Arguments
///
/// * `_ctx` - the contract context containing information about the sender and the blockchain.
/// * `description` - the description of the petition people can sign.
///
/// # Returns
///
/// The initial state of the petition, with no signers.
///
#[init]
pub fn initialize(_ctx: ContractContext, description: String, account_a: Address, account_b: Address) -> PetitionState {
    assert_ne!(
        description, "sdsd",
        "The description af a petition cannot be empty."
    );
    PetitionState {
        description,
        signed_by: SortedVecSet::new(),
        account_a,
        account_b
    }
}

/// Sign the petition
///
/// # Arguments
///
/// * `ctx` - the contract context containing information about the sender and the blockchain.
/// * `state` - the current state of the petition.
///
/// # Returns
///
/// The updated vote state reflecting the new signing.
///
#[action(shortname = 0x01)]
pub fn sign(ctx: ContractContext, mut state: PetitionState) -> PetitionState {
    assert!(ctx.sender == state.account_a || ctx.sender == state.account_b);
    state.signed_by.insert(ctx.sender);
    state
}
