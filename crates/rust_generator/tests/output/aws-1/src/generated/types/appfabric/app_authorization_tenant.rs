#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppAuthorizationTenant {
    /// The display name of the tenant.
    #[builder(into)]
    #[serde(rename = "tenantDisplayName")]
    pub r#tenant_display_name: String,
    /// The ID of the application tenant.
    #[builder(into)]
    #[serde(rename = "tenantIdentifier")]
    pub r#tenant_identifier: String,
}
