//! CP-Swap account layouts the indexer reads from account data.

use carbon_core::borsh;

/// `AmmConfig` (Anchor `#[account]`, 8-byte discriminator plus 219 bytes).
#[derive(borsh::BorshDeserialize, Debug, PartialEq, Eq, Clone)]
pub struct AmmConfig {
    pub bump: u8,
    pub index: u16,
    pub trade_fee_rate: u64,
    pub protocol_fee_rate: u64,
    pub fund_fee_rate: u64,
    pub protocol_owner: solana_pubkey::Pubkey,
    pub fund_owner: solana_pubkey::Pubkey,
    pub creator_fee_rate: u64,
    pub padding: [u64; 15],
}

impl_carbon_deserialize!(AmmConfig, "0xdaf42168cbcb2b6f");

#[cfg(test)]
mod tests {
    use {super::AmmConfig, carbon_core::deserialize::CarbonDeserialize};

    /// Field order and widths from the program's `AmmConfig` (LEN 227).
    fn account_data(trade_fee_rate: u64, creator_fee_rate: u64) -> Vec<u8> {
        let mut data = AmmConfig::DISCRIMINATOR.to_vec();
        data.push(254);
        data.extend_from_slice(&1_u16.to_le_bytes());
        for value in [trade_fee_rate, 120_000, 80_000] {
            data.extend_from_slice(&value.to_le_bytes());
        }
        data.extend_from_slice(&[0x11; 32]);
        data.extend_from_slice(&[0x22; 32]);
        data.extend_from_slice(&creator_fee_rate.to_le_bytes());
        data.extend_from_slice(&[0; 120]);
        data
    }

    #[test]
    fn amm_config_account_data_decodes_its_fee_rates() {
        let data = account_data(3_500, 1_500);
        assert_eq!(data.len(), 227);

        let account = AmmConfig::deserialize(&data).unwrap();

        assert_eq!(
            (
                account.index,
                account.trade_fee_rate,
                account.creator_fee_rate
            ),
            (1, 3_500, 1_500)
        );
        assert_eq!(account.fund_owner.to_bytes(), [0x22; 32]);
    }

    #[test]
    fn truncated_amm_config_data_does_not_decode() {
        let mut data = account_data(3_500, 1_500);
        data.truncate(100);
        assert!(AmmConfig::deserialize(&data).is_none());
    }
}
