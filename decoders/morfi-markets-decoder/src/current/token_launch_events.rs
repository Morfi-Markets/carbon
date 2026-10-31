//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::{borsh, deserialize::CarbonDeserialize};
use solana_pubkey::Pubkey;

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
    PrincipalOpinionCoinLaunchEvent,
    "0xe445a52e51cb9a1db995f23b084f211a"
);
impl_carbon_deserialize!(WinnerTokenLaunchEvent, "0xe445a52e51cb9a1d03bfa0feea134484");

pub(super) fn decode_token_launch_events(data: &[u8]) -> Option<super::CurrentEvent> {
    if let Some(value) = PrincipalOpinionCoinLaunchEvent::deserialize(data) {
        return Some(super::CurrentEvent::PrincipalOpinionCoinLaunchEvent(value));
    }
    if let Some(value) = WinnerTokenLaunchEvent::deserialize(data) {
        return Some(super::CurrentEvent::WinnerTokenLaunchEvent(value));
    }
    None
}
