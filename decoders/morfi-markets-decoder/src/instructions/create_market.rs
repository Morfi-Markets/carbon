//! Legacy typed creation arguments and 19-account bindings shared by both modes.
//! Current creation contexts use `current::CurrentInstruction` and a different layout.

use carbon_core::{borsh, deserialize::ArrangeAccounts};
use solana_pubkey::Pubkey;

/// PDA bumps serialized in the preserved legacy creation argument struct.
#[derive(
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct CreateMarketPdaBumps {
    pub market: u8,
    pub outcome_mint_authority: u8,
    pub market_quote_vault_authority: u8,
    pub market_quote_vault: u8,
    pub backer_share_mint: u8,
}

/// Shared Borsh arguments for legacy conviction and timed creation.
#[derive(
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct CreateMarketParams {
    pub n_outcomes: u8,
    pub market_nonce: u64,
    pub initial_backing: u64,
    pub pda_bumps: CreateMarketPdaBumps,
}

/// Legacy conviction creation instruction layout.
#[derive(
    borsh::BorshDeserialize, Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct CreateMarket {
    pub params: CreateMarketParams,
}

impl_carbon_deserialize!(CreateMarket, "0x67e261ebc8bcfbfe");

/// Legacy timed creation instruction, using the same 19-account layout.
#[derive(
    borsh::BorshDeserialize, Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct CreateTimedMarket {
    pub params: CreateMarketParams,
}

impl_carbon_deserialize!(CreateTimedMarket, "0xa660b67f1288aaca");

/// Preserved legacy creation accounts for custody, backing, and schedule identity.
#[derive(Debug, Clone)]
pub struct CreateMarketAccounts {
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
    type ArrangedAccounts = CreateMarketAccounts;

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
    ) -> Option<CreateMarketAccounts> {
        let [creator, _payer, market_config, market, _outcome_metadata, quote_mint, creator_quote_account, platform_fee_vault, market_quote_vault, backer_share_mint, creator_backer_share_ata, _system_program, _token_program, _associated_token_program, _admin_config, _config_admin, event_schedule, _event_authority, _program, ..] =
            accounts
        else {
            return None;
        };
        Some(CreateMarketAccounts {
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
