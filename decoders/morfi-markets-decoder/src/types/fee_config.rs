//! Current contract fee policy, embedded in market configuration parameters.
use carbon_core::borsh;

/// Immutable per-market fee rates in their on-chain Borsh field order.
#[derive(
    Debug,
    Clone,
    Copy,
    borsh::BorshSerialize,
    borsh::BorshDeserialize,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Hash,
)]
pub struct FeeConfig {
    pub platform_buy_fee_bps: u16,
    pub platform_sell_fee_bps: u16,
    pub platform_claim_fee_bps: u16,
    pub platform_back_fee_bps: u16,
    pub platform_unback_fee_bps: u16,
    pub backer_trade_fee_bps: u16,
    pub prize_pool_min_fee_bps: u16,
    pub buy_prize_pool_max_fee_bps: u16,
    pub sell_prize_pool_max_fee_bps: u16,
    pub sell_progress_delay_factor: u16,
    pub sell_gap_weight_bps: u16,
    pub buy_reference_threshold_ppb: u64,
}
