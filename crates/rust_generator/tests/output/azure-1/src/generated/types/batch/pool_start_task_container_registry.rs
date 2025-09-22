#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PoolStartTaskContainerRegistry {
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// The container registry URL. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "registryServer")]
    pub r#registry_server: String,
    /// The User Assigned Identity to use for Container Registry access.
    #[builder(into)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "userName")]
    pub r#user_name: Option<String>,
}
