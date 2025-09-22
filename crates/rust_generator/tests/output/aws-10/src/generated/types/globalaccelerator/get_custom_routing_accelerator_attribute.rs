#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCustomRoutingAcceleratorAttribute {
    #[builder(into)]
    #[serde(rename = "flowLogsEnabled")]
    pub r#flow_logs_enabled: bool,
    #[builder(into)]
    #[serde(rename = "flowLogsS3Bucket")]
    pub r#flow_logs_s_3_bucket: String,
    #[builder(into)]
    #[serde(rename = "flowLogsS3Prefix")]
    pub r#flow_logs_s_3_prefix: String,
}
