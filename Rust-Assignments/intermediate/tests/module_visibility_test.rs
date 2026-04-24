#[path = "../src/easy/module_visibility.rs"]
mod module_visibility;
use module_visibility::Account;


#[test]
fn test_new_account() {
    let acc = Account::new(100.0);
    assert!((acc.balance() - 100.0).abs() < f64::EPSILON);
}

#[test]
fn test_deposit() {
    let mut acc = Account::new(50.0);
    acc.deposit(25.0);
    assert!((acc.balance() - 75.0).abs() < f64::EPSILON);
}

#[test]
fn test_withdraw_ok() {
    let mut acc = Account::new(100.0);
    assert!(acc.withdraw(50.0).is_ok());
    assert!((acc.balance() - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_withdraw_insufficient() {
    let mut acc = Account::new(10.0);
    assert!(acc.withdraw(20.0).is_err());
}

#[test]
fn test_withdraw_exact_balance() {
    let mut acc = Account::new(100.0);
    assert!(acc.withdraw(100.0).is_ok());
    assert!((acc.balance() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_deposit_zero() {
    let mut acc = Account::new(50.0);
    acc.deposit(0.0);
    assert!((acc.balance() - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_multiple_deposits() {
    let mut acc = Account::new(0.0);
    acc.deposit(10.0);
    acc.deposit(20.0);
    acc.deposit(30.0);
    assert!((acc.balance() - 60.0).abs() < f64::EPSILON);
}

#[test]
fn test_withdraw_then_deposit() {
    let mut acc = Account::new(100.0);
    acc.withdraw(50.0).unwrap();
    acc.deposit(25.0);
    assert!((acc.balance() - 75.0).abs() < f64::EPSILON);
}

#[test]
fn test_initial_balance_zero() {
    let acc = Account::new(0.0);
    assert!((acc.balance() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_withdraw_one_cent() {
    let mut acc = Account::new(1.0);
    assert!(acc.withdraw(1.0).is_ok());
}
