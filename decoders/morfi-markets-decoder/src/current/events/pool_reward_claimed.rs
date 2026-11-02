//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct PoolRewardClaimed {
    pub market: Pubkey,
    pub snapshot_wallet: Pubkey,
    pub payout_wallet: Pubkey,
    pub member_index: u8,
    pub amount_quote: u64,
    pub claimed_quote: u64,
    pub cumulative_collected_quote: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(PoolRewardClaimed, "0xe445a52e51cb9a1dc1e60e0cde3bb1c4");
