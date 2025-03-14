#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRepositoryVirtualRepositoryConfig {
    /// Policies that configure the upstream artifacts distributed by the Virtual
    /// Repository. Upstream policies cannot be set on a standard repository.
    #[builder(into)]
    #[serde(rename = "upstreamPolicies")]
    pub r#upstream_policies: Box<Vec<super::super::types::artifactregistry::GetRepositoryVirtualRepositoryConfigUpstreamPolicy>>,
}
