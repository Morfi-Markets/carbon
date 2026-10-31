//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::{borsh, deserialize::CarbonDeserialize};
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct PrincipalTradeEvent {
    pub market: Pubkey,
    pub trader: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub outcome_index: u8,
    pub side: u8,
    pub trader_quote_amount_d6: u64,
    pub principal_delta_d6: u64,
    pub pricing_quote_delta_d6: u64,
    pub outcome_token_amount_d6: u64,
    pub base_fee_to_backers_d6: u64,
    pub base_fee_to_platform_d6: u64,
    pub prize_pool_contribution_d6: u64,
    pub post_position_principal_quote_d6: u64,
    pub post_outcome_principal_quote_total_d6: u64,
    pub post_total_principal_quote_d6: u64,
    pub post_pricing_quote_reserve_d6: u64,
    pub post_issued_tokens_d6: u64,
    pub used_penalty_fee: bool,
    pub post_outcome_conviction_ppb: Vec<u32>,
    pub post_outcome_conviction_target_ppb: Vec<u32>,
    pub post_conviction_retention_per_second_ppb: u32,
    pub post_last_conviction_update_ts: i64,
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
    #[serde(serialize_with = "super::u128_as_string::serialize_u128")]
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
pub struct WalletBackingEvent {
    pub market: Pubkey,
    pub backer: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub side: u8,
    pub gross_amount: u64,
    pub platform_fee: u64,
    pub backed_total: u64,
    pub share_supply: u64,
    pub backer_balance: u64,
    pub ts: i64,
}

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

impl_carbon_deserialize!(PrincipalTradeEvent, "0xe445a52e51cb9a1d539092d0bc043664");
impl_carbon_deserialize!(WalletTradeEvent, "0xe445a52e51cb9a1dfbd80b0838f398f7");
impl_carbon_deserialize!(WalletBackingEvent, "0xe445a52e51cb9a1d11c2ab6068d7b7e7");
impl_carbon_deserialize!(
    WalletBackerFeeClaimedEvent,
    "0xe445a52e51cb9a1dca1a0aaf4e078bca"
);
impl_carbon_deserialize!(
    WalletBackerEntitlementForfeitedEvent,
    "0xe445a52e51cb9a1d2a155df0e31bbbef"
);
impl_carbon_deserialize!(
    WalletExpiredBackingRefundEvent,
    "0xe445a52e51cb9a1d1f43904ad2fec074"
);

pub(super) fn decode_trade_and_backing_events(data: &[u8]) -> Option<super::CurrentEvent> {
    if let Some(value) = PrincipalTradeEvent::deserialize(data) {
        return Some(super::CurrentEvent::PrincipalTradeEvent(value));
    }
    if let Some(value) = WalletTradeEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletTradeEvent(value));
    }
    if let Some(value) = WalletBackingEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletBackingEvent(value));
    }
    if let Some(value) = WalletBackerFeeClaimedEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletBackerFeeClaimedEvent(value));
    }
    if let Some(value) = WalletBackerEntitlementForfeitedEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletBackerEntitlementForfeitedEvent(
            value,
        ));
    }
    if let Some(value) = WalletExpiredBackingRefundEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletExpiredBackingRefundEvent(value));
    }
    None
}
