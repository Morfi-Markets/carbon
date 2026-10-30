//! Current markets BackingEvent emitted by back/unback.

use carbon_core::borsh;

#[derive(
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Clone,
    Hash,
)]
pub struct BackingEvent {
    pub market: solana_pubkey::Pubkey,
    pub backer: solana_pubkey::Pubkey,
    pub side: u8,
    /// Curve input / net stake — the instruction's `amount` argument under
    /// fee-on-top semantics. Mirrors `TradeEvent.quote_amount_gross` (which
    /// also equals the curve input). On BACK (side=0) the backer pays
    /// `gross_amount + platform_fee` total; on UNBACK (side=1) it's the
    /// burn amount. Field name kept for IDL/wire stability — see
    /// `contracts/markets/programs/markets/src/instructions/backing/back.rs`.
    pub gross_amount: u64,
    pub platform_fee: u64,
    pub prize_pool_delta: i128,
    pub shares_delta: i128,
    pub backed_total: u64,
    pub share_supply: u64,
    pub backer_balance: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(BackingEvent, "0xe445a52e51cb9a1d2b46662ed2c62729");
