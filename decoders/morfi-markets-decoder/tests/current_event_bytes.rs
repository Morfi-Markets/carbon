//! Current events decoded from Borsh bytes written field by field in the
//! contract's order, so no expected value comes from the decoder. Each
//! discriminator is the Anchor self-CPI tag followed by
//! `sha256("event:<Name>")[..8]`.
use carbon_core::instruction::InstructionDecoder;
use carbon_morfi_markets_decoder::{
    current::CurrentEvent, MorfiMarketsDecoder, MorfiMarketsEvent, PROGRAM_ID,
};
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

const ANCHOR_EVENT_TAG: [u8; 8] = [0xe4, 0x45, 0xa5, 0x2e, 0x51, 0xcb, 0x9a, 0x1d];
/// Same hash as the legacy `events::TradeEvent`, because the names match.
const TRADE_EVENT: [u8; 8] = [0xbd, 0xdb, 0x7f, 0xd3, 0x4e, 0xe6, 0x61, 0xee];
const WALLET_PRINCIPAL_PRIZE_CLAIMED_EVENT: [u8; 8] =
    [0x8e, 0xd2, 0x0a, 0x7c, 0x61, 0x8f, 0xa2, 0xb8];
const USER_POSITIONS_INITIALIZED_EVENT: [u8; 8] = [0xd9, 0xc8, 0xd8, 0x7c, 0x43, 0x38, 0x1e, 0xfe];
const USER_POSITIONS_RESIZED_EVENT: [u8; 8] = [0x40, 0x2a, 0x75, 0x41, 0xdd, 0x0b, 0xc3, 0xd4];
const USER_POSITIONS_CLOSED_EVENT: [u8; 8] = [0x60, 0x86, 0x87, 0xa5, 0xf1, 0x2c, 0xc3, 0xce];
const MARKET_FUNDING_COMPLETED: [u8; 8] = [0xba, 0xb4, 0x95, 0xe6, 0xc1, 0x7f, 0xcf, 0x0e];

const TS: i64 = 1_780_000_123;

#[test]
fn a_principal_sell_decodes_as_the_current_trade_event() {
    let data = principal_full_exit();
    let Some(MorfiMarketsEvent::CurrentEvent(CurrentEvent::TradeEvent(trade))) =
        decode(data.clone())
    else {
        panic!("a current TradeEvent decodes as the current event, never the legacy Trade");
    };
    assert_eq!(trade.market, key(1));
    assert_eq!(trade.trader, key(2));
    assert_eq!(trade.user_positions, key(3));
    assert_eq!(trade.pda_slot_idx, 4);
    assert_eq!(trade.outcome_index, 2);
    assert_eq!(trade.side, 1, "sell");
    assert_eq!(trade.pool_quote_delta_d6, 93_500_000);
    assert_eq!(trade.outcome_token_amount_d6, 85_505_258_344);
    assert_eq!(trade.base_fee_to_backers_d6, 467_500);
    assert_eq!(trade.base_fee_to_platform_d6, 2_805_000);
    assert_eq!(trade.prize_pool_contribution_d6, 935_000);
    assert_eq!(trade.post_position_principal_quote_d6, 0);
    assert_eq!(trade.post_total_quote_reserve_d6, 935_000_000);
    assert_eq!(trade.post_pool_quote_d6, 1_935_000_000, "x");
    assert_eq!(
        trade.post_token_entitlement_supply_d6, 421_519_636_898,
        "units held, T0 - y"
    );
    assert_eq!(
        trade.post_outcome_conviction_ppb,
        [360_000_000, 640_000_000]
    );
    assert_eq!(trade.post_outcome_conviction_target_ppb, [0, 1_000_000_000]);
    assert_eq!(trade.ts, 1_780_000_000);
    assert_eq!(
        CurrentEvent::TradeEvent(trade).timestamp(),
        Some(1_780_000_000)
    );
    assert!(
        CurrentEvent::decode(&data[..data.len() - 1]).is_none(),
        "ts is the last field and is required"
    );
}

