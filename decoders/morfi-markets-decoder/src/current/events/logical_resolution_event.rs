//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct LogicalResolutionEvent {
    pub market: Pubkey,
    pub winning_outcome: u8,
    pub winning_outcome_supply: u64,
    pub winning_prize_supply: u64,
    pub prize_pool_balance_at_resolution: u64,
    pub migration_seedable: bool,
    pub migration_non_seedable_reason: u8,
    pub post_outcome_conviction_ppb: Vec<u32>,
    pub post_outcome_conviction_target_ppb: Vec<u32>,
    pub post_conviction_retention_per_second_ppb: u32,
    pub post_last_conviction_update_ts: i64,
    pub ts: i64,
}

impl_carbon_deserialize!(LogicalResolutionEvent, "0xe445a52e51cb9a1d31e17ecde731e4fd");
