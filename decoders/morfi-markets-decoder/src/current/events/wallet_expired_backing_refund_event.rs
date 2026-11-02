//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct WalletExpiredBackingRefundEvent {
    pub market: Pubkey,
    pub backer: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub units_debited: u64,
    pub quote_refunded: u64,
    pub remaining_prize_liability: u64,
    pub remaining_share_supply: u64,
    pub backer_balance: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(
    WalletExpiredBackingRefundEvent,
    "0xe445a52e51cb9a1d1f43904ad2fec074"
);
