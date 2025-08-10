

#![no_std]
#[cfg(feature = "frozen-abi")]
extern crate std;

#[cfg(feature = "sysvar")]
pub mod sysvar;

#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};
use solana_sdk_macro::CloneZeroed;



#[repr(C)]
#[cfg_attr(feature = "frozen-abi", derive(solana_frozen_abi_macro::AbiExample))]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase")
)]
#[derive(Debug, CloneZeroed, PartialEq,Default)]
pub struct SlotSummary {
    pub start_slot: u64,
    pub end_slot: u64,
    pub avg_tx_count: u64,
    pub avg_success_rate: f32,
}