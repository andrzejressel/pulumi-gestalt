#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceTenantAccess {
    /// Is access to the Management API enabled (presumably "for this Tenant")?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Primary access key for the tenant access information contract.
    #[builder(into)]
    #[serde(rename = "primaryKey")]
    pub r#primary_key: String,
    /// Secondary access key for the tenant access information contract.
    #[builder(into)]
    #[serde(rename = "secondaryKey")]
    pub r#secondary_key: String,
    /// The ID of the Tenant which has access to this API Management instance.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: String,
}
