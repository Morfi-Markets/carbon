//! Carbon shadow struct for current `BackerFeeClaimedEvent`.
//!
//! Field order and types must match
//! `contracts/markets/programs/markets/src/instructions/post_resolution/claim_backer_fees/events.rs`.

use carbon_core::borsh;

#[derive(
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Clone,
    Hash,
)]
pub struct BackerFeeClaimedEvent {
    pub market: solana_pubkey::Pubkey,
    pub claimer: solana_pubkey::Pubkey,
    pub burn_amount: u64,
    pub quote_payout: u64,
    pub vault_balance_after: u64,
    pub supply_after: u64,
    pub ts: i64,
}

impl_carbon_deserialize!(BackerFeeClaimedEvent, "0xe445a52e51cb9a1db0dec29172df83f9");

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{MorfiMarketsDecoder, MorfiMarketsEvent, PROGRAM_ID},
        carbon_core::{deserialize::CarbonDeserialize, instruction::InstructionDecoder},
        solana_instruction::Instruction,
        solana_pubkey::Pubkey,
    };

    #[test]
    fn decoder_reads_every_field_and_tail_ts() {
        let expected = golden_event();
        let MorfiMarketsEvent::BackerFeeClaimed(event) = decode(&event_bytes(&expected)) else {
            panic!("expected BackerFeeClaimedEvent");
        };
        assert_eq!(event.market, Pubkey::new_from_array([0x11; 32]));
        assert_eq!(event.claimer, Pubkey::new_from_array([0x22; 32]));
        assert_eq!(event.burn_amount, 2_000_000);
        assert_eq!(event.quote_payout, 3_000_000);
        assert_eq!(event.vault_balance_after, 8_000_000);
        assert_eq!(event.supply_after, 4_000_000);
        assert_eq!(event.ts, 1_700_000_099);
        assert_eq!(event, expected);
    }

    #[test]
    fn decoder_requires_complete_current_layout() {
        let mut data = event_bytes(&golden_event());
        data.truncate(data.len() - 8);
        assert!(BackerFeeClaimedEvent::deserialize(&data).is_none());
        assert!(decode_opt(&data).is_none());
    }

    fn golden_event() -> BackerFeeClaimedEvent {
        BackerFeeClaimedEvent {
            market: Pubkey::new_from_array([0x11; 32]),
            claimer: Pubkey::new_from_array([0x22; 32]),
            burn_amount: 2_000_000,
            quote_payout: 3_000_000,
            vault_balance_after: 8_000_000,
            supply_after: 4_000_000,
            ts: 1_700_000_099,
        }
    }

    fn event_bytes(event: &BackerFeeClaimedEvent) -> Vec<u8> {
        let mut data = BackerFeeClaimedEvent::DISCRIMINATOR.to_vec();
        data.extend(borsh::to_vec(event).expect("BackerFeeClaimedEvent serializes"));
        data
    }

    fn decode(data: &[u8]) -> MorfiMarketsEvent {
        decode_opt(data).expect("BackerFeeClaimedEvent decodes")
    }

    fn decode_opt(data: &[u8]) -> Option<MorfiMarketsEvent> {
        MorfiMarketsDecoder::default().decode_instruction(&Instruction {
            program_id: PROGRAM_ID,
            accounts: Vec::new(),
            data: data.to_vec(),
        })
    }
}
