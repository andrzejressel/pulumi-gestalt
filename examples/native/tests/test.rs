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
    let stack_secret = export_stack_secret()?;

    let result = stack.get_string("/result")?;
    let transformed_result = stack.get_string("/transformed_result")?;
    let number = stack.get_i64("/number")?;
    let combined_string = stack.get_string("/combined_string")?;
    let combined_2_string = stack.get_string("/combined_2_string")?;
    let keepers = stack.get_string("/keepers")?;
    let result_2 = stack.get_string("/result_2")?;
    let secret = stack.get_string("/secret_config")?;
    let secret_uncovered = stack_secret.get_string("/secret_config")?;

    assert_eq!(result.len(), 36);
    assert_eq!(transformed_result, format!("Result: {}", result));
    assert_eq!(number, 0);
    assert_eq!(combined_string, "Values: (1, \"abc\")");
    assert_eq!(combined_2_string, "Values: (1, \"abc\")");
    assert_eq!(keepers, "Keepers: None");
    assert_eq!(result_2.len(), 13);
    assert_eq!(secret, "[secret]");
    assert_eq!(secret_uncovered, "secret_value");

    Ok(())
}
