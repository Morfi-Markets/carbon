//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct WalletCashEntitlementClearedEvent {
    pub market: Pubkey,
    pub owner: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub outcome_index: u8,
    pub outcome_units_cleared: u64,
    pub prize_units_cleared: u64,
    pub principal_cleared: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(
    WalletCashEntitlementClearedEvent,
    "0xe445a52e51cb9a1d5b2907e12cd2ebf1"
);
