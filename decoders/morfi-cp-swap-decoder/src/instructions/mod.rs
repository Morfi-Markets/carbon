//! MorfiCpSwapEvent dispatch enum and InstructionDecoder impl.
//! Imports the SHADOW types from `super::events` (NOT from morfi_events) —
//! see plan architecture section for shadow-struct rationale.

use {
    super::events::{LpChangeEvent, SwapEvent},
    carbon_core::{account_utils::next_account, borsh},
};

#[derive(
    borsh::BorshDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreateAmmConfig {
    pub index: u16,
    pub trade_fee_rate: u64,
    pub protocol_fee_rate: u64,
    pub fund_fee_rate: u64,
    pub creator_fee_rate: u64,
}

impl_carbon_deserialize!(CreateAmmConfig, "0x8934edd4d7756c68");

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CreateAmmConfigAccounts {
    pub amm_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateAmmConfig {
    type ArrangedAccounts = CreateAmmConfigAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut accounts = accounts.iter();
        next_account(&mut accounts)?;
        let amm_config = next_account(&mut accounts)?;
        next_account(&mut accounts)?;
        Some(CreateAmmConfigAccounts { amm_config })
    }
}

#[derive(
    borsh::BorshDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateAmmConfig {
    pub param: u8,
    pub value: u64,
}

impl_carbon_deserialize!(UpdateAmmConfig, "0x313cae889a1c74c8");

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct UpdateAmmConfigAccounts {
    pub amm_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateAmmConfig {
    type ArrangedAccounts = UpdateAmmConfigAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut accounts = accounts.iter();
        next_account(&mut accounts)?;
        Some(UpdateAmmConfigAccounts {
            amm_config: next_account(&mut accounts)?,
        })
    }
}

#[derive(
    borsh::BorshDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdatePoolStatus {
    pub status: u8,
}

impl_carbon_deserialize!(UpdatePoolStatus, "0x82576c062ee0757b");

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct UpdatePoolStatusAccounts {
    pub pool_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePoolStatus {
    type ArrangedAccounts = UpdatePoolStatusAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut accounts = accounts.iter();
        next_account(&mut accounts)?;
        Some(UpdatePoolStatusAccounts {
            pool_state: next_account(&mut accounts)?,
        })
    }
}

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum MorfiCpSwapEvent {
    CreateAmmConfig(CreateAmmConfig),
    LpChange(LpChangeEvent),
    Swap(SwapEvent),
    UpdateAmmConfig(UpdateAmmConfig),
    UpdatePoolStatus(UpdatePoolStatus),
}

impl carbon_core::instruction::InstructionDecoder<'_> for super::MorfiCpSwapDecoder {
    type InstructionType = MorfiCpSwapEvent;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<Self::InstructionType> {
        if !instruction.program_id.eq(&self.program_id) {
            return None;
        }

        decode_first_variant!(instruction.data.as_slice(),
            MorfiCpSwapEvent::CreateAmmConfig => CreateAmmConfig,
            MorfiCpSwapEvent::LpChange => LpChangeEvent,
            MorfiCpSwapEvent::Swap => SwapEvent,
            MorfiCpSwapEvent::UpdateAmmConfig => UpdateAmmConfig,
            MorfiCpSwapEvent::UpdatePoolStatus => UpdatePoolStatus,
        )
    }
}

#[cfg(test)]
mod tests {
    use {
        super::{CreateAmmConfig, MorfiCpSwapEvent, SwapEvent, UpdateAmmConfig, UpdatePoolStatus},
        crate::{MorfiCpSwapDecoder, PROGRAM_ID},
        carbon_core::{
            borsh::BorshSerialize,
            deserialize::{ArrangeAccounts, CarbonDeserialize},
            instruction::InstructionDecoder,
        },
        solana_instruction::{AccountMeta, Instruction},
        solana_pubkey::Pubkey,
    };

    #[test]
    fn decoder_accepts_fee_complete_swap_event() {
        let event = sample_swap();
        let decoded = MorfiCpSwapDecoder::default()
            .decode_instruction(&instruction(encoded(&event)))
            .expect("decoder must accept the fee-complete SwapEvent");

        assert_eq!(decoded, MorfiCpSwapEvent::Swap(event));
    }

