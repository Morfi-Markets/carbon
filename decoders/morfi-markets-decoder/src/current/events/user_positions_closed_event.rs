//! Generated from the frozen Markets IDL; do not modify the contract source.
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
}

impl_carbon_deserialize!(
    UserPositionsClosedEvent,
    "0xe445a52e51cb9a1d608687a5f12cc3ce"
);
