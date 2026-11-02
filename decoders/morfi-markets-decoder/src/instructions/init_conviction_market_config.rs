//! Current conviction preset initializer and named accounts from the contract IDL.
use super::super::types::InitConvictionMarketConfigParams;
use carbon_core::account_utils::next_account;
use carbon_core::borsh;
use carbon_core::deserialize::ArrangeAccounts;

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
pub struct InitConvictionMarketConfig {
    pub params: InitConvictionMarketConfigParams,
}

impl_carbon_deserialize!(InitConvictionMarketConfig, "0x0f811026e5c8d1f9");

#[derive(Debug, Clone, PartialEq)]
pub struct InitConvictionMarketConfigInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub config_admin: solana_pubkey::Pubkey,
    pub admin_config: solana_pubkey::Pubkey,
    pub market_config: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub platform_fee_vault: solana_pubkey::Pubkey,
    pub platform_fee_vault1: solana_pubkey::Pubkey,
    pub platform_fee_vault2: solana_pubkey::Pubkey,
    pub platform_fee_vault3: solana_pubkey::Pubkey,
    pub platform_fee_vault4: solana_pubkey::Pubkey,
    pub platform_fee_vault5: solana_pubkey::Pubkey,
    pub platform_fee_vault6: solana_pubkey::Pubkey,
    pub platform_fee_vault7: solana_pubkey::Pubkey,
    pub morfi_amm_config: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub program_data: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
    pub remaining: Vec<solana_instruction::AccountMeta>,
}

impl ArrangeAccounts for InitConvictionMarketConfig {
    type ArrangedAccounts = InitConvictionMarketConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let payer = next_account(&mut iter)?;
        let config_admin = next_account(&mut iter)?;
        let admin_config = next_account(&mut iter)?;
        let market_config = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let platform_fee_vault = next_account(&mut iter)?;
        let platform_fee_vault1 = next_account(&mut iter)?;
        let platform_fee_vault2 = next_account(&mut iter)?;
        let platform_fee_vault3 = next_account(&mut iter)?;
        let platform_fee_vault4 = next_account(&mut iter)?;
        let platform_fee_vault5 = next_account(&mut iter)?;
        let platform_fee_vault6 = next_account(&mut iter)?;
        let platform_fee_vault7 = next_account(&mut iter)?;
        let morfi_amm_config = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let program_data = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(InitConvictionMarketConfigInstructionAccounts {
            payer,
            config_admin,
            admin_config,
            market_config,
            quote_mint,
            platform_fee_vault,
            platform_fee_vault1,
            platform_fee_vault2,
            platform_fee_vault3,
            platform_fee_vault4,
            platform_fee_vault5,
            platform_fee_vault6,
            platform_fee_vault7,
            morfi_amm_config,
            system_program,
            token_program,
            program_data,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
