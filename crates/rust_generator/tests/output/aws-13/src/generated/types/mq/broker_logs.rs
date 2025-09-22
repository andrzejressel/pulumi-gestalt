#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BrokerLogs {
    /// Enables audit logging. Auditing is only possible for `engine_type` of `ActiveMQ`. User management action made using JMX or the ActiveMQ Web Console is logged. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "audit")]
    pub r#audit: Option<bool>,
    /// Enables general logging via CloudWatch. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "general")]
    pub r#general: Option<bool>,
}
