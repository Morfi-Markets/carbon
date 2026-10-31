//! Carbon shadow struct for morfi_events::markets::boosting::MarketBoostedEvent.

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
pub struct MarketBoostedEvent {
    pub ts: i64,
    pub market: solana_pubkey::Pubkey,
    pub booster: solana_pubkey::Pubkey,
    pub prize_pool_address: solana_pubkey::Pubkey,
    pub amount_quote: u64,
}

impl_carbon_deserialize!(MarketBoostedEvent, "0xe445a52e51cb9a1d943a6a50aa0ad64b");
