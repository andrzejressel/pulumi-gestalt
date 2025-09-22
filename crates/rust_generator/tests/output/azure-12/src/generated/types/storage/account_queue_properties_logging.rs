#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountQueuePropertiesLogging {
    /// Indicates whether all delete requests should be logged.
    #[builder(into)]
    #[serde(rename = "delete")]
    pub r#delete: bool,
    /// Indicates whether all read requests should be logged.
    #[builder(into)]
    #[serde(rename = "read")]
    pub r#read: bool,
    /// Specifies the number of days that logs will be retained.
    #[builder(into)]
    #[serde(rename = "retentionPolicyDays")]
    pub r#retention_policy_days: Option<i32>,
    /// The version of storage analytics to configure.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
    /// Indicates whether all write requests should be logged.
    #[builder(into)]
    #[serde(rename = "write")]
    pub r#write: bool,
}
