//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::{borsh, deserialize::CarbonDeserialize};
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct WalletPrizeClaimedEvent {
    pub market: Pubkey,
    pub claimer: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub winning_outcome: u8,
    pub units_debited: u64,
    pub quote_payout: u64,
    pub vault_balance_after: u64,
    pub supply_after: u64,
    pub ts: i64,
}

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct WalletPrincipalPrizeClaimedEvent {
    pub market: Pubkey,
    pub claimer: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub winning_outcome: u8,
    pub outcome_units_debited: u64,
    pub prize_units_debited: u64,
    pub principal_gross: u64,
    pub platform_sell_fee: u64,
    pub prize_payout: u64,
    pub principal_claim_liability_after: u64,
    pub prize_liability_after: u64,
    pub fee_vault_index: u8,
    pub ts: i64,
}

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct WalletCashEntitlementClearedEvent {
    pub market: Pubkey,
    pub owner: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub outcome_index: u8,
    pub outcome_units_cleared: u64,
    pub prize_units_cleared: u64,
    pub principal_cleared: u64,
    pub ts: i64,
}

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct WalletOutcomeEntitlementClearedEvent {
    pub market: Pubkey,
    pub owner: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub outcome_index: u8,
    pub units_cleared: u64,
    pub supply_after: u64,
    pub ts: i64,
}

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
    WalletPrizeClaimedEvent,
    "0xe445a52e51cb9a1d5bf2fe1dc51ef3a1"
);
impl_carbon_deserialize!(
    WalletPrincipalPrizeClaimedEvent,
    "0xe445a52e51cb9a1d8ed20a7c618fa2b8"
);
impl_carbon_deserialize!(
    WalletCashEntitlementClearedEvent,
    "0xe445a52e51cb9a1d5b2907e12cd2ebf1"
);
impl_carbon_deserialize!(
    WalletOutcomeEntitlementClearedEvent,
    "0xe445a52e51cb9a1d7c22be5c4c6237a5"
);
impl_carbon_deserialize!(
    WalletPrizeEntitlementClearedEvent,
    "0xe445a52e51cb9a1d64ac70535de8d6c9"
);
impl_carbon_deserialize!(
    WalletOutcomeTokensWithdrawnEvent,
    "0xe445a52e51cb9a1d59ac00ac4e115bdd"
);

pub(super) fn decode_claim_events(data: &[u8]) -> Option<super::CurrentEvent> {
    if let Some(value) = WalletPrizeClaimedEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletPrizeClaimedEvent(value));
    }
    if let Some(value) = WalletPrincipalPrizeClaimedEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletPrincipalPrizeClaimedEvent(value));
    }
    if let Some(value) = WalletCashEntitlementClearedEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletCashEntitlementClearedEvent(
            value,
        ));
    }
    if let Some(value) = WalletOutcomeEntitlementClearedEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletOutcomeEntitlementClearedEvent(
            value,
        ));
    }
    if let Some(value) = WalletPrizeEntitlementClearedEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletPrizeEntitlementClearedEvent(
            value,
        ));
    }
    if let Some(value) = WalletOutcomeTokensWithdrawnEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletOutcomeTokensWithdrawnEvent(
            value,
        ));
    }
    None
}
