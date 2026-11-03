//! Hand-written mirror of the contract's `MarketFundingCompleted`.
//! `tests/current_event_bytes.rs` checks its Borsh field order, and Morfi's
//! `backend/indexer-service/tests/idl_decoder_layouts.rs` checks its name and
//! discriminator against the committed IDL.
use carbon_core::borsh;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct MarketFundingCompleted {
    pub trading_opens_ts: i64,
    pub ts: i64,
}

impl_carbon_deserialize!(MarketFundingCompleted, "0xe445a52e51cb9a1dbab495e6c17fcf0e");
