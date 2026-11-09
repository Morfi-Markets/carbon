//! Hand-written mirror of the contract's `UserPositionsClosedEvent`.
//! `tests/current_event_bytes.rs` checks its Borsh field order, and Morfi's
//! `backend/indexer-service/tests/idl_decoder_layouts.rs` checks its name and
//! discriminator against the committed IDL.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct UserPositionsClosedEvent {
    pub wallet: Pubkey,
    pub user_positions: Pubkey,
    pub capacity: u16,
    pub treasury: Pubkey,
    pub rent_refunded_lamports: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(
    UserPositionsClosedEvent,
    "0xe445a52e51cb9a1d608687a5f12cc3ce"
);
