//! Legacy creation arguments mirrored from the pinned contract IDL.

use super::CreateMarketPdaBumps;
use carbon_core::borsh;

/// Shared Borsh arguments for legacy conviction and timed creation.
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
pub struct CreateMarketParams {
    pub n_outcomes: u8,
    pub market_nonce: u64,
    pub initial_backing: u64,
    pub pda_bumps: CreateMarketPdaBumps,
}
