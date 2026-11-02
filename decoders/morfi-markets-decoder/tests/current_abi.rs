//! Account orders below are copied from the Markets IDL, not from the decoder.
use carbon_core::deserialize::ArrangeAccounts;
use carbon_morfi_markets_decoder::{current::CurrentInstruction, instructions::Back};
use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

const IDL_TRADE_ACCOUNTS: [&str; 16] = [
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

const IDL_BACK_ACCOUNTS: [&str; 16] = [
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
];

fn distinct_accounts(count: u8) -> Vec<AccountMeta> {
    (0..count)
        .map(|index| AccountMeta::new_readonly(Pubkey::new_from_array([index; 32]), false))
        .collect()
}

fn recognized(discriminator: [u8; 8]) -> CurrentInstruction {
    CurrentInstruction::decode(&discriminator).expect("current discriminator")
}

#[test]
fn buy_and_sell_map_the_sponsored_trade_accounts() {
    let accounts = distinct_accounts(16);
    for (discriminator, name) in [
        ([102, 6, 61, 18, 1, 218, 235, 234], "buy"),
        ([51, 230, 133, 164, 1, 127, 131, 173], "sell"),
    ] {
        let instruction = recognized(discriminator);
        assert_eq!(instruction.name, name);
        assert_eq!(instruction.account_names, IDL_TRADE_ACCOUNTS);
        let named = instruction.accounts(&accounts).expect("complete trade");
        assert_eq!(named["payer"], accounts[1].pubkey, "{name}");
        assert_eq!(named["transaction_authority"], accounts[2].pubkey, "{name}");
        assert_eq!(named["market"], accounts[3].pubkey, "{name}");
        assert_eq!(named["system_program"], accounts[12].pubkey, "{name}");
        assert_eq!(named["program"], accounts[15].pubkey, "{name}");
        assert!(instruction.accounts(&accounts[..15]).is_err(), "{name}");
    }
}

#[test]
fn back_maps_the_sponsored_backing_accounts() {
    let accounts = distinct_accounts(16);
    let instruction = recognized([1, 144, 109, 177, 110, 131, 124, 0]);
    assert_eq!(instruction.name, "back");
    assert_eq!(instruction.account_names, IDL_BACK_ACCOUNTS);
    let named = instruction.accounts(&accounts).expect("complete back");
    assert_eq!(named["payer"], accounts[1].pubkey);
    assert_eq!(named["market"], accounts[3].pubkey);
    assert_eq!(named["user_positions"], accounts[7].pubkey);
    assert_eq!(named["system_program"], accounts[12].pubkey);
    assert!(instruction.accounts(&accounts[..15]).is_err());
}

#[test]
fn clear_settled_loser_is_a_current_instruction() {
    let accounts = distinct_accounts(5);
    let instruction = recognized([176, 29, 84, 15, 73, 92, 145, 141]);
    assert_eq!(instruction.name, "clear_settled_loser");
    assert_eq!(
        instruction.account_names,
        [
            "owner",
            "market",
            "user_positions",
            "event_authority",
            "program"
        ]
    );
    let named = instruction.accounts(&accounts).expect("complete clear");
    assert_eq!(named["owner"], accounts[0].pubkey);
    assert_eq!(named["user_positions"], accounts[2].pubkey);
}

#[test]
fn legacy_back_binds_the_market_at_the_sponsored_position() {
    let accounts = distinct_accounts(16);
    assert_eq!(Back::arrange_accounts(&accounts), Some(accounts[3].pubkey));
    assert!(Back::arrange_accounts(&accounts[..15]).is_none());
}
