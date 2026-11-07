//! `TradeEvent` ends with `callout_code: Option<[u8; 8]>` after the Markets upgrade.
//! Fixtures: one buy per economic mode, from the pre-upgrade and the upgraded
//! program (see provenance.json). Failure modes:
//! 1. A pre-upgrade event without the field fails to decode, so a replay drops history.
//! 2. A new event loses or misreads its code.
//! 3. A malformed option tag or a truncated code decodes instead of failing.
//! 4. The stored JSON of an event without a code changes.
use carbon_morfi_markets_decoder::current::{CurrentEvent, TradeEvent};

const CODE: [u8; 8] = *b"MORFI-42";

fn fixture(name: &str) -> Vec<u8> {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read(&path).unwrap_or_else(|error| panic!("{path}: {error}"))
}

fn trade(data: &[u8]) -> Option<TradeEvent> {
    match CurrentEvent::decode(data)? {
        CurrentEvent::TradeEvent(event) => Some(event),
        other => panic!("decoded {}", other.name()),
    }
}

#[test]
fn trade_events_decode_before_and_after_the_callout_field() {
    for mode in ["curve", "principal"] {
        let before =
            trade(&fixture(&format!("trade-{mode}-before-callout.bin"))).expect("old layout");
        let after = trade(&fixture(&format!("trade-{mode}-callout.bin"))).expect("new layout");
        assert_eq!(before.callout_code, None, "{mode}");
        assert_eq!(after.callout_code, Some(CODE), "{mode}");
        assert_eq!(
            TradeEvent {
                callout_code: None,
                ..after
            },
            before,
            "{mode}: only the code differs"
        );
    }
}

#[test]
fn an_explicit_empty_code_decodes_as_none() {
    let mut data = fixture("trade-curve-before-callout.bin");
    data.push(0);
    assert_eq!(trade(&data).map(|event| event.callout_code), Some(None));
}

#[test]
fn a_malformed_callout_code_fails_to_decode() {
    let coded = fixture("trade-curve-callout.bin");
    let mut bad_tag = coded.clone();
    let tag = bad_tag.len() - 9;
    bad_tag[tag] = 2;
    assert!(trade(&bad_tag).is_none());
    assert!(trade(&coded[..coded.len() - 1]).is_none());
    assert!(CurrentEvent::recognizes(&bad_tag));
}

#[test]
fn stored_json_has_no_callout_key_without_a_code() -> serde_json::Result<()> {
    let before = trade(&fixture("trade-curve-before-callout.bin")).expect("old layout");
    assert!(serde_json::to_value(&before)?.get("callout_code").is_none());
    let after = trade(&fixture("trade-principal-callout.bin")).expect("new layout");
    assert_eq!(
        serde_json::to_value(&after)?["callout_code"],
        serde_json::json!(CODE)
    );
    Ok(())
}
