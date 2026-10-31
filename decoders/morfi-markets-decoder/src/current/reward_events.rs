//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::{borsh, deserialize::CarbonDeserialize};
use solana_pubkey::Pubkey;

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
pub struct PlatformRewardPayoutWalletUpdated {
    pub market: Pubkey,
    pub old_wallet: Pubkey,
    pub new_wallet: Pubkey,
    pub ts: i64,
}

impl_carbon_deserialize!(PoolRewardClaimed, "0xe445a52e51cb9a1dc1e60e0cde3bb1c4");
impl_carbon_deserialize!(PoolRewardsCollected, "0xe445a52e51cb9a1daa886088a2adb2db");
impl_carbon_deserialize!(
    RewardPayoutWalletUpdated,
    "0xe445a52e51cb9a1dc611e35c4902c17c"
);
impl_carbon_deserialize!(
    PlatformRewardPayoutWalletUpdated,
    "0xe445a52e51cb9a1dff4ed81151a9f14d"
);

pub(super) fn decode_reward_events(data: &[u8]) -> Option<super::CurrentEvent> {
    if let Some(value) = PoolRewardClaimed::deserialize(data) {
        return Some(super::CurrentEvent::PoolRewardClaimed(value));
    }
    if let Some(value) = PoolRewardsCollected::deserialize(data) {
        return Some(super::CurrentEvent::PoolRewardsCollected(value));
    }
    if let Some(value) = RewardPayoutWalletUpdated::deserialize(data) {
        return Some(super::CurrentEvent::RewardPayoutWalletUpdated(value));
    }
    if let Some(value) = PlatformRewardPayoutWalletUpdated::deserialize(data) {
        return Some(super::CurrentEvent::PlatformRewardPayoutWalletUpdated(
            value,
        ));
    }
    None
}
