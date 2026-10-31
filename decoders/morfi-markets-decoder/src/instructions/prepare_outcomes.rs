//! Legacy preparation uses fixed accounts, four-account tuples and an optional creator signer.

use carbon_core::{borsh, deserialize::ArrangeAccounts};
use solana_pubkey::Pubkey;

/// Immutable token metadata supplied by the creator for one active outcome.
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
pub struct OutcomePreparation {
    pub outcome_index: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

/// Preserved legacy batched outcome preparation instruction.
#[derive(
    borsh::BorshDeserialize, Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct PrepareOutcomes {
    pub outcomes: Vec<OutcomePreparation>,
}

impl_carbon_deserialize!(PrepareOutcomes, "0x7002b5f27a101177");

/// Legacy preparation accounts needed to bind outcome creation.
#[derive(Debug, Clone)]
pub struct PrepareOutcomesAccounts {
    pub market: Pubkey,
    pub market_config: Pubkey,
    pub outcome_mint_authority: Pubkey,
    pub outcomes: Vec<PreparedOutcomeAccounts>,
}

/// Actual mint/custody identities in each remaining-account tuple.
#[derive(Debug, Clone)]
pub struct PreparedOutcomeAccounts {
    pub mint: Pubkey,
    pub inventory: Pubkey,
    pub claim_rights_mint: Pubkey,
    pub metadata: Pubkey,
}

impl ArrangeAccounts for PrepareOutcomes {
    type ArrangedAccounts = PrepareOutcomesAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [_payer, market, _outcome_metadata, market_config, outcome_mint_authority, _token_metadata_program, _system_program, _token_program, _rent, _event_schedule, _event_authority, _program, remaining @ ..] =
            accounts
        else {
            return None;
        };
        let tuples = remaining.chunks_exact(4);
        if tuples.remainder().len() > 1 {
            return None;
        }
        let outcomes = tuples
            .map(|tuple| PreparedOutcomeAccounts {
                mint: tuple[0].pubkey,
                inventory: tuple[1].pubkey,
                claim_rights_mint: tuple[2].pubkey,
                metadata: tuple[3].pubkey,
            })
            .collect();
        Some(PrepareOutcomesAccounts {
            market: market.pubkey,
            market_config: market_config.pubkey,
            outcome_mint_authority: outcome_mint_authority.pubkey,
            outcomes,
        })
    }
}
