use {
    instruction_data::{Buffer, SetValueContextArgs},
    litesvm::LiteSVM,
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        pubkey,
        signature::Keypair,
        signer::Signer,
        system_program,
        transaction::Transaction,
    },
    std::path::PathBuf,
};

fn read_program() -> Vec<u8> {
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push("../../target/deploy/instruction_data.so");

    std::fs::read(so_path).unwrap()
}

#[test]
fn integration_test() {
    let mut svm = LiteSVM::new();
    let admin_kp = Keypair::new();
    let admin_pk = admin_kp.pubkey();

    svm.airdrop(&admin_pk, 10 * LAMPORTS_PER_SOL).unwrap();

    let program_id = pubkey::Pubkey::new_from_array(instruction_data::id());
    let program_bytes = read_program();

    svm.add_program(program_id, &program_bytes);

    let buffer_kp = Keypair::new();
    let buffer_pk = buffer_kp.pubkey();
    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(admin_pk, true),
            AccountMeta::new(buffer_pk, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: vec![0],
    };
    let hash = svm.latest_blockhash();
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp, &buffer_kp], hash);
    svm.send_transaction(tx).unwrap();

    let raw_account = svm.get_account(&buffer_pk).unwrap();
    let buffer_account: &Buffer = bytemuck::try_from_bytes(raw_account.data.as_slice()).unwrap();
    assert!(buffer_account.value == 0);

    let ix_args = SetValueContextArgs {
        value: 10,
        other_value: 5,
    };
    let ix = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(buffer_pk, false)],
        data: vec![1]
            .iter()
            .chain(bytemuck::bytes_of(&ix_args).iter())
            .cloned()
            .collect(),
    };
    let hash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&admin_pk), &[&admin_kp], hash);
    svm.send_transaction(tx).unwrap();

    let raw_account = svm.get_account(&buffer_pk).unwrap();
    let buffer_account: &Buffer = bytemuck::try_from_bytes(raw_account.data.as_slice()).unwrap();
    assert!(buffer_account.value == ix_args.value);
}
