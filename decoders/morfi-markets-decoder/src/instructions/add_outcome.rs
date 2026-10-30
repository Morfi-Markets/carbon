use super::OutcomePreparation;
use carbon_core::{borsh, deserialize::ArrangeAccounts};
use solana_pubkey::Pubkey;

#[derive(
    borsh::BorshDeserialize, Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct AddOutcome {
    pub outcome: OutcomePreparation,
}

impl_carbon_deserialize!(AddOutcome, "0xa2decab9b0def834");

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
