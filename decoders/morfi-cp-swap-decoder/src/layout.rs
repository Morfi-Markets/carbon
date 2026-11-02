//! Discriminators and ordered account names of user-signed CP-Swap instructions.

/// ABI name, discriminator, and ordered account names of an instruction.
///
/// Use it to read an instruction layout without instruction data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstructionLayout {
    pub name: &'static str,
    pub discriminator: [u8; 8],
    pub account_names: &'static [&'static str],
}

impl InstructionLayout {
    /// Finds the position of an account in the instruction account list.
    pub fn position(&self, account: &str) -> Option<usize> {
        self.account_names.iter().position(|name| *name == account)
    }
}

/// `swap_base_input`: an exact input amount for a minimum output amount.
pub const SWAP_BASE_INPUT: InstructionLayout = InstructionLayout {
    name: "swap_base_input",
    discriminator: [143, 190, 90, 218, 196, 30, 51, 222],
    account_names: &[
        "payer",
        "authority",
        "amm_config",
        "pool_state",
        "input_token_account",
        "output_token_account",
        "input_vault",
        "output_vault",
        "input_token_program",
        "output_token_program",
        "input_token_mint",
        "output_token_mint",
        "observation_state",
        "event_authority",
        "program",
    ],
};
