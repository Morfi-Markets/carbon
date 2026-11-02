//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct MarketInitializedEvent {
    pub market: Pubkey,
    pub creator: Pubkey,
    pub config: Pubkey,
    pub directory_cid: String,
    pub n_outcomes: u8,
    pub resolution_mode: u8,
    pub ts: i64,
}

impl_carbon_deserialize!(MarketInitializedEvent, "0xe445a52e51cb9a1d46ad60ca648f2d19");
