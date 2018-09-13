//! external transaction-generation helpers.
//!


/// trim leading zero bytes from slice.
///
/// required for rlp encoding of integer-like values.
///
#[inline]
pub fn trim<'a>(bytes: &'a [u8]) -> &'a [u8] {
    let start = bytes.iter().position(|b| *b != 0).unwrap_or(bytes.len());
    &bytes[start..]
}
