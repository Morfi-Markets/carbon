//! Current contract instruction parameters, following Carbon 0.12 codegen layouts.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug,
    Clone,
    borsh::BorshSerialize,
    borsh::BorshDeserialize,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Hash,
)]
pub struct CommonMarketConfigParams {
    pub config_nonce: u64,
    pub platform_treasury: Pubkey,
    pub transaction_authority: Pubkey,
    pub min_outcomes: u8,
    pub max_outcomes: u8,
    pub initial_virtual_quote: u64,
    pub initial_token_supply: u64,
    pub min_prize_pool_quote: u64,
    pub min_back_amount_quote: u64,
    pub min_quote_in_per_trade: u64,
    pub backer_share_bps: u16,
    pub platform_share_bps: u16,
    pub fee_bps_backing: u16,
    pub fee_bps_unbacking: u16,
    pub fee_bps_buy: u16,
    pub fee_bps_sell: u16,
    pub fee_bps_sell_penalty_extra: u16,
    pub fee_bps_prize_pool: u16,
    pub conviction_baseline_reserve_quote: u64,
    pub conviction_reference_margin_ppb: u64,
    pub unclaimed_prize_clawback_delay_secs: u64,
    pub economic_mode: u8,
}
