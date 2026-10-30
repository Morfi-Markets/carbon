//! Carbon shadow struct for morfi_events::markets::gift::GiftCreated.

use super::types::GiftKind;
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
pub struct GiftCreated {
    pub ts: i64,
    pub gift_config: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub handler_hash: [u8; 32],
    pub kind: GiftKind,
    pub amount: u64,
}

impl_carbon_deserialize!(GiftCreated, "0xe445a52e51cb9a1d80579c640556b17e");
