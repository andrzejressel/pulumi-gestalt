use anyhow::{Context, Result, anyhow, bail};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

pub fn cwd() -> Result<String> {
    let path = std::env::current_dir().context("Failed to read current working directory")?;

    path.into_os_string().into_string().map_err(|e| {
        anyhow!(
            "Current working directory path is not valid UTF-8 and cannot be represented as a string: [{:?}]",
            e
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

pub fn element<T: Clone, I: TryInto<i64>>(list: impl AsRef<[T]>, index: I) -> Result<T> {
    let index = index
        .try_into()
        .map_err(|_| anyhow!("Failed to convert list index to i64"))?;
    if index < 0 {
        bail!("List index cannot be negative: {index}");
    }
    let index = usize::try_from(index).context("Failed to convert list index to usize")?;
    let list_ref = list.as_ref();
    list_ref.get(index).cloned().ok_or_else(|| {
        anyhow!(
            "List index {index} is out of bounds for length {}",
            list_ref.len()
        )
    })
}

pub fn join<T: AsRef<str>>(separator: impl AsRef<str>, list: impl AsRef<[T]>) -> String {
    list.as_ref()
        .iter()
        .map(AsRef::as_ref)
        .collect::<Vec<_>>()
        .join(separator.as_ref())
}

pub fn length<T>(list: impl AsRef<[T]>) -> i64 {
    i64::try_from(list.as_ref().len()).expect("List length exceeds i64::MAX")
}

pub fn split(separator: impl AsRef<str>, text: impl AsRef<str>) -> Vec<String> {
    text.as_ref()
        .split(separator.as_ref())
        .map(ToOwned::to_owned)
        .collect()
}

pub fn single_or_none<T: Clone>(list: impl AsRef<[T]>) -> Result<Option<T>> {
    let list_ref = list.as_ref();
    if list_ref.is_empty() {
        return Ok(None);
    }
    if list_ref.len() != 1 {
        bail!(
            "singleOrNone expected input list to have at most one element, got {}",
            list_ref.len()
        );
    }
    Ok(Some(list_ref[0].clone()))
}

#[cfg(test)]
mod tests {
    use super::{
        cwd, element, from_base64, join, length, single_or_none, split, to_base64,
    };

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

    #[test]
    fn element_returns_item_for_valid_index() {
        let values = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(element(&values, 1).unwrap(), "b");
    }

    #[test]
    fn element_returns_error_for_negative_index() {
        let values = vec!["a", "b"];
        let error = element(&values, -1).unwrap_err().to_string();
        assert!(error.contains("cannot be negative"));
    }

    #[test]
    fn element_returns_error_for_out_of_bounds_index() {
        let values = vec!["a", "b"];
        let error = element(&values, 2).unwrap_err().to_string();
        assert!(error.contains("out of bounds"));
    }

    #[test]
    fn join_joins_values_with_separator() {
        let values = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(join("|", &values), "a|b|c");
    }

    #[test]
    fn length_returns_i64_length() {
        let values = vec!["x", "y", "z"];
        assert_eq!(length(&values), 3_i64);
    }

    #[test]
    fn split_returns_segments() {
        assert_eq!(split("-", "a-b-c"), vec!["a", "b", "c"]);
    }

    #[test]
    fn single_or_none_returns_none_for_empty_list() {
        let values: Vec<String> = vec![];
        assert_eq!(single_or_none(&values).unwrap(), None);
    }

    #[test]
    fn single_or_none_returns_some_for_single_item_list() {
        let values = vec!["only".to_string()];
        assert_eq!(single_or_none(&values).unwrap(), Some("only".to_string()));
    }

    #[test]
    fn single_or_none_returns_error_for_multi_item_list() {
        let values = vec!["a", "b"];
        let error = single_or_none(&values).unwrap_err().to_string();
        assert!(error.contains("at most one element"));
    }
}
