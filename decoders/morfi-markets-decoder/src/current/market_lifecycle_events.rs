//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::{borsh, deserialize::CarbonDeserialize};
use solana_pubkey::Pubkey;

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
pub struct WinningOutcomeVaultClosedEvent {
    pub market: Pubkey,
    pub finalizer: Pubkey,
    pub outcome_index: u8,
    pub outcome_mint: Pubkey,
    pub outcome_token_vault: Pubkey,
    pub surplus_burned: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(MarketInitializedEvent, "0xe445a52e51cb9a1d46ad60ca648f2d19");
impl_carbon_deserialize!(MarketBoostedEvent, "0xe445a52e51cb9a1d943a6a50aa0ad64b");
impl_carbon_deserialize!(MarketFundingCompleted, "0xe445a52e51cb9a1dbab495e6c17fcf0e");
impl_carbon_deserialize!(CidOutcomeAddedEvent, "0xe445a52e51cb9a1dba84f932007149db");
impl_carbon_deserialize!(
    MarketQuoteCustodyClosedEvent,
    "0xe445a52e51cb9a1d42cd7fd1c6fe0b3a"
);
impl_carbon_deserialize!(
    WinningOutcomeVaultClosedEvent,
    "0xe445a52e51cb9a1d8adb68302784b95d"
);

pub(super) fn decode_market_lifecycle_events(data: &[u8]) -> Option<super::CurrentEvent> {
    if let Some(value) = MarketInitializedEvent::deserialize(data) {
        return Some(super::CurrentEvent::MarketInitializedEvent(value));
    }
    if let Some(value) = MarketBoostedEvent::deserialize(data) {
        return Some(super::CurrentEvent::MarketBoostedEvent(value));
    }
    if let Some(value) = MarketFundingCompleted::deserialize(data) {
        return Some(super::CurrentEvent::MarketFundingCompleted(value));
    }
    if let Some(value) = CidOutcomeAddedEvent::deserialize(data) {
        return Some(super::CurrentEvent::CidOutcomeAddedEvent(value));
    }
    if let Some(value) = MarketQuoteCustodyClosedEvent::deserialize(data) {
        return Some(super::CurrentEvent::MarketQuoteCustodyClosedEvent(value));
    }
    if let Some(value) = WinningOutcomeVaultClosedEvent::deserialize(data) {
        return Some(super::CurrentEvent::WinningOutcomeVaultClosedEvent(value));
    }
    None
}
