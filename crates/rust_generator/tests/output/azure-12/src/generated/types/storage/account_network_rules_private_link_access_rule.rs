#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountNetworkRulesPrivateLinkAccessRule {
    /// The resource id of the resource access rule to be granted access.
    #[builder(into)]
    #[serde(rename = "endpointResourceId")]
    pub r#endpoint_resource_id: String,
    /// The tenant id of the resource of the resource access rule to be granted access. Defaults to the current tenant id.
    #[builder(into)]
    #[serde(rename = "endpointTenantId")]
    pub r#endpoint_tenant_id: Option<String>,
}
