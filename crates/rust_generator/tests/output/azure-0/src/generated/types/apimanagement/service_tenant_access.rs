#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTenantAccess {
    /// Should the access to the management API be enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Primary access key for the tenant access information contract.
    #[builder(into)]
    #[serde(rename = "primaryKey")]
    pub r#primary_key: Option<String>,
    /// Secondary access key for the tenant access information contract.
    #[builder(into)]
    #[serde(rename = "secondaryKey")]
    pub r#secondary_key: Option<String>,
    /// The identifier for the tenant access information contract.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Option<String>,
}
