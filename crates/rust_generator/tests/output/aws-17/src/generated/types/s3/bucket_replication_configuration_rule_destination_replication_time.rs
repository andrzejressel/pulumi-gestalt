#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketReplicationConfigurationRuleDestinationReplicationTime {
    /// Threshold within which objects are to be replicated. The only valid value is `15`.
    #[builder(into)]
    #[serde(rename = "minutes")]
    pub r#minutes: Option<i32>,
    /// The status of RTC. Either `Enabled` or `Disabled`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}
