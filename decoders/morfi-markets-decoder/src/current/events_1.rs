//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::{borsh, deserialize::CarbonDeserialize};
use solana_pubkey::Pubkey;

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
pub struct PrincipalOpinionCoinLaunchEvent {
    pub market: Pubkey,
    pub winning_outcome: u8,
    pub winning_mint: Pubkey,
    pub amm_pool: Pubkey,
    pub platform_payout_wallet: Pubkey,
    pub quote_seeded_to_amm: u64,
    pub amm_seed_tokens: u64,
    pub winning_mint_supply_after_d6: u64,
    pub ts: i64,
}

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
pub struct RewardPayoutWalletUpdated {
    pub market: Pubkey,
    pub member_index: u8,
    pub snapshot_wallet: Pubkey,
    pub old_wallet: Pubkey,
    pub new_wallet: Pubkey,
    pub ts: i64,
}

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct UserPositionsClosedEvent {
    pub wallet: Pubkey,
    pub user_positions: Pubkey,
    pub capacity: u16,
    pub treasury: Pubkey,
    pub rent_refunded_lamports: u64,
}

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct UserPositionsInitializedEvent {
    pub wallet: Pubkey,
    pub user_positions: Pubkey,
    pub capacity: u16,
    pub payer: Pubkey,
    pub rent_lamports: u64,
}

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct UserPositionsResizedEvent {
    pub wallet: Pubkey,
    pub user_positions: Pubkey,
    pub old_capacity: u16,
    pub new_capacity: u16,
    pub payer: Pubkey,
    pub treasury: Pubkey,
    pub rent_paid_lamports: u64,
    pub rent_refunded_lamports: u64,
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

impl_carbon_deserialize!(
    PrincipalCashResolutionEvent,
    "0xe445a52e51cb9a1d194f5e95567c4ba9"
);
impl_carbon_deserialize!(
    PrincipalOpinionCoinLaunchEvent,
    "0xe445a52e51cb9a1db995f23b084f211a"
);
impl_carbon_deserialize!(PrincipalTradeEvent, "0xe445a52e51cb9a1d539092d0bc043664");
impl_carbon_deserialize!(
    RewardPayoutWalletUpdated,
    "0xe445a52e51cb9a1dc611e35c4902c17c"
);
impl_carbon_deserialize!(
    UserPositionsClosedEvent,
    "0xe445a52e51cb9a1d608687a5f12cc3ce"
);
impl_carbon_deserialize!(
    UserPositionsInitializedEvent,
    "0xe445a52e51cb9a1dd9c8d87c43381efe"
);
impl_carbon_deserialize!(
    UserPositionsResizedEvent,
    "0xe445a52e51cb9a1d402a7541dd0bc3d4"
);
impl_carbon_deserialize!(
    WalletBackerEntitlementForfeitedEvent,
    "0xe445a52e51cb9a1d2a155df0e31bbbef"
);
impl_carbon_deserialize!(
    WalletBackerFeeClaimedEvent,
    "0xe445a52e51cb9a1dca1a0aaf4e078bca"
);
impl_carbon_deserialize!(WalletBackingEvent, "0xe445a52e51cb9a1d11c2ab6068d7b7e7");

pub(super) fn decode_events_1(data: &[u8]) -> Option<super::CurrentEvent> {
    if let Some(value) = PrincipalCashResolutionEvent::deserialize(data) {
        return Some(super::CurrentEvent::PrincipalCashResolutionEvent(value));
    }
    if let Some(value) = PrincipalOpinionCoinLaunchEvent::deserialize(data) {
        return Some(super::CurrentEvent::PrincipalOpinionCoinLaunchEvent(value));
    }
    if let Some(value) = PrincipalTradeEvent::deserialize(data) {
        return Some(super::CurrentEvent::PrincipalTradeEvent(value));
    }
    if let Some(value) = RewardPayoutWalletUpdated::deserialize(data) {
        return Some(super::CurrentEvent::RewardPayoutWalletUpdated(value));
    }
    if let Some(value) = UserPositionsClosedEvent::deserialize(data) {
        return Some(super::CurrentEvent::UserPositionsClosedEvent(value));
    }
    if let Some(value) = UserPositionsInitializedEvent::deserialize(data) {
        return Some(super::CurrentEvent::UserPositionsInitializedEvent(value));
    }
    if let Some(value) = UserPositionsResizedEvent::deserialize(data) {
        return Some(super::CurrentEvent::UserPositionsResizedEvent(value));
    }
    if let Some(value) = WalletBackerEntitlementForfeitedEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletBackerEntitlementForfeitedEvent(
            value,
        ));
    }
    if let Some(value) = WalletBackerFeeClaimedEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletBackerFeeClaimedEvent(value));
    }
    if let Some(value) = WalletBackingEvent::deserialize(data) {
        return Some(super::CurrentEvent::WalletBackingEvent(value));
    }
    None
}
