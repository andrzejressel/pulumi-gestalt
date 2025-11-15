#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPolicyAssignmentIdentity {
    /// A `identity_ids` block as defined below.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Vec<String>,
    /// The Principal ID of the Policy Assignment for this Resource.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: String,
    /// The Tenant ID of the Policy Assignment for this Resource.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: String,
    /// The Type of Managed Identity which is added to this Policy Assignment.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
