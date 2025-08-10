#[cfg(feature = "bincode")]
use crate::SysvarSerialize;
use crate::{impl_sysvar_get, Sysvar};
pub use {
    solana_slot_summary::SlotSummary,
    solana_sdk_ids::sysvar::rent::{check_id, id, ID},
};
impl Sysvar for SlotSummary {
    impl_sysvar_get!(sol_get_rent_sysvar);
}

#[cfg(feature = "bincode")]
impl SysvarSerialize for SlotSummary {}