    #[test]
    fn decoder_rejects_legacy_swap_layout() {
        let mut legacy_layout = encoded(&sample_swap());
        // The old SwapEvent omitted these two u64 fee-liability fields between
        // trade_fee and creator_fee. The normal event discriminator now
        // requires the complete layout, so historical payloads must fail.
        let fee_split_start = SwapEvent::DISCRIMINATOR.len() + 8 + 32 + (8 * 6) + 1 + (32 * 2) + 8;
        legacy_layout.drain(fee_split_start..fee_split_start + 16);

        assert!(
            MorfiCpSwapDecoder::default()
                .decode_instruction(&instruction(legacy_layout))
                .is_none(),
            "the decoder must not accept the historical SwapEvent layout"
        );
    }

    #[test]
    fn decoder_pins_admin_mutation_contract_discriminators() {
        let mut create = CreateAmmConfig::DISCRIMINATOR.to_vec();
        create.extend_from_slice(&7_u16.to_le_bytes());
        for value in [2_500_u64, 120_000, 80_000, 1_500] {
            create.extend_from_slice(&value.to_le_bytes());
        }
        let create = MorfiCpSwapDecoder::default()
            .decode_instruction(&instruction(create))
            .expect("decode config creation");
        assert!(matches!(
            create,
            MorfiCpSwapEvent::CreateAmmConfig(CreateAmmConfig {
                index: 7,
                trade_fee_rate: 2_500,
                protocol_fee_rate: 120_000,
                fund_fee_rate: 80_000,
                creator_fee_rate: 1_500,
                ..
            })
        ));

        let mut config = UpdateAmmConfig::DISCRIMINATOR.to_vec();
        config.push(7);
        config.extend_from_slice(&1_500_u64.to_le_bytes());
        let config = MorfiCpSwapDecoder::default()
            .decode_instruction(&instruction(config))
            .expect("decode config mutation");
        assert_eq!(
            config,
            MorfiCpSwapEvent::UpdateAmmConfig(UpdateAmmConfig {
                param: 7,
                value: 1_500,
            })
        );

        let mut status = UpdatePoolStatus::DISCRIMINATOR.to_vec();
        status.push(4);
        let status = MorfiCpSwapDecoder::default()
            .decode_instruction(&instruction(status))
            .expect("decode status mutation");
        assert!(matches!(status, MorfiCpSwapEvent::UpdatePoolStatus(_)));
    }

    #[test]
    fn config_instruction_accounts_require_the_canonical_prefix() {
        let owner = Pubkey::new_unique();
        let config = Pubkey::new_unique();
        let system = Pubkey::new_unique();
        let create_accounts = vec![
            AccountMeta::new(owner, true),
            AccountMeta::new(config, false),
            AccountMeta::new_readonly(system, false),
        ];
        assert_eq!(
            CreateAmmConfig::arrange_accounts(&create_accounts)
                .expect("complete create accounts")
                .amm_config,
            config
        );
        assert!(CreateAmmConfig::arrange_accounts(&create_accounts[..2]).is_none());

        let update_accounts = &create_accounts[..2];
        assert_eq!(
            UpdateAmmConfig::arrange_accounts(update_accounts)
                .expect("complete update accounts")
                .amm_config,
            config
        );
        assert!(UpdateAmmConfig::arrange_accounts(&update_accounts[..1]).is_none());
    }

    fn instruction(data: Vec<u8>) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            accounts: Vec::new(),
            data,
        }
    }

    fn encoded<T>(event: &T) -> Vec<u8>
    where
        T: CarbonDeserialize + BorshSerialize,
    {
        let mut bytes = T::DISCRIMINATOR.to_vec();
        event
            .serialize(&mut bytes)
            .expect("serialize CP-swap event fixture");
        bytes
    }

    fn sample_swap() -> SwapEvent {
        SwapEvent {
            ts: 1,
            pool_id: Pubkey::new_from_array([1; 32]),
            input_vault_before: 2,
            output_vault_before: 3,
            input_amount: 4,
            output_amount: 5,
            input_transfer_fee: 6,
            output_transfer_fee: 7,
            base_input: true,
            input_mint: Pubkey::new_from_array([2; 32]),
            output_mint: Pubkey::new_from_array([3; 32]),
            trade_fee: 8,
            protocol_fee: 2,
            fund_fee: 1,
            creator_fee: 9,
            creator_fee_on_input: false,
            input_token_owner: Pubkey::new_from_array([4; 32]),
            output_token_owner: Pubkey::new_from_array([5; 32]),
        }
    }
}
