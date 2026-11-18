#![cfg(feature = "decoder")]

use {
    carbon_core::deserialize::CarbonDeserialize,
    carbon_morfi_markets_decoder::{
        current::{
            AdminConfig, CommonMarketConfig, ConvictionMarketConfig, EconomicMode, GiftConfig,
            TimedMarketConfig,
        },
        types::FeeConfig,
    },
    solana_pubkey::Pubkey,
};

const ADMIN: &[u8] = include_bytes!("fixtures/admin-config-account.bin");
const CONVICTION: &[u8] = include_bytes!("fixtures/conviction-config-account.bin");
const TIMED: &[u8] = include_bytes!("fixtures/timed-config-account.bin");
const FIXTURE_TREASURY: Pubkey =
    Pubkey::from_str_const("6vSzVje725Ya3PDzvXNC6uxAUYWXDYPJWT25GsjzyQCt");

#[test]
fn decodes_captured_admin_config() {
    assert_eq!(AdminConfig::DISCRIMINATOR, ADMIN[..8]);
    assert_eq!(
        AdminConfig::deserialize(ADMIN),
        Some(AdminConfig {
            config_admin: Pubkey::from_str_const("7rjDQgatV28V1rF9cbFc2dYY2M1ddkVFRW1XGquNtCSM"),
            gift_admin: Pubkey::from_str_const("DRnNmmZVuozvMxzfwUBxXiPXT1V2w4cP1FFnfFaam9gT"),
            bump: 255,
            conviction_market_creation_restricted: false,
            timed_market_creation_restricted: true,
            transaction_authority: Pubkey::from_str_const(
                "FBRhWeWzD32JRd63hWG4Uvj3hVKgWVASvETNXQPBpq6Q"
            ),
            rent_treasury: FIXTURE_TREASURY,
        })
    );
}

#[test]
fn decodes_captured_market_configs() {
    assert_eq!(ConvictionMarketConfig::DISCRIMINATOR, CONVICTION[..8]);
    assert_eq!(TimedMarketConfig::DISCRIMINATOR, TIMED[..8]);
    let conviction_common = CommonMarketConfig {
        enabled: true,
        config_nonce: 18_446_744_073_709_551_614,
        bump: 254,
        quote_mint: Pubkey::from_str_const("DYLJDcQJnppFNG1cCD2YFUE7h9HQ3ZNyEqMt8SwiKS5x"),
        platform_treasury: FIXTURE_TREASURY,
        transaction_authority: Pubkey::from_str_const(
            "4paxjyxh6qAKtDXfiwPrkKfkCFcqUhZDN1qFV4ioUeu6",
        ),
        min_outcomes: 2,
        max_outcomes: 10,
        initial_virtual_quote: 1_500_000,
        initial_token_supply: 10_000_000_000_000,
        min_prize_pool_quote: 1_250_000,
        min_back_amount_quote: 25_000,
        min_quote_in_per_trade: 5_000,
        fees: FeeConfig {
            platform_buy_fee_bps: 110,
            platform_sell_fee_bps: 120,
            platform_claim_fee_bps: 130,
            platform_back_fee_bps: 510,
            platform_unback_fee_bps: 520,
            backer_trade_fee_bps: 140,
            prize_pool_min_fee_bps: 150,
            buy_prize_pool_max_fee_bps: 350,
            sell_prize_pool_max_fee_bps: 9_700,
            sell_progress_delay_factor: 17,
            sell_gap_weight_bps: 9_000,
            buy_reference_threshold_ppb: 650_000_001,
        },
        conviction_baseline_reserve_quote: 125_000,
        unclaimed_prize_clawback_delay_secs: 172_800,
        economic_mode: EconomicMode::PrincipalBack,
    };
    let timed_common = CommonMarketConfig {
        enabled: false,
        config_nonce: 410_002,
        transaction_authority: Pubkey::from_str_const(
            "2Lss87utL5qcyQTyJsCB6ycVdsuM9XzJbpCcnWt4oqQz",
        ),
        initial_virtual_quote: 2_500_000,
        ..conviction_common.clone()
    };
    assert_eq!(
        ConvictionMarketConfig::deserialize(CONVICTION),
        Some(ConvictionMarketConfig {
            common: conviction_common,
            conviction_retention_per_second_ppb: 999_500_000,
            conviction_resolution_threshold_ppb: 700_000_000,
            resolution_threshold_floor_margin_ppb: 25_000_001,
            resolution_threshold_decay_delay_secs: 9_007_199_254_740_991,
            resolution_threshold_drop_interval_secs: 86_400,
            resolution_threshold_drop_ppb: 25_000_000,
        })
    );
    assert_eq!(
        TimedMarketConfig::deserialize(TIMED),
        Some(TimedMarketConfig {
            common: timed_common,
            conviction_retention_per_second_ppb: 999_500_000,
        })
    );
}

