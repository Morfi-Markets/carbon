//! Outcome token metadata mirrored from the pinned contract IDL.

use carbon_core::borsh;

/// Immutable token metadata supplied by the creator for one active outcome.
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
pub struct OutcomePreparation {
    pub outcome_index: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
