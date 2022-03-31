/// Test-relevant connector capabilities.
#[enumflags2::bitflags]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Capabilities {
    ScalarLists = 1 << 1,
    Enums = 1 << 2,
    Json = 1 << 3,
    CreateDatabase = 1 << 4,
    Ltree = 1 << 5,
}
