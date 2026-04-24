#[path = "../src/hard/solana_discriminator.rs"]
mod solana_discriminator;
use solana_discriminator::{AccountData, TokenAccount};


#[test]
fn test_serialize_length() {
    let acct = TokenAccount { owner: [1u8; 32], amount: 1000 };
    assert_eq!(acct.serialize().len(), 48); // 8 (disc) + 32 (owner) + 8 (amount)
}

#[test]
fn test_roundtrip() {
    let acct = TokenAccount { owner: [0xAB; 32], amount: 999999 };
    let bytes = acct.serialize();
    let recovered = TokenAccount::deserialize(&bytes).unwrap();
    assert_eq!(acct, recovered);
}

#[test]
fn test_invalid_discriminator() {
    let mut bytes = vec![0u8; 48];
    assert!(TokenAccount::deserialize(&bytes).is_err());
}

#[test]
fn test_too_short() {
    assert!(TokenAccount::deserialize(&[0u8; 10]).is_err());
}

#[test]
fn test_zero_amount() {
    let acct = TokenAccount { owner: [0; 32], amount: 0 };
    let bytes = acct.serialize();
    let recovered = TokenAccount::deserialize(&bytes).unwrap();
    assert_eq!(recovered.amount, 0);
}

#[test]
fn test_owner_preserved() {
    let owner = [0x42u8; 32];
    let acct = TokenAccount { owner, amount: 1 };
    let bytes = acct.serialize();
    let recovered = TokenAccount::deserialize(&bytes).unwrap();
    assert_eq!(recovered.owner, owner);
}

#[test]
fn test_amount_preserved() {
    let acct = TokenAccount { owner: [1u8; 32], amount: 42 };
    let bytes = acct.serialize();
    let recovered = TokenAccount::deserialize(&bytes).unwrap();
    assert_eq!(recovered.amount, 42);
}

#[test]
fn test_max_amount() {
    let acct = TokenAccount { owner: [0u8; 32], amount: u64::MAX };
    let bytes = acct.serialize();
    let recovered = TokenAccount::deserialize(&bytes).unwrap();
    assert_eq!(recovered.amount, u64::MAX);
}

#[test]
fn test_empty_bytes_err() {
    assert!(TokenAccount::deserialize(&[]).is_err());
}

#[test]
fn test_serialize_is_deterministic() {
    let acct = TokenAccount { owner: [5u8; 32], amount: 100 };
    let b1 = acct.serialize();
    let b2 = acct.serialize();
    assert_eq!(b1, b2);
}
