//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct PositionSlotLifecycleEvent {
    pub action: u8,
    pub wallet: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub kind: u8,
    pub market: Pubkey,
    pub outcome_index: u8,
    pub ts: i64,
}

impl_carbon_deserialize!(
    PositionSlotLifecycleEvent,
    "0xe445a52e51cb9a1d3b7342710d125cee"
);
