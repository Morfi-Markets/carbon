# Decoder extraction verification

Source: Morfi monorepo revision `ef225925c085ed1d0b5c31063b695729767a473c`.
Carbon base: `40a06e1a3fc4970cbbbfc5a3fd63b37495484ba9`.

Before extraction, identified failure modes: discriminator/dispatch drift,
wrong-program acceptance, truncated known events silently treated as unknown,
initializer length drift, trailing-byte rejection, positional account drift,
fee-field offsets, u128 JSON precision loss, missing standalone dependencies,
and dependence on the private Morfi checkout.

Existing ABI regression tests move with their production owner. New public-boundary
cases were written before extracting the implementation. They cover configurable
program IDs, raw instruction-context recognition, account prefix/extra-account
semantics, malformed recognition, exact initializer sizes, and a captured boost
with independently recorded amount, market, and timestamp. CP-Swap retains its
fee-complete swap and historical-layout rejection tests.

Fixture provenance is recorded in `tests/fixtures/provenance.json`.
The generator is intentionally omitted: this is a pinned extraction, not a
regeneration. Neither crate needs the Morfi contracts or application checkout.

## Results — 2026-10-30

- Focused crate tests: **22 passed**, no failures.
- Clippy for both crates, all targets and features, with `-D warnings`: passed.
- Rustdoc for both crates with `RUSTDOCFLAGS="-D warnings"`: passed.
- External consumers depending on only one decoder crate each: both passed,
  with independent dependency resolution outside the Carbon workspace.
- Targeted `cargo machete`: no unused dependencies in either added crate.
- Compatibility replay: **267 captured instruction/CPI inputs**, each replayed
  unchanged, with trailing bytes, truncated, and under the wrong program ID:
  **1,068 matching comparisons** against the pre-extraction decoder modules.
  Compared Debug/serde output, current-event recognition, and warning messages.
  205 original inputs decoded, including 3 CP-Swap instructions/events; unsupported
  and rejected inputs were compared as well. Covered current and legacy dispatch.
- Corpus SHA256: `8b3a9add3136211ea52049f487540a5def5ccf200615f196a22236687cc515ae`. Each local corpus record carries its source-file
  SHA256, signature, slot when available, and instruction/CPI path.

Run these focused commands from the Carbon repository root. On Morfi workstations,
invoke Cargo through `node /path/to/morfi/scripts/storage/run.mjs --`.

```sh
cargo fmt --check -p carbon-morfi-markets-decoder -p carbon-morfi-cp-swap-decoder
cargo test --locked -p carbon-morfi-markets-decoder -p carbon-morfi-cp-swap-decoder --all-targets
cargo clippy --locked -p carbon-morfi-markets-decoder -p carbon-morfi-cp-swap-decoder --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p carbon-morfi-markets-decoder -p carbon-morfi-cp-swap-decoder
cargo machete decoders/morfi-markets-decoder decoders/morfi-cp-swap-decoder
```

The compatibility replay artifact is retained locally under
`target/morfi-extraction-verification/`, including the corpus, frozen baseline
modules, executable comparator, command, build log, and JSON result. It is not a
public dependency of either crate. The smaller committed fixture slices remain
self-contained and have independently recorded ABI expectations.

Three malformed raw transaction fixture files were skipped during collection;
their JSON-encoded companion captures were available. The corpus is a compatibility
check, not proof of every program layout or a complete product lifecycle run.
Full-workspace CI is left to the existing Carbon workflow. The parent indexer and
its generator are unchanged.

## Public API review corrections

A strict public-type compiler check reproduced four inaccessible exported-field or
account-result types before the fix. All four now have public reexports. The check
passes with the diagnostic denied:

```sh
cargo rustc --locked -p carbon-morfi-markets-decoder --lib -- -D unnameable_types
```

The documentation distinguishes current raw instruction contexts from preserved
legacy typed layouts and makes the partial compatibility scope explicit. The
CP-Swap legacy-layout regression now removes protocol/fund fee fields at byte 177,
including the complete CPI discriminator and timestamp in the offset.

