//! Arguments for Markets instructions that initialize or update program configuration.

use {crate::current::CurrentInstruction, borsh::BorshDeserialize, solana_pubkey::Pubkey};

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
pub enum ConfigInstructionArgs {
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

impl CurrentInstruction {
    /// Reads Borsh arguments for the instruction name that the decoder matched.
    /// Returns `None` for other instructions or invalid arguments. Accepts trailing bytes.
    pub fn config_args(&self) -> Option<ConfigInstructionArgs> {
        let (_, mut args) = self.data.split_first_chunk::<8>()?;
        Some(match self.name.as_str() {
            "init_admin_config" => ConfigInstructionArgs::InitAdminConfig(
                InitAdminConfig::deserialize(&mut args).ok()?,
            ),
            "init_gift_config" => {
                ConfigInstructionArgs::InitGiftConfig(InitGiftConfig::deserialize(&mut args).ok()?)
            }
            "set_market_config_enabled" => {
                ConfigInstructionArgs::SetMarketConfigEnabled(bool::deserialize(&mut args).ok()?)
            }
            "set_market_creation_restricted" => {
                ConfigInstructionArgs::SetMarketCreationRestricted {
                    resolution_mode: ResolutionMode::deserialize(&mut args).ok()?,
                    restricted: bool::deserialize(&mut args).ok()?,
                }
            }
            "update_config_admin" => {
                ConfigInstructionArgs::UpdateConfigAdmin(Pubkey::deserialize(&mut args).ok()?)
            }
            "update_gift_admin" => {
                ConfigInstructionArgs::UpdateGiftAdmin(Pubkey::deserialize(&mut args).ok()?)
            }
            "update_rent_treasury" => {
                ConfigInstructionArgs::UpdateRentTreasury(Pubkey::deserialize(&mut args).ok()?)
            }
            "update_transaction_authority" => ConfigInstructionArgs::UpdateTransactionAuthority(
                Pubkey::deserialize(&mut args).ok()?,
            ),
            "update_market_transaction_authority" => {
                ConfigInstructionArgs::UpdateMarketTransactionAuthority(
                    Pubkey::deserialize(&mut args).ok()?,
                )
            }
            "update_gift_backend_authority" => ConfigInstructionArgs::UpdateGiftBackendAuthority(
                Pubkey::deserialize(&mut args).ok()?,
            ),
            _ => return None,
        })
    }
}
