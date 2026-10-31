//! Current timed preset initializer and named accounts from the contract IDL.
use super::super::types::InitTimedMarketConfigParams;
use carbon_core::borsh;
use carbon_core::deserialize::ArrangeAccounts;

/// Current timed preset initializer with shared economics and retention.
#[derive(
    Debug,
    Clone,
    borsh::BorshSerialize,
    borsh::BorshDeserialize,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Hash,
)]
pub struct InitTimedMarketConfig {
    pub params: InitTimedMarketConfigParams,
}

impl_carbon_deserialize!(InitTimedMarketConfig, "0xaa50f23c735f8873");

impl ArrangeAccounts for InitTimedMarketConfig {
    type ArrangedAccounts =
        super::init_conviction_market_config::InitConvictionMarketConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        // Both current initializers have the same 19 named accounts in the same order.
        super::InitConvictionMarketConfig::arrange_accounts(accounts)
    }
}
