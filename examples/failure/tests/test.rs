use anyhow::bail;
use pulumi_gestalt_examples_common::{
    init_stack, select_stack, up_stack,
};

#[test]
#[cfg_attr(not(feature = "example_test"), ignore)]
fn test_integration() -> Result<(), anyhow::Error> {
    let github_token_env_vars = if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        vec![("GITHUB_TOKEN".to_string(), token)]
    } else {
        vec![]
    };

    init_stack("test", &github_token_env_vars)?;
    select_stack("test")?;
    match up_stack(&github_token_env_vars) {
        Ok(_) => {
            bail!("Upstack should have failed");
        }
        Err(e) => {
            let error_message = format!("{}", e);
            assert!(error_message.contains("Important error message"), "Error message did not contain expected text. Actual error: {}", error_message);
        }
    }
    Ok(())
}
