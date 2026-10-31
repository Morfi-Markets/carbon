//! Legacy typed backing layout retained for compatibility.
//! Use `current::CurrentInstruction` for the current account ordering.

use carbon_core::{borsh, deserialize::ArrangeAccounts};

/// Quote deposit argument for the preserved legacy backing layout.
#[derive(
    borsh::BorshDeserialize, Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Back {
    pub amount: u64,
}

impl_carbon_deserialize!(Back, "0x01906db16e837c00");

impl ArrangeAccounts for Back {
    type ArrangedAccounts = solana_pubkey::Pubkey;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [_backer, market, _market_config, _quote_mint, _backer_quote_account, _backer_share_mint, _backer_share_account, _market_quote_vault_authority, _market_quote_vault, _platform_fee_vault, _system_program, _token_program, _associated_token_program, _event_schedule, _event_authority, _program, ..] =
            accounts
        else {
            return None;
        };
        Some(market.pubkey)
    }
}
