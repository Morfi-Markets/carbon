//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct WalletPrincipalBonusClaimedEvent {
    pub market: Pubkey,
    pub claimer: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub winning_outcome: u8,
    pub outcome_units_debited: u64,
    pub prize_units_debited: u64,
    pub principal_gross: u64,
    pub platform_claim_fee: u64,
    pub prize_payout: u64,
    pub principal_claim_liability_after: u64,
    pub prize_liability_after: u64,
    pub fee_vault_index: u8,
    pub ts: i64,
}

impl_carbon_deserialize!(
    WalletPrincipalBonusClaimedEvent,
    "0xe445a52e51cb9a1d85a1030eb53f3d09"
);
