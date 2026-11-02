//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct MarketQuoteCustodyClosedEvent {
    pub market: Pubkey,
    pub rent_recipient: Pubkey,
    pub surplus_quote: u64,
    pub rent_lamports: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(
    MarketQuoteCustodyClosedEvent,
    "0xe445a52e51cb9a1d42cd7fd1c6fe0b3a"
);
