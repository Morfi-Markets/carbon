# carbon-morfi-cp-swap-decoder

Partial Morfi CP-Swap compatibility decoder and ABI types for Carbon 2.
Extracted from Morfi monorepo `ef225925c085ed1d0b5c31063b695729767a473c`.

```rust
use carbon_morfi_cp_swap_decoder::MorfiCpSwapDecoder;

let decoder = MorfiCpSwapDecoder::default();
// For another deployment, pass its address to MorfiCpSwapDecoder::new(program_id).
```

The decoder owns byte layouts, discriminators, and positional account arrangement.
It has no dependency on the Morfi indexer, database, environment, or contracts.
The public tuple-variant enum deliberately preserves the existing Morfi API instead
of adopting Carbon 2's generated struct-variant schema. Serde remains enabled.

The decoder accepts only its configured program ID. Unknown or malformed data
returns `None`. Borsh layouts permit trailing bytes. Account arrangement checks the
required positional prefix; it does not verify signer or writable privileges.

This PR makes decoders available in the Morfi public fork. It does not switch
the indexer to these crates or change deployed behavior. The legacy generator is
not included; the ABI source is pinned rather than regenerated.

Supported instructions are `CreateAmmConfig`, `UpdateAmmConfig`, and
`UpdatePoolStatus`, plus `LpChange` and `Swap` CPI events. `AmmConfig` supports
`CarbonDeserialize` byte decoding; an `AccountDecoder` is not implemented. This
crate is not a complete generated IDL package.

The swap layout includes trade, protocol, fund, and creator fee fields, fee-side
flags, and token owners. Short historical swap layouts are rejected.
No new logging is added to this existing decoding flow.