#[test]
fn rejects_wrong_discriminators_truncation_and_trailing_bytes() {
    assert_exact_account_bytes::<AdminConfig>(ADMIN);
    assert_exact_account_bytes::<ConvictionMarketConfig>(CONVICTION);
    assert_exact_account_bytes::<TimedMarketConfig>(TIMED);
}

#[test]
fn rejects_unknown_borsh_tags() {
    const ADMIN_CREATION_FLAG: usize = 73;
    const MARKET_ENABLED_FLAG: usize = 8;
    const MARKET_ECONOMIC_MODE: usize = 202;

    let mut admin = ADMIN.to_vec();
    admin[ADMIN_CREATION_FLAG] = 2;
    assert!(AdminConfig::deserialize(&admin).is_none());

    let mut conviction = CONVICTION.to_vec();
    conviction[MARKET_ENABLED_FLAG] = 2;
    assert!(ConvictionMarketConfig::deserialize(&conviction).is_none());

    let mut timed = TIMED.to_vec();
    timed[MARKET_ECONOMIC_MODE] = 0;
    let curve = TimedMarketConfig::deserialize(&timed).expect("curve is a valid economic mode");
    assert_eq!(curve.common.economic_mode, EconomicMode::Curve);
    timed[MARKET_ECONOMIC_MODE] = 2;
    assert!(TimedMarketConfig::deserialize(&timed).is_none());
}

fn assert_exact_account_bytes<T: CarbonDeserialize>(capture: &[u8]) {
    let account = std::any::type_name::<T>();
    for len in 0..capture.len() {
        assert!(
            <T as CarbonDeserialize>::deserialize(&capture[..len]).is_none(),
            "{account} truncated to {len}"
        );
    }

    let mut wrong_discriminator = capture.to_vec();
    wrong_discriminator[0] ^= 1;
    assert!(
        <T as CarbonDeserialize>::deserialize(&wrong_discriminator).is_none(),
        "{account} wrong discriminator"
    );

    let mut extra = capture.to_vec();
    extra.push(0);
    assert!(
        <T as CarbonDeserialize>::deserialize(&extra).is_none(),
        "{account} extra byte"
    );
}

#[test]
fn gift_config_preserves_the_mint_independent_preset_and_fee() {
    let mut bytes = vec![30, 120, 111, 208, 222, 196, 72, 201];
    bytes.extend_from_slice(&[17; 32]);
    bytes.extend_from_slice(&[23; 32]);
    bytes.extend_from_slice(&[0xff; 32]);
    bytes.extend_from_slice(&[0x2c, 0x01, 253, 254]);

    let config = GiftConfig::deserialize(&bytes).expect("IDL GiftConfig");
    assert_eq!(config.market_config, Pubkey::new_from_array([17; 32]));
    assert_eq!(config.backend_authority, Pubkey::new_from_array([23; 32]));
    assert_eq!(config.label, [0xff; 32]);
    assert_eq!(
        (config.gift_fee_bps, config.admin_bump, config.bump),
        (300, 253, 254)
    );
    assert_exact_account_bytes::<GiftConfig>(&bytes);
}
