//! Shadow types for embedded enums/structs that markets events reference.
//! Mirror the field layout of the morfi-events versions exactly.

use carbon_core::borsh;

/// Mirror of `morfi_events::markets::gift::GiftKind`. Variant order MUST match
/// the source — Borsh discriminant uses positional order.
#[derive(
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
)]
pub enum GiftKind {
    /// `mint` is an outcome_mint (classic SPL Token in this program).
    BondingCurveToken,
    /// `mint` is a claim_rights_mint (classic SPL Token in this program).
    /// `mint` is USDC (the recognised stable quote mint for the network).
    Usdc,
}