#[test]
fn the_principal_claim_event_decodes_under_its_v1_name_and_fields() {
    let data = Borsh::event(WALLET_PRINCIPAL_PRIZE_CLAIMED_EVENT)
        .key(1)
        .key(2)
        .key(3)
        .u16(7)
        .u8(1)
        .u64(421_519_636_898)
        .u64(210_759_818_449)
        .u64(935_000_000)
        .u64(339_350)
        .u64(33_935_000)
        .u64(1_000_000_000)
        .u64(0)
        .u8(2)
        .i64(TS)
        .0;
    let Some(event) = CurrentEvent::decode(&data) else {
        panic!("the v1 claim event decodes");
    };
    assert_eq!(event.name(), "WalletPrincipalPrizeClaimedEvent");
    assert_eq!(event.timestamp(), Some(TS));
    let CurrentEvent::WalletPrincipalPrizeClaimedEvent(claim) = event else {
        unreachable!("the name selects the variant");
    };
    assert_eq!(claim.market, key(1));
    assert_eq!(claim.claimer, key(2));
    assert_eq!(claim.user_positions, key(3));
    assert_eq!(claim.pda_slot_idx, 7);
    assert_eq!(claim.winning_outcome, 1);
    assert_eq!(claim.outcome_units_debited, 421_519_636_898);
    assert_eq!(claim.prize_units_debited, 210_759_818_449);
    assert_eq!(claim.principal_gross, 935_000_000);
    assert_eq!(
        claim.platform_sell_fee, 339_350,
        "the claim fee on the bonus"
    );
    assert_eq!(claim.prize_payout, 33_935_000);
    assert_eq!(
        claim.winner_quote_reserve_after, 1_000_000_000,
        "the winner's x, V0 included"
    );
    assert_eq!(claim.prize_liability_after, 0);
    assert_eq!(claim.fee_vault_index, 2);
    assert_eq!(claim.ts, TS);
    assert!(CurrentEvent::decode(&data[..data.len() - 1]).is_none());
}

#[test]
fn user_positions_initialized_event_ends_with_ts() {
    let body = Borsh::event(USER_POSITIONS_INITIALIZED_EVENT)
        .key(1)
        .key(2)
        .u16(10)
        .key(3)
        .u64(2_039_280);
    let CurrentEvent::UserPositionsInitializedEvent(event) = decode_with_trailing_ts(body) else {
        panic!("UserPositionsInitializedEvent");
    };
    assert_eq!(event.wallet, key(1));
    assert_eq!(event.user_positions, key(2));
    assert_eq!(event.capacity, 10);
    assert_eq!(event.payer, key(3));
    assert_eq!(event.rent_lamports, 2_039_280);
    assert_eq!(event.ts, TS);
}

#[test]
fn user_positions_resized_event_ends_with_ts() {
    let body = Borsh::event(USER_POSITIONS_RESIZED_EVENT)
        .key(1)
        .key(2)
        .u16(10)
        .u16(20)
        .key(3)
        .key(4)
        .u64(1_113_600)
        .u64(0);
    let CurrentEvent::UserPositionsResizedEvent(event) = decode_with_trailing_ts(body) else {
        panic!("UserPositionsResizedEvent");
    };
    assert_eq!(event.wallet, key(1));
    assert_eq!(event.user_positions, key(2));
    assert_eq!(event.old_capacity, 10);
    assert_eq!(event.new_capacity, 20);
    assert_eq!(event.payer, key(3));
    assert_eq!(event.treasury, key(4));
    assert_eq!(event.rent_paid_lamports, 1_113_600);
    assert_eq!(event.rent_refunded_lamports, 0);
    assert_eq!(event.ts, TS);
}

