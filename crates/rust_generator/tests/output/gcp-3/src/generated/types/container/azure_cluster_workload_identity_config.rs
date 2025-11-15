#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AzureClusterWorkloadIdentityConfig {
    /// The ID of the OIDC Identity Provider (IdP) associated to the Workload Identity Pool.
    #[builder(into)]
    #[serde(rename = "identityProvider")]
    pub r#identity_provider: Option<String>,
    /// The OIDC issuer URL for this cluster.
    #[builder(into)]
    #[serde(rename = "issuerUri")]
    pub r#issuer_uri: Option<String>,
    /// The Workload Identity Pool associated to the cluster.
    #[builder(into)]
    #[serde(rename = "workloadPool")]
    pub r#workload_pool: Option<String>,
}
