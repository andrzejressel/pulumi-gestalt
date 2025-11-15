#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkManagerCrossTenantScope {
    /// List of management groups.
    #[builder(into)]
    #[serde(rename = "managementGroups")]
    pub r#management_groups: Option<Vec<String>>,
    /// List of subscriptions.
    #[builder(into)]
    #[serde(rename = "subscriptions")]
    pub r#subscriptions: Option<Vec<String>>,
    /// Tenant ID.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Option<String>,
}
