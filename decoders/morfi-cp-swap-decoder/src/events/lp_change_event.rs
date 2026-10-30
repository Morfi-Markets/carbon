//! Carbon shadow struct for morfi_events::cp_swap::LpChangeEvent.
//! Field layout MUST match the morfi-events definition exactly (Borsh
//! equivalence). This event advances the pool's effective reserves without
//! creating normalized user trade activity.

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
pub struct LpChangeEvent {
    pub ts: i64,
    pub pool_id: solana_pubkey::Pubkey,
    pub lp_amount_before: u64,
    pub token_0_vault_before: u64,
    pub token_1_vault_before: u64,
    pub token_0_amount: u64,
    pub token_1_amount: u64,
    pub token_0_transfer_fee: u64,
    pub token_1_transfer_fee: u64,
    pub change_type: u8,
}

impl_carbon_deserialize!(LpChangeEvent, "0xe445a52e51cb9a1d79a3cdc939da753c");
