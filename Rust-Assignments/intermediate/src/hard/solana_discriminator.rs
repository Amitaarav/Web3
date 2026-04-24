/*
  Problem 53: Solana-Style Account Data — Discriminator

  Simulate a Solana-style account data structure. Define a trait AccountData
  with methods discriminator() -> [u8; 8], serialize(&self) -> Vec<u8> and
  deserialize(data) -> Result<Self, String>. Implement it for TokenAccount
  { owner: [u8; 32], amount: u64 }. The serialized format is
  [discriminator: 8 bytes][owner: 32 bytes][amount: 8 bytes LE].

  Run the tests for this problem with:
    cargo test --test solana_discriminator_test
*/

pub trait AccountData: Sized {
    fn discriminator() -> [u8; 8];
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Result<Self, String>;
}

#[derive(Debug, PartialEq)]
pub struct TokenAccount {
    pub owner: [u8; 32],
    pub amount: u64,
}

impl AccountData for TokenAccount {
    fn discriminator() -> [u8; 8] {
        // Use a fixed discriminator: "TOKENACC"
        [0x54, 0x4f, 0x4b, 0x45, 0x4e, 0x41, 0x43, 0x43]
    }

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }

    fn deserialize(data: &[u8]) -> Result<Self, String> {
        todo!()
    }
}
