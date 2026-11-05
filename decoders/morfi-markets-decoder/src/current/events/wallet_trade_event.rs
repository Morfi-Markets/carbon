//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

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
    #[serde(serialize_with = "crate::current::u128_as_string::serialize_u128")]
    pub post_quote_reserves: u128,
    pub bc_supply_after: u64,
    pub used_penalty_fee: bool,
    pub post_outcome_conviction_ppb: Vec<u32>,
    pub post_outcome_conviction_target_ppb: Vec<u32>,
    pub post_conviction_retention_per_second_ppb: u32,
    pub post_last_conviction_update_ts: i64,
    pub ts: i64,
    pub post_total_real_quote_reserves: u64,
    /// Pre-upgrade events end before this field; it then decodes as `None`.
    #[borsh(deserialize_with = "crate::current::callout_code::deserialize")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callout_code: Option<[u8; 8]>,
}

impl_carbon_deserialize!(WalletTradeEvent, "0xe445a52e51cb9a1dfbd80b0838f398f7");
