# carbon-morfi-markets-decoder

Morfi Markets ABI types and instruction/event decoding for Carbon 2.
Extracted from Morfi monorepo `ef225925c085ed1d0b5c31063b695729767a473c`.

```rust
use carbon_morfi_markets_decoder::MorfiMarketsDecoder;

let decoder = MorfiMarketsDecoder::default();
// For another deployment, pass its address to MorfiMarketsDecoder::new(program_id).
```

The decoder owns byte layouts, discriminators, and positional account arrangement.
It has no dependency on the Morfi indexer, database, environment, or contracts.
The public tuple-variant enum deliberately preserves the existing Morfi API instead
of adopting Carbon 2's generated struct-variant schema. Serde remains enabled.

The decoder accepts only its configured program ID. Unknown or malformed data
returns `None`. Borsh layouts permit trailing bytes except the Markets configuration
initializers, which retain exact-size checks. Account arrangement checks the
required positional prefix; it does not verify signer or writable privileges.

This PR makes decoders available in the Morfi public fork. It does not switch
the indexer to these crates or change deployed behavior. The legacy generator is
not included; the ABI source is pinned rather than regenerated.

`CurrentInstruction` recognizes an instruction discriminator and retains its raw
bytes and positional account names. It does not validate instruction arguments.
`CurrentEvent::recognizes` distinguishes known discriminators from unknown data,
including recognized but truncated events. CPI events use the 8-byte Anchor
self-CPI tag followed by the 8-byte event discriminator.

`markets.undecodable_event` reports undecodable emitted events. A valid
`SellCapApplied` is intentionally omitted without a warning; malformed payloads
remain reported. Logging is performed once by the decoder, through `log`.

Storage-specific JSON conversion, signed `i64` amount validation, accounting wallet
selection, and transaction locking stay in the indexer. The existing u128 string
serializer is preserved without adding a new deserializer.

See [verification](VERIFICATION.md) and fixture provenance under `tests/fixtures`.

For custom deployments, call `CreateMarket::arrange_accounts_for_program` or
`CreateTimedMarket::arrange_accounts_for_program` with the configured program ID.
Their `ArrangeAccounts` trait implementations use the default devnet program ID
for the optional schedule sentinel. The account list's program slot is not trusted
to select the deployment.
