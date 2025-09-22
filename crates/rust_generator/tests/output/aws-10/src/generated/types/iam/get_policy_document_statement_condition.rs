#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPolicyDocumentStatementCondition {
    /// Name of the [IAM condition operator](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_condition_operators.html) to evaluate.
    #[builder(into)]
    #[serde(rename = "test")]
    pub r#test: String,
    /// Values to evaluate the condition against. If multiple values are provided, the condition matches if at least one of them applies. That is, AWS evaluates multiple values as though using an "OR" boolean operation.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
    /// Name of a [Context Variable](http://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements.html#AvailableKeys) to apply the condition to. Context variables may either be standard AWS variables starting with `aws:` or service-specific variables prefixed with the service name.
    #[builder(into)]
    #[serde(rename = "variable")]
    pub r#variable: String,
}
