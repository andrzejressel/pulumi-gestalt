#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountIdentity {
    /// Specifies a list of User Assigned Managed Identity IDs to be assigned to this Cosmos Account.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Option<Vec<String>>,
    /// The Principal ID associated with this Managed Service Identity.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Option<String>,
    /// The Tenant ID associated with this Managed Service Identity.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Option<String>,
    /// The Type of Managed Identity assigned to this Cosmos account. Possible values are `SystemAssigned`, `UserAssigned` and `SystemAssigned, UserAssigned`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