#[test]
fn user_positions_closed_event_ends_with_ts() {
    let body = Borsh::event(USER_POSITIONS_CLOSED_EVENT)
        .key(1)
        .key(2)
        .u16(20)
        .key(4)
        .u64(3_152_880);
    let CurrentEvent::UserPositionsClosedEvent(event) = decode_with_trailing_ts(body) else {
        panic!("UserPositionsClosedEvent");
    };
    assert_eq!(event.wallet, key(1));
    assert_eq!(event.user_positions, key(2));
    assert_eq!(event.capacity, 20);
    assert_eq!(event.treasury, key(4));
    assert_eq!(event.rent_refunded_lamports, 3_152_880);
    assert_eq!(event.ts, TS);
}

#[test]
fn market_funding_completed_ends_with_ts() {
    let body = Borsh::event(MARKET_FUNDING_COMPLETED).i64(1_780_003_600);
    let CurrentEvent::MarketFundingCompleted(event) = decode_with_trailing_ts(body) else {
        panic!("MarketFundingCompleted");
    };
    assert_eq!(event.trading_opens_ts, 1_780_003_600);
    assert_eq!(event.ts, TS);
}

/// The worked example's full exit by A on a Principal outcome (V0 = 1,000
/// USDC, T0 = 1,000,000 tokens): 93.5 USDC of principal leaves the pool, so x
/// falls to 1,935 USDC, and the units go back into y = 578,480.363102.
fn principal_full_exit() -> Vec<u8> {
    Borsh::event(TRADE_EVENT)
        .key(1)
        .key(2)
        .key(3)
        .u16(4)
        .u8(2)
        .u8(1)
        .u64(93_500_000)
        .u64(85_505_258_344)
        .u64(467_500)
        .u64(2_805_000)
        .u64(935_000)
        .u64(0)
        .u64(935_000_000)
        .u64(1_935_000_000)
        .u64(421_519_636_898)
        .ppb(&[360_000_000, 640_000_000])
        .ppb(&[0, 1_000_000_000])
        .i64(1_780_000_000)
        .0
}

/// Decodes `body` with [`TS`] appended, and checks that `body` alone, the
/// layout before `ts` was added, no longer decodes. Borsh ignores trailing
/// bytes, so without this check a decoder missing `ts` would still decode.
fn decode_with_trailing_ts(body: Borsh) -> CurrentEvent {
    let without_ts = body.0.clone();
    let event = CurrentEvent::decode(&body.i64(TS).0).expect("event with ts decodes");
    assert!(
        CurrentEvent::decode(&without_ts).is_none(),
        "{} without ts",
        event.name()
    );
    assert_eq!(event.timestamp(), Some(TS), "{}", event.name());
    event
}

fn decode(data: Vec<u8>) -> Option<MorfiMarketsEvent> {
    MorfiMarketsDecoder::default().decode_instruction(&Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![],
        data,
    })
}

fn key(byte: u8) -> Pubkey {
    Pubkey::new_from_array([byte; 32])
}

/// Little-endian Borsh writer for one tagged event payload.
struct Borsh(Vec<u8>);

impl Borsh {
    fn event(name_hash: [u8; 8]) -> Self {
        let mut data = ANCHOR_EVENT_TAG.to_vec();
        data.extend_from_slice(&name_hash);
        Self(data)
    }

    fn key(mut self, byte: u8) -> Self {
        self.0.extend_from_slice(&[byte; 32]);
        self
    }

    fn u8(mut self, value: u8) -> Self {
        self.0.push(value);
        self
    }

    fn u16(mut self, value: u16) -> Self {
        self.0.extend_from_slice(&value.to_le_bytes());
        self
    }

    fn u64(mut self, value: u64) -> Self {
        self.0.extend_from_slice(&value.to_le_bytes());
        self
    }

    fn i64(mut self, value: i64) -> Self {
        self.0.extend_from_slice(&value.to_le_bytes());
        self
    }

    fn ppb(mut self, values: &[u32]) -> Self {
        let len = u32::try_from(values.len()).expect("vector length fits u32");
        self.0.extend_from_slice(&len.to_le_bytes());
        for value in values {
            self.0.extend_from_slice(&value.to_le_bytes());
        }
        self
    }
}
