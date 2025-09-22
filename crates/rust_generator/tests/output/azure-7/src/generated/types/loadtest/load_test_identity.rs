#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LoadTestIdentity {
    /// A list of the User Assigned Identity IDs that should be assigned to this Load Test.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Option<Vec<String>>,
    /// The Principal ID for the System-Assigned Managed Identity assigned to this Load Test.
    /// *
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Option<String>,
    /// The Tenant ID for the System-Assigned Managed Identity assigned to this Load Test.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Option<String>,
    /// Specifies the type of Managed Identity that should be assigned to this Load Test Encryption. Possible values are `SystemAssigned` or `UserAssigned`. Changing this forces a new Load Test to be created.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
