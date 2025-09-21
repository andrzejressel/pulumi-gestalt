use anyhow::{anyhow, Result};
use assert_cmd::prelude::*;
use serde_json::Value;
use std::process::Command;
use std::str;

pub struct Stack {
    value: Value,
}

impl Stack {
    pub fn get_string(&self, pointer: &str) -> Result<&str> {
        self.value
            .pointer(pointer)
            .ok_or(anyhow!("Cannot find [{}] in stack export", pointer))?
            .as_str()
            .ok_or(anyhow!("[{}] is not a string", pointer))
    }

    pub fn get_i64(&self, pointer: &str) -> Result<i64> {
        self.value
            .pointer(pointer)
            .ok_or(anyhow!("Cannot find [{}] in stack export", pointer))?
            .as_i64()
            .ok_or(anyhow!("[{}] is not an i64", pointer))
    }

    pub fn get_array_as_string(&self, pointer: &str) -> Result<String> {
        let array = self
            .value
            .pointer(pointer)
            .ok_or(anyhow!("Cannot find [{}] in stack export", pointer))?
            .as_array()
            .ok_or(anyhow!("[{}] is not an array", pointer))?;

        serde_json::to_string(&array).map_err(anyhow::Error::new)
    }
}

pub fn get_string(pointer: &str) -> &str {
    pointer
}

pub fn init_stack(
    stack_name: &str,
    github_token_env_vars: &[(String, String)],
) -> Result<()> {
    Command::new("pulumi")
        .args(["stack", "init", stack_name])
        .env("PULUMI_CONFIG_PASSPHRASE", " ")
        .envs(github_token_env_vars.to_owned())
        .current_dir(".")
        .output()?;
    Ok(())
}

pub fn select_stack(stack_name: &str) -> Result<()> {
    Command::new("pulumi")
        .args(["stack", "select", stack_name])
        .current_dir(".")
        .assert()
        .success();
    Ok(())
}

pub fn up_stack(github_token_env_vars: &[(String, String)]) -> Result<()> {
    Command::new("pulumi")
        .args(["up", "-y"])
        .current_dir(".")
        .env("PULUMI_CONFIG_PASSPHRASE", " ")
        .envs(github_token_env_vars.to_owned())
        .assert()
        .try_success()
        .map(|_| ())
        .map_err(anyhow::Error::from)
}

pub fn export_stack() -> Result<Stack> {
    let binding = Command::new("pulumi")
        .args(["stack", "output", "--json"])
        .current_dir(".")
        .env("PULUMI_CONFIG_PASSPHRASE", " ")
        .assert()
        .success();
    let stack = &binding.get_output().stdout;
    let stack: Value = serde_json::from_str(str::from_utf8(stack)?)?;
    Ok(Stack { value: stack })
}

pub fn export_stack_secret() -> Result<Stack> {
    let binding = Command::new("pulumi")
        .args(["stack", "output", "--json", "--show-secrets"])
        .current_dir(".")
        .env("PULUMI_CONFIG_PASSPHRASE", " ")
        .assert()
        .success();
    let stack = &binding.get_output().stdout;
    let stack: Value = serde_json::from_str(str::from_utf8(stack)?)?;
    Ok(Stack { value: stack })
}
