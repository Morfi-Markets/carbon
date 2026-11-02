//! Morfi Markets ABI layouts and Carbon instruction decoding.
//!
//! This crate preserves Morfi's existing tuple-variant decoder API on Carbon 2.
//! Program IDs are supplied explicitly; no process environment is read.
//!
//! `layout` and `seeds` have no dependencies. Disable the default `decoder`
//! feature to build only them, for example in a service that checks instruction
//! accounts or derives PDAs.

#[cfg(feature = "decoder")]
use solana_pubkey::Pubkey;

/// Decodes instructions and Anchor self-CPI events for a Morfi deployment.
#[cfg(feature = "decoder")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MorfiMarketsDecoder {
    program_id: Pubkey,
}

#[cfg(feature = "decoder")]
impl MorfiMarketsDecoder {
    /// Selects the deployment whose instructions this decoder accepts.
    pub const fn new(program_id: Pubkey) -> Self {
        Self { program_id }
    }
}

#[cfg(feature = "decoder")]
impl Default for MorfiMarketsDecoder {
    fn default() -> Self {
        Self::new(PROGRAM_ID)
    }
}

/// Default Morfi devnet program address.
#[cfg(feature = "decoder")]
pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("XuXkWAz3xRktfEjAfFDdkFksYBqFzet8JPkQxZMoRfi");

#[cfg(feature = "decoder")]
#[macro_use]
mod deserialize;

#[cfg(feature = "decoder")]
pub mod current;
#[cfg(feature = "decoder")]
pub mod events;
#[cfg(feature = "decoder")]
pub mod instructions;
pub mod layout;
pub mod seeds;
#[cfg(feature = "decoder")]
pub mod types;
#[cfg(feature = "decoder")]
pub use instructions::MorfiMarketsEvent;
