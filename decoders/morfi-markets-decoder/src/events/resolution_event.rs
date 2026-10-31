//! Carbon shadow struct for current `ResolutionEvent`.
//!
//! Field order and types must match
//! `contracts/markets/programs/markets/src/instructions/resolution/events.rs`.

use carbon_core::borsh;

#[derive(
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Clone,
    Hash,
)]
pub struct ResolutionEvent {
    pub market: solana_pubkey::Pubkey,
    pub winning_outcome: u8,
    pub winning_mint: solana_pubkey::Pubkey,
    pub winning_supply: u64,
    pub prize_pool_balance_at_resolution: u64,
    pub post_outcome_conviction_ppb: Vec<u32>,
    pub post_outcome_conviction_target_ppb: Vec<u32>,
    pub post_conviction_retention_per_second_ppb: u32,
    pub post_last_conviction_update_ts: i64,
    pub ts: i64,
}

impl_carbon_deserialize!(ResolutionEvent, "0xe445a52e51cb9a1d3adf6db7b795a9fb");
