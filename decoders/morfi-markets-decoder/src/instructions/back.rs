//! Typed backing amount with the current `back` account order.
//! The decoder returns `current::CurrentInstruction` for this discriminator first.
//! Use this arrangement to find the market of a parent `back` instruction.

use carbon_core::{borsh, deserialize::ArrangeAccounts};

/// Quote deposit argument of the backing instruction.
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
        let [_backer, _payer, _transaction_authority, market, _market_config, _quote_mint, _backer_quote_account, _user_positions, _market_quote_vault_authority, _market_quote_vault, _platform_fee_vault, _token_program, _system_program, _event_schedule, _event_authority, _program, ..] =
            accounts
        else {
            return None;
        };
        Some(market.pubkey)
    }
}
