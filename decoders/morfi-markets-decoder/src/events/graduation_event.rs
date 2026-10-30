//! Carbon shadow struct for current `GraduationEvent`.
//!
//! Field order and types must match
//! `contracts/markets/programs/markets/src/instructions/resolution/resolve_drain_and_migrate/events.rs`.

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
pub struct GraduationEvent {
    pub market: solana_pubkey::Pubkey,
    pub winning_outcome: u8,
    pub winning_bc_mint: solana_pubkey::Pubkey,
    pub amm_pool: solana_pubkey::Pubkey,
    pub quote_seeded_to_amm: u64,
    pub amm_seed_tokens: u64,
    pub winning_outcome_mint_supply_after_d6: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(GraduationEvent, "0xe445a52e51cb9a1d0af6df7f30629537");
