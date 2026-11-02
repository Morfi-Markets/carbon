//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct PrincipalCashLiabilitiesSweptEvent {
    pub market: Pubkey,
    pub principal_swept: u64,
    pub prize_swept: u64,
    pub platform_fee_vault: Pubkey,
    pub ts: i64,
}

impl_carbon_deserialize!(
    PrincipalCashLiabilitiesSweptEvent,
    "0xe445a52e51cb9a1d3bf021b7a9e8a955"
);
