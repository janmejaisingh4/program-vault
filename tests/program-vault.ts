use {
  anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
  litesvm::LiteSVM,
  solana_keypair::Keypair,
  solana_message::{Message, VersionedMessage},
  solana_signer::Signer,
  solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_program_vault() {
  let program_id = "298AoxcTPxtnyx7a5k12wPEpvVHmhSKoAbEv81jwTWc3".parse().unwrap();
  let instruction = Instruction {
    program_id,
    accounts: vec![],
    data: vec![],
  };

  let message = VersionedMessage::Legacy(Message::new(&[instruction], None));
  let transaction = VersionedTransaction::try_new(message, &[&Keypair::new()]).unwrap();

  let mut litesvm = LiteSVM::new();
  litesvm.execute_transaction(&transaction).unwrap();
}