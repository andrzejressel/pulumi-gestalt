#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResourceSetResource {
    #[builder(into)]
    #[serde(rename = "componentId")]
    pub r#component_id: Option<String>,
    /// Component for DNS/Routing Control Readiness Checks.
    #[builder(into)]
    #[serde(rename = "dnsTargetResource")]
    pub r#dns_target_resource: Box<Option<super::super::types::route53recoveryreadiness::ResourceSetResourceDnsTargetResource>>,
    /// Recovery group ARN or cell ARN that contains this resource set.
    #[builder(into)]
    #[serde(rename = "readinessScopes")]
    pub r#readiness_scopes: Option<Vec<String>>,
    /// ARN of the resource.
    #[builder(into)]
    #[serde(rename = "resourceArn")]
    pub r#resource_arn: Option<String>,
}
