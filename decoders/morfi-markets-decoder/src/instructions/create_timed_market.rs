//! Legacy timed creation, reusing the conviction creation account layout.

use super::super::types::CreateMarketParams;
use super::{CreateMarket, CreateMarketAccounts};
use carbon_core::{borsh, deserialize::ArrangeAccounts};
use solana_pubkey::Pubkey;

/// Legacy timed creation instruction, using the same 19-account layout.
#[derive(
    borsh::BorshDeserialize, Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct CreateTimedMarket {
    pub params: CreateMarketParams,
}

impl_carbon_deserialize!(CreateTimedMarket, "0xa660b67f1288aaca");

impl CreateTimedMarket {
    /// Arranges legacy creation accounts using the deployment's absent-schedule sentinel.
    pub fn arrange_accounts_for_program(
        accounts: &[solana_instruction::AccountMeta],
        program_id: Pubkey,
    ) -> Option<CreateMarketAccounts> {
        CreateMarket::arrange_accounts_for_program(accounts, program_id)
    }
}

impl ArrangeAccounts for CreateTimedMarket {
    type ArrangedAccounts = CreateMarketAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        Self::arrange_accounts_for_program(accounts, crate::PROGRAM_ID)
    }
}
