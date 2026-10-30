//! MorfiMarketsEvent dispatch enum and InstructionDecoder impl.
//! Imports the SHADOW types from `super::events` (NOT from morfi_events).

use super::events::{
    BackerFeeClaimedEvent, BackingEvent, GiftClaimed, GiftCreated, GraduationEvent,
    MarketBoostedEvent, MarketCreatedEvent, MarketFundingCompleted, PrizeClaimedEvent,
    ResolutionEvent, SellCapApplied, TradeEvent,
};
use carbon_core::deserialize::CarbonDeserialize;

mod add_outcome;
mod back;
pub use add_outcome::AddOutcome;
mod create_market;
mod prepare_outcomes;
pub use back::Back;
pub use create_market::{
    CreateMarket, CreateMarketAccounts, CreateMarketParams, CreateTimedMarket,
};
pub use prepare_outcomes::{
    OutcomePreparation, PrepareOutcomes, PrepareOutcomesAccounts, PreparedOutcomeAccounts,
};

mod init_conviction_market_config;
mod init_event_schedule;
mod init_timed_market_config;
mod update_timed_market_schedule;
pub use init_conviction_market_config::{
    InitConvictionMarketConfig, InitConvictionMarketConfigInstructionAccounts,
};
pub use init_event_schedule::InitEventSchedule;
pub use init_timed_market_config::InitTimedMarketConfig;
pub use update_timed_market_schedule::UpdateTimedMarketSchedule;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum MorfiMarketsEvent {
    CurrentInstruction(super::current::CurrentInstruction),
    CurrentEvent(super::current::CurrentEvent),
    Trade(TradeEvent),
    Backing(BackingEvent),
    MarketBoosted(MarketBoostedEvent),
    CreateMarket(CreateMarket),
    CreateTimedMarket(CreateTimedMarket),
    PrepareOutcomes(PrepareOutcomes),
    AddOutcome(AddOutcome),
    MarketFundingCompleted(MarketFundingCompleted),
    MarketCreated(MarketCreatedEvent),
    GiftCreated(GiftCreated),
    GiftClaimed(GiftClaimed),
    PrizeClaimed(PrizeClaimedEvent),
    BackerFeeClaimed(BackerFeeClaimedEvent),
    Graduation(GraduationEvent),
    Resolution(ResolutionEvent),
    InitConvictionMarketConfig(InitConvictionMarketConfig),
    InitTimedMarketConfig(InitTimedMarketConfig),
    InitEventSchedule(InitEventSchedule),
    UpdateTimedMarketSchedule(UpdateTimedMarketSchedule),
}

