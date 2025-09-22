#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ContinuousDeploymentPolicyTrafficConfigSingleHeaderConfig {
    /// Request header name to send to the staging distribution. The header must contain the prefix `aws-cf-cd-`.
    #[builder(into)]
    #[serde(rename = "header")]
    pub r#header: String,
    /// Request header value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
