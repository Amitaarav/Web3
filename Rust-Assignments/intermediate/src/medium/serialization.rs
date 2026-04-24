/*
  Problem 47: Serialization — Manual to_bytes / from_bytes

  Define a struct Record { id: u32, value: u16 }. Implement methods
  to_bytes(&self) -> Vec<u8> and from_bytes(data: &[u8]) -> Result<Self, String>
  using little-endian byte order. The serialized format should be
  [id: 4 bytes][value: 2 bytes] = 6 bytes total.

  Run the tests for this problem with:
    cargo test --test serialization_test
*/

#[derive(Debug, PartialEq)]
pub struct Record {
    pub id: u32,
    pub value: u16,
}

impl Record {
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        todo!()
    }
}