impl carbon_core::instruction::InstructionDecoder<'_> for super::MorfiMarketsDecoder {
    type InstructionType = MorfiMarketsEvent;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<Self::InstructionType> {
        if !instruction.program_id.eq(&self.program_id) {
            return None;
        }
        if let Some(value) = super::current::CurrentInstruction::decode(&instruction.data) {
            return Some(MorfiMarketsEvent::CurrentInstruction(value));
        }
        if let Some(value) = super::current::CurrentEvent::decode(&instruction.data) {
            return Some(MorfiMarketsEvent::CurrentEvent(value));
        }
        if has_invalid_config_initializer_length(&instruction.data) {
            return None;
        }

        let decoded = decode_first_variant!(instruction.data.as_slice(),
            MorfiMarketsEvent::Trade => TradeEvent,
            MorfiMarketsEvent::Backing => BackingEvent,
            MorfiMarketsEvent::MarketBoosted => MarketBoostedEvent,
            MorfiMarketsEvent::CreateMarket => CreateMarket,
            MorfiMarketsEvent::CreateTimedMarket => CreateTimedMarket,
            MorfiMarketsEvent::PrepareOutcomes => PrepareOutcomes,
            MorfiMarketsEvent::AddOutcome => AddOutcome,
            MorfiMarketsEvent::MarketFundingCompleted => MarketFundingCompleted,
            MorfiMarketsEvent::MarketCreated => MarketCreatedEvent,
            MorfiMarketsEvent::GiftCreated => GiftCreated,
            MorfiMarketsEvent::GiftClaimed => GiftClaimed,
            MorfiMarketsEvent::PrizeClaimed => PrizeClaimedEvent,
            MorfiMarketsEvent::BackerFeeClaimed => BackerFeeClaimedEvent,
            MorfiMarketsEvent::Graduation => GraduationEvent,
            MorfiMarketsEvent::Resolution => ResolutionEvent,
            MorfiMarketsEvent::InitConvictionMarketConfig => InitConvictionMarketConfig,
            MorfiMarketsEvent::InitTimedMarketConfig => InitTimedMarketConfig,
            MorfiMarketsEvent::InitEventSchedule => InitEventSchedule,
            MorfiMarketsEvent::UpdateTimedMarketSchedule => UpdateTimedMarketSchedule,
        );

        if decoded.is_none()
            && is_emitted_event(&instruction.data)
            && !is_unindexed_event(&instruction.data)
        {
            log::warn!(
                "markets.undecodable_event discriminator={} payload_len={}",
                instruction
                    .data
                    .iter()
                    .take(DISCRIMINATOR_LEN)
                    .map(|byte| format!("{byte:02x}"))
                    .collect::<String>(),
                instruction.data.len().saturating_sub(DISCRIMINATOR_LEN),
            );
        }

        decoded
    }
}

/// Anchor `emit_cpi!` self-CPI tag. Every markets event discriminator starts with
/// it, so a payload carrying it is an emitted event rather than a program call.
const ANCHOR_EVENT_TAG: [u8; 8] = [0xe4, 0x45, 0xa5, 0x2e, 0x51, 0xcb, 0x9a, 0x1d];

/// Full event discriminator: the Anchor tag plus the per-event name hash.
const DISCRIMINATOR_LEN: usize = 16;

/// Separates emitted events from ordinary program calls, which reach the decoder
/// on every user instruction and must not be reported as decode failures.
fn is_emitted_event(data: &[u8]) -> bool {
    data.starts_with(&ANCHOR_EVENT_TAG)
}

/// Known events that are deliberately not indexed. They decode to nothing
/// and are not reported; a changed layout no longer decodes and is reported.
fn is_unindexed_event(data: &[u8]) -> bool {
    SellCapApplied::deserialize(data).is_some()
}

const CONVICTION_CONFIG_INITIALIZER_DISCRIMINATOR: [u8; 8] =
    [0x0f, 0x81, 0x10, 0x26, 0xe5, 0xc8, 0xd1, 0xf9];
const TIMED_CONFIG_INITIALIZER_DISCRIMINATOR: [u8; 8] =
    [0xaa, 0x50, 0xf2, 0x3c, 0x73, 0x5f, 0x88, 0x73];
const CONVICTION_CONFIG_INITIALIZER_LEN: usize = 211;
const TIMED_CONFIG_INITIALIZER_LEN: usize = 171;

fn has_invalid_config_initializer_length(data: &[u8]) -> bool {
    (data.starts_with(&CONVICTION_CONFIG_INITIALIZER_DISCRIMINATOR)
        && data.len() != CONVICTION_CONFIG_INITIALIZER_LEN)
        || (data.starts_with(&TIMED_CONFIG_INITIALIZER_DISCRIMINATOR)
            && data.len() != TIMED_CONFIG_INITIALIZER_LEN)
}

#[cfg(test)]
mod tests {
    use {
        super::{is_emitted_event, is_unindexed_event, MorfiMarketsEvent, SellCapApplied},
        carbon_core::{borsh, deserialize::CarbonDeserialize, instruction::InstructionDecoder},
    };

