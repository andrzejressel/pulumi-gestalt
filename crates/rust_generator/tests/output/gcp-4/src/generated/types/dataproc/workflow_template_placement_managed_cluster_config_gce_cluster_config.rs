#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowTemplatePlacementManagedClusterConfigGceClusterConfig {
    /// If true, all instances in the cluster will only have internal IP addresses. By default, clusters are not restricted to internal IP addresses, and will have ephemeral external IP addresses assigned to each instance. This `internal_ip_only` restriction can only be enabled for subnetwork enabled networks, and all off-cluster dependencies must be configured to be accessible without external IP addresses.
    #[builder(into)]
    #[serde(rename = "internalIpOnly")]
    pub r#internal_ip_only: Option<bool>,
    /// The Compute Engine metadata entries to add to all instances (see [About VM metadata](https://cloud.google.com/compute/docs/metadata/overview)).
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: Option<std::collections::HashMap<String, String>>,
    /// The Compute Engine network to be used for machine communications. Cannot be specified with subnetwork_uri. If neither `network_uri` nor `subnetwork_uri` is specified, the "default" network of the project is used, if it exists. Cannot be a "Custom Subnet Network" (see /regions/global/default` * `default`
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// Node Group Affinity for sole-tenant clusters.
    #[builder(into)]
    #[serde(rename = "nodeGroupAffinity")]
    pub r#node_group_affinity: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigGceClusterConfigNodeGroupAffinity>>,
    /// The type of IPv6 access for a cluster. Possible values: PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED, INHERIT_FROM_SUBNETWORK, OUTBOUND, BIDIRECTIONAL
    #[builder(into)]
    #[serde(rename = "privateIpv6GoogleAccess")]
    pub r#private_ipv_6_google_access: Option<String>,
    /// Reservation Affinity for consuming Zonal reservation.
    #[builder(into)]
    #[serde(rename = "reservationAffinity")]
    pub r#reservation_affinity: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigGceClusterConfigReservationAffinity>>,
    /// The (https://cloud.google.com/compute/docs/access/service-accounts#default_service_account) is used.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Option<String>,
    /// The URIs of service account scopes to be included in Compute Engine instances. The following base set of scopes is always included: * https://www.googleapis.com/auth/cloud.useraccounts.readonly * https://www.googleapis.com/auth/devstorage.read_write * https://www.googleapis.com/auth/logging.write If no scopes are specified, the following defaults are also provided: * https://www.googleapis.com/auth/bigquery * https://www.googleapis.com/auth/bigtable.admin.table * https://www.googleapis.com/auth/bigtable.data * https://www.googleapis.com/auth/devstorage.full_control
    #[builder(into)]
    #[serde(rename = "serviceAccountScopes")]
    pub r#service_account_scopes: Option<Vec<String>>,
    /// Shielded Instance Config for clusters using [Compute Engine Shielded VMs](https://cloud.google.com/security/shielded-cloud/shielded-vm). Structure defined below.
    #[builder(into)]
    #[serde(rename = "shieldedInstanceConfig")]
    pub r#shielded_instance_config: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigGceClusterConfigShieldedInstanceConfig>>,
    /// The Compute Engine subnetwork to be used for machine communications. Cannot be specified with network_uri. A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects//regions/us-east1/subnetworks/sub0` * `sub0`
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Option<String>,
    /// The Compute Engine tags to add to all instances (see [Manage tags for resources](https://cloud.google.com/compute/docs/tag-resources)).
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<Vec<String>>,
    /// The zone where the Compute Engine cluster will be located. On a create request, it is required in the "global" region. If omitted in a non-global Dataproc region, the service will pick a zone in the corresponding Compute Engine region. On a get request, zone will always be present. A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/` * `us-central1-f`
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: Option<String>,
}