After these corrections, all 22 focused tests, Clippy with warnings denied, and
Rustdoc with warnings denied passed again. Logs are retained in the local
verification artifact as `review-tests.log`, `review-clippy.log`, and
`review-rustdoc.log`. No decoding or account-arrangement behavior changed.

Both crates also passed actual `cargo package --locked --exclude-lockfile`
verification against registry dependencies, not just local workspace paths.

## Sponsored transaction ABI update

Source: Morfi stack revision `7fb762faec`.
The Markets instructions `buy`, `sell`, and `back` now have the `payer` and
`system_program` accounts. `clear_settled_loser` is a new current instruction.
The `Back` account arrangement follows the current `back` order.

`tests/current_abi.rs` was written first. It failed four of four cases against
the extracted ABI. Its expected account orders come from the committed Markets IDL.
After the change, all 26 focused tests, Clippy with warnings denied, and Rustdoc
with warnings denied passed.

## Instruction layout table

`current::INSTRUCTION_LAYOUTS` replaces the discriminator match in
`CurrentInstruction::decode`. The table was generated from the match arms.
`every_layout_is_unique_and_matches_its_decoded_context` passed against the
match implementation before the switch, and it passed again after it. The test
also checks that names and discriminators are unique. CP-Swap has the
`layout::SWAP_BASE_INPUT` layout, with expected values from the committed
`raydium_cp_swap` IDL. The CP-Swap decoder output did not change.

After this change, all 29 focused tests, Clippy with warnings denied, Rustdoc with
warnings denied, and the `unnameable_types` check passed.

`InstructionLayout::position` is a `const fn` in both crates, so a consumer can hold
a position in a constant. Two tests read positions in constants; they did not
compile before this change. After it, all 31 focused tests, Clippy with warnings
denied, and Rustdoc with warnings denied passed.

## Layout-only build

The layouts moved to the dependency-free `layout` module; `current` re-exports
them. Both crates have a default `decoder` feature that holds every other module
and dependency. A consumer that sets `default-features = false` gets only
`layout`, so carbon-core does not pin its Solana versions. The reason: carbon-core
2.0 pins `solana-transaction-status = "=4.2.2"`, which needs
`solana-transaction >=4.1.4, <4.2.0` and `solana-message >=4.2.4, <4.5.0`.

## Progressive fee ABI — contracts `2f0a2ce`

Contracts PR #27 replaces the eight fee fields and
`conviction_reference_margin_ppb` in `CommonMarketConfigParams` with one embedded
`FeeConfig` (ten `u16` rates and `buy_reference_threshold_ppb: u64`). The typed
conviction and timed initializers grow from 211 to 215 bytes and from 171 to 175
bytes. `WalletPrincipalPrizeClaimedEvent` is now `WalletPrincipalBonusClaimedEvent`
(discriminator `85a1030eb53f3d09`), and its `platform_sell_fee` field is now
`platform_claim_fee` at the same position. Earlier initializer and claim layouts
no longer decode.

The two initializer fixtures are the captured 81ace2a initializers re-encoded with
the 2f0a2ce IDL; `tests/fixtures/provenance.json` records the replaced fields and
the FeeConfig values. The tests assert the exact lengths, the decoded FeeConfig,
and the claim fee offset. Focused tests (25), Clippy with `-D warnings`, Rustdoc
with `-D warnings`, and `cargo fmt --check` passed for this crate.

## Back and unback fee ABI — contracts `af0d6bb`

Contracts PR #31 replaces `FeeConfig.platform_general_fee_bps` with
`platform_back_fee_bps` and `platform_unback_fee_bps` at the same position.
`FeeConfig` grows by 2 bytes. The typed conviction and timed initializers grow
from 215 to 217 bytes and from 175 to 177 bytes. Contracts PR #30 removes `payer`
and `system_program` from `resize_user_positions`, which now has 7 accounts.
Earlier initializer layouts no longer decode.

The two initializer fixtures are the `2f0a2ce` fixtures with one `u16` inserted
after `platform_back_fee_bps`. Both new rates keep the earlier general rate (300),
because that rate applied to back and unback. Focused tests (25), Clippy with
`-D warnings`, Rustdoc with `-D warnings`, the layout-only build and
`cargo fmt --check` passed for this crate.
