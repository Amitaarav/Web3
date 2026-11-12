use super::address::Address;

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub addresses: Vec<Address>,
    pub birthdays: Vec<u32>,
}