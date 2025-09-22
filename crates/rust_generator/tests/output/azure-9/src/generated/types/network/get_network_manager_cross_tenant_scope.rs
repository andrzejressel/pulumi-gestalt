#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNetworkManagerCrossTenantScope {
    /// A list of management groups used as cross tenant scope for the Network Manager.
    #[builder(into)]
    #[serde(rename = "managementGroups")]
    pub r#management_groups: Vec<String>,
    /// A list of subscriptions used as cross tenant scope for the Network Manager.
    #[builder(into)]
    #[serde(rename = "subscriptions")]
    pub r#subscriptions: Vec<String>,
    /// The tenant ID of the cross tenant scope.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: String,
}
