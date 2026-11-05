//! Trade events end with `callout_code: Option<[u8; 8]>` after the Markets upgrade.
//! Fixtures: one buy per mode, from the pre-upgrade and the upgraded program
//! (see provenance.json). Failure modes:
//! 1. A pre-upgrade event without the field fails to decode, so a replay drops history.
//! 2. A new event loses or misreads its code.
//! 3. A malformed option tag or a truncated code decodes instead of failing.
//! 4. The stored JSON of an event without a code changes.
use carbon_morfi_markets_decoder::current::{CurrentEvent, PrincipalTradeEvent, WalletTradeEvent};

const CODE: [u8; 8] = *b"MORFI-42";

fn fixture(name: &str) -> Vec<u8> {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read(&path).unwrap_or_else(|error| panic!("{path}: {error}"))
}

fn wallet(data: &[u8]) -> Option<WalletTradeEvent> {
    match CurrentEvent::decode(data)? {
        CurrentEvent::WalletTradeEvent(event) => Some(event),
        other => panic!("decoded {}", other.name()),
    }
}

fn principal(data: &[u8]) -> Option<PrincipalTradeEvent> {
    match CurrentEvent::decode(data)? {
        CurrentEvent::PrincipalTradeEvent(event) => Some(event),
        other => panic!("decoded {}", other.name()),
    }
}

#[test]
fn wallet_trade_events_decode_before_and_after_the_callout_field() {
    let before = wallet(&fixture("wallet-trade-before-callout.bin")).expect("old layout");
    let after = wallet(&fixture("wallet-trade-callout.bin")).expect("new layout");
    assert_eq!(before.callout_code, None);
    assert_eq!(after.callout_code, Some(CODE));
    assert_eq!(
        WalletTradeEvent {
            callout_code: None,
            ..after
        },
        before
    );
}

#[test]
fn principal_trade_events_decode_before_and_after_the_callout_field() {
    let before = principal(&fixture("principal-trade-before-callout.bin")).expect("old layout");
    let after = principal(&fixture("principal-trade-callout.bin")).expect("new layout");
    assert_eq!(before.callout_code, None);
    assert_eq!(after.callout_code, Some(CODE));
    assert_eq!(
        PrincipalTradeEvent {
            callout_code: None,
            ..after
        },
        before
    );
}

#[test]
fn an_explicit_empty_code_decodes_as_none() {
    let mut data = fixture("wallet-trade-before-callout.bin");
    data.push(0);
    assert_eq!(wallet(&data).map(|event| event.callout_code), Some(None));
}

#[test]
fn a_malformed_callout_code_fails_to_decode() {
    let coded = fixture("wallet-trade-callout.bin");
    let mut bad_tag = coded.clone();
    let tag = bad_tag.len() - 9;
    bad_tag[tag] = 2;
    assert!(wallet(&bad_tag).is_none());
    assert!(wallet(&coded[..coded.len() - 1]).is_none());
    assert!(CurrentEvent::recognizes(&bad_tag));
}

#[test]
fn stored_json_has_no_callout_key_without_a_code() -> serde_json::Result<()> {
    let before = wallet(&fixture("wallet-trade-before-callout.bin")).expect("old layout");
    assert!(serde_json::to_value(&before)?.get("callout_code").is_none());
    let after = principal(&fixture("principal-trade-callout.bin")).expect("new layout");
    assert_eq!(
        serde_json::to_value(&after)?["callout_code"],
        serde_json::json!(CODE)
    );
    Ok(())
}
