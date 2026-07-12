use serde::{Deserialize, Serialize};
use std::fmt;

/// A globally-unique, sortable identifier for any stored object
/// (message, mailbox, account, ...).
///
/// Internally this is a 64-bit value so it is cheap to copy, index, and
/// store, but it's wrapped in a newtype so an `Id` can never be silently
/// mixed up with an unrelated `u64` (account counter, byte length, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Id(u64);

impl Id {
    pub fn new(value: u64) -> Self {
        Id(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Base32 (Crockford-style) is used for the external representation
        // so ids are URL-safe and case-insensitive when users type them.
        write!(f, "{}", crate::id::to_base32(self.0))
    }
}

impl std::str::FromStr for Id {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_base32(s)
            .map(Id)
            .ok_or_else(|| crate::Error::InvalidId(s.to_string()))
    }
}

const ALPHABET: &[u8] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

fn to_base32(mut value: u64) -> String {
    if value == 0 {
        return "0".to_string();
    }
    let mut out = Vec::new();
    while value > 0 {
        out.push(ALPHABET[(value % 32) as usize]);
        value /= 32;
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}

fn from_base32(s: &str) -> Option<u64> {
    let mut value: u64 = 0;
    for c in s.to_ascii_uppercase().bytes() {
        let digit = ALPHABET.iter().position(|&b| b == c)? as u64;
        value = value.checked_mul(32)?.checked_add(digit)?;
    }
    Some(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn roundtrip() {
        for raw in [0u64, 1, 31, 32, 12345, u64::MAX] {
            let id = Id::new(raw);
            let text = id.to_string();
            let parsed = Id::from_str(&text).unwrap();
            assert_eq!(id, parsed);
        }
    }

    #[test]
    fn rejects_invalid_chars() {
        assert!(Id::from_str("not-an-id!").is_err());
    }
}
