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

    let result = stack.get_string("/result")?;
    let result_with_provider = stack.get_string("/result_with_provider")?;
    let provider_prefix = stack.get_string("/provider_prefix")?;
    let provider_urn = stack.get_string("/provider_urn")?;

    assert_eq!(result, "SUFFIX");
    assert_eq!(result_with_provider, "MY_PREFIXSUFFIX");
    assert_eq!(provider_prefix, "MY_PREFIX");
    assert_eq!(
        provider_urn,
        "urn:pulumi:test::Pulumi-Gestalt-Examples-Test::pulumi:providers:test::test_provider"
    );

    Ok(())
}
