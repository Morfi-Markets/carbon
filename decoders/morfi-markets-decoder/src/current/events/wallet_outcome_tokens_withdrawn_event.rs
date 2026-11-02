//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct WalletOutcomeTokensWithdrawnEvent {
    pub market: Pubkey,
    pub owner: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub outcome_index: u8,
    pub outcome_mint: Pubkey,
    pub amount: u64,
    pub entitlements_remaining: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(
    WalletOutcomeTokensWithdrawnEvent,
    "0xe445a52e51cb9a1d59ac00ac4e115bdd"
);
