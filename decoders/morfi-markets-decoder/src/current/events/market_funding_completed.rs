//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct MarketFundingCompleted {
    pub trading_opens_ts: i64,
}

impl_carbon_deserialize!(MarketFundingCompleted, "0xe445a52e51cb9a1dbab495e6c17fcf0e");
