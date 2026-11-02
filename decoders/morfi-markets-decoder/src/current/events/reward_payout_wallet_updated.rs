//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct RewardPayoutWalletUpdated {
    pub market: Pubkey,
    pub member_index: u8,
    pub snapshot_wallet: Pubkey,
    pub old_wallet: Pubkey,
    pub new_wallet: Pubkey,
    pub ts: i64,
}

impl_carbon_deserialize!(
    RewardPayoutWalletUpdated,
    "0xe445a52e51cb9a1dc611e35c4902c17c"
);
