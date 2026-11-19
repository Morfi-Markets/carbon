//! Arguments for Markets instructions that initialize or update program configuration.

use {crate::layout::InstructionLayout, borsh::BorshDeserialize, solana_pubkey::Pubkey};

/// Roles supplied to the Markets admin config initializer.
#[derive(Debug, Clone, BorshDeserialize, PartialEq, Eq)]
pub struct InitAdminConfig {
    pub config_admin: Pubkey,
    pub gift_admin: Pubkey,
    pub transaction_authority: Pubkey,
    pub rent_treasury: Pubkey,
}

/// Gift policy supplied when creating a gift config.
#[derive(Debug, Clone, BorshDeserialize, PartialEq, Eq)]
pub struct InitGiftConfig {
    pub label: [u8; 32],
    pub backend_authority: Pubkey,
    pub gift_fee_bps: u16,
}

/// Market family selected by a creation-restriction update.
#[derive(Debug, Clone, Copy, BorshDeserialize, PartialEq, Eq)]
pub enum ResolutionMode {
    Conviction,
    Timed,
}

/// Decoded arguments. Preset initializers have separate types in `instructions`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigMutation {
    InitAdminConfig(InitAdminConfig),
    InitGiftConfig(InitGiftConfig),
    SetMarketConfigEnabled(bool),
    SetMarketCreationRestricted {
        resolution_mode: ResolutionMode,
        restricted: bool,
    },
    UpdateConfigAdmin(Pubkey),
    UpdateGiftAdmin(Pubkey),
    UpdateRentTreasury(Pubkey),
    UpdateTransactionAuthority(Pubkey),
    UpdateMarketTransactionAuthority(Pubkey),
    UpdateGiftBackendAuthority(Pubkey),
}

impl ConfigMutation {
    /// Reads a recognized Anchor discriminator and its Borsh arguments. Trailing
    /// bytes are accepted, as with the Markets instruction decoders.
    pub fn decode(data: &[u8]) -> Option<Self> {
        let layout = InstructionLayout::for_data(data)?;
        let mut args = data.get(layout.discriminator.len()..)?;
        Some(match layout.name {
            "init_admin_config" => {
                Self::InitAdminConfig(InitAdminConfig::deserialize(&mut args).ok()?)
            }
            "init_gift_config" => {
                Self::InitGiftConfig(InitGiftConfig::deserialize(&mut args).ok()?)
            }
            "set_market_config_enabled" => {
                Self::SetMarketConfigEnabled(bool::deserialize(&mut args).ok()?)
            }
            "set_market_creation_restricted" => Self::SetMarketCreationRestricted {
                resolution_mode: ResolutionMode::deserialize(&mut args).ok()?,
                restricted: bool::deserialize(&mut args).ok()?,
            },
            "update_config_admin" => Self::UpdateConfigAdmin(Pubkey::deserialize(&mut args).ok()?),
            "update_gift_admin" => Self::UpdateGiftAdmin(Pubkey::deserialize(&mut args).ok()?),
            "update_rent_treasury" => {
                Self::UpdateRentTreasury(Pubkey::deserialize(&mut args).ok()?)
            }
            "update_transaction_authority" => {
                Self::UpdateTransactionAuthority(Pubkey::deserialize(&mut args).ok()?)
            }
            "update_market_transaction_authority" => {
                Self::UpdateMarketTransactionAuthority(Pubkey::deserialize(&mut args).ok()?)
            }
            "update_gift_backend_authority" => {
                Self::UpdateGiftBackendAuthority(Pubkey::deserialize(&mut args).ok()?)
            }
            _ => return None,
        })
    }
}
