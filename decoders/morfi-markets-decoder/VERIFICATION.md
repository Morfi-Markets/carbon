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
