#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TunnelConfigConfigIngressRuleOriginRequestIpRule {
    /// Whether to allow the IP prefix.
    #[builder(into)]
    #[serde(rename = "allow")]
    pub r#allow: Option<bool>,
    /// Ports to use within the IP rule.
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Option<Vec<i32>>,
    /// IP rule prefix.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
}
