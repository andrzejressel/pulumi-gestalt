#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetKubernetesClusterServiceMeshProfile {
    /// A `certificate_authority` block as documented below.
    #[builder(into)]
    #[serde(rename = "certificateAuthorities")]
    pub r#certificate_authorities: Vec<super::super::types::containerservice::GetKubernetesClusterServiceMeshProfileCertificateAuthority>,
    /// Is Istio External Ingress Gateway enabled?
    #[builder(into)]
    #[serde(rename = "externalIngressGatewayEnabled")]
    pub r#external_ingress_gateway_enabled: bool,
    /// Is Istio Internal Ingress Gateway enabled?
    #[builder(into)]
    #[serde(rename = "internalIngressGatewayEnabled")]
    pub r#internal_ingress_gateway_enabled: bool,
    /// The mode of the service mesh.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: String,
    /// List of revisions of the Istio control plane. When an upgrade is not in progress, this holds one value. When canary upgrade is in progress, this can only hold two consecutive values. Learn More.
    #[builder(into)]
    #[serde(rename = "revisions")]
    pub r#revisions: Vec<String>,
}
