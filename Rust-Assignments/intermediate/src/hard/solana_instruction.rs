/*
  Problem 60: Solana-Style Instruction — Unpack

  Simulate unpacking a Solana instruction. Define an enum Instruction with variants
  Initialize, Mint { amount: u64 }, and Transfer { amount: u64 }.
  Write a function unpack(data: &[u8]) -> Result<Instruction, String>.
  Data format: [tag: 1 byte][data: remaining bytes LE].
  Tags: 0 = Initialize, 1 = Mint, 2 = Transfer.

  Run the tests for this problem with:
    cargo test --test solana_instruction_test
*/

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Initialize,
    Mint { amount: u64 },
    Transfer { amount: u64 },
}

pub fn unpack(data: &[u8]) -> Result<Instruction, String> {
    todo!()
}
