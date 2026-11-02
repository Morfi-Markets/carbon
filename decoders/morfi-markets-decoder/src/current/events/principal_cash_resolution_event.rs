//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct PrincipalCashResolutionEvent {
    pub market: Pubkey,
    pub winning_outcome: u8,
    pub winning_outcome_units: u64,
    pub winning_prize_units: u64,
    pub winner_principal_reserved: u64,
    pub loser_principal_reclassified: u64,
    pub prize_liability_after: u64,
    pub post_outcome_conviction_ppb: Vec<u32>,
    pub post_outcome_conviction_target_ppb: Vec<u32>,
    pub post_conviction_retention_per_second_ppb: u32,
    pub post_last_conviction_update_ts: i64,
    pub ts: i64,
}

impl_carbon_deserialize!(
    PrincipalCashResolutionEvent,
    "0xe445a52e51cb9a1d194f5e95567c4ba9"
);
