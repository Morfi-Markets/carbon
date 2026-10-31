//! Current contract instruction parameters, following Carbon 0.12 codegen layouts.
use super::CommonMarketConfigParams;
use carbon_core::borsh;

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
pub struct InitConvictionMarketConfigParams {
    pub common: CommonMarketConfigParams,
    pub conviction_retention_per_second_ppb: u64,
    pub conviction_resolution_threshold_ppb: u64,
    pub resolution_threshold_floor_margin_ppb: u64,
    pub resolution_threshold_decay_delay_secs: u64,
    pub resolution_threshold_drop_interval_secs: u64,
    pub resolution_threshold_drop_ppb: u64,
}
