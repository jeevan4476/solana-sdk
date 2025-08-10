pub use solana_sdk_ids::sysvar::slot_summary::{check_id, id, ID};
use {crate::SlotSummary, solana_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(SlotSummary);
