#![cfg(feature = "decoder")]

use {
    carbon_morfi_markets_decoder::current::{
        ConfigMutation, InitAdminConfig, InitGiftConfig, ResolutionMode,
    },
    solana_pubkey::Pubkey,
};

fn assert_decoded(discriminator: [u8; 8], args: &[u8], expected: ConfigMutation) {
    let mut data = discriminator.to_vec();
    data.extend_from_slice(args);
    assert_eq!(ConfigMutation::decode(&data).as_ref(), Some(&expected));
    let mut with_trailing_bytes = data.clone();
    with_trailing_bytes.push(0xff);
    assert_eq!(ConfigMutation::decode(&with_trailing_bytes), Some(expected));
    data.pop();
    assert_eq!(ConfigMutation::decode(&data), None);
}

#[test]
fn writable_config_payloads_follow_the_markets_idl() {
    let key = |n| Pubkey::new_from_array([n; 32]);
    assert_decoded(
        [148, 140, 143, 75, 126, 18, 172, 147],
        &[[1; 32], [2; 32], [3; 32], [4; 32]].concat(),
        ConfigMutation::InitAdminConfig(InitAdminConfig {
            config_admin: key(1),
            gift_admin: key(2),
            transaction_authority: key(3),
            rent_treasury: key(4),
        }),
    );
    let gift_data = [[5u8; 32].as_slice(), &[6; 32], &[0x34, 0x12]].concat();
    assert_decoded(
        [14, 130, 243, 115, 117, 251, 24, 217],
        &gift_data,
        ConfigMutation::InitGiftConfig(InitGiftConfig {
            label: [5; 32],
            backend_authority: key(6),
            gift_fee_bps: 0x1234,
        }),
    );
    let value = key(7);
    for (discriminator, expected) in [
        (
            [30, 254, 195, 1, 169, 26, 214, 39],
            ConfigMutation::UpdateConfigAdmin(value),
        ),
        (
            [244, 27, 60, 224, 86, 203, 69, 248],
            ConfigMutation::UpdateGiftAdmin(value),
        ),
        (
            [239, 180, 137, 176, 183, 208, 20, 235],
            ConfigMutation::UpdateTransactionAuthority(value),
        ),
        (
            [228, 93, 206, 229, 249, 63, 54, 221],
            ConfigMutation::UpdateRentTreasury(value),
        ),
        (
            [156, 132, 11, 78, 215, 147, 149, 56],
            ConfigMutation::UpdateMarketTransactionAuthority(value),
        ),
        (
            [142, 49, 62, 5, 223, 232, 11, 236],
            ConfigMutation::UpdateGiftBackendAuthority(value),
        ),
    ] {
        assert_decoded(discriminator, value.as_ref(), expected);
    }
    assert_decoded(
        [91, 105, 92, 26, 190, 44, 15, 10],
        &[0],
        ConfigMutation::SetMarketConfigEnabled(false),
    );
    for (encoded, resolution_mode) in [(0, ResolutionMode::Conviction), (1, ResolutionMode::Timed)]
    {
        assert_decoded(
            [147, 98, 22, 211, 160, 233, 243, 55],
            &[encoded, 1],
            ConfigMutation::SetMarketCreationRestricted {
                resolution_mode,
                restricted: true,
            },
        );
    }
}

#[test]
fn malformed_flags_modes_and_unknown_discriminators_have_no_config_arguments() {
    assert_eq!(
        ConfigMutation::decode(&[91, 105, 92, 26, 190, 44, 15, 10, 2]),
        None
    );
    assert_eq!(
        ConfigMutation::decode(&[147, 98, 22, 211, 160, 233, 243, 55, 2, 0]),
        None
    );
    assert_eq!(ConfigMutation::decode(&[0xff; 16]), None);
}
