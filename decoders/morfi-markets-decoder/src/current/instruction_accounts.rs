//! Positional context for recognized Markets instructions.
use super::instruction_context::CurrentInstruction;
use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;
use std::{collections::BTreeMap, error::Error, fmt};

/// The instruction's required account prefix is incomplete.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IncompleteInstructionAccounts {
    /// Number of accounts required by the instruction ABI.
    pub expected: usize,
    /// Number of accounts supplied by the caller.
    pub actual: usize,
}

impl fmt::Display for IncompleteInstructionAccounts {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "current instruction account list incomplete: expected at least {} accounts, got {}",
            self.expected, self.actual
        )
    }
}

impl Error for IncompleteInstructionAccounts {}

impl CurrentInstruction {
    /// Maps the required account prefix by ABI name, accepting additional accounts.
    ///
    /// Signer and writable flags are not validated by this positional mapping.
    ///
    /// # Errors
    /// Returns [`IncompleteInstructionAccounts`] when the account prefix is too short.
    pub fn accounts(
        &self,
        accounts: &[AccountMeta],
    ) -> Result<BTreeMap<String, Pubkey>, IncompleteInstructionAccounts> {
        if accounts.len() < self.account_names.len() {
            return Err(IncompleteInstructionAccounts {
                expected: self.account_names.len(),
                actual: accounts.len(),
            });
        }
        Ok(self
            .account_names
            .iter()
            .cloned()
            .zip(accounts.iter().map(|account| account.pubkey))
            .collect())
    }
}
