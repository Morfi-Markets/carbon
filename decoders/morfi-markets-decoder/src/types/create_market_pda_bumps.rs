//! Legacy creation PDA bumps mirrored from the pinned contract IDL.

use carbon_core::borsh;

/// PDA bumps serialized in the preserved legacy creation argument struct.
#[derive(
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct CreateMarketPdaBumps {
    pub market: u8,
    pub outcome_mint_authority: u8,
    pub market_quote_vault_authority: u8,
    pub market_quote_vault: u8,
    pub backer_share_mint: u8,
}
