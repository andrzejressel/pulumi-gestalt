#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVolumeGroupIdentity {
    /// A list of the User Assigned Identity IDs assigned to this Elastic SAN Volume Group.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Vec<String>,
    /// The Principal ID associated with the Managed Service Identity assigned to this Elastic SAN Volume Group.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: String,
    /// The Tenant ID associated with this Managed Service Identity assigned to this Elastic SAN Volume Group.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: String,
    /// The type of Managed Identity assigned to this Elastic SAN Volume Group.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
