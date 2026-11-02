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
    ///
    /// It is a `const fn`, so a consumer can hold a position in a constant.
    pub const fn position(&self, account: &str) -> Option<usize> {
        let mut index = 0;
        while index < self.account_names.len() {
            if same_name(self.account_names[index], account) {
                return Some(index);
            }
            index += 1;
        }
        None
    }
}

const fn same_name(left: &str, right: &str) -> bool {
    let (left, right) = (left.as_bytes(), right.as_bytes());
    if left.len() != right.len() {
        return false;
    }
    let mut index = 0;
    while index < left.len() {
        if left[index] != right[index] {
            return false;
        }
        index += 1;
    }
    true
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
