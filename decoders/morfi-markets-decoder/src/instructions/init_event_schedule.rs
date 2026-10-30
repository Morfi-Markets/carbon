//! Current schedule initializer decoded from its signed instruction, without a CPI event.

use super::super::types::InitEventScheduleParams;
use carbon_core::{borsh, deserialize::ArrangeAccounts};

/// Initializer arguments in the pinned IDL's Borsh layout.
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
pub struct InitEventSchedule {
    pub params: InitEventScheduleParams,
}

impl_carbon_deserialize!(InitEventSchedule, "0x95d1e3ca8cb760ad");

/// Immutable schedule and preset identities from the six-account prefix.
#[derive(Debug, Clone)]
pub struct InitEventScheduleAccounts {
    pub market_config: solana_pubkey::Pubkey,
    pub event_schedule: solana_pubkey::Pubkey,
}

impl ArrangeAccounts for InitEventSchedule {
    type ArrangedAccounts = InitEventScheduleAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [_payer, _config_admin, _admin_config, market_config, event_schedule, _system_program, ..] =
            accounts
        else {
            return None;
        };
        Some(InitEventScheduleAccounts {
            market_config: market_config.pubkey,
            event_schedule: event_schedule.pubkey,
        })
    }
}
