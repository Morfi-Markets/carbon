//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::{borsh, deserialize::CarbonDeserialize};
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct CidOutcomeAddedEvent {
    pub market: Pubkey,
    pub creator: Pubkey,
    pub outcome_index: u8,
    pub directory_cid: String,
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
pub struct MarketBoostedEvent {
    pub ts: i64,
    pub market: Pubkey,
    pub booster: Pubkey,
    pub prize_pool_address: Pubkey,
    pub amount_quote: u64,
}

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct MarketFundingCompleted {
    pub trading_opens_ts: i64,
}

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct MarketInitializedEvent {
    pub market: Pubkey,
    pub creator: Pubkey,
    pub config: Pubkey,
    pub directory_cid: String,
    pub n_outcomes: u8,
    pub resolution_mode: u8,
    pub ts: i64,
}

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct MarketQuoteCustodyClosedEvent {
    pub market: Pubkey,
    pub rent_recipient: Pubkey,
    pub surplus_quote: u64,
    pub rent_lamports: u64,
    pub ts: i64,
}

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct PlatformRewardPayoutWalletUpdated {
    pub market: Pubkey,
    pub old_wallet: Pubkey,
    pub new_wallet: Pubkey,
    pub ts: i64,
}

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct PoolRewardClaimed {
    pub market: Pubkey,
    pub snapshot_wallet: Pubkey,
    pub payout_wallet: Pubkey,
    pub member_index: u8,
    pub amount_quote: u64,
    pub claimed_quote: u64,
    pub cumulative_collected_quote: u64,
    pub ts: i64,
}

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct PoolRewardsCollected {
    pub market: Pubkey,
    pub pool: Pubkey,
    pub quote_mint: Pubkey,
    pub collected_quote: u64,
    pub cumulative_collected_quote: u64,
    pub platform_paid_quote: u64,
    pub holder_budget_quote: u64,
    pub ts: i64,
}

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct PositionSlotLifecycleEvent {
    pub action: u8,
    pub wallet: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub kind: u8,
    pub market: Pubkey,
    pub outcome_index: u8,
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

impl_carbon_deserialize!(CidOutcomeAddedEvent, "0xe445a52e51cb9a1dba84f932007149db");
impl_carbon_deserialize!(
    ExpiredWinningTokensSweptEvent,
    "0xe445a52e51cb9a1df65c97b1977759eb"
);
impl_carbon_deserialize!(LogicalResolutionEvent, "0xe445a52e51cb9a1d31e17ecde731e4fd");
impl_carbon_deserialize!(MarketBoostedEvent, "0xe445a52e51cb9a1d943a6a50aa0ad64b");
impl_carbon_deserialize!(MarketFundingCompleted, "0xe445a52e51cb9a1dbab495e6c17fcf0e");
impl_carbon_deserialize!(MarketInitializedEvent, "0xe445a52e51cb9a1d46ad60ca648f2d19");
impl_carbon_deserialize!(
    MarketQuoteCustodyClosedEvent,
    "0xe445a52e51cb9a1d42cd7fd1c6fe0b3a"
);
impl_carbon_deserialize!(
    PlatformRewardPayoutWalletUpdated,
    "0xe445a52e51cb9a1dff4ed81151a9f14d"
);
impl_carbon_deserialize!(PoolRewardClaimed, "0xe445a52e51cb9a1dc1e60e0cde3bb1c4");
impl_carbon_deserialize!(PoolRewardsCollected, "0xe445a52e51cb9a1daa886088a2adb2db");
impl_carbon_deserialize!(
    PositionSlotLifecycleEvent,
    "0xe445a52e51cb9a1d3b7342710d125cee"
);
impl_carbon_deserialize!(
    PrincipalCashLiabilitiesSweptEvent,
    "0xe445a52e51cb9a1d3bf021b7a9e8a955"
);

pub(super) fn decode_events_0(data: &[u8]) -> Option<super::CurrentEvent> {
    if let Some(value) = CidOutcomeAddedEvent::deserialize(data) {
        return Some(super::CurrentEvent::CidOutcomeAddedEvent(value));
    }
    if let Some(value) = ExpiredWinningTokensSweptEvent::deserialize(data) {
        return Some(super::CurrentEvent::ExpiredWinningTokensSweptEvent(value));
    }
    if let Some(value) = LogicalResolutionEvent::deserialize(data) {
        return Some(super::CurrentEvent::LogicalResolutionEvent(value));
    }
    if let Some(value) = MarketBoostedEvent::deserialize(data) {
        return Some(super::CurrentEvent::MarketBoostedEvent(value));
    }
    if let Some(value) = MarketFundingCompleted::deserialize(data) {
        return Some(super::CurrentEvent::MarketFundingCompleted(value));
    }
    if let Some(value) = MarketInitializedEvent::deserialize(data) {
        return Some(super::CurrentEvent::MarketInitializedEvent(value));
    }
    if let Some(value) = MarketQuoteCustodyClosedEvent::deserialize(data) {
        return Some(super::CurrentEvent::MarketQuoteCustodyClosedEvent(value));
    }
    if let Some(value) = PlatformRewardPayoutWalletUpdated::deserialize(data) {
        return Some(super::CurrentEvent::PlatformRewardPayoutWalletUpdated(
            value,
        ));
    }
    if let Some(value) = PoolRewardClaimed::deserialize(data) {
        return Some(super::CurrentEvent::PoolRewardClaimed(value));
    }
    if let Some(value) = PoolRewardsCollected::deserialize(data) {
        return Some(super::CurrentEvent::PoolRewardsCollected(value));
    }
    if let Some(value) = PositionSlotLifecycleEvent::deserialize(data) {
        return Some(super::CurrentEvent::PositionSlotLifecycleEvent(value));
    }
    if let Some(value) = PrincipalCashLiabilitiesSweptEvent::deserialize(data) {
        return Some(super::CurrentEvent::PrincipalCashLiabilitiesSweptEvent(
            value,
        ));
    }
    None
}
