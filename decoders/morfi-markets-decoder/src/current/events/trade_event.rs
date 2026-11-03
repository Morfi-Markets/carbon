//! Hand-written mirror of the contract's `TradeEvent`, one event for both
//! economic modes. Field order and types must match
//! `markets/programs/markets/src/instructions/trading/trade/events.rs`.
//! `tests/current_event_bytes.rs` checks the Borsh field order, and Morfi's
//! `backend/indexer-service/tests/idl_decoder_layouts.rs` checks the name and
//! discriminator against the committed IDL.
//!
//! The name, and so the discriminator, equals the legacy `events::TradeEvent`.
//! `MorfiMarketsDecoder` stops at a recognized current discriminator, so a body
//! this layout rejects never decodes as the legacy event.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct TradeEvent {
    pub market: Pubkey,
    pub trader: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub outcome_index: u8,
    /// Zero for buy and one for sell.
    pub side: u8,
    /// D6 quote added to or removed from this outcome's pool (x), which is also
    /// the change of its real quote. The wallet amount is this delta plus the
    /// fees on a buy and minus the fees on a sell.
    pub pool_quote_delta_d6: u64,
    /// D6 units credited on a buy or debited on a sell.
    pub outcome_token_amount_d6: u64,
    pub base_fee_to_backers_d6: u64,
    pub base_fee_to_platform_d6: u64,
    pub prize_pool_contribution_d6: u64,
    /// Trader's slot principal after the trade; zero in Curve mode.
    pub post_position_principal_quote_d6: u64,
    /// Market real quote after the trade: the sum of (x − V0) over the active
    /// outcomes.
    pub post_total_quote_reserve_d6: u64,
    /// x after the trade: the USDC in this outcome's pool, V0 included.
    /// x − V0 is the outcome's real quote.
    pub post_pool_quote_d6: u64,
    /// Units held for this outcome after the trade. The pool's token side is
    /// y = T0 − this.
    pub post_token_entitlement_supply_d6: u64,
    pub post_outcome_conviction_ppb: Vec<u32>,
    pub post_outcome_conviction_target_ppb: Vec<u32>,
    /// Cluster time; conviction is always advanced to exactly this time.
    pub ts: i64,
}

impl_carbon_deserialize!(TradeEvent, "0xe445a52e51cb9a1dbddb7fd34ee661ee");
