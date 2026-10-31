//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::{borsh, deserialize::CarbonDeserialize};
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct LogicalResolutionEvent {
    pub market: Pubkey,
    pub winning_outcome: u8,
    pub winning_outcome_supply: u64,
    pub winning_prize_supply: u64,
    pub prize_pool_balance_at_resolution: u64,
    pub migration_seedable: bool,
    pub migration_non_seedable_reason: u8,
    pub post_outcome_conviction_ppb: Vec<u32>,
    pub post_outcome_conviction_target_ppb: Vec<u32>,
    pub post_conviction_retention_per_second_ppb: u32,
    pub post_last_conviction_update_ts: i64,
    pub ts: i64,
}

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct PrincipalCashResolutionEvent {
    pub market: Pubkey,
    pub winning_outcome: u8,
    pub winning_outcome_units: u64,
    pub winning_prize_units: u64,
    pub winner_principal_reserved: u64,
    pub loser_principal_reclassified: u64,
    pub prize_liability_after: u64,
    pub post_outcome_conviction_ppb: Vec<u32>,
    pub post_outcome_conviction_target_ppb: Vec<u32>,
    pub post_conviction_retention_per_second_ppb: u32,
    pub post_last_conviction_update_ts: i64,
    pub ts: i64,
}

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

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct PrincipalCashLiabilitiesSweptEvent {
    pub market: Pubkey,
    pub principal_swept: u64,
    pub prize_swept: u64,
    pub platform_fee_vault: Pubkey,
    pub ts: i64,
}

impl_carbon_deserialize!(LogicalResolutionEvent, "0xe445a52e51cb9a1d31e17ecde731e4fd");
impl_carbon_deserialize!(
    PrincipalCashResolutionEvent,
    "0xe445a52e51cb9a1d194f5e95567c4ba9"
);
impl_carbon_deserialize!(
    ExpiredWinningTokensSweptEvent,
    "0xe445a52e51cb9a1df65c97b1977759eb"
);
impl_carbon_deserialize!(
    PrincipalCashLiabilitiesSweptEvent,
    "0xe445a52e51cb9a1d3bf021b7a9e8a955"
);

pub(super) fn decode_resolution_events(data: &[u8]) -> Option<super::CurrentEvent> {
    if let Some(value) = LogicalResolutionEvent::deserialize(data) {
        return Some(super::CurrentEvent::LogicalResolutionEvent(value));
    }
    if let Some(value) = PrincipalCashResolutionEvent::deserialize(data) {
        return Some(super::CurrentEvent::PrincipalCashResolutionEvent(value));
    }
    if let Some(value) = ExpiredWinningTokensSweptEvent::deserialize(data) {
        return Some(super::CurrentEvent::ExpiredWinningTokensSweptEvent(value));
    }
    if let Some(value) = PrincipalCashLiabilitiesSweptEvent::deserialize(data) {
        return Some(super::CurrentEvent::PrincipalCashLiabilitiesSweptEvent(
            value,
        ));
    }
    None
}
