#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterKubeletIdentity {
    /// The Client ID of the user-defined Managed Identity to be assigned to the Kubelets. If not specified a Managed Identity is created automatically. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    /// The Object ID of the user-defined Managed Identity assigned to the Kubelets.If not specified a Managed Identity is created automatically. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "objectId")]
    pub r#object_id: Option<String>,
    /// The ID of the User Assigned Identity assigned to the Kubelets. If not specified a Managed Identity is created automatically. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Option<String>,
}
