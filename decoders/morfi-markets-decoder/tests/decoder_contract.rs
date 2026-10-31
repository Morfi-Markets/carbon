use carbon_core::instruction::InstructionDecoder;
use carbon_morfi_markets_decoder::{
    current::{CurrentEvent, CurrentInstruction},
    MorfiMarketsDecoder, MorfiMarketsEvent, PROGRAM_ID,
};
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

#[test]
fn configured_program_is_checked_without_process_environment() {
    let data = vec![157, 233, 139, 121, 246, 62, 234, 235];
    let custom = Pubkey::new_from_array([7; 32]);
    let mut instruction = Instruction {
        program_id: custom,
        accounts: vec![],
        data,
    };
    let decoder = MorfiMarketsDecoder::new(custom);
    assert!(
        matches!(decoder.decode_instruction(&instruction), Some(MorfiMarketsEvent::CurrentInstruction(value)) if value.name == "claim_prize")
    );
    assert!(MorfiMarketsDecoder::default()
        .decode_instruction(&instruction)
        .is_none());
    instruction.program_id = PROGRAM_ID;
    assert!(decoder.decode_instruction(&instruction).is_none());
    assert!(MorfiMarketsDecoder::default()
        .decode_instruction(&instruction)
        .is_some());
}

#[test]
fn instruction_context_retains_raw_bytes_and_accepts_extra_accounts() {
    let data = [157, 233, 139, 121, 246, 62, 234, 235, 255];
    let instruction = CurrentInstruction::decode(&data).expect("known instruction");
    assert_eq!(instruction.name, "claim_prize");
    assert_eq!(instruction.data, data);
    let accounts: Vec<_> = (0..11)
        .map(|index| AccountMeta::new_readonly(Pubkey::new_from_array([index; 32]), false))
        .collect();
    let arranged = instruction
        .accounts(&accounts)
        .expect("prefix plus extra account");
    assert_eq!(arranged.len(), 10);
    assert_eq!(arranged["claimer"], accounts[0].pubkey);
    assert_eq!(arranged["program"], accounts[9].pubkey);
    let error = instruction
        .accounts(&accounts[..9])
        .expect_err("incomplete prefix");
    assert_eq!(
        error.to_string(),
        "current instruction account list incomplete: expected at least 10 accounts, got 9"
    );
}

#[test]
fn malformed_known_event_is_distinct_from_an_unknown_discriminator() {
    let known = [
        0xe4, 0x45, 0xa5, 0x2e, 0x51, 0xcb, 0x9a, 0x1d, 0xba, 0x84, 0xf9, 0x32, 0x00, 0x71, 0x49,
        0xdb,
    ];
    assert!(CurrentEvent::recognizes(&known));
    assert!(CurrentEvent::decode(&known).is_none());
    assert!(!CurrentEvent::recognizes(&[0; 16]));
    assert!(CurrentEvent::decode(&[0; 16]).is_none());
}

#[test]
fn captured_initializer_lengths_remain_exact() {
    let decoder = MorfiMarketsDecoder::default();
    for bytes in [
        include_bytes!("fixtures/conviction-config.bin").as_slice(),
        include_bytes!("fixtures/timed-config.bin").as_slice(),
    ] {
        let mut instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![],
            data: bytes.to_vec(),
        };
        assert!(decoder.decode_instruction(&instruction).is_some());
        instruction.data.push(0);
        assert!(decoder.decode_instruction(&instruction).is_none());
        instruction.data.truncate(bytes.len() - 1);
        assert!(decoder.decode_instruction(&instruction).is_none());
    }
}

#[test]
fn captured_boost_preserves_amount_identity_and_permissive_tail() {
    let mut instruction = Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![],
        data: bs58::decode(include_str!("fixtures/boost.base58").trim())
            .into_vec()
            .expect("captured base58"),
    };
    let decoder = MorfiMarketsDecoder::default();
    let original = decoder
        .decode_instruction(&instruction)
        .expect("captured CPI");
    let MorfiMarketsEvent::CurrentEvent(CurrentEvent::MarketBoostedEvent(event)) = &original else {
        panic!("current boost ABI");
    };
    assert_eq!(event.amount_quote, 1_000_000);
    assert_eq!(event.ts, 1780215538);
    assert_eq!(
        event.market.to_string(),
        "329Eipge74ewkm9E6dhRHPutiniwn7h41kwfr7txoTDt"
    );
    instruction.data.extend_from_slice(&[1, 2, 3]);
    assert_eq!(decoder.decode_instruction(&instruction), Some(original));
}

#[test]
fn custom_deployment_optional_schedule_uses_its_program_sentinel() {
    use carbon_morfi_markets_decoder::instructions::{CreateMarket, CreateTimedMarket};
    let program = Pubkey::new_from_array([7; 32]);
    let mut accounts: Vec<_> = (0..19)
        .map(|index| AccountMeta::new_readonly(Pubkey::new_from_array([index; 32]), false))
        .collect();
    accounts[16].pubkey = program;
    let arranged = CreateMarket::arrange_accounts_for_program(&accounts, program)
        .expect("complete creation prefix");
    assert_eq!(arranged.event_schedule, None);
    assert_eq!(
        CreateTimedMarket::arrange_accounts_for_program(&accounts, program)
            .expect("timed creation prefix")
            .event_schedule,
        None
    );
    accounts[16].pubkey = PROGRAM_ID;
    assert_eq!(
        CreateMarket::arrange_accounts_for_program(&accounts, program)
            .expect("default ID is not custom sentinel")
            .event_schedule,
        Some(PROGRAM_ID)
    );
    assert_eq!(
        CreateTimedMarket::arrange_accounts_for_program(&accounts, program)
            .expect("timed default ID is not custom sentinel")
            .event_schedule,
        Some(PROGRAM_ID)
    );
    accounts[16].pubkey = Pubkey::new_from_array([99; 32]);
    let arranged =
        CreateMarket::arrange_accounts_for_program(&accounts, program).expect("concrete schedule");
    assert_eq!(arranged.event_schedule, Some(accounts[16].pubkey));
    assert!(CreateMarket::arrange_accounts_for_program(&accounts[..18], program).is_none());
}
