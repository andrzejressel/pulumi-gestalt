#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleRetentionPeriod {
    /// The unit of time in which the retention period is measured. Currently, only DAYS is supported.
    #[builder(into)]
    #[serde(rename = "retentionPeriodUnit")]
    pub r#retention_period_unit: String,
    /// The period value for which the retention rule is to retain resources. The period is measured using the unit specified for RetentionPeriodUnit.
    #[builder(into)]
    #[serde(rename = "retentionPeriodValue")]
    pub r#retention_period_value: i32,
}
