//! Carbon shadow struct for current `PrizeClaimedEvent`.
//!
//! Field order and types must match
//! `contracts/markets/programs/markets/src/instructions/post_resolution/claim_prize/events.rs`.

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
pub struct PrizeClaimedEvent {
    pub market: solana_pubkey::Pubkey,
    pub claimer: solana_pubkey::Pubkey,
    pub winning_outcome: u8,
    pub outcome_mint: solana_pubkey::Pubkey,
    pub burn_amount: u64,
    pub quote_payout: u64,
    pub vault_balance_after: u64,
    pub supply_after: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(PrizeClaimedEvent, "0xe445a52e51cb9a1dc54655eaeb0806f1");
