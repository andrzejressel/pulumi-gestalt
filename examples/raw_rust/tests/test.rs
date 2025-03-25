use pulumi_gestalt_examples_common::{
    export_stack, export_stack_secret, init_stack, select_stack, up_stack,
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
    up_stack(&github_token_env_vars)?;

    let stack = export_stack()?;
    let secret_stack = export_stack_secret()?;

    let result = stack.get_string("/result")?;
    let double_length = stack.get_i64("/double_length")?;
    let static_string = stack.get_string("/static_string")?;
    let whoami_stdout = stack.get_string("/whoami_stdout")?;
    let combined = stack.get_array_as_string("/combined")?;

    let secret = stack.get_string("/secret")?;
    let secret_plaintext = secret_stack.get_string("/secret")?;

    let secret_namespace = stack.get_string("/secret_namespace")?;
    let secret_namespace_plaintext = secret_stack.get_string("/secret_namespace")?;

    assert_eq!(result.len(), 16);
    assert_eq!(double_length, 32);
    assert_eq!(static_string, "my_string");
    assert!(!whoami_stdout.is_empty());
    assert_eq!(combined, "[16,32,\"my_string\"]");
    assert_eq!(secret, "[secret]");
    assert_eq!(secret_plaintext, "secret_value");
    assert_eq!(secret_namespace, "[secret]");
    assert_eq!(secret_namespace_plaintext, "secret_value_namespace");

    Ok(())
}
