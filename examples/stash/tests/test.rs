use anyhow::Result;
use pulumi_gestalt_examples_common::{export_stack, init_stack, select_stack, up_stack};

#[test]
#[cfg_attr(not(feature = "example_test"), ignore)]
fn test_integration() -> Result<()> {
    let github_token_env_vars = if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        vec![("GITHUB_TOKEN".to_string(), token)]
    } else {
        vec![]
    };

    init_stack("test", &github_token_env_vars)?;
    select_stack("test")?;
    up_stack(&github_token_env_vars)?;

    let stack = export_stack()?;

    let stash_input = stack.get_string("/stash_input")?;
    let stash_output = stack.get_string("/stash_output")?;
    let stash_id = stack.get_string("/stash_id")?;
    let stash_urn = stack.get_string("/stash_urn")?;

    assert_eq!(stash_input, "stash-value");
    assert_eq!(stash_output, "stash-value");
    assert!(!stash_id.is_empty());
    assert_eq!(
        stash_urn,
        "urn:pulumi:test::Pulumi-Gestalt-Examples-Stash::pulumi:index:Stash::example_stash"
    );

    Ok(())
}
