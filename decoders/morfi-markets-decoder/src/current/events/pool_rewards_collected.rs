//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct PoolRewardsCollected {
    pub market: Pubkey,
    pub pool: Pubkey,
    pub quote_mint: Pubkey,
    pub collected_quote: u64,
    pub cumulative_collected_quote: u64,
    pub platform_paid_quote: u64,
    pub holder_budget_quote: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(PoolRewardsCollected, "0xe445a52e51cb9a1daa886088a2adb2db");
