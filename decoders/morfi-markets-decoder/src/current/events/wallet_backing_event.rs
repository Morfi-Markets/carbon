//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct WalletBackingEvent {
    pub market: Pubkey,
    pub backer: Pubkey,
    pub user_positions: Pubkey,
    pub pda_slot_idx: u16,
    pub side: u8,
    pub gross_amount: u64,
    pub platform_fee: u64,
    pub backed_total: u64,
    pub share_supply: u64,
    pub backer_balance: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(WalletBackingEvent, "0xe445a52e51cb9a1d11c2ab6068d7b7e7");
