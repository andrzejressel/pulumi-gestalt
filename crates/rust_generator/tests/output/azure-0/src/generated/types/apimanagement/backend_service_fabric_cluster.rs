#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BackendServiceFabricCluster {
    /// The client certificate resource id for the management endpoint.
    /// 
    /// > **Note:** At least one of `client_certificate_thumbprint`, and `client_certificate_id` must be set.
    /// >
    #[builder(into)]
    #[serde(rename = "clientCertificateId")]
    pub r#client_certificate_id: Option<String>,
    /// The client certificate thumbprint for the management endpoint.
    #[builder(into)]
    #[serde(rename = "clientCertificateThumbprint")]
    pub r#client_certificate_thumbprint: Option<String>,
    /// A list of cluster management endpoints.
    #[builder(into)]
    #[serde(rename = "managementEndpoints")]
    pub r#management_endpoints: Vec<String>,
    /// The maximum number of retries when attempting resolve the partition.
    #[builder(into)]
    #[serde(rename = "maxPartitionResolutionRetries")]
    pub r#max_partition_resolution_retries: i32,
    /// A list of thumbprints of the server certificates of the Service Fabric cluster.
    #[builder(into)]
    #[serde(rename = "serverCertificateThumbprints")]
    pub r#server_certificate_thumbprints: Option<Vec<String>>,
    /// One or more `server_x509_name` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "serverX509Names")]
    pub r#server_x_509_names: Option<Vec<super::super::types::apimanagement::BackendServiceFabricClusterServerX509Name>>,
}
