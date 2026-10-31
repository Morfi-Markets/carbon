use carbon_core::instruction::InstructionDecoder;
use carbon_morfi_cp_swap_decoder::{MorfiCpSwapDecoder, MorfiCpSwapEvent, PROGRAM_ID};
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
