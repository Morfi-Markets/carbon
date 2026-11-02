//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct WinningOutcomeVaultClosedEvent {
    pub market: Pubkey,
    pub finalizer: Pubkey,
    pub outcome_index: u8,
    pub outcome_mint: Pubkey,
    pub outcome_token_vault: Pubkey,
    pub surplus_burned: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(
    WinningOutcomeVaultClosedEvent,
    "0xe445a52e51cb9a1d8adb68302784b95d"
);
