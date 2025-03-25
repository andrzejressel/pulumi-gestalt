use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::env::VarError;

const CONFIG_ENV_KEY: &str = "PULUMI_CONFIG";
const CONFIG_SECRET_KEYS_ENV_KEY: &str = "PULUMI_CONFIG_SECRET_KEYS";

pub struct Config {
    config_map: HashMap<String, String>,
    secret: HashSet<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum RawConfigValue {
    PlainText(String),
    Secret(String),
}

impl Config {
    pub fn from_env_vars() -> Result<Self> {
        let config_map = match std::env::var(CONFIG_ENV_KEY) {
            Ok(v) => v,
            Err(VarError::NotPresent) => "{}".to_string(),
            Err(VarError::NotUnicode(v)) => {
                anyhow::bail!(
                    "{} env var with value [{:?}] is not a valid UTF-8 string",
                    CONFIG_ENV_KEY,
                    v
                )
            }
        };
        let config_map: HashMap<String, String> = serde_json::from_str(&config_map)
            .with_context(|| format!("{} is not a valid configmap", CONFIG_ENV_KEY))?;

        let secret = match std::env::var(CONFIG_SECRET_KEYS_ENV_KEY) {
            Ok(v) => v,
            Err(VarError::NotPresent) => "[]".to_string(),
            Err(VarError::NotUnicode(v)) => {
                anyhow::bail!(
                    "{} env var with value [{:?}] is not a valid UTF-8 string",
                    CONFIG_SECRET_KEYS_ENV_KEY,
                    v
                )
            }
        };
        let secret: HashSet<String> = serde_json::from_str(&secret).with_context(|| {
            format!("{} is not a valid secret list", CONFIG_SECRET_KEYS_ENV_KEY)
        })?;

        Ok(Self::new(config_map, secret))
    }

    pub(crate) fn new(config_map: HashMap<String, String>, secret: HashSet<String>) -> Self {
        Self { config_map, secret }
    }

    pub(crate) fn get(&self, name: &str, key: &str) -> Option<RawConfigValue> {
        let full_key = Self::full_key(name, key);
        match self.config_map.get(&full_key) {
            Some(value) => {
                if self.secret.contains(&full_key) {
                    Some(RawConfigValue::Secret(value.clone()))
                } else {
                    Some(RawConfigValue::PlainText(value.clone()))
                }
            }
            None => None,
        }
    }

    fn full_key(project_name: &str, key: &str) -> String {
        format!("{}:{}", project_name, key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_none_when_value_does_not_exist() {
        let config = Config {
            config_map: HashMap::from([("name:key".to_string(), "value".to_string())]),
            secret: HashSet::new(),
        };
        let different_key = config.get("name", "key2");
        let different_name = config.get("name2", "key");
        let different_name_and_key = config.get("name2", "key2");
        assert_eq!(different_key, None);
        assert_eq!(different_name, None);
        assert_eq!(different_name_and_key, None);
    }

    #[test]
    fn should_return_plain_text_value() {
        let config = Config {
            config_map: HashMap::from([("name:key".to_string(), "value".to_string())]),
            secret: HashSet::new(),
        };
        let value = config.get("name", "key");
        assert_eq!(value, Some(RawConfigValue::PlainText("value".to_string())));
    }

    #[test]
    fn should_return_secret_value() {
        let config = Config {
            config_map: HashMap::from([("name:key".to_string(), "value".to_string())]),
            secret: HashSet::from(["name:key".to_string()]),
        };
        let value = config.get("name", "key");
        assert_eq!(value, Some(RawConfigValue::Secret("value".to_string())));
    }
}
