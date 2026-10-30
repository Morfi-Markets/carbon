//! Morfi Markets ABI layouts and Carbon instruction decoding.
//!
//! This crate preserves Morfi's existing tuple-variant decoder API on Carbon 2.
//! Program IDs are supplied explicitly; no process environment is read.

use solana_pubkey::Pubkey;

/// Decodes instructions and Anchor self-CPI events for a Morfi deployment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MorfiMarketsDecoder {
    program_id: Pubkey,
}

impl MorfiMarketsDecoder {
    /// Selects the deployment whose instructions this decoder accepts.
    pub const fn new(program_id: Pubkey) -> Self {
        Self { program_id }
    }
}

impl Default for MorfiMarketsDecoder {
    fn default() -> Self {
        Self::new(PROGRAM_ID)
    }
}

/// Default Morfi devnet program address.
pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("XuXkWAz3xRktfEjAfFDdkFksYBqFzet8JPkQxZMoRfi");

#[macro_use]
mod deserialize;

pub mod current;
pub mod events;
pub mod instructions;
pub mod types;
pub use instructions::MorfiMarketsEvent;
