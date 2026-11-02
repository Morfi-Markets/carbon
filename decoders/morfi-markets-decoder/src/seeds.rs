//! First seeds of the Markets program PDAs.
//!
//! Each comment gives the other seeds in order. A nonce is a little-endian
//! `u64`; an index is one byte. `program` is the Markets program address.

/// `[MARKET, creator, market_nonce]`
pub const MARKET: &[u8] = b"market";
/// `[PLATFORM_FEE_VAULT, market_config, fee_vault_index]`
pub const PLATFORM_FEE_VAULT: &[u8] = b"platform_fee_vault";
/// `[OUTCOME_MINT_AUTHORITY, market]`
pub const OUTCOME_MINT_AUTHORITY: &[u8] = b"outcome_mint_authority";
/// `[MARKET_QUOTE_VAULT_AUTHORITY, market]`
pub const MARKET_QUOTE_VAULT_AUTHORITY: &[u8] = b"market_quote_vault_authority";
/// `[MARKET_QUOTE_VAULT, market]`
pub const MARKET_QUOTE_VAULT: &[u8] = b"market_quote_vault";
/// `[OUTCOME_TOKEN_VAULT, market, outcome_index]`
pub const OUTCOME_TOKEN_VAULT: &[u8] = b"outcome_token_vault";
/// `[USER_POSITIONS, wallet]`
pub const USER_POSITIONS: &[u8] = b"user_positions";
/// `[ADMIN_CONFIG]`
pub const ADMIN_CONFIG: &[u8] = b"admin_config";
/// `[MARKET_REWARDS, market]`
pub const MARKET_REWARDS: &[u8] = b"market_rewards";
/// `[CONVICTION_MARKET_CONFIG, program, config_nonce]`
pub const CONVICTION_MARKET_CONFIG: &[u8] = b"conviction_market_config";
/// `[TIMED_MARKET_CONFIG, program, config_nonce]`
pub const TIMED_MARKET_CONFIG: &[u8] = b"timed_market_config";
/// `[EVENT_SCHEDULE, program, schedule_nonce]`
pub const EVENT_SCHEDULE: &[u8] = b"event_schedule";
/// `[GIFT_CONFIG, market_config, label]`
pub const GIFT_CONFIG: &[u8] = b"gift_config";
/// `[GIFT_ADMIN, gift_config]`
pub const GIFT_ADMIN: &[u8] = b"gift_admin";
/// `[GIFT, gift_config, mint, handler_hash]`
pub const GIFT: &[u8] = b"gift";
/// `[EVENT_AUTHORITY]`: the Anchor `#[event_cpi]` signer of the CPI events.
pub const EVENT_AUTHORITY: &[u8] = b"__event_authority";

/// Every seed in this module.
pub const ALL: &[&[u8]] = &[
    MARKET,
    PLATFORM_FEE_VAULT,
    OUTCOME_MINT_AUTHORITY,
    MARKET_QUOTE_VAULT_AUTHORITY,
    MARKET_QUOTE_VAULT,
    OUTCOME_TOKEN_VAULT,
    USER_POSITIONS,
    ADMIN_CONFIG,
    MARKET_REWARDS,
    CONVICTION_MARKET_CONFIG,
    TIMED_MARKET_CONFIG,
    EVENT_SCHEDULE,
    GIFT_CONFIG,
    GIFT_ADMIN,
    GIFT,
    EVENT_AUTHORITY,
];
