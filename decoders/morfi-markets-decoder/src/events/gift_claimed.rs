//! Carbon shadow struct for morfi_events::markets::gift::GiftClaimed.

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
pub struct GiftClaimed {
    pub ts: i64,
    pub gift_config: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub handler_hash: [u8; 32],
    pub recipient: solana_pubkey::Pubkey,
    pub amount: u64,
}

impl_carbon_deserialize!(GiftClaimed, "0xe445a52e51cb9a1d46b0191ded8db9a3");
