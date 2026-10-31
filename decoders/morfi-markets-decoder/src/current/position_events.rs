//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::{borsh, deserialize::CarbonDeserialize};
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
    PositionSlotLifecycleEvent,
    "0xe445a52e51cb9a1d3b7342710d125cee"
);
impl_carbon_deserialize!(
    UserPositionsInitializedEvent,
    "0xe445a52e51cb9a1dd9c8d87c43381efe"
);
impl_carbon_deserialize!(
    UserPositionsResizedEvent,
    "0xe445a52e51cb9a1d402a7541dd0bc3d4"
);
impl_carbon_deserialize!(
    UserPositionsClosedEvent,
    "0xe445a52e51cb9a1d608687a5f12cc3ce"
);

pub(super) fn decode_position_events(data: &[u8]) -> Option<super::CurrentEvent> {
    if let Some(value) = PositionSlotLifecycleEvent::deserialize(data) {
        return Some(super::CurrentEvent::PositionSlotLifecycleEvent(value));
    }
    if let Some(value) = UserPositionsInitializedEvent::deserialize(data) {
        return Some(super::CurrentEvent::UserPositionsInitializedEvent(value));
    }
    if let Some(value) = UserPositionsResizedEvent::deserialize(data) {
        return Some(super::CurrentEvent::UserPositionsResizedEvent(value));
    }
    if let Some(value) = UserPositionsClosedEvent::deserialize(data) {
        return Some(super::CurrentEvent::UserPositionsClosedEvent(value));
    }
    None
}
