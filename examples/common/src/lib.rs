use anyhow::{Context, anyhow, bail};
use serde_json::Value;
use std::process::Command;
use std::str;

pub struct Stack {
    value: Value,
}

impl Stack {
    pub fn get_string(&self, pointer: &str) -> Result<&str, anyhow::Error> {
        self.value
            .pointer(pointer)
            .ok_or(anyhow!("Cannot find [{}] in stack export", pointer))?
            .as_str()
            .ok_or(anyhow!("[{}] is not a string", pointer))
    }

    pub fn get_i64(&self, pointer: &str) -> Result<i64, anyhow::Error> {
        self.value
            .pointer(pointer)
            .ok_or(anyhow!("Cannot find [{}] in stack export", pointer))?
            .as_i64()
            .ok_or(anyhow!("[{}] is not an i64", pointer))
    }

    pub fn get_array_as_string(&self, pointer: &str) -> Result<String, anyhow::Error> {
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
) -> Result<(), anyhow::Error> {
    Command::new("pulumi")
        .args(["stack", "init", stack_name])
        .env("PULUMI_CONFIG_PASSPHRASE", " ")
        .envs(github_token_env_vars.to_owned())
        .current_dir(".")
        .output()
        .context("Failed to execute pulumi stack init command")?;
    Ok(())
}

pub fn select_stack(stack_name: &str) -> Result<(), anyhow::Error> {
    let output = Command::new("pulumi")
        .args(["stack", "select", stack_name])
        .current_dir(".")
        .output()
        .context("Failed to execute pulumi stack select command")?;

    if !output.status.success() {
        bail!(
            "Pulumi stack select failed with exit code: {}\nStdout: {}\nStderr: {}",
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

pub fn up_stack(github_token_env_vars: &[(String, String)]) -> Result<(), anyhow::Error> {
    let output = Command::new("pulumi")
        .args(["up", "-y"])
        .current_dir(".")
        .env("PULUMI_CONFIG_PASSPHRASE", " ")
        .envs(github_token_env_vars.to_owned())
        .output()
        .context("Failed to execute pulumi up command")?;

    if !output.status.success() {
        bail!(
            "Pulumi up failed with exit code: {}\nStdout: {}\nStderr: {}",
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

pub fn export_stack() -> Result<Stack, anyhow::Error> {
    let output = Command::new("pulumi")
        .args(["stack", "output", "--json"])
        .current_dir(".")
        .env("PULUMI_CONFIG_PASSPHRASE", " ")
        .output()
        .context("Failed to execute pulumi stack output command")?;

    if !output.status.success() {
        bail!(
            "Pulumi stack output failed with exit code: {}\nStdout: {}\nStderr: {}",
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout_str = str::from_utf8(&output.stdout)
        .context("Failed to convert pulumi output to UTF-8 string")?;
    let stack: Value =
        serde_json::from_str(stdout_str).context("Failed to parse pulumi output as JSON")?;
    Ok(Stack { value: stack })
}

pub fn export_stack_secret() -> Result<Stack, anyhow::Error> {
    let output = Command::new("pulumi")
        .args(["stack", "output", "--json", "--show-secrets"])
        .current_dir(".")
        .env("PULUMI_CONFIG_PASSPHRASE", " ")
        .output()
        .context("Failed to execute pulumi stack output --show-secrets command")?;

    if !output.status.success() {
        bail!(
            "Pulumi stack output --show-secrets failed with exit code: {}\nStdout: {}\nStderr: {}",
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout_str = str::from_utf8(&output.stdout)
        .context("Failed to convert pulumi output to UTF-8 string")?;
    let stack: Value =
        serde_json::from_str(stdout_str).context("Failed to parse pulumi output as JSON")?;
    Ok(Stack { value: stack })
}
