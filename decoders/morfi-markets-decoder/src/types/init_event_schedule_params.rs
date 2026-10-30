//! Schedule initialization parameters mirrored from the pinned contract IDL.

use carbon_core::borsh;

/// Nonce and inclusive opening/exclusive closing times, in integer chain seconds.
#[derive(
    Debug,
    Clone,
    borsh::BorshSerialize,
    borsh::BorshDeserialize,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Hash,
)]
pub struct InitEventScheduleParams {
    pub schedule_nonce: u64,
    pub trading_started_ts: i64,
    pub trading_end_ts: i64,
}
