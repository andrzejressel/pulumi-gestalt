use anyhow::{Context, Result};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

pub fn to_base64(input: impl AsRef<[u8]>) -> String {
    STANDARD.encode(input)
}

pub fn from_base64(input: impl AsRef<str>) -> Result<Vec<u8>> {
    STANDARD
        .decode(input.as_ref())
        .context("Failed to decode base64 data")
}

pub use from_base64 as fromBase64;
pub use to_base64 as toBase64;

#[cfg(test)]
mod tests {
    use super::{fromBase64, toBase64};

    #[test]
    fn to_base64_encodes_known_text() {
        assert_eq!(toBase64("hello"), "aGVsbG8=");
    }

    #[test]
    fn from_base64_decodes_known_text() {
        assert_eq!(fromBase64("aGVsbG8=").unwrap(), b"hello");
    }

    #[test]
    fn roundtrip_binary_data() {
        let payload = vec![0x00, 0xff, 0x10, 0x41];
        let encoded = toBase64(&payload);
        let decoded = fromBase64(encoded).unwrap();

        assert_eq!(decoded, payload);
    }

    #[test]
    fn from_base64_invalid_input_returns_error() {
        assert!(fromBase64("%%%").is_err());
    }

    #[test]
    fn to_base64_accepts_multiple_input_types() {
        let string = "xyz".to_string();
        let from_string= toBase64(string);
        let from_str = toBase64("abc");
        let from_bytes = toBase64(vec![97_u8, 98_u8, 99_u8]);

        assert_eq!(from_string, "eHl6");
        assert_eq!(from_str, "YWJj");
        assert_eq!(from_str, from_bytes);
    }
}
