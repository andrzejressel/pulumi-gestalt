#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterMasterAuth {
    /// Base64 encoded public certificate used by clients to authenticate to the cluster endpoint.
    #[builder(into)]
    #[serde(rename = "clientCertificate")]
    pub r#client_certificate: String,
    /// Whether client certificate authorization is enabled for this cluster.
    #[builder(into)]
    #[serde(rename = "clientCertificateConfigs")]
    pub r#client_certificate_configs: Vec<super::super::types::container::GetClusterMasterAuthClientCertificateConfig>,
    /// Base64 encoded private key used by clients to authenticate to the cluster endpoint.
    #[builder(into)]
    #[serde(rename = "clientKey")]
    pub r#client_key: String,
    /// Base64 encoded public certificate that is the root of trust for the cluster.
    #[builder(into)]
    #[serde(rename = "clusterCaCertificate")]
    pub r#cluster_ca_certificate: String,
}
