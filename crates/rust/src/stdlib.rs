use anyhow::{Context, Result};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

pub fn cwd() -> Result<String> {
    let path = std::env::current_dir().context("Failed to read current working directory")?;

    path.into_os_string().into_string().map_err(|_| {
        anyhow::anyhow!(
            "Current working directory path is not valid UTF-8 and cannot be represented as a string"
        )
    })
}

pub fn to_base64(input: impl AsRef<[u8]>) -> String {
    STANDARD.encode(input)
}

pub fn from_base64(input: impl AsRef<str>) -> Result<String> {
    let bytes = STANDARD
        .decode(input.as_ref())
        .context("Failed to decode base64 data")?;

    String::from_utf8(bytes).context(
        "Decoded base64 data is not valid UTF-8 string. \
         The data may be binary content that cannot be represented as a string.",
    )
}

#[cfg(test)]
mod tests {
    use super::{cwd, from_base64, to_base64};

    #[test]
    fn to_base64_encodes_known_text() {
        assert_eq!(to_base64("hello"), "aGVsbG8=");
    }

    #[test]
    fn cwd_matches_std_env_current_dir() {
        let expected = std::env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        let actual = cwd().unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn from_base64_decodes_known_text() {
        assert_eq!(from_base64("aGVsbG8=").unwrap(), "hello");
    }

    #[test]
    fn roundtrip_text_data() {
        let text = "Hello, World! 🌍";
        let encoded = to_base64(text);
        let decoded = from_base64(encoded).unwrap();

        assert_eq!(decoded, text);
    }

    #[test]
    fn from_base64_binary_data_returns_error() {
        // Binary data with invalid UTF-8 sequence
        let payload = vec![0x00, 0xff, 0x10, 0x41];
        let encoded = to_base64(&payload);
        let result = from_base64(encoded);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("not valid UTF-8"));
        assert!(error_msg.contains("binary content"));
    }

    #[test]
    fn from_base64_invalid_input_returns_error() {
        assert!(from_base64("%%%").is_err());
    }

    #[test]
    fn to_base64_accepts_multiple_input_types() {
        let string = "xyz".to_string();
        let from_string = to_base64(string);
        let from_str = to_base64("abc");
        let from_bytes = to_base64(vec![97_u8, 98_u8, 99_u8]);

        assert_eq!(from_string, "eHl6");
        assert_eq!(from_str, "YWJj");
        assert_eq!(from_str, from_bytes);
    }
}
