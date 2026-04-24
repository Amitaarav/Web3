#[path = "../src/easy/coin_value.rs"]
mod coin_value;
use coin_value::{coin_value, Coin};


#[test]
fn test_penny() {
    assert_eq!(coin_value(Coin::Penny), 1);
}

#[test]
fn test_nickel() {
    assert_eq!(coin_value(Coin::Nickel), 5);
}

#[test]
fn test_dime() {
    assert_eq!(coin_value(Coin::Dime), 10);
}

#[test]
fn test_quarter() {
    assert_eq!(coin_value(Coin::Quarter), 25);
}

#[test]
fn test_five_pennies_eq_nickel() {
    assert_eq!(5 * coin_value(Coin::Penny), coin_value(Coin::Nickel));
}

#[test]
fn test_two_nickels_eq_dime() {
    assert_eq!(2 * coin_value(Coin::Nickel), coin_value(Coin::Dime));
}

#[test]
fn test_two_dimes_plus_nickel_eq_quarter() {
    assert_eq!(2 * coin_value(Coin::Dime) + coin_value(Coin::Nickel), coin_value(Coin::Quarter));
}

#[test]
fn test_penny_is_minimum() {
    assert!(coin_value(Coin::Penny) < coin_value(Coin::Nickel));
}

#[test]
fn test_quarter_is_maximum() {
    assert!(coin_value(Coin::Quarter) > coin_value(Coin::Dime));
}

#[test]
fn test_dime_gt_nickel() {
    assert!(coin_value(Coin::Dime) > coin_value(Coin::Nickel));
}
