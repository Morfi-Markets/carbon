//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct PlatformRewardPayoutWalletUpdated {
    pub market: Pubkey,
    pub old_wallet: Pubkey,
    pub new_wallet: Pubkey,
    pub ts: i64,
}

impl_carbon_deserialize!(
    PlatformRewardPayoutWalletUpdated,
    "0xe445a52e51cb9a1dff4ed81151a9f14d"
);
