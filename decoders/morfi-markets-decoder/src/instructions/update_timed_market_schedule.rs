//! Current schedule update instruction; the program emits no update CPI event.

use carbon_core::{borsh, deserialize::ArrangeAccounts};

/// Optimistic date comparison and replacement arguments in contract wire order.
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
pub struct UpdateTimedMarketSchedule {
    pub expected_start_ts: i64,
    pub expected_end_ts: i64,
    pub new_start_ts: i64,
    pub new_end_ts: i64,
}

impl_carbon_deserialize!(UpdateTimedMarketSchedule, "0xe5292091ea9160dd");

/// Updated schedule identity from the five-account prefix.
#[derive(Debug, Clone)]
pub struct UpdateTimedMarketScheduleAccounts {
    pub event_schedule: solana_pubkey::Pubkey,
}

impl ArrangeAccounts for UpdateTimedMarketSchedule {
    type ArrangedAccounts = UpdateTimedMarketScheduleAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [_config_admin, _admin_config, event_schedule, _event_authority, _program, ..] =
            accounts
        else {
            return None;
        };
        Some(UpdateTimedMarketScheduleAccounts {
            event_schedule: event_schedule.pubkey,
        })
    }
}
