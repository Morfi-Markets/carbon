//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::{borsh, deserialize::CarbonDeserialize};
use solana_pubkey::Pubkey;

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
pub struct WalletExpiredBackingRefundEvent {
    pub market: Pubkey,
    pub backer: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub units_debited: u64,
    pub quote_refunded: u64,
    pub remaining_prize_liability: u64,
    pub remaining_share_supply: u64,
    pub backer_balance: u64,
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
pub struct WalletTradeEvent {
    pub market: Pubkey,
    pub trader: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub outcome_index: u8,
    pub side: u8,
    pub quote_amount_gross: u64,
    pub bc_amount: u64,
    pub base_fee_to_backers: u64,
    pub base_fee_to_platform: u64,
    pub prize_pool_contribution: u64,
    #[serde(serialize_with = "super::serialization::serialize_u128")]
    pub post_quote_reserves: u128,
    pub bc_supply_after: u64,
    pub used_penalty_fee: bool,
    pub post_outcome_conviction_ppb: Vec<u32>,
    pub post_outcome_conviction_target_ppb: Vec<u32>,
    pub post_conviction_retention_per_second_ppb: u32,
    pub post_last_conviction_update_ts: i64,
    pub ts: i64,
    pub post_total_real_quote_reserves: u64,
}

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct WinnerTokenLaunchEvent {
    pub market: Pubkey,
    pub winning_outcome: u8,
    pub winning_mint: Pubkey,
    pub amm_pool: Pubkey,
    pub holder_units_d6: u64,
    pub quote_seeded_to_amm: u64,
    pub amm_seed_tokens: u64,
    pub winning_outcome_mint_supply_after_d6: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(
    WalletCashEntitlementClearedEvent,
    "0xe445a52e51cb9a1d5b2907e12cd2ebf1"
);
impl_carbon_deserialize!(
    WalletExpiredBackingRefundEvent,
    "0xe445a52e51cb9a1d1f43904ad2fec074"
);
impl_carbon_deserialize!(
    WalletOutcomeEntitlementClearedEvent,
    "0xe445a52e51cb9a1d7c22be5c4c6237a5"
);
impl_carbon_deserialize!(
    WalletOutcomeTokensWithdrawnEvent,
    "0xe445a52e51cb9a1d59ac00ac4e115bdd"
);
impl_carbon_deserialize!(
    WalletPrincipalPrizeClaimedEvent,
    "0xe445a52e51cb9a1d8ed20a7c618fa2b8"
);
impl_carbon_deserialize!(
    WalletPrizeClaimedEvent,
    "0xe445a52e51cb9a1d5bf2fe1dc51ef3a1"
);
impl_carbon_deserialize!(
    WalletPrizeEntitlementClearedEvent,
    "0xe445a52e51cb9a1d64ac70535de8d6c9"
);
impl_carbon_deserialize!(WalletTradeEvent, "0xe445a52e51cb9a1dfbd80b0838f398f7");
impl_carbon_deserialize!(WinnerTokenLaunchEvent, "0xe445a52e51cb9a1d03bfa0feea134484");

pub(super) fn decode_events_2(data: &[u8]) -> Option<super::CurrentEvent> {
    if let Some(value) = WalletCashEntitlementClearedEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletCashEntitlementClearedEvent(
            value,
        ));
    }
    if let Some(value) = WalletExpiredBackingRefundEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletExpiredBackingRefundEvent(value));
    }
    if let Some(value) = WalletOutcomeEntitlementClearedEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletOutcomeEntitlementClearedEvent(
            value,
        ));
    }
    if let Some(value) = WalletOutcomeTokensWithdrawnEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletOutcomeTokensWithdrawnEvent(
            value,
        ));
    }
    if let Some(value) = WalletPrincipalPrizeClaimedEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletPrincipalPrizeClaimedEvent(value));
    }
    if let Some(value) = WalletPrizeClaimedEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletPrizeClaimedEvent(value));
    }
    if let Some(value) = WalletPrizeEntitlementClearedEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletPrizeEntitlementClearedEvent(
            value,
        ));
    }
    if let Some(value) = WalletTradeEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletTradeEvent(value));
    }
    if let Some(value) = WinnerTokenLaunchEvent::deserialize(data) {
        return Some(super::CurrentEvent::WinnerTokenLaunchEvent(value));
    }
    None
}
