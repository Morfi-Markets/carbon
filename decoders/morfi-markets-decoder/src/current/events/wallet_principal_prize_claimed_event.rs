//! Hand-written mirror of the contract's `WalletPrincipalPrizeClaimedEvent`.
//! `tests/current_event_bytes.rs` checks its Borsh field order, and Morfi's
//! `backend/indexer-service/tests/idl_decoder_layouts.rs` checks its name and
//! discriminator against the committed IDL.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct WalletPrincipalPrizeClaimedEvent {
    pub market: Pubkey,
    pub claimer: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub winning_outcome: u8,
    pub outcome_units_debited: u64,
    pub prize_units_debited: u64,
    pub principal_gross: u64,
    /// v1 field name kept with the v1 event name. It carries the claim fee on
    /// the bonus, not a sell fee.
    pub platform_sell_fee: u64,
    pub prize_payout: u64,
    /// The winner's pool_quote (x) after the claim, V0 included.
    pub winner_quote_reserve_after: u64,
    pub prize_liability_after: u64,
    pub fee_vault_index: u8,
    pub ts: i64,
}

impl_carbon_deserialize!(
    WalletPrincipalPrizeClaimedEvent,
    "0xe445a52e51cb9a1d8ed20a7c618fa2b8"
);