    /// `emit_cpi!` payload of devnet prize claim `2aDe…k8aH`, whose layout predates
    /// the `outcome_mint` field and therefore matches no decoder.
    const UNDECODABLE_EVENT_B58: &str = "YeADJEDSy5XKK7h1qdqfLRjRsVmQcpJvDjSh4CFVCQsVwCobj1qGgZUnnXeTPWRhaNWJg6wQZkEN831NBWGQR5LUp53PNGp6qaPYH7xoEGTESgCGLkX61Dodo1kmyzkdhjbTRdVXfEcVC3Lj6bxi6bGrWqRxcm6GUGYP";

    /// Top-level `claim_prize` call from that same transaction. Ordinary program
    /// calls reach the decoder on every user instruction.
    const PROGRAM_CALL_B58: &str = "LVyvyYahHmQKhmf8wQwyfd";

    fn decode(data: Vec<u8>) -> Option<MorfiMarketsEvent> {
        let instruction = solana_instruction::Instruction {
            program_id: crate::PROGRAM_ID,
            accounts: Vec::new(),
            data,
        };
        super::super::MorfiMarketsDecoder::default().decode_instruction(&instruction)
    }

    #[test]
    fn unknown_event_layout_is_reported_not_dropped() {
        let data = bs58::decode(UNDECODABLE_EVENT_B58)
            .into_vec()
            .expect("captured event data is base58");

        assert!(
            is_emitted_event(&data),
            "a real emit_cpi! payload must be recognised as an event",
        );
        assert!(
            decode(data).is_none(),
            "this layout predates outcome_mint and must not decode",
        );
    }

    #[test]
    fn program_calls_are_not_reported_as_events() {
        let data = bs58::decode(PROGRAM_CALL_B58)
            .into_vec()
            .expect("captured instruction data is base58");

        assert!(
            !is_emitted_event(&data),
            "claim_prize is a program call; reporting it would flood the log on every claim",
        );
        assert!(
            matches!(decode(data),Some(MorfiMarketsEvent::CurrentInstruction(value)) if value.name=="claim_prize"),
            "program calls provide parent context, never an emitted event"
        );
    }

    fn sell_cap_applied() -> Vec<u8> {
        let event = SellCapApplied {
            market: solana_pubkey::Pubkey::new_from_array([1; 32]),
            trader: solana_pubkey::Pubkey::new_from_array([2; 32]),
            outcome_index: 1,
            outcome_mint: solana_pubkey::Pubkey::new_from_array([3; 32]),
            tokens_sold: 5_000_000,
            curve_gross: 4_000_000,
            capped_gross: 3_000_000,
            tokens_returned_to_pool: 4_500_000,
            tokens_burned: 500_000,
            ts: 1_760_000_000,
        };
        let mut data = SellCapApplied::DISCRIMINATOR.to_vec();
        data.extend(borsh::to_vec(&event).expect("event serializes"));
        data
    }

    #[test]
    fn sell_cap_applied_matches_the_sell_cap_idl() {
        let idl: serde_json::Value =
            serde_json::from_str(include_str!("../../tests/fixtures/sell-cap.json")).unwrap();
        let event = idl["events"]
            .as_array()
            .unwrap()
            .iter()
            .find(|event| event["name"] == "SellCapApplied")
            .unwrap();
        let name_hash: Vec<u8> = serde_json::from_value(event["discriminator"].clone()).unwrap();

        assert_eq!(&SellCapApplied::DISCRIMINATOR[8..], name_hash.as_slice());
        assert_eq!(sell_cap_applied().len() - 16, 145, "payload bytes");
    }

    #[test]
    fn a_sell_cap_event_is_known_and_not_indexed() {
        let data = sell_cap_applied();

        assert!(is_emitted_event(&data));
        assert!(is_unindexed_event(&data), "no undecodable-event warning");
        assert!(decode(data).is_none(), "nothing is stored for it");
    }

    #[test]
    fn a_sell_cap_event_with_another_layout_is_still_reported() {
        let mut data = sell_cap_applied();
        data.truncate(data.len() - 8);

        assert!(is_emitted_event(&data));
        assert!(!is_unindexed_event(&data));
    }
}
