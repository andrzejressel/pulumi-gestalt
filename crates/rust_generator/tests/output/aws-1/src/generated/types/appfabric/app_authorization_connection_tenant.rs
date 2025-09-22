#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AppAuthorizationConnectionTenant {
    #[builder(into)]
    #[serde(rename = "tenantDisplayName")]
    pub r#tenant_display_name: String,
    #[builder(into)]
    #[serde(rename = "tenantIdentifier")]
    pub r#tenant_identifier: String,
}
