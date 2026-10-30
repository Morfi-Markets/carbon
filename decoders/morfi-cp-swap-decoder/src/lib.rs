//! Morfi CP-Swap ABI layouts and Carbon instruction decoding.
//!
//! This crate preserves Morfi's existing tuple-variant decoder API on Carbon 2.
//! Program IDs are supplied explicitly; no process environment is read.

use solana_pubkey::Pubkey;

/// Decodes instructions and Anchor self-CPI events for a Morfi deployment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MorfiCpSwapDecoder {
    program_id: Pubkey,
}

impl MorfiCpSwapDecoder {
    /// Selects the deployment whose instructions this decoder accepts.
    pub const fn new(program_id: Pubkey) -> Self {
        Self { program_id }
    }
}

impl Default for MorfiCpSwapDecoder {
    fn default() -> Self {
        Self::new(PROGRAM_ID)
    }
}

/// Default Morfi devnet program address.
pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("morfijP38RdW7M9jJ4rAEpcsyXSQgHpCafN3vgvYwGC");

#[macro_use]
mod deserialize;

pub mod accounts;
pub mod events;
pub mod instructions;
pub use instructions::MorfiCpSwapEvent;
