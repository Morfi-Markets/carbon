//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct ExpiredWinningTokensSweptEvent {
    pub market: Pubkey,
    pub outcome_index: u8,
    pub outcome_mint: Pubkey,
    pub treasury_outcome_account: Pubkey,
    pub tokens_transferred: u64,
    pub expired_entitlements: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(
    ExpiredWinningTokensSweptEvent,
    "0xe445a52e51cb9a1df65c97b1977759eb"
);
