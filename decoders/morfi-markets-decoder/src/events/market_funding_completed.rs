//! Current readiness event; market identity comes from its enclosing instruction.

use carbon_core::borsh;

/// First instant when funding and preparation permit trading.
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
pub struct MarketFundingCompleted {
    pub trading_opens_ts: i64,
}

impl_carbon_deserialize!(MarketFundingCompleted, "0xe445a52e51cb9a1dbab495e6c17fcf0e");
