use base64::{prelude::BASE64_STANDARD, Engine};
use std::{borrow::Cow, path::Path};

#[macro_export]
macro_rules! non_zero_u32 {
    ( $val:literal ) => {{
        use ::std::num::NonZero;
        const VAL: NonZero<u32> = match NonZero::new($val) {
            Some(val) => val,
            None => panic!(concat!(stringify!($val), " is zero")),
        };
        VAL
    }};
}

pub use non_zero_u32;

pub fn file_name(path: &Path) -> Cow<str> {
    path.file_name()
        .map(|file| file.to_string_lossy())
        .unwrap_or_default()
}

/// Decodes a chat link to bytes.
pub fn chatcode_bytes(code: &str) -> Option<Vec<u8>> {
    code.strip_prefix("[&")
        .and_then(|text| text.strip_suffix(']'))
        .and_then(|text| BASE64_STANDARD.decode(text).ok())
}

/// Decodes a skill id from a chat link.
pub fn decode_skill(code: &str) -> Option<u32> {
    if let [0x06, b1, b2, b3, b4] = *chatcode_bytes(code)?.as_slice() {
        return Some(u32::from_le_bytes([b1, b2, b3, b4]));
    }
    None
}

/// Decodes a trait id from a chat link.
pub fn decode_trait(code: &str) -> Option<u32> {
    if let [0x07, b1, b2, b3, b4] = *chatcode_bytes(code)?.as_slice() {
        return Some(u32::from_le_bytes([b1, b2, b3, b4]));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chatcode_skill() {
        assert_eq!(decode_skill("[&BuQCAAA=]"), Some(740)); // might
        assert_eq!(decode_skill("[&BgAZAQA=]"), Some(71936)); // fire bullet

        assert_eq!(decode_skill("[&BgAZAQA="), None); // broken
        assert_eq!(decode_skill("[&B/IDAAA=]"), None); // trait
        assert_eq!(decode_skill("[&AQEAAAA=]"), None); // coin
        assert_eq!(decode_skill("[&AgH1WQAA]"), None); // item
        assert_eq!(decode_skill("[&BDgAAAA=]"), None); // poi
    }

    #[test]
    fn chatcode_trait() {
        assert_eq!(decode_trait("[&B/IDAAA=]"), Some(1010)); // opening strike
        assert_eq!(decode_trait("[&BwMJAAA=]"), Some(2307)); // eternal champion

        assert_eq!(decode_trait("[&B/IDAAA="), None); // broken
        assert_eq!(decode_trait("[&BuQCAAA=]"), None); // skill
        assert_eq!(decode_trait("[&AQEAAAA=]"), None); // coin
        assert_eq!(decode_trait("[&AgH1WQAA]"), None); // item
        assert_eq!(decode_trait("[&BDgAAAA=]"), None); // poi
    }
}
