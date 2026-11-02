//! Legacy conviction creation and the 19-account bindings shared with timed creation.
//! Current creation contexts use `current::CurrentInstruction` and a different layout.

use super::super::types::CreateMarketParams;
use carbon_core::{borsh, deserialize::ArrangeAccounts};
use solana_pubkey::Pubkey;

/// Legacy conviction creation instruction layout.
#[derive(
    borsh::BorshDeserialize, Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct CreateMarket {
    pub params: CreateMarketParams,
}

impl_carbon_deserialize!(CreateMarket, "0x67e261ebc8bcfbfe");

/// Preserved legacy creation accounts for custody, backing, and schedule identity.
#[derive(Debug, Clone)]
pub struct CreateMarketInstructionAccounts {
    pub creator: Pubkey,
    pub market_config: Pubkey,
    pub market: Pubkey,
    pub quote_mint: Pubkey,
    pub creator_quote_account: Pubkey,
    pub platform_fee_vault: Pubkey,
    pub market_quote_vault: Pubkey,
    pub backer_share_mint: Pubkey,
    pub creator_backer_share_ata: Pubkey,
    pub event_schedule: Option<Pubkey>,
}

impl ArrangeAccounts for CreateMarket {
    type ArrangedAccounts = CreateMarketInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        Self::arrange_accounts_for_program(accounts, crate::PROGRAM_ID)
    }
}

impl CreateMarket {
    /// Arranges legacy creation accounts using the deployment's absent-schedule sentinel.
    ///
    /// This preserves Anchor optional-account semantics for custom deployments.
    pub fn arrange_accounts_for_program(
        accounts: &[solana_instruction::AccountMeta],
        program_id: Pubkey,
    ) -> Option<CreateMarketInstructionAccounts> {
        let [creator, _payer, market_config, market, _outcome_metadata, quote_mint, creator_quote_account, platform_fee_vault, market_quote_vault, backer_share_mint, creator_backer_share_ata, _system_program, _token_program, _associated_token_program, _admin_config, _config_admin, event_schedule, _event_authority, _program, ..] =
            accounts
        else {
            return None;
        };
        Some(CreateMarketInstructionAccounts {
            creator: creator.pubkey,
            market_config: market_config.pubkey,
            market: market.pubkey,
            quote_mint: quote_mint.pubkey,
            creator_quote_account: creator_quote_account.pubkey,
            platform_fee_vault: platform_fee_vault.pubkey,
            market_quote_vault: market_quote_vault.pubkey,
            backer_share_mint: backer_share_mint.pubkey,
            creator_backer_share_ata: creator_backer_share_ata.pubkey,
            event_schedule: (event_schedule.pubkey != program_id).then_some(event_schedule.pubkey),
        })
    }
}
