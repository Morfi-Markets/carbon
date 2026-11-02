//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct WalletBackerEntitlementForfeitedEvent {
    pub market: Pubkey,
    pub owner: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub units_debited: u64,
    pub supply_after: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(
    WalletBackerEntitlementForfeitedEvent,
    "0xe445a52e51cb9a1d2a155df0e31bbbef"
);
