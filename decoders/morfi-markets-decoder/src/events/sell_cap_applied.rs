//! Carbon shadow struct for the sell-cap build's `SellCapApplied` event.
//! Recognised so a capped sell is not reported as undecodable; it is not
//! indexed, because the paired `TradeEvent` already carries the position and
//! supply changes.

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
pub struct SellCapApplied {
    pub market: solana_pubkey::Pubkey,
    pub trader: solana_pubkey::Pubkey,
    pub outcome_index: u8,
    pub outcome_mint: solana_pubkey::Pubkey,
    pub tokens_sold: u64,
    pub curve_gross: u64,
    pub capped_gross: u64,
    pub tokens_returned_to_pool: u64,
    pub tokens_burned: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(SellCapApplied, "0xe445a52e51cb9a1d3e9b2382bc64233a");
