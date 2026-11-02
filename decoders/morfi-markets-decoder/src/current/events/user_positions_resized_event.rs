//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct UserPositionsResizedEvent {
    pub wallet: Pubkey,
    pub user_positions: Pubkey,
    pub old_capacity: u16,
    pub new_capacity: u16,
    pub payer: Pubkey,
    pub treasury: Pubkey,
    pub rent_paid_lamports: u64,
    pub rent_refunded_lamports: u64,
}

impl_carbon_deserialize!(
    UserPositionsResizedEvent,
    "0xe445a52e51cb9a1d402a7541dd0bc3d4"
);
