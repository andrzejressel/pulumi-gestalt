#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ActionGroupWebhookReceiverAadAuth {
    /// The identifier URI for AAD auth.
    #[builder(into)]
    #[serde(rename = "identifierUri")]
    pub r#identifier_uri: Option<String>,
    /// The webhook application object Id for AAD auth.
    #[builder(into)]
    #[serde(rename = "objectId")]
    pub r#object_id: String,
    /// The tenant id for AAD auth.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Option<String>,
}
