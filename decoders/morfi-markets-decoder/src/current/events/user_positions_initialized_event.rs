//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct UserPositionsInitializedEvent {
    pub wallet: Pubkey,
    pub user_positions: Pubkey,
    pub capacity: u16,
    pub payer: Pubkey,
    pub rent_lamports: u64,
}

impl_carbon_deserialize!(
    UserPositionsInitializedEvent,
    "0xe445a52e51cb9a1dd9c8d87c43381efe"
);
