//! First seeds of the Morfi CP-Swap program PDAs.
//!
//! Each comment gives the other seeds in order. A config index is a big-endian
//! `u16`.

/// `[AMM_CONFIG, index]`
pub const AMM_CONFIG: &[u8] = b"amm_config";
/// `[AUTHORITY]`: the vault and LP mint authority of every pool.
pub const AUTHORITY: &[u8] = b"vault_and_lp_mint_auth_seed";
/// `[POOL, amm_config, token_0_mint, token_1_mint]`
pub const POOL: &[u8] = b"pool";
/// `[POOL_LP_MINT, pool_state]`
pub const POOL_LP_MINT: &[u8] = b"pool_lp_mint";
/// `[POOL_VAULT, pool_state, mint]`
pub const POOL_VAULT: &[u8] = b"pool_vault";
/// `[OBSERVATION, pool_state]`
pub const OBSERVATION: &[u8] = b"observation";
/// `[MARKETS_LP_LOCK, pool_state]`: the LP tokens of a pool that Markets creates.
pub const MARKETS_LP_LOCK: &[u8] = b"markets_lp_lock";
/// `[EVENT_AUTHORITY]`: the Anchor `#[event_cpi]` signer of the CPI events.
pub const EVENT_AUTHORITY: &[u8] = b"__event_authority";

/// Every seed in this module.
pub const ALL: &[&[u8]] = &[
    AMM_CONFIG,
    AUTHORITY,
    POOL,
    POOL_LP_MINT,
    POOL_VAULT,
    OBSERVATION,
    MARKETS_LP_LOCK,
    EVENT_AUTHORITY,
];
