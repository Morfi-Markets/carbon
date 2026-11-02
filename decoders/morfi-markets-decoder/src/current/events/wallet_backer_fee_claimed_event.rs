//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct WalletBackerFeeClaimedEvent {
    pub market: Pubkey,
    pub claimer: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub units_debited: u64,
    pub quote_payout: u64,
    pub vault_balance_after: u64,
    pub supply_after: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(
    WalletBackerFeeClaimedEvent,
    "0xe445a52e51cb9a1dca1a0aaf4e078bca"
);
