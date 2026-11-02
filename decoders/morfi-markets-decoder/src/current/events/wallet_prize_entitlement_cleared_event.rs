//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct WalletPrizeEntitlementClearedEvent {
    pub market: Pubkey,
    pub owner: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub outcome_index: u8,
    pub units_cleared: u64,
    pub supply_after: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(
    WalletPrizeEntitlementClearedEvent,
    "0xe445a52e51cb9a1d64ac70535de8d6c9"
);
