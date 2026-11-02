//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct MarketBoostedEvent {
    pub ts: i64,
    pub market: Pubkey,
    pub booster: Pubkey,
    pub prize_pool_address: Pubkey,
    pub amount_quote: u64,
}

impl_carbon_deserialize!(MarketBoostedEvent, "0xe445a52e51cb9a1d943a6a50aa0ad64b");
