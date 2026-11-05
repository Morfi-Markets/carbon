//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
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
    /// Pre-upgrade events end before this field; it then decodes as `None`.
    #[borsh(deserialize_with = "crate::current::callout_code::deserialize")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callout_code: Option<[u8; 8]>,
}

impl_carbon_deserialize!(PrincipalTradeEvent, "0xe445a52e51cb9a1d539092d0bc043664");
