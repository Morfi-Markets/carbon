//! Borsh layouts for the current Markets config accounts.
//!
//! Decoding checks the discriminator and consumes the full account data.
//! Callers verify the account owner, executable flag, and PDA.

use {
    crate::{deserialize::discriminator, types::FeeConfig},
    carbon_core::{borsh, deserialize::CarbonDeserialize},
    solana_pubkey::Pubkey,
};

/// Operational roles and market creation policy.
#[derive(Debug, Clone, borsh::BorshDeserialize, PartialEq, Eq)]
pub struct AdminConfig {
    pub config_admin: Pubkey,
    pub gift_admin: Pubkey,
    pub bump: u8,
    pub conviction_market_creation_restricted: bool,
    pub timed_market_creation_restricted: bool,
    pub transaction_authority: Pubkey,
    pub rent_treasury: Pubkey,
}

impl AdminConfig {
    /// Anchor discriminator at the start of the account data.
    pub const DISCRIMINATOR: [u8; 8] = discriminator("0x9c0a4fa147093e4d");
}

impl CarbonDeserialize for AdminConfig {
    const DISCRIMINATOR: &'static [u8] = &Self::DISCRIMINATOR;

    fn deserialize(data: &[u8]) -> Option<Self> {
        deserialize_config(data)
    }
}

/// Per-preset gifting authority and immutable fee.
#[derive(Debug, Clone, borsh::BorshDeserialize, PartialEq, Eq)]
pub struct GiftConfig {
    pub market_config: Pubkey,
    pub backend_authority: Pubkey,
    pub label: [u8; 32],
    pub gift_fee_bps: u16,
    pub admin_bump: u8,
    pub bump: u8,
}

impl GiftConfig {
    /// Anchor discriminator at the start of the account data.
    pub const DISCRIMINATOR: [u8; 8] = discriminator("0x1e786fd0dec448c9");
}

impl CarbonDeserialize for GiftConfig {
    const DISCRIMINATOR: &'static [u8] = &Self::DISCRIMINATOR;

    fn deserialize(data: &[u8]) -> Option<Self> {
        deserialize_config(data)
    }
}

/// Preset for a market that resolves at a conviction threshold.
#[derive(Debug, Clone, borsh::BorshDeserialize, PartialEq, Eq)]
pub struct ConvictionMarketConfig {
    pub common: CommonMarketConfig,
    pub conviction_retention_per_second_ppb: u64,
    pub conviction_resolution_threshold_ppb: u64,
    pub resolution_threshold_floor_margin_ppb: u64,
    pub resolution_threshold_decay_delay_secs: u64,
    pub resolution_threshold_drop_interval_secs: u64,
    pub resolution_threshold_drop_ppb: u64,
}

impl ConvictionMarketConfig {
    /// Anchor discriminator at the start of the account data.
    pub const DISCRIMINATOR: [u8; 8] = discriminator("0xdee45531ac921219");
}

impl CarbonDeserialize for ConvictionMarketConfig {
    const DISCRIMINATOR: &'static [u8] = &Self::DISCRIMINATOR;

    fn deserialize(data: &[u8]) -> Option<Self> {
        deserialize_config(data)
    }
}

/// Preset for a market with a scheduled resolution time.
#[derive(Debug, Clone, borsh::BorshDeserialize, PartialEq, Eq)]
pub struct TimedMarketConfig {
    pub common: CommonMarketConfig,
    pub conviction_retention_per_second_ppb: u64,
}

impl TimedMarketConfig {
    /// Anchor discriminator at the start of the account data.
    pub const DISCRIMINATOR: [u8; 8] = discriminator("0x7cdf250017eba0bd");
}

impl CarbonDeserialize for TimedMarketConfig {
    const DISCRIMINATOR: &'static [u8] = &Self::DISCRIMINATOR;

    fn deserialize(data: &[u8]) -> Option<Self> {
        deserialize_config(data)
    }
}

/// Initial fields of both market config account bodies.
#[derive(Debug, Clone, borsh::BorshDeserialize, PartialEq, Eq)]
pub struct CommonMarketConfig {
    pub enabled: bool,
    pub config_nonce: u64,
    pub bump: u8,
    pub quote_mint: Pubkey,
    pub platform_treasury: Pubkey,
    pub transaction_authority: Pubkey,
    pub min_outcomes: u8,
    pub max_outcomes: u8,
    pub initial_virtual_quote: u64,
    pub initial_token_supply: u64,
    pub min_prize_pool_quote: u64,
    pub min_back_amount_quote: u64,
    pub min_quote_in_per_trade: u64,
    pub fees: FeeConfig,
    pub conviction_baseline_reserve_quote: u64,
    pub unclaimed_prize_clawback_delay_secs: u64,
    pub economic_mode: EconomicMode,
}

/// Economic mode in on-chain Borsh variant order.
#[derive(Debug, Clone, Copy, borsh::BorshDeserialize, PartialEq, Eq)]
pub enum EconomicMode {
    Curve,
    PrincipalBack,
}

fn deserialize_config<T: CarbonDeserialize>(data: &[u8]) -> Option<T> {
    T::try_from_slice(data.strip_prefix(T::DISCRIMINATOR)?).ok()
}
