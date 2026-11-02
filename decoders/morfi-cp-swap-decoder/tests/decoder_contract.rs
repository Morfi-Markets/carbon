use carbon_core::instruction::InstructionDecoder;
use carbon_morfi_cp_swap_decoder::{
    layout::SWAP_BASE_INPUT, MorfiCpSwapDecoder, MorfiCpSwapEvent, PROGRAM_ID,
};
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

#[test]
fn configured_cp_swap_program_is_checked_before_decoding() {
    let custom = Pubkey::new_from_array([8; 32]);
    let data = vec![0x82, 0x57, 0x6c, 0x06, 0x2e, 0xe0, 0x75, 0x7b, 4];
    let mut instruction = Instruction {
        program_id: custom,
        accounts: vec![],
        data,
    };
    let decoder = MorfiCpSwapDecoder::new(custom);
    assert!(
        matches!(decoder.decode_instruction(&instruction), Some(MorfiCpSwapEvent::UpdatePoolStatus(value)) if value.status == 4)
    );
    assert!(MorfiCpSwapDecoder::default()
        .decode_instruction(&instruction)
        .is_none());
    instruction.program_id = PROGRAM_ID;
    assert!(decoder.decode_instruction(&instruction).is_none());
    assert!(MorfiCpSwapDecoder::default()
        .decode_instruction(&instruction)
        .is_some());
}

/// Policy code can read positions in constants.
const SWAP_PAYER: Option<usize> = SWAP_BASE_INPUT.position("payer");
const SWAP_OUTPUT_MINT: Option<usize> = SWAP_BASE_INPUT.position("output_token_mint");
const SWAP_CREATOR: Option<usize> = SWAP_BASE_INPUT.position("creator");

#[test]
fn swap_positions_are_constants() {
    assert_eq!(SWAP_PAYER, Some(0));
    assert_eq!(SWAP_OUTPUT_MINT, Some(11));
    assert_eq!(SWAP_CREATOR, None);
}

/// Expected values come from the committed `raydium_cp_swap` IDL.
#[test]
fn swap_base_input_layout_is_readable_without_decoding_the_swap() {
    assert_eq!(SWAP_BASE_INPUT.name, "swap_base_input");
    assert_eq!(
        SWAP_BASE_INPUT.discriminator,
        [143, 190, 90, 218, 196, 30, 51, 222]
    );
    assert_eq!(SWAP_BASE_INPUT.account_names.len(), 15);
    assert_eq!(SWAP_BASE_INPUT.position("payer"), Some(0));
    assert_eq!(SWAP_BASE_INPUT.position("amm_config"), Some(2));
    assert_eq!(SWAP_BASE_INPUT.position("input_token_mint"), Some(10));
    assert_eq!(SWAP_BASE_INPUT.position("output_token_mint"), Some(11));
    assert_eq!(SWAP_BASE_INPUT.position("program"), Some(14));
    assert_eq!(SWAP_BASE_INPUT.position("creator"), None);
    let mut data = SWAP_BASE_INPUT.discriminator.to_vec();
    data.extend([0; 16]);
    let instruction = Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![],
        data,
    };
    assert!(MorfiCpSwapDecoder::default()
        .decode_instruction(&instruction)
        .is_none());
}
