//! Legacy typed outcome-addition layout with exactly five arranged accounts.
//! Current outcome-addition contexts use `current::CurrentInstruction`.

use super::OutcomePreparation;
use carbon_core::{borsh, deserialize::ArrangeAccounts};
use solana_pubkey::Pubkey;

/// Preserved legacy outcome-addition instruction layout.
#[derive(
    borsh::BorshDeserialize, Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct AddOutcome {
    pub outcome: OutcomePreparation,
}

impl_carbon_deserialize!(AddOutcome, "0xa2decab9b0def834");

/// Market identity from the exact five-account legacy outcome-addition layout.
#[derive(Debug, Clone)]
pub struct AddOutcomeAccounts {
    pub market: Pubkey,
}

impl ArrangeAccounts for AddOutcome {
    type ArrangedAccounts = AddOutcomeAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [_creator, market, _outcome_metadata, _config, _schedule] = accounts else {
            return None;
        };
        Some(AddOutcomeAccounts {
            market: market.pubkey,
        })
    }
}
