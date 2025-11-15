#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterIngressApplicationGatewayIngressApplicationGatewayIdentity {
    /// The Client ID of the user-defined Managed Identity used for Web App Routing.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    /// The Object ID of the user-defined Managed Identity used for Web App Routing
    #[builder(into)]
    #[serde(rename = "objectId")]
    pub r#object_id: Option<String>,
    /// The ID of the User Assigned Identity used for Web App Routing.
    #[builder(into)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Option<String>,
}
