#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSecurityGroupRulesFilter {
    /// Name of the field to filter by, as defined by
    /// [the underlying AWS API](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSecurityGroupRules.html).
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Set of values that are accepted for the given field.
    /// Security group rule IDs will be selected if any one of the given values match.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}
