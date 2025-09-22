#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterNetworkConfig {
    /// The name of the allocated IP range for the private IP AlloyDB cluster. For example: "google-managed-services-default".
    /// If set, the instance IPs for this cluster will be created in the allocated range.
    #[builder(into)]
    #[serde(rename = "allocatedIpRange")]
    pub r#allocated_ip_range: Option<String>,
    /// The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster.
    /// It is specified in the form: "projects/{projectNumber}/global/networks/{network_id}".
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
}
