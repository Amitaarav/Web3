#[path = "../src/hard/solana_instruction.rs"]
mod solana_instruction;
use solana_instruction::{unpack, Instruction};


#[test]
fn test_init() {
    assert_eq!(unpack(&[0]), Ok(Instruction::Initialize));
}

#[test]
fn test_mint() {
    let mut data = vec![1];
    data.extend_from_slice(&1000u64.to_le_bytes());
    assert_eq!(unpack(&data), Ok(Instruction::Mint { amount: 1000 }));
}

#[test]
fn test_transfer() {
    let mut data = vec![2];
    data.extend_from_slice(&50u64.to_le_bytes());
    assert_eq!(unpack(&data), Ok(Instruction::Transfer { amount: 50 }));
}

#[test]
fn test_invalid_tag() {
    assert!(unpack(&[3]).is_err());
}

#[test]
fn test_too_short() {
    assert!(unpack(&[1, 0, 0, 0]).is_err()); // Mint needs 1 + 8 bytes
}

#[test]
fn test_empty() {
    assert!(unpack(&[]).is_err());
}

#[test]
fn test_mint_zero_amount() {
    let mut data = vec![1];
    data.extend_from_slice(&0u64.to_le_bytes());
    assert_eq!(unpack(&data), Ok(Instruction::Mint { amount: 0 }));
}

#[test]
fn test_transfer_zero_amount() {
    let mut data = vec![2];
    data.extend_from_slice(&0u64.to_le_bytes());
    assert_eq!(unpack(&data), Ok(Instruction::Transfer { amount: 0 }));
}

#[test]
fn test_tag_255_invalid() {
    assert!(unpack(&[255]).is_err());
}

#[test]
fn test_transfer_too_short() {
    assert!(unpack(&[2, 0, 0, 0]).is_err());
}
