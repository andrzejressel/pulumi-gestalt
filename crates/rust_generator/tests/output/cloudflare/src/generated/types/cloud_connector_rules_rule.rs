#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CloudConnectorRulesRule {
    /// Brief summary of the cloud connector rule and its intended use.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Whether the headers rule is active.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Criteria for an HTTP request to trigger the cloud connector rule. Uses the Firewall Rules expression language based on Wireshark display filters.
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: String,
    /// Cloud Connector Rule Parameters
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<super::types::CloudConnectorRulesRuleParameters>>,
    /// Type of provider. Available values: `aws_s3`, `cloudflare_r2`, `azure_storage`, `gcp_storage`
    #[builder(into)]
    #[serde(rename = "provider")]
    pub r#provider: String,
}
