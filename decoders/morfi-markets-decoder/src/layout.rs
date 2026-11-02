//! Discriminators and ordered account names of the current Markets instructions.

/// ABI name, discriminator, and ordered account names of a current instruction.
///
/// Use it to read an instruction layout without instruction data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstructionLayout {
    pub name: &'static str,
    pub discriminator: [u8; 8],
    pub account_names: &'static [&'static str],
}

impl InstructionLayout {
    /// Finds a current instruction layout by its ABI name.
    pub fn named(name: &str) -> Option<&'static Self> {
        INSTRUCTION_LAYOUTS
            .iter()
            .find(|layout| layout.name == name)
    }

    /// Finds the current instruction layout whose discriminator starts `data`.
    pub fn for_data(data: &[u8]) -> Option<&'static Self> {
        let discriminator = data.get(..8)?;
        INSTRUCTION_LAYOUTS
            .iter()
            .find(|layout| layout.discriminator == discriminator)
    }

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

/// Buy and sell share one account list.
const TRADE_ACCOUNTS: &[&str] = &[
    "trader",
    "payer",
    "transaction_authority",
    "market",
    "market_config",
    "quote_mint",
    "user_positions",
    "market_quote_vault_authority",
    "market_quote_vault",
    "platform_fee_vault",
    "trader_quote_account",
    "token_program",
    "system_program",
    "event_schedule",
    "event_authority",
    "program",
];

