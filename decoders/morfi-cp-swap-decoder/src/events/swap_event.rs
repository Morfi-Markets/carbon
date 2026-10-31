//! Carbon shadow struct for the fee-complete cp-swap SwapEvent.
//! Field layout MUST match the contract definition exactly (Borsh equivalence).

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
pub struct SwapEvent {
    pub ts: i64,
    pub pool_id: solana_pubkey::Pubkey,
    pub input_vault_before: u64,
    pub output_vault_before: u64,
    pub input_amount: u64,
    pub output_amount: u64,
    pub input_transfer_fee: u64,
    pub output_transfer_fee: u64,
    pub base_input: bool,
    pub input_mint: solana_pubkey::Pubkey,
    pub output_mint: solana_pubkey::Pubkey,
    pub trade_fee: u64,
    pub protocol_fee: u64,
    pub fund_fee: u64,
    pub creator_fee: u64,
    pub creator_fee_on_input: bool,
    pub input_token_owner: solana_pubkey::Pubkey,
    pub output_token_owner: solana_pubkey::Pubkey,
}

impl_carbon_deserialize!(SwapEvent, "0xe445a52e51cb9a1d40c6cde8260871e2");
