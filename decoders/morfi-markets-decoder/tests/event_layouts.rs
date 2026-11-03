//! The event layout table must agree with the typed current event decoders.
use carbon_morfi_markets_decoder::{
    current::CurrentEvent,
    layout::{EventLayout, ANCHOR_EVENT_TAG, EVENT_LAYOUTS},
};
use std::collections::HashSet;

/// `CurrentEvent` has one variant per current event.
const CURRENT_EVENT_VARIANTS: usize = 31;

/// Name hashes of events the contract no longer emits: the two trade events
/// that `TradeEvent` replaced, and the bonus claim name that the claim event
/// gave up for `WalletPrincipalPrizeClaimedEvent`.
const RETIRED_EVENTS: [(&str, [u8; 8]); 3] = [
    ("WalletTradeEvent", [251, 216, 11, 8, 56, 243, 152, 247]),
    ("PrincipalTradeEvent", [83, 144, 146, 208, 188, 4, 54, 100]),
    (
        "WalletPrincipalBonusClaimedEvent",
        [133, 161, 3, 14, 181, 63, 61, 9],
    ),
];

/// Zero-valued Borsh scalars, empty vectors and strings, and zero public keys
/// make a valid payload for every current event.
fn zero_payload(discriminator: [u8; 8]) -> Vec<u8> {
    let mut data = ANCHOR_EVENT_TAG.to_vec();
    data.extend_from_slice(&discriminator);
    data.resize(2048, 0);
    data
}

#[test]
fn every_event_layout_decodes_to_its_named_current_event() {
    assert_eq!(EVENT_LAYOUTS.len(), CURRENT_EVENT_VARIANTS);
    let mut names = HashSet::new();
    let mut discriminators = HashSet::new();
    for layout in EVENT_LAYOUTS {
        assert!(names.insert(layout.name), "duplicate {}", layout.name);
        assert!(
            discriminators.insert(layout.discriminator),
            "duplicate discriminator of {}",
            layout.name
        );
        let data = zero_payload(layout.discriminator);
        let decoded = CurrentEvent::decode(&data).expect(layout.name);
        assert_eq!(decoded.name(), layout.name);
        assert!(CurrentEvent::recognizes(&data[..16]), "{}", layout.name);
        assert_eq!(EventLayout::for_data(&data), Some(layout));
        assert_eq!(EventLayout::named(layout.name), Some(layout));
    }
}

#[test]
fn retired_events_are_not_current_events() {
    for (name, discriminator) in RETIRED_EVENTS {
        let data = zero_payload(discriminator);
        assert_eq!(EventLayout::named(name), None, "{name}");
        assert!(!CurrentEvent::recognizes(&data), "{name}");
        assert_eq!(CurrentEvent::decode(&data), None, "{name}");
    }
}

#[test]
fn an_event_layout_needs_the_anchor_tag_and_the_full_discriminator() {
    let layout = &EVENT_LAYOUTS[0];
    let data = zero_payload(layout.discriminator);
    assert_eq!(EventLayout::for_data(&data[..15]), None);
    assert!(!CurrentEvent::recognizes(&data[..15]));
    let mut untagged = data.clone();
    untagged[0] ^= 0xff;
    assert_eq!(EventLayout::for_data(&untagged), None);
    assert!(!CurrentEvent::recognizes(&untagged));
    assert_eq!(EventLayout::for_data(&layout.discriminator), None);
    assert_eq!(EventLayout::named("GiftClaimed"), None);
}
