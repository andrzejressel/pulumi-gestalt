#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobIdentity {
    /// A list of Managed Identity IDs to assign to the Container App Job.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Option<String>,
    /// The type of identity used for the Container App Job. Possible values are `SystemAssigned`, `UserAssigned` and `None`. Defaults to `None`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
