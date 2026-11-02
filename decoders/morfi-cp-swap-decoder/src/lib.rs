//! Morfi CP-Swap ABI layouts and Carbon instruction decoding.
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
pub struct MorfiCpSwapDecoder {
    program_id: Pubkey,
}

#[cfg(feature = "decoder")]
impl MorfiCpSwapDecoder {
    /// Selects the deployment whose instructions this decoder accepts.
    pub const fn new(program_id: Pubkey) -> Self {
        Self { program_id }
    }
}

#[cfg(feature = "decoder")]
impl Default for MorfiCpSwapDecoder {
    fn default() -> Self {
        Self::new(PROGRAM_ID)
    }
}

/// Default Morfi devnet program address.
#[cfg(feature = "decoder")]
pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("morfijP38RdW7M9jJ4rAEpcsyXSQgHpCafN3vgvYwGC");

#[cfg(feature = "decoder")]
#[macro_use]
mod deserialize;

#[cfg(feature = "decoder")]
pub mod accounts;
#[cfg(feature = "decoder")]
pub mod events;
#[cfg(feature = "decoder")]
pub mod instructions;
pub mod layout;
pub mod seeds;
#[cfg(feature = "decoder")]
pub use instructions::MorfiCpSwapEvent;
