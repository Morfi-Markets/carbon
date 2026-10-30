//! Carbon shadow struct for current `TradeEvent`.
//!
//! Field order and types must match
//! `contracts/markets/programs/markets/src/instructions/trading/trade/events.rs`.

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
pub struct TradeEvent {
    pub market: solana_pubkey::Pubkey,
    pub trader: solana_pubkey::Pubkey,
    pub outcome_index: u8,
    pub outcome_mint: solana_pubkey::Pubkey,
    pub side: u8,
    pub quote_amount_gross: u64,
    pub bc_amount: u64,
    pub base_fee_to_backers: u64,
    pub base_fee_to_platform: u64,
    pub prize_pool_contribution: u64,
    pub post_quote_reserves: u128,
    pub bc_supply_after: u64,
    pub used_penalty_fee: bool,
    pub claim_rights_supply_after_d6: u64,
    pub post_outcome_conviction_ppb: Vec<u32>,
    pub post_outcome_conviction_target_ppb: Vec<u32>,
    pub post_conviction_retention_per_second_ppb: u32,
    pub post_last_conviction_update_ts: i64,
    pub ts: i64,
    pub post_total_real_quote_reserves: u64,
}

impl_carbon_deserialize!(TradeEvent, "0xe445a52e51cb9a1dbddb7fd34ee661ee");

#[cfg(test)]
mod tests {
    use {super::*, carbon_core::deserialize::CarbonDeserialize, solana_pubkey::Pubkey};

    #[test]
    fn decoder_requires_complete_current_layout() {
        let mut data = event_bytes();
        data.truncate(data.len() - 8);
        assert!(TradeEvent::deserialize(&data).is_none());
    }

    #[test]
    fn decoder_reads_claim_rights_and_u128_reserves() {
        let event = TradeEvent::deserialize(&event_bytes()).expect("TradeEvent decodes");
        assert_eq!(event.claim_rights_supply_after_d6, 7);
        assert_eq!(event.post_quote_reserves, 6);
        assert_eq!(event.post_total_real_quote_reserves, 42_000_000);
        assert_eq!(event.trader, Pubkey::new_from_array([4; 32]));
        assert_eq!(event.market, Pubkey::new_from_array([1; 32]));
    }

    fn event_bytes() -> Vec<u8> {
        let event = TradeEvent {
            market: Pubkey::new_from_array([1; 32]),
            trader: Pubkey::new_from_array([4; 32]),
            outcome_index: 1,
            outcome_mint: Pubkey::new_from_array([3; 32]),
            side: 0,
            quote_amount_gross: 10,
            bc_amount: 4,
            base_fee_to_backers: 1,
            base_fee_to_platform: 2,
            prize_pool_contribution: 3,
            post_quote_reserves: 6,
            bc_supply_after: 5,
            used_penalty_fee: false,
            claim_rights_supply_after_d6: 7,
            post_outcome_conviction_ppb: vec![500_000_000, 500_000_000],
            post_outcome_conviction_target_ppb: vec![500_000_000, 500_000_000],
            post_conviction_retention_per_second_ppb: 999,
            post_last_conviction_update_ts: 1_700_000_000,
            ts: 1_700_000_000,
            post_total_real_quote_reserves: 42_000_000,
        };
        let mut data = TradeEvent::DISCRIMINATOR.to_vec();
        data.extend(borsh::to_vec(&event).expect("TradeEvent serializes"));
        data
    }
}
