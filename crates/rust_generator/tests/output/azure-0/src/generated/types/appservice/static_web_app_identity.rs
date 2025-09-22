#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StaticWebAppIdentity {
    /// A list of Managed Identity IDs which should be assigned to this Static Web App resource.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Option<String>,
    /// The Type of Managed Identity assigned to this Static Web App resource. Possible values are `SystemAssigned`, `UserAssigned` and `SystemAssigned, UserAssigned`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
