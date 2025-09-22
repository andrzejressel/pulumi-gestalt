#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFunctionAppIdentity {
    /// A list of User Assigned Identity IDs assigned to the Function App.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Vec<String>,
    /// The ID of the Managed Identity assigned to the Function App.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: String,
    /// The ID of the Tenant where the Managed Identity assigned to the Function App is located.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: String,
    /// The identity type of the Managed Identity assigned to the Function App.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
