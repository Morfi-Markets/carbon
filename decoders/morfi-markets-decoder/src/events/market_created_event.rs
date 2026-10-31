//! Carbon shadow of the current Markets creation event, in pinned IDL field order.

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
pub struct MarketCreatedEvent {
    pub market: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub backer_share_mint: solana_pubkey::Pubkey,
    pub n_outcomes: u8,
    pub resolution_mode: u8,
    pub ts: i64,
}

impl_carbon_deserialize!(MarketCreatedEvent, "0xe445a52e51cb9a1d828e05106ba0497c");
