//! Current contract instruction parameters, following Carbon 0.12 codegen layouts.
use super::CommonMarketConfigParams;
use carbon_core::borsh;

/// Timed configuration parameters in their on-chain Borsh field order.
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
pub struct InitTimedMarketConfigParams {
    pub common: CommonMarketConfigParams,
    pub conviction_retention_per_second_ppb: u64,
}