/// Every recognized current Markets instruction.
pub const INSTRUCTION_LAYOUTS: &[InstructionLayout] = &[
    InstructionLayout {
        name: "add_outcome",
        discriminator: [162, 222, 202, 185, 176, 222, 248, 52],
        account_names: &[
            "creator",
            "transaction_authority",
            "market",
            "market_config",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "admin_set_platform_reward_payout",
        discriminator: [156, 95, 212, 138, 153, 240, 224, 157],
        account_names: &[
            "upgrade_authority",
            "program_data",
            "market_rewards",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "admin_set_reward_payout",
        discriminator: [43, 230, 135, 141, 81, 107, 140, 74],
        account_names: &[
            "upgrade_authority",
            "program_data",
            "market_rewards",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "back",
        discriminator: [1, 144, 109, 177, 110, 131, 124, 0],
        account_names: &[
            "backer",
            "payer",
            "transaction_authority",
            "market",
            "market_config",
            "quote_mint",
            "backer_quote_account",
            "user_positions",
            "market_quote_vault_authority",
            "market_quote_vault",
            "platform_fee_vault",
            "token_program",
            "system_program",
            "event_schedule",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "boost_market",
        discriminator: [110, 99, 228, 247, 160, 18, 61, 103],
        account_names: &[
            "booster",
            "market",
            "quote_mint",
            "booster_quote_ata",
            "market_quote_vault_authority",
            "market_quote_vault",
            "token_program",
            "event_schedule",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "buy",
        discriminator: [102, 6, 61, 18, 1, 218, 235, 234],
        account_names: TRADE_ACCOUNTS,
    },
    InstructionLayout {
        name: "claim_backer_fees",
        discriminator: [234, 26, 209, 192, 20, 160, 240, 25],
        account_names: &[
            "claimer",
            "market",
            "quote_mint",
            "user_positions",
            "claimer_quote_account",
            "market_quote_vault_authority",
            "market_quote_vault",
            "token_program",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "claim_pool_reward",
        discriminator: [184, 228, 250, 102, 47, 97, 202, 169],
        account_names: &[
            "payer",
            "market_rewards",
            "quote_mint",
            "rewards_quote_vault",
            "payout_wallet",
            "payout_quote_ata",
            "token_program",
            "associated_token_program",
            "system_program",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "claim_principal_prize",
        discriminator: [151, 133, 55, 116, 67, 217, 1, 46],
        account_names: &[
            "claimer",
            "market",
            "quote_mint",
            "user_positions",
            "claimer_quote_account",
            "market_quote_vault_authority",
            "market_quote_vault",
            "platform_fee_vault",
            "token_program",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "claim_prize",
        discriminator: [157, 233, 139, 121, 246, 62, 234, 235],
        account_names: &[
            "claimer",
            "market",
            "quote_mint",
            "user_positions",
            "claimer_quote_account",
            "market_quote_vault_authority",
            "market_quote_vault",
            "token_program",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "clear_expired_cash_entitlement",
        discriminator: [228, 47, 91, 160, 4, 178, 237, 157],
        account_names: &[
            "owner",
            "market",
            "user_positions",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "clear_expired_outcome_entitlement",
        discriminator: [54, 198, 117, 44, 22, 120, 93, 136],
        account_names: &[
            "owner",
            "market",
            "user_positions",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "clear_expired_prize_entitlement",
        discriminator: [223, 251, 206, 38, 191, 118, 102, 148],
        account_names: &[
            "owner",
            "market",
            "user_positions",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "clear_settled_loser",
        discriminator: [176, 29, 84, 15, 73, 92, 145, 141],
        account_names: &[
            "owner",
            "market",
            "user_positions",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "close_market_custody",
        discriminator: [42, 2, 136, 229, 133, 122, 226, 112],
        account_names: &[
            "finalizer",
            "market",
            "quote_mint",
            "market_quote_vault_authority",
            "market_quote_vault",
            "platform_fee_vault",
            "token_program",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "close_user_positions",
        discriminator: [217, 238, 110, 87, 241, 93, 110, 250],
        account_names: &[
            "wallet",
            "user_positions",
            "admin_config",
            "treasury",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "close_winning_outcome_vault",
        discriminator: [22, 33, 233, 111, 91, 39, 243, 124],
        account_names: &[
            "finalizer",
            "market",
            "outcome_mint",
            "outcome_token_vault",
            "market_quote_vault_authority",
            "token_program",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "collect_pool_rewards",
        discriminator: [251, 249, 182, 161, 95, 116, 167, 188],
        account_names: &[
            "payer",
            "market",
            "market_rewards",
            "quote_mint",
            "rewards_quote_vault",
            "cp_program",
            "cp_authority",
            "pool_state",
            "amm_config",
            "quote_vault",
            "platform_payout_wallet",
            "platform_quote_ata",
            "token_program",
            "associated_token_program",
            "system_program",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "create_market",
        discriminator: [103, 226, 97, 235, 200, 188, 251, 254],
        account_names: &[
            "creator",
            "payer",
            "transaction_authority",
            "market_config",
            "market",
            "quote_mint",
            "market_quote_vault",
            "system_program",
            "token_program",
            "admin_config",
            "event_schedule",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "forfeit_backer_entitlement",
        discriminator: [125, 197, 45, 126, 231, 97, 29, 16],
        account_names: &[
            "owner",
            "market",
            "user_positions",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "init_user_positions",
        discriminator: [172, 140, 197, 227, 61, 163, 59, 76],
        account_names: &[
            "wallet",
            "payer",
            "transaction_authority",
            "user_positions",
            "admin_config",
            "system_program",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "launch_winner_token_with_rewards",
        discriminator: [130, 208, 176, 136, 45, 46, 50, 82],
        account_names: &[
            "payer",
            "transaction_authority",
            "funding_authority",
            "winning_outcome_mint",
            "market",
            "market_config",
            "quote_mint",
            "funding_quote_account",
            "market_rewards",
            "reward_quote_vault",
            "reward_token_vault",
            "outcome_mint_authority",
            "winner_metadata",
            "token_metadata_program",
            "morfi_amm_program",
            "morfi_amm_config",
            "cp_authority",
            "pool_state",
            "lp_mint",
            "lp_lock",
            "token_0_vault",
            "token_1_vault",
            "observation_state",
            "token_program",
            "associated_token_program",
            "system_program",
            "rent",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "refund_expired_backed",
        discriminator: [55, 140, 59, 210, 157, 132, 63, 249],
        account_names: &[
            "backer",
            "market",
            "quote_mint",
            "backer_quote_account",
            "user_positions",
            "market_quote_vault_authority",
            "market_quote_vault",
            "platform_fee_vault",
            "token_program",
            "event_schedule",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "resize_user_positions",
        discriminator: [59, 43, 92, 134, 202, 40, 13, 236],
        account_names: &[
            "wallet",
            "payer",
            "transaction_authority",
            "user_positions",
            "admin_config",
            "treasury",
            "system_program",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "resolve_drain_and_migrate",
        discriminator: [103, 98, 101, 94, 184, 122, 4, 234],
        account_names: &[
            "payer",
            "transaction_authority",
            "winning_outcome_mint",
            "market",
            "market_config",
            "quote_mint",
            "market_quote_vault_authority",
            "market_quote_vault",
            "event_schedule",
            "token_program",
            "system_program",
            "rent",
            "outcome_mint_authority",
            "winner_metadata",
            "winner_token_vault",
            "token_metadata_program",
            "morfi_amm_program",
            "morfi_amm_config",
            "cp_authority",
            "pool_state",
            "lp_mint",
            "lp_lock",
            "token_0_vault",
            "token_1_vault",
            "observation_state",
            "cp_event_authority",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "sell",
        discriminator: [51, 230, 133, 164, 1, 127, 131, 173],
        account_names: TRADE_ACCOUNTS,
    },
    InstructionLayout {
        name: "sweep_expired_backer_fee_liability",
        discriminator: [134, 2, 3, 113, 55, 80, 131, 82],
        account_names: &[
            "sweeper",
            "market",
            "quote_mint",
            "market_quote_vault_authority",
            "market_quote_vault",
            "platform_fee_vault",
            "token_program",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "sweep_expired_prize_liability",
        discriminator: [190, 227, 64, 191, 68, 100, 227, 180],
        account_names: &[
            "sweeper",
            "market",
            "quote_mint",
            "market_quote_vault_authority",
            "market_quote_vault",
            "platform_fee_vault",
            "token_program",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "sweep_expired_winning_tokens",
        discriminator: [153, 217, 87, 62, 133, 231, 227, 25],
        account_names: &[
            "sweeper",
            "market",
            "outcome_mint",
            "outcome_token_vault",
            "treasury_outcome_account",
            "market_quote_vault_authority",
            "token_program",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "unback",
        discriminator: [192, 60, 119, 238, 20, 4, 41, 58],
        account_names: &[
            "backer",
            "market",
            "quote_mint",
            "backer_quote_account",
            "user_positions",
            "market_quote_vault_authority",
            "market_quote_vault",
            "platform_fee_vault",
            "token_program",
            "event_schedule",
            "event_authority",
            "program",
        ],
    },
    InstructionLayout {
        name: "withdraw_outcome_tokens",
        discriminator: [38, 205, 190, 42, 83, 119, 9, 68],
        account_names: &[
            "owner",
            "market",
            "user_positions",
            "outcome_mint",
            "outcome_token_vault",
            "owner_outcome_account",
            "market_quote_vault_authority",
            "token_program",
            "event_authority",
            "program",
        ],
    },
];
