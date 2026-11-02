//! Generated from the frozen Markets IDL; do not modify the contract source.
use carbon_core::borsh;
use solana_pubkey::Pubkey;

#[derive(
    Debug, Clone, borsh::BorshDeserialize, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash,
)]
pub struct CidOutcomeAddedEvent {
    pub market: Pubkey,
    pub creator: Pubkey,
    pub outcome_index: u8,
    pub directory_cid: String,
    pub ts: i64,
}

impl_carbon_deserialize!(CidOutcomeAddedEvent, "0xe445a52e51cb9a1dba84f932007149db");
