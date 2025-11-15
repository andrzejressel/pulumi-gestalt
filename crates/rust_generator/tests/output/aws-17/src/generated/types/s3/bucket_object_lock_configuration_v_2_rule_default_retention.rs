#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketObjectLockConfigurationV2RuleDefaultRetention {
    /// Number of days that you want to specify for the default retention period.
    #[builder(into)]
    #[serde(rename = "days")]
    pub r#days: Option<i32>,
    /// Default Object Lock retention mode you want to apply to new objects placed in the specified bucket. Valid values: `COMPLIANCE`, `GOVERNANCE`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// Number of years that you want to specify for the default retention period.
    #[builder(into)]
    #[serde(rename = "years")]
    pub r#years: Option<i32>,
}
