/// Manages a Google Kubernetes Engine (GKE) cluster.
///
/// To get more information about GKE clusters, see:
///   * [The API reference](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/projects.locations.clusters)
///   * How-to guides
///     * [GKE overview](https://cloud.google.com/kubernetes-engine/docs/concepts/kubernetes-engine-overview)
///     * [About cluster configuration choices](https://cloud.google.com/kubernetes-engine/docs/concepts/types-of-clusters)
///
/// > On version 5.0.0+ of the provider, you must explicitly set `deletion_protection = false`
/// and run `pulumi up` to write the field to state in order to destroy a cluster.
///
/// > All arguments and attributes (including certificate outputs) will be stored in the raw state as
/// plaintext. [Read more about secrets in state](https://www.pulumi.com/docs/intro/concepts/programming-model/#secrets).
///
/// ## Example Usage
///
/// ### With A Separately Managed Node Pool (Recommended)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = account::create(
///         "default",
///         AccountArgs::builder()
///             .account_id("service-account-id")
///             .display_name("Service Account")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .initial_node_count(1)
///             .location("us-central1")
///             .name("my-gke-cluster")
///             .remove_default_node_pool(true)
///             .build_struct(),
///     );
///     let primaryPreemptibleNodes = node_pool::create(
///         "primaryPreemptibleNodes",
///         NodePoolArgs::builder()
///             .cluster("${primary.name}")
///             .location("us-central1")
///             .name("my-node-pool")
///             .node_config(
///                 NodePoolNodeConfig::builder()
///                     .machineType("e2-medium")
///                     .oauthScopes(vec!["https://www.googleapis.com/auth/cloud-platform",])
///                     .preemptible(true)
///                     .serviceAccount("${default.email}")
///                     .build_struct(),
///             )
///             .node_count(1)
///             .build_struct(),
///     );
/// }
/// ```
///
/// > **Note:** It is recommended that node pools be created and managed as separate resources as in the example above.
/// This allows node pools to be added and removed without recreating the cluster.  Node pools defined directly in the
/// `gcp.container.Cluster` resource cannot be removed without re-creating the cluster.
///
/// ### With The Default Node Pool
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: service-account-id
///       displayName: Service Account
///   primary:
///     type: gcp:container:Cluster
///     properties:
///       name: marcellus-wallace
///       location: us-central1-a
///       initialNodeCount: 3
///       nodeConfig:
///         serviceAccount: ${default.email}
///         oauthScopes:
///           - https://www.googleapis.com/auth/cloud-platform
///         labels:
///           foo: bar
///         tags:
///           - foo
///           - bar
/// ```
///
/// ### Autopilot
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = account::create(
///         "default",
///         AccountArgs::builder()
///             .account_id("service-account-id")
///             .display_name("Service Account")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .enable_autopilot(true)
///             .location("us-central1-a")
///             .name("marcellus-wallace")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// GKE clusters can be imported using the `project` , `location`, and `name`. If the project is omitted, the default
///
/// provider value will be used. Examples:
///
/// * `projects/{{project_id}}/locations/{{location}}/clusters/{{cluster_id}}`
///
/// * `{{project_id}}/{{location}}/{{cluster_id}}`
///
/// * `{{location}}/{{cluster_id}}`
///
/// When using the `pulumi import` command, GKE clusters can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:container/cluster:Cluster default projects/{{project_id}}/locations/{{location}}/clusters/{{cluster_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/cluster:Cluster default {{project_id}}/{{location}}/{{cluster_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/cluster:Cluster default {{location}}/{{cluster_id}}
/// ```
///
/// For example, the following fields will show diffs if set in config:
///
/// - `min_master_version`
///
/// - `remove_default_node_pool`
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// The configuration for addons supported by GKE.
        /// Structure is documented below.
        #[builder(into, default)]
        pub addons_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterAddonsConfig>,
        >,
        /// Enable NET_ADMIN for the cluster. Defaults to
        /// `false`. This field should only be enabled for Autopilot clusters (`enable_autopilot`
        /// set to `true`).
        #[builder(into, default)]
        pub allow_net_admin: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration for the
        /// [Google Groups for GKE](https://cloud.google.com/kubernetes-engine/docs/how-to/role-based-access-control#groups-setup-gsuite) feature.
        /// Structure is documented below.
        #[builder(into, default)]
        pub authenticator_groups_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterAuthenticatorGroupsConfig>,
        >,
        /// Configuration options for the Binary
        /// Authorization feature. Structure is documented below.
        #[builder(into, default)]
        pub binary_authorization: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterBinaryAuthorization>,
        >,
        /// Per-cluster configuration of Node Auto-Provisioning with Cluster Autoscaler to
        /// automatically adjust the size of the cluster and create/delete node pools based
        /// on the current needs of the cluster's workload. See the
        /// [guide to using Node Auto-Provisioning](https://cloud.google.com/kubernetes-engine/docs/how-to/node-auto-provisioning)
        /// for more details. Structure is documented below.
        #[builder(into, default)]
        pub cluster_autoscaling: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterClusterAutoscaling>,
        >,
        /// The IP address range of the Kubernetes pods
        /// in this cluster in CIDR notation (e.g. `10.96.0.0/14`). Leave blank to have one
        /// automatically chosen or specify a `/14` block in `10.0.0.0/8`. This field will
        /// default a new cluster to routes-based, where `ip_allocation_policy` is not defined.
        #[builder(into, default)]
        pub cluster_ipv4_cidr: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration for
        /// [ClusterTelemetry](https://cloud.google.com/monitoring/kubernetes-engine/installing#controlling_the_collection_of_application_logs) feature,
        /// Structure is documented below.
        #[builder(into, default)]
        pub cluster_telemetry: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterClusterTelemetry>,
        >,
        /// Configuration for [Confidential Nodes](https://cloud.google.com/kubernetes-engine/docs/how-to/confidential-gke-nodes) feature. Structure is documented below documented below.
        #[builder(into, default)]
        pub confidential_nodes: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterConfidentialNodes>,
        >,
        /// Configuration for all of the cluster's control plane endpoints.
        /// Structure is documented below.
        #[builder(into, default)]
        pub control_plane_endpoints_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterControlPlaneEndpointsConfig>,
        >,
        /// Configuration for the
        /// [Cost Allocation](https://cloud.google.com/kubernetes-engine/docs/how-to/cost-allocations) feature.
        /// Structure is documented below.
        #[builder(into, default)]
        pub cost_management_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterCostManagementConfig>,
        >,
        /// Structure is documented below.
        #[builder(into, default)]
        pub database_encryption: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterDatabaseEncryption>,
        >,
        /// The desired datapath provider for this cluster. This is set to `LEGACY_DATAPATH` by default, which uses the IPTables-based kube-proxy implementation. Set to `ADVANCED_DATAPATH` to enable Dataplane v2.
        #[builder(into, default)]
        pub datapath_provider: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The default maximum number of pods
        /// per node in this cluster. This doesn't work on "routes-based" clusters, clusters
        /// that don't have IP Aliasing enabled. See the [official documentation](https://cloud.google.com/kubernetes-engine/docs/how-to/flexible-pod-cidr)
        /// for more information.
        #[builder(into, default)]
        pub default_max_pods_per_node: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// [GKE SNAT](https://cloud.google.com/kubernetes-engine/docs/how-to/ip-masquerade-agent#how_ipmasq_works) DefaultSnatStatus contains the desired state of whether default sNAT should be disabled on the cluster, [API doc](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/projects.locations.clusters#networkconfig). Structure is documented below
        #[builder(into, default)]
        pub default_snat_status: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterDefaultSnatStatus>,
        >,
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Description of the cluster.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration for [Using Cloud DNS for GKE](https://cloud.google.com/kubernetes-engine/docs/how-to/cloud-dns). Structure is documented below.
        #[builder(into, default)]
        pub dns_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterDnsConfig>,
        >,
        /// Enable Autopilot for this cluster. Defaults to `false`.
        /// Note that when this option is enabled, certain features of Standard GKE are not available.
        /// See the [official documentation](https://cloud.google.com/kubernetes-engine/docs/concepts/autopilot-overview#comparison)
        /// for available features.
        #[builder(into, default)]
        pub enable_autopilot: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether CiliumClusterWideNetworkPolicy is enabled on this cluster. Defaults to false.
        #[builder(into, default)]
        pub enable_cilium_clusterwide_network_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether FQDN Network Policy is enabled on this cluster. Users who enable this feature for existing Standard clusters must restart the GKE Dataplane V2 `anetd` DaemonSet after enabling it. See the [Enable FQDN Network Policy in an existing cluster](https://cloud.google.com/kubernetes-engine/docs/how-to/fqdn-network-policies#enable_fqdn_network_policy_in_an_existing_cluster) for more information.
        #[builder(into, default)]
        pub enable_fqdn_network_policy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether Intra-node visibility is enabled for this cluster. This makes same node pod to pod traffic visible for VPC network.
        #[builder(into, default)]
        pub enable_intranode_visibility: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Configuration for Kubernetes Beta APIs.
        /// Structure is documented below.
        #[builder(into, default)]
        pub enable_k8s_beta_apis: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterEnableK8SBetaApis>,
        >,
        /// Whether to enable Kubernetes Alpha features for
        /// this cluster. Note that when this option is enabled, the cluster cannot be upgraded
        /// and will be automatically deleted after 30 days.
        #[builder(into, default)]
        pub enable_kubernetes_alpha: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether L4ILB Subsetting is enabled for this cluster.
        #[builder(into, default)]
        pub enable_l4_ilb_subsetting: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether the ABAC authorizer is enabled for this cluster.
        /// When enabled, identities in the system, including service accounts, nodes, and controllers,
        /// will have statically granted permissions beyond those provided by the RBAC configuration or IAM.
        /// Defaults to `false`
        #[builder(into, default)]
        pub enable_legacy_abac: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether multi-networking is enabled for this cluster.
        #[builder(into, default)]
        pub enable_multi_networking: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Enable Shielded Nodes features on all nodes in this cluster.  Defaults to `true`.
        #[builder(into, default)]
        pub enable_shielded_nodes: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to enable Cloud TPU resources in this cluster.
        /// See the [official documentation](https://cloud.google.com/tpu/docs/kubernetes-engine-setup).
        #[builder(into, default)]
        pub enable_tpu: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration for [Enterprise edition].(https://cloud.google.com/kubernetes-engine/enterprise/docs/concepts/gke-editions). Structure is documented below.
        ///
        ///
        /// <a name="nested_default_snat_status"></a>The `default_snat_status` block supports
        #[builder(into, default)]
        pub enterprise_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterEnterpriseConfig>,
        >,
        /// Fleet configuration for the cluster. Structure is documented below.
        #[builder(into, default)]
        pub fleet: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterFleet>,
        >,
        /// Configuration for [GKE Gateway API controller](https://cloud.google.com/kubernetes-engine/docs/concepts/gateway-api). Structure is documented below.
        #[builder(into, default)]
        pub gateway_api_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterGatewayApiConfig>,
        >,
        /// . Structure is documented below.
        #[builder(into, default)]
        pub identity_service_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterIdentityServiceConfig>,
        >,
        /// The number of nodes to create in this
        /// cluster's default node pool. In regional or multi-zonal clusters, this is the
        /// number of nodes per zone. Must be set if `node_pool` is not set. If you're using
        /// `gcp.container.NodePool` objects with no default node pool, you'll need to
        /// set this to a value of at least `1`, alongside setting
        /// `remove_default_node_pool` to `true`.
        #[builder(into, default)]
        pub initial_node_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Configuration of cluster IP allocation for
        /// VPC-native clusters. If this block is unset during creation, it will be set by the GKE backend.
        /// Structure is documented below.
        #[builder(into, default)]
        pub ip_allocation_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterIpAllocationPolicy>,
        >,
        /// The location (region or zone) in which the cluster
        /// master will be created, as well as the default node location. If you specify a
        /// zone (such as `us-central1-a`), the cluster will be a zonal cluster with a
        /// single cluster master. If you specify a region (such as `us-west1`), the
        /// cluster will be a regional cluster with multiple masters spread across zones in
        /// the region, and with default node locations in those zones as well
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Logging configuration for the cluster.
        /// Structure is documented below.
        #[builder(into, default)]
        pub logging_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterLoggingConfig>,
        >,
        /// The logging service that the cluster should
        /// write logs to. Available options include `logging.googleapis.com`(Legacy Stackdriver),
        /// `logging.googleapis.com/kubernetes`(Stackdriver Kubernetes Engine Logging), and `none`. Defaults to `logging.googleapis.com/kubernetes`
        #[builder(into, default)]
        pub logging_service: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maintenance policy to use for the cluster. Structure is
        /// documented below.
        #[builder(into, default)]
        pub maintenance_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterMaintenancePolicy>,
        >,
        /// The authentication information for accessing the
        /// Kubernetes master. Some values in this block are only returned by the API if
        /// your service account has permission to get credentials for your GKE cluster. If
        /// you see an unexpected diff unsetting your client cert, ensure you have the
        /// `container.clusters.getCredentials` permission.
        /// Structure is documented below.
        #[builder(into, default)]
        pub master_auth: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterMasterAuth>,
        >,
        /// The desired
        /// configuration options for master authorized networks. Omit the
        /// nested `cidr_blocks` attribute to disallow external access (except
        /// the cluster node IPs, which GKE automatically whitelists).
        /// Structure is documented below.
        #[builder(into, default)]
        pub master_authorized_networks_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterMasterAuthorizedNetworksConfig>,
        >,
        /// Structure is documented below.
        #[builder(into, default)]
        pub mesh_certificates: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterMeshCertificates>,
        >,
        /// The minimum version of the master. GKE
        /// will auto-update the master to new versions, so this does not guarantee the
        /// current master version--use the read-only `master_version` field to obtain that.
        /// If unset, the cluster's version will be set by GKE to the version of the most recent
        /// official release (which is not necessarily the latest version).  Most users will find
        /// the `gcp.container.getEngineVersions` data source useful - it indicates which versions
        /// are available. If you intend to specify versions manually,
        /// [the docs](https://cloud.google.com/kubernetes-engine/versioning-and-upgrades#specifying_cluster_version)
        /// describe the various acceptable formats for this field.
        ///
        /// > If you are using the `gcp.container.getEngineVersions` datasource with a regional cluster, ensure that you have provided a `location`
        /// to the datasource. A region can have a different set of supported versions than its corresponding zones, and not all zones in a
        /// region are guaranteed to support the same version.
        #[builder(into, default)]
        pub min_master_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Monitoring configuration for the cluster.
        /// Structure is documented below.
        #[builder(into, default)]
        pub monitoring_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterMonitoringConfig>,
        >,
        /// The monitoring service that the cluster
        /// should write metrics to.
        /// Automatically send metrics from pods in the cluster to the Google Cloud Monitoring API.
        /// VM metrics will be collected by Google Compute Engine regardless of this setting
        /// Available options include
        /// `monitoring.googleapis.com`(Legacy Stackdriver), `monitoring.googleapis.com/kubernetes`(Stackdriver Kubernetes Engine Monitoring), and `none`.
        /// Defaults to `monitoring.googleapis.com/kubernetes`
        #[builder(into, default)]
        pub monitoring_service: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the cluster, unique within the project and
        /// location.
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name or self_link of the Google Compute Engine
        /// network to which the cluster is connected. For Shared VPC, set this to the self link of the
        /// shared network.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration options for the
        /// [NetworkPolicy](https://kubernetes.io/docs/concepts/services-networking/networkpolicies/)
        /// feature. Structure is documented below.
        #[builder(into, default)]
        pub network_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterNetworkPolicy>,
        >,
        /// Determines whether alias IPs or routes will be used for pod IPs in the cluster.
        /// Options are `VPC_NATIVE` or `ROUTES`. `VPC_NATIVE` enables [IP aliasing](https://cloud.google.com/kubernetes-engine/docs/how-to/ip-aliases). Newly created clusters will default to `VPC_NATIVE`.
        #[builder(into, default)]
        pub networking_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Parameters used in creating the default node pool.
        /// Generally, this field should not be used at the same time as a
        /// `gcp.container.NodePool` or a `node_pool` block; this configuration
        /// manages the default node pool, which isn't recommended to be used.
        /// Structure is documented below.
        #[builder(into, default)]
        pub node_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterNodeConfig>,
        >,
        /// The list of zones in which the cluster's nodes
        /// are located. Nodes must be in the region of their regional cluster or in the
        /// same region as their cluster's zone for zonal clusters. If this is specified for
        /// a zonal cluster, omit the cluster's zone.
        ///
        /// > A "multi-zonal" cluster is a zonal cluster with at least one additional zone
        /// defined; in a multi-zonal cluster, the cluster master is only present in a
        /// single zone while nodes are present in each of the primary zone and the node
        /// locations. In contrast, in a regional cluster, cluster master nodes are present
        /// in multiple zones in the region. For that reason, regional clusters should be
        /// preferred.
        #[builder(into, default)]
        pub node_locations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Node pool configs that apply to auto-provisioned node pools in
        /// [autopilot](https://cloud.google.com/kubernetes-engine/docs/concepts/autopilot-overview#comparison) clusters and
        /// [node auto-provisioning](https://cloud.google.com/kubernetes-engine/docs/how-to/node-auto-provisioning)-enabled clusters. Structure is documented below.
        #[builder(into, default)]
        pub node_pool_auto_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterNodePoolAutoConfig>,
        >,
        /// Default NodePool settings for the entire cluster. These settings are overridden if specified on the specific NodePool object. Structure is documented below.
        #[builder(into, default)]
        pub node_pool_defaults: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterNodePoolDefaults>,
        >,
        /// List of node pools associated with this cluster.
        /// See gcp.container.NodePool for schema.
        /// **Warning:** node pools defined inside a cluster can't be changed (or added/removed) after
        /// cluster creation without deleting and recreating the entire cluster. Unless you absolutely need the ability
        /// to say "these are the _only_ node pools associated with this cluster", use the
        /// gcp.container.NodePool resource instead of this property.
        #[builder(into, default)]
        pub node_pools: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::container::ClusterNodePool>>,
        >,
        /// The Kubernetes version on the nodes. Must either be unset
        /// or set to the same value as `min_master_version` on create. Defaults to the default
        /// version set by GKE which is not necessarily the latest version. This only affects
        /// nodes in the default node pool. While a fuzzy version can be specified, it's
        /// recommended that you specify explicit versions as the provider will see spurious diffs
        /// when fuzzy versions are used. See the `gcp.container.getEngineVersions` data source's
        /// `version_prefix` field to approximate fuzzy versions.
        /// To update nodes in other node pools, use the `version` attribute on the node pool.
        #[builder(into, default)]
        pub node_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration for the [cluster upgrade notifications](https://cloud.google.com/kubernetes-engine/docs/how-to/cluster-upgrade-notifications) feature. Structure is documented below.
        #[builder(into, default)]
        pub notification_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterNotificationConfig>,
        >,
        /// Configuration for the
        /// [PodSecurityPolicy](https://cloud.google.com/kubernetes-engine/docs/how-to/pod-security-policies) feature.
        /// Structure is documented below.
        #[builder(into, default)]
        pub pod_security_policy_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterPodSecurityPolicyConfig>,
        >,
        /// Configuration for [private clusters](https://cloud.google.com/kubernetes-engine/docs/how-to/private-clusters),
        /// clusters with private nodes. Structure is documented below.
        #[builder(into, default)]
        pub private_cluster_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterPrivateClusterConfig>,
        >,
        /// The desired state of IPv6 connectivity to Google Services. By default, no private IPv6 access to or from Google Services (all access will be via IPv4).
        #[builder(into, default)]
        pub private_ipv6_google_access: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enable/Disable Protect API features for the cluster. Structure is documented below.
        #[builder(into, default)]
        pub protect_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterProtectConfig>,
        >,
        /// Configuration options for the [Release channel](https://cloud.google.com/kubernetes-engine/docs/concepts/release-channels)
        /// feature, which provide more control over automatic upgrades of your GKE clusters.
        /// When updating this field, GKE imposes specific version requirements. See
        /// [Selecting a new release channel](https://cloud.google.com/kubernetes-engine/docs/concepts/release-channels#selecting_a_new_release_channel)
        /// for more details; the `gcp.container.getEngineVersions` datasource can provide
        /// the default version for a channel. Note that removing the `release_channel`
        /// field from your config will cause the provider to stop managing your cluster's
        /// release channel, but will not unenroll it. Instead, use the `"UNSPECIFIED"`
        /// channel. Structure is documented below.
        #[builder(into, default)]
        pub release_channel: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterReleaseChannel>,
        >,
        /// If `true`, deletes the default node
        /// pool upon cluster creation. If you're using `gcp.container.NodePool`
        /// resources with no default node pool, this should be set to `true`, alongside
        /// setting `initial_node_count` to at least `1`.
        #[builder(into, default)]
        pub remove_default_node_pool: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The GCE resource labels (a map of key/value pairs) to be applied to the cluster.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub resource_labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration for the
        /// [ResourceUsageExportConfig](https://cloud.google.com/kubernetes-engine/docs/how-to/cluster-usage-metering) feature.
        /// Structure is documented below.
        #[builder(into, default)]
        pub resource_usage_export_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterResourceUsageExportConfig>,
        >,
        /// Configuration for the
        /// [SecretManagerConfig](https://cloud.google.com/secret-manager/docs/secret-manager-managed-csi-component) feature.
        /// Structure is documented below.
        #[builder(into, default)]
        pub secret_manager_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterSecretManagerConfig>,
        >,
        /// Enable/Disable Security Posture API features for the cluster. Structure is documented below.
        #[builder(into, default)]
        pub security_posture_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterSecurityPostureConfig>,
        >,
        /// Structure is documented below.
        #[builder(into, default)]
        pub service_external_ips_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterServiceExternalIpsConfig>,
        >,
        /// The name or self_link of the Google Compute Engine
        /// subnetwork in which the cluster's instances are launched.
        #[builder(into, default)]
        pub subnetwork: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// TPU configuration for the cluster.
        #[builder(into, default)]
        pub tpu_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterTpuConfig>,
        >,
        /// The custom keys configuration of the cluster.
        #[builder(into, default)]
        pub user_managed_keys_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterUserManagedKeysConfig>,
        >,
        /// Vertical Pod Autoscaling automatically adjusts the resources of pods controlled by it.
        /// Structure is documented below.
        #[builder(into, default)]
        pub vertical_pod_autoscaling: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterVerticalPodAutoscaling>,
        >,
        /// Configuration for [direct-path (via ALTS) with workload identity.](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/projects.locations.clusters#workloadaltsconfig). Structure is documented below.
        #[builder(into, default)]
        pub workload_alts_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterWorkloadAltsConfig>,
        >,
        /// Workload Identity allows Kubernetes service accounts to act as a user-managed
        /// [Google IAM Service Account](https://cloud.google.com/iam/docs/service-accounts#user-managed_service_accounts).
        /// Structure is documented below.
        #[builder(into, default)]
        pub workload_identity_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::ClusterWorkloadIdentityConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// The configuration for addons supported by GKE.
        /// Structure is documented below.
        pub addons_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterAddonsConfig,
        >,
        /// Enable NET_ADMIN for the cluster. Defaults to
        /// `false`. This field should only be enabled for Autopilot clusters (`enable_autopilot`
        /// set to `true`).
        pub allow_net_admin: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Configuration for the
        /// [Google Groups for GKE](https://cloud.google.com/kubernetes-engine/docs/how-to/role-based-access-control#groups-setup-gsuite) feature.
        /// Structure is documented below.
        pub authenticator_groups_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterAuthenticatorGroupsConfig,
        >,
        /// Configuration options for the Binary
        /// Authorization feature. Structure is documented below.
        pub binary_authorization: pulumi_gestalt_rust::Output<
            Option<super::super::types::container::ClusterBinaryAuthorization>,
        >,
        /// Per-cluster configuration of Node Auto-Provisioning with Cluster Autoscaler to
        /// automatically adjust the size of the cluster and create/delete node pools based
        /// on the current needs of the cluster's workload. See the
        /// [guide to using Node Auto-Provisioning](https://cloud.google.com/kubernetes-engine/docs/how-to/node-auto-provisioning)
        /// for more details. Structure is documented below.
        pub cluster_autoscaling: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterClusterAutoscaling,
        >,
        /// The IP address range of the Kubernetes pods
        /// in this cluster in CIDR notation (e.g. `10.96.0.0/14`). Leave blank to have one
        /// automatically chosen or specify a `/14` block in `10.0.0.0/8`. This field will
        /// default a new cluster to routes-based, where `ip_allocation_policy` is not defined.
        pub cluster_ipv4_cidr: pulumi_gestalt_rust::Output<String>,
        /// Configuration for
        /// [ClusterTelemetry](https://cloud.google.com/monitoring/kubernetes-engine/installing#controlling_the_collection_of_application_logs) feature,
        /// Structure is documented below.
        pub cluster_telemetry: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterClusterTelemetry,
        >,
        /// Configuration for [Confidential Nodes](https://cloud.google.com/kubernetes-engine/docs/how-to/confidential-gke-nodes) feature. Structure is documented below documented below.
        pub confidential_nodes: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterConfidentialNodes,
        >,
        /// Configuration for all of the cluster's control plane endpoints.
        /// Structure is documented below.
        pub control_plane_endpoints_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterControlPlaneEndpointsConfig,
        >,
        /// Configuration for the
        /// [Cost Allocation](https://cloud.google.com/kubernetes-engine/docs/how-to/cost-allocations) feature.
        /// Structure is documented below.
        pub cost_management_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterCostManagementConfig,
        >,
        /// Structure is documented below.
        pub database_encryption: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterDatabaseEncryption,
        >,
        /// The desired datapath provider for this cluster. This is set to `LEGACY_DATAPATH` by default, which uses the IPTables-based kube-proxy implementation. Set to `ADVANCED_DATAPATH` to enable Dataplane v2.
        pub datapath_provider: pulumi_gestalt_rust::Output<String>,
        /// The default maximum number of pods
        /// per node in this cluster. This doesn't work on "routes-based" clusters, clusters
        /// that don't have IP Aliasing enabled. See the [official documentation](https://cloud.google.com/kubernetes-engine/docs/how-to/flexible-pod-cidr)
        /// for more information.
        pub default_max_pods_per_node: pulumi_gestalt_rust::Output<i32>,
        /// [GKE SNAT](https://cloud.google.com/kubernetes-engine/docs/how-to/ip-masquerade-agent#how_ipmasq_works) DefaultSnatStatus contains the desired state of whether default sNAT should be disabled on the cluster, [API doc](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/projects.locations.clusters#networkconfig). Structure is documented below
        pub default_snat_status: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterDefaultSnatStatus,
        >,
        pub deletion_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Description of the cluster.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration for [Using Cloud DNS for GKE](https://cloud.google.com/kubernetes-engine/docs/how-to/cloud-dns). Structure is documented below.
        pub dns_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::container::ClusterDnsConfig>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Enable Autopilot for this cluster. Defaults to `false`.
        /// Note that when this option is enabled, certain features of Standard GKE are not available.
        /// See the [official documentation](https://cloud.google.com/kubernetes-engine/docs/concepts/autopilot-overview#comparison)
        /// for available features.
        pub enable_autopilot: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether CiliumClusterWideNetworkPolicy is enabled on this cluster. Defaults to false.
        pub enable_cilium_clusterwide_network_policy: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Whether FQDN Network Policy is enabled on this cluster. Users who enable this feature for existing Standard clusters must restart the GKE Dataplane V2 `anetd` DaemonSet after enabling it. See the [Enable FQDN Network Policy in an existing cluster](https://cloud.google.com/kubernetes-engine/docs/how-to/fqdn-network-policies#enable_fqdn_network_policy_in_an_existing_cluster) for more information.
        pub enable_fqdn_network_policy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether Intra-node visibility is enabled for this cluster. This makes same node pod to pod traffic visible for VPC network.
        pub enable_intranode_visibility: pulumi_gestalt_rust::Output<bool>,
        /// Configuration for Kubernetes Beta APIs.
        /// Structure is documented below.
        pub enable_k8s_beta_apis: pulumi_gestalt_rust::Output<
            Option<super::super::types::container::ClusterEnableK8SBetaApis>,
        >,
        /// Whether to enable Kubernetes Alpha features for
        /// this cluster. Note that when this option is enabled, the cluster cannot be upgraded
        /// and will be automatically deleted after 30 days.
        pub enable_kubernetes_alpha: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether L4ILB Subsetting is enabled for this cluster.
        pub enable_l4_ilb_subsetting: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether the ABAC authorizer is enabled for this cluster.
        /// When enabled, identities in the system, including service accounts, nodes, and controllers,
        /// will have statically granted permissions beyond those provided by the RBAC configuration or IAM.
        /// Defaults to `false`
        pub enable_legacy_abac: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether multi-networking is enabled for this cluster.
        pub enable_multi_networking: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Enable Shielded Nodes features on all nodes in this cluster.  Defaults to `true`.
        pub enable_shielded_nodes: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether to enable Cloud TPU resources in this cluster.
        /// See the [official documentation](https://cloud.google.com/tpu/docs/kubernetes-engine-setup).
        pub enable_tpu: pulumi_gestalt_rust::Output<bool>,
        /// The IP address of this cluster's Kubernetes master.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Configuration for [Enterprise edition].(https://cloud.google.com/kubernetes-engine/enterprise/docs/concepts/gke-editions). Structure is documented below.
        ///
        ///
        /// <a name="nested_default_snat_status"></a>The `default_snat_status` block supports
        pub enterprise_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterEnterpriseConfig,
        >,
        /// Fleet configuration for the cluster. Structure is documented below.
        pub fleet: pulumi_gestalt_rust::Output<
            Option<super::super::types::container::ClusterFleet>,
        >,
        /// Configuration for [GKE Gateway API controller](https://cloud.google.com/kubernetes-engine/docs/concepts/gateway-api). Structure is documented below.
        pub gateway_api_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterGatewayApiConfig,
        >,
        /// . Structure is documented below.
        pub identity_service_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterIdentityServiceConfig,
        >,
        /// The number of nodes to create in this
        /// cluster's default node pool. In regional or multi-zonal clusters, this is the
        /// number of nodes per zone. Must be set if `node_pool` is not set. If you're using
        /// `gcp.container.NodePool` objects with no default node pool, you'll need to
        /// set this to a value of at least `1`, alongside setting
        /// `remove_default_node_pool` to `true`.
        pub initial_node_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Configuration of cluster IP allocation for
        /// VPC-native clusters. If this block is unset during creation, it will be set by the GKE backend.
        /// Structure is documented below.
        pub ip_allocation_policy: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterIpAllocationPolicy,
        >,
        /// The fingerprint of the set of labels for this cluster.
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The location (region or zone) in which the cluster
        /// master will be created, as well as the default node location. If you specify a
        /// zone (such as `us-central1-a`), the cluster will be a zonal cluster with a
        /// single cluster master. If you specify a region (such as `us-west1`), the
        /// cluster will be a regional cluster with multiple masters spread across zones in
        /// the region, and with default node locations in those zones as well
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Logging configuration for the cluster.
        /// Structure is documented below.
        pub logging_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterLoggingConfig,
        >,
        /// The logging service that the cluster should
        /// write logs to. Available options include `logging.googleapis.com`(Legacy Stackdriver),
        /// `logging.googleapis.com/kubernetes`(Stackdriver Kubernetes Engine Logging), and `none`. Defaults to `logging.googleapis.com/kubernetes`
        pub logging_service: pulumi_gestalt_rust::Output<String>,
        /// The maintenance policy to use for the cluster. Structure is
        /// documented below.
        pub maintenance_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::container::ClusterMaintenancePolicy>,
        >,
        /// The authentication information for accessing the
        /// Kubernetes master. Some values in this block are only returned by the API if
        /// your service account has permission to get credentials for your GKE cluster. If
        /// you see an unexpected diff unsetting your client cert, ensure you have the
        /// `container.clusters.getCredentials` permission.
        /// Structure is documented below.
        pub master_auth: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterMasterAuth,
        >,
        /// The desired
        /// configuration options for master authorized networks. Omit the
        /// nested `cidr_blocks` attribute to disallow external access (except
        /// the cluster node IPs, which GKE automatically whitelists).
        /// Structure is documented below.
        pub master_authorized_networks_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterMasterAuthorizedNetworksConfig,
        >,
        /// The current version of the master in the cluster. This may
        /// be different than the `min_master_version` set in the config if the master
        /// has been updated by GKE.
        pub master_version: pulumi_gestalt_rust::Output<String>,
        /// Structure is documented below.
        pub mesh_certificates: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterMeshCertificates,
        >,
        /// The minimum version of the master. GKE
        /// will auto-update the master to new versions, so this does not guarantee the
        /// current master version--use the read-only `master_version` field to obtain that.
        /// If unset, the cluster's version will be set by GKE to the version of the most recent
        /// official release (which is not necessarily the latest version).  Most users will find
        /// the `gcp.container.getEngineVersions` data source useful - it indicates which versions
        /// are available. If you intend to specify versions manually,
        /// [the docs](https://cloud.google.com/kubernetes-engine/versioning-and-upgrades#specifying_cluster_version)
        /// describe the various acceptable formats for this field.
        ///
        /// > If you are using the `gcp.container.getEngineVersions` datasource with a regional cluster, ensure that you have provided a `location`
        /// to the datasource. A region can have a different set of supported versions than its corresponding zones, and not all zones in a
        /// region are guaranteed to support the same version.
        pub min_master_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Monitoring configuration for the cluster.
        /// Structure is documented below.
        pub monitoring_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterMonitoringConfig,
        >,
        /// The monitoring service that the cluster
        /// should write metrics to.
        /// Automatically send metrics from pods in the cluster to the Google Cloud Monitoring API.
        /// VM metrics will be collected by Google Compute Engine regardless of this setting
        /// Available options include
        /// `monitoring.googleapis.com`(Legacy Stackdriver), `monitoring.googleapis.com/kubernetes`(Stackdriver Kubernetes Engine Monitoring), and `none`.
        /// Defaults to `monitoring.googleapis.com/kubernetes`
        pub monitoring_service: pulumi_gestalt_rust::Output<String>,
        /// The name of the cluster, unique within the project and
        /// location.
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name or self_link of the Google Compute Engine
        /// network to which the cluster is connected. For Shared VPC, set this to the self link of the
        /// shared network.
        pub network: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration options for the
        /// [NetworkPolicy](https://kubernetes.io/docs/concepts/services-networking/networkpolicies/)
        /// feature. Structure is documented below.
        pub network_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::container::ClusterNetworkPolicy>,
        >,
        /// Determines whether alias IPs or routes will be used for pod IPs in the cluster.
        /// Options are `VPC_NATIVE` or `ROUTES`. `VPC_NATIVE` enables [IP aliasing](https://cloud.google.com/kubernetes-engine/docs/how-to/ip-aliases). Newly created clusters will default to `VPC_NATIVE`.
        pub networking_mode: pulumi_gestalt_rust::Output<String>,
        /// Parameters used in creating the default node pool.
        /// Generally, this field should not be used at the same time as a
        /// `gcp.container.NodePool` or a `node_pool` block; this configuration
        /// manages the default node pool, which isn't recommended to be used.
        /// Structure is documented below.
        pub node_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterNodeConfig,
        >,
        /// The list of zones in which the cluster's nodes
        /// are located. Nodes must be in the region of their regional cluster or in the
        /// same region as their cluster's zone for zonal clusters. If this is specified for
        /// a zonal cluster, omit the cluster's zone.
        ///
        /// > A "multi-zonal" cluster is a zonal cluster with at least one additional zone
        /// defined; in a multi-zonal cluster, the cluster master is only present in a
        /// single zone while nodes are present in each of the primary zone and the node
        /// locations. In contrast, in a regional cluster, cluster master nodes are present
        /// in multiple zones in the region. For that reason, regional clusters should be
        /// preferred.
        pub node_locations: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Node pool configs that apply to auto-provisioned node pools in
        /// [autopilot](https://cloud.google.com/kubernetes-engine/docs/concepts/autopilot-overview#comparison) clusters and
        /// [node auto-provisioning](https://cloud.google.com/kubernetes-engine/docs/how-to/node-auto-provisioning)-enabled clusters. Structure is documented below.
        pub node_pool_auto_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterNodePoolAutoConfig,
        >,
        /// Default NodePool settings for the entire cluster. These settings are overridden if specified on the specific NodePool object. Structure is documented below.
        pub node_pool_defaults: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterNodePoolDefaults,
        >,
        /// List of node pools associated with this cluster.
        /// See gcp.container.NodePool for schema.
        /// **Warning:** node pools defined inside a cluster can't be changed (or added/removed) after
        /// cluster creation without deleting and recreating the entire cluster. Unless you absolutely need the ability
        /// to say "these are the _only_ node pools associated with this cluster", use the
        /// gcp.container.NodePool resource instead of this property.
        pub node_pools: pulumi_gestalt_rust::Output<
            Vec<super::super::types::container::ClusterNodePool>,
        >,
        /// The Kubernetes version on the nodes. Must either be unset
        /// or set to the same value as `min_master_version` on create. Defaults to the default
        /// version set by GKE which is not necessarily the latest version. This only affects
        /// nodes in the default node pool. While a fuzzy version can be specified, it's
        /// recommended that you specify explicit versions as the provider will see spurious diffs
        /// when fuzzy versions are used. See the `gcp.container.getEngineVersions` data source's
        /// `version_prefix` field to approximate fuzzy versions.
        /// To update nodes in other node pools, use the `version` attribute on the node pool.
        pub node_version: pulumi_gestalt_rust::Output<String>,
        /// Configuration for the [cluster upgrade notifications](https://cloud.google.com/kubernetes-engine/docs/how-to/cluster-upgrade-notifications) feature. Structure is documented below.
        pub notification_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterNotificationConfig,
        >,
        pub operation: pulumi_gestalt_rust::Output<String>,
        /// Configuration for the
        /// [PodSecurityPolicy](https://cloud.google.com/kubernetes-engine/docs/how-to/pod-security-policies) feature.
        /// Structure is documented below.
        pub pod_security_policy_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::container::ClusterPodSecurityPolicyConfig>,
        >,
        /// Configuration for [private clusters](https://cloud.google.com/kubernetes-engine/docs/how-to/private-clusters),
        /// clusters with private nodes. Structure is documented below.
        pub private_cluster_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterPrivateClusterConfig,
        >,
        /// The desired state of IPv6 connectivity to Google Services. By default, no private IPv6 access to or from Google Services (all access will be via IPv4).
        pub private_ipv6_google_access: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Enable/Disable Protect API features for the cluster. Structure is documented below.
        pub protect_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterProtectConfig,
        >,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration options for the [Release channel](https://cloud.google.com/kubernetes-engine/docs/concepts/release-channels)
        /// feature, which provide more control over automatic upgrades of your GKE clusters.
        /// When updating this field, GKE imposes specific version requirements. See
        /// [Selecting a new release channel](https://cloud.google.com/kubernetes-engine/docs/concepts/release-channels#selecting_a_new_release_channel)
        /// for more details; the `gcp.container.getEngineVersions` datasource can provide
        /// the default version for a channel. Note that removing the `release_channel`
        /// field from your config will cause the provider to stop managing your cluster's
        /// release channel, but will not unenroll it. Instead, use the `"UNSPECIFIED"`
        /// channel. Structure is documented below.
        pub release_channel: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterReleaseChannel,
        >,
        /// If `true`, deletes the default node
        /// pool upon cluster creation. If you're using `gcp.container.NodePool`
        /// resources with no default node pool, this should be set to `true`, alongside
        /// setting `initial_node_count` to at least `1`.
        pub remove_default_node_pool: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The GCE resource labels (a map of key/value pairs) to be applied to the cluster.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub resource_labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration for the
        /// [ResourceUsageExportConfig](https://cloud.google.com/kubernetes-engine/docs/how-to/cluster-usage-metering) feature.
        /// Structure is documented below.
        pub resource_usage_export_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::container::ClusterResourceUsageExportConfig>,
        >,
        /// Configuration for the
        /// [SecretManagerConfig](https://cloud.google.com/secret-manager/docs/secret-manager-managed-csi-component) feature.
        /// Structure is documented below.
        pub secret_manager_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::container::ClusterSecretManagerConfig>,
        >,
        /// Enable/Disable Security Posture API features for the cluster. Structure is documented below.
        pub security_posture_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterSecurityPostureConfig,
        >,
        /// The server-defined URL for the resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Structure is documented below.
        pub service_external_ips_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterServiceExternalIpsConfig,
        >,
        /// The IP address range of the Kubernetes services in this
        /// cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing)
        /// notation (e.g. `1.2.3.4/29`). Service addresses are typically put in the last
        /// `/16` from the container CIDR.
        pub services_ipv4_cidr: pulumi_gestalt_rust::Output<String>,
        /// The name or self_link of the Google Compute Engine
        /// subnetwork in which the cluster's instances are launched.
        pub subnetwork: pulumi_gestalt_rust::Output<String>,
        /// TPU configuration for the cluster.
        pub tpu_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterTpuConfig,
        >,
        /// The IP address range of the Cloud TPUs in this cluster, in
        /// [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing)
        /// notation (e.g. `1.2.3.4/29`).
        pub tpu_ipv4_cidr_block: pulumi_gestalt_rust::Output<String>,
        /// The custom keys configuration of the cluster.
        pub user_managed_keys_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::container::ClusterUserManagedKeysConfig>,
        >,
        /// Vertical Pod Autoscaling automatically adjusts the resources of pods controlled by it.
        /// Structure is documented below.
        pub vertical_pod_autoscaling: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterVerticalPodAutoscaling,
        >,
        /// Configuration for [direct-path (via ALTS) with workload identity.](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/projects.locations.clusters#workloadaltsconfig). Structure is documented below.
        pub workload_alts_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterWorkloadAltsConfig,
        >,
        /// Workload Identity allows Kubernetes service accounts to act as a user-managed
        /// [Google IAM Service Account](https://cloud.google.com/iam/docs/service-accounts#user-managed_service_accounts).
        /// Structure is documented below.
        pub workload_identity_config: pulumi_gestalt_rust::Output<
            super::super::types::container::ClusterWorkloadIdentityConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let addons_config_binding = args.addons_config.get_output(context);
        let allow_net_admin_binding = args.allow_net_admin.get_output(context);
        let authenticator_groups_config_binding = args
            .authenticator_groups_config
            .get_output(context);
        let binary_authorization_binding = args.binary_authorization.get_output(context);
        let cluster_autoscaling_binding = args.cluster_autoscaling.get_output(context);
        let cluster_ipv4_cidr_binding = args.cluster_ipv4_cidr.get_output(context);
        let cluster_telemetry_binding = args.cluster_telemetry.get_output(context);
        let confidential_nodes_binding = args.confidential_nodes.get_output(context);
        let control_plane_endpoints_config_binding = args
            .control_plane_endpoints_config
            .get_output(context);
        let cost_management_config_binding = args
            .cost_management_config
            .get_output(context);
        let database_encryption_binding = args.database_encryption.get_output(context);
        let datapath_provider_binding = args.datapath_provider.get_output(context);
        let default_max_pods_per_node_binding = args
            .default_max_pods_per_node
            .get_output(context);
        let default_snat_status_binding = args.default_snat_status.get_output(context);
        let deletion_protection_binding = args.deletion_protection.get_output(context);
        let description_binding = args.description.get_output(context);
        let dns_config_binding = args.dns_config.get_output(context);
        let enable_autopilot_binding = args.enable_autopilot.get_output(context);
        let enable_cilium_clusterwide_network_policy_binding = args
            .enable_cilium_clusterwide_network_policy
            .get_output(context);
        let enable_fqdn_network_policy_binding = args
            .enable_fqdn_network_policy
            .get_output(context);
        let enable_intranode_visibility_binding = args
            .enable_intranode_visibility
            .get_output(context);
        let enable_k8s_beta_apis_binding = args.enable_k8s_beta_apis.get_output(context);
        let enable_kubernetes_alpha_binding = args
            .enable_kubernetes_alpha
            .get_output(context);
        let enable_l4_ilb_subsetting_binding = args
            .enable_l4_ilb_subsetting
            .get_output(context);
        let enable_legacy_abac_binding = args.enable_legacy_abac.get_output(context);
        let enable_multi_networking_binding = args
            .enable_multi_networking
            .get_output(context);
        let enable_shielded_nodes_binding = args
            .enable_shielded_nodes
            .get_output(context);
        let enable_tpu_binding = args.enable_tpu.get_output(context);
        let enterprise_config_binding = args.enterprise_config.get_output(context);
        let fleet_binding = args.fleet.get_output(context);
        let gateway_api_config_binding = args.gateway_api_config.get_output(context);
        let identity_service_config_binding = args
            .identity_service_config
            .get_output(context);
        let initial_node_count_binding = args.initial_node_count.get_output(context);
        let ip_allocation_policy_binding = args.ip_allocation_policy.get_output(context);
        let location_binding = args.location.get_output(context);
        let logging_config_binding = args.logging_config.get_output(context);
        let logging_service_binding = args.logging_service.get_output(context);
        let maintenance_policy_binding = args.maintenance_policy.get_output(context);
        let master_auth_binding = args.master_auth.get_output(context);
        let master_authorized_networks_config_binding = args
            .master_authorized_networks_config
            .get_output(context);
        let mesh_certificates_binding = args.mesh_certificates.get_output(context);
        let min_master_version_binding = args.min_master_version.get_output(context);
        let monitoring_config_binding = args.monitoring_config.get_output(context);
        let monitoring_service_binding = args.monitoring_service.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let network_policy_binding = args.network_policy.get_output(context);
        let networking_mode_binding = args.networking_mode.get_output(context);
        let node_config_binding = args.node_config.get_output(context);
        let node_locations_binding = args.node_locations.get_output(context);
        let node_pool_auto_config_binding = args
            .node_pool_auto_config
            .get_output(context);
        let node_pool_defaults_binding = args.node_pool_defaults.get_output(context);
        let node_pools_binding = args.node_pools.get_output(context);
        let node_version_binding = args.node_version.get_output(context);
        let notification_config_binding = args.notification_config.get_output(context);
        let pod_security_policy_config_binding = args
            .pod_security_policy_config
            .get_output(context);
        let private_cluster_config_binding = args
            .private_cluster_config
            .get_output(context);
        let private_ipv6_google_access_binding = args
            .private_ipv6_google_access
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let protect_config_binding = args.protect_config.get_output(context);
        let release_channel_binding = args.release_channel.get_output(context);
        let remove_default_node_pool_binding = args
            .remove_default_node_pool
            .get_output(context);
        let resource_labels_binding = args.resource_labels.get_output(context);
        let resource_usage_export_config_binding = args
            .resource_usage_export_config
            .get_output(context);
        let secret_manager_config_binding = args
            .secret_manager_config
            .get_output(context);
        let security_posture_config_binding = args
            .security_posture_config
            .get_output(context);
        let service_external_ips_config_binding = args
            .service_external_ips_config
            .get_output(context);
        let subnetwork_binding = args.subnetwork.get_output(context);
        let tpu_config_binding = args.tpu_config.get_output(context);
        let user_managed_keys_config_binding = args
            .user_managed_keys_config
            .get_output(context);
        let vertical_pod_autoscaling_binding = args
            .vertical_pod_autoscaling
            .get_output(context);
        let workload_alts_config_binding = args.workload_alts_config.get_output(context);
        let workload_identity_config_binding = args
            .workload_identity_config
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:container/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addonsConfig".into(),
                    value: &addons_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowNetAdmin".into(),
                    value: &allow_net_admin_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticatorGroupsConfig".into(),
                    value: &authenticator_groups_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "binaryAuthorization".into(),
                    value: &binary_authorization_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterAutoscaling".into(),
                    value: &cluster_autoscaling_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIpv4Cidr".into(),
                    value: &cluster_ipv4_cidr_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterTelemetry".into(),
                    value: &cluster_telemetry_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "confidentialNodes".into(),
                    value: &confidential_nodes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "controlPlaneEndpointsConfig".into(),
                    value: &control_plane_endpoints_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "costManagementConfig".into(),
                    value: &cost_management_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseEncryption".into(),
                    value: &database_encryption_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "datapathProvider".into(),
                    value: &datapath_provider_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultMaxPodsPerNode".into(),
                    value: &default_max_pods_per_node_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultSnatStatus".into(),
                    value: &default_snat_status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsConfig".into(),
                    value: &dns_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableAutopilot".into(),
                    value: &enable_autopilot_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableCiliumClusterwideNetworkPolicy".into(),
                    value: &enable_cilium_clusterwide_network_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableFqdnNetworkPolicy".into(),
                    value: &enable_fqdn_network_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableIntranodeVisibility".into(),
                    value: &enable_intranode_visibility_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableK8sBetaApis".into(),
                    value: &enable_k8s_beta_apis_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableKubernetesAlpha".into(),
                    value: &enable_kubernetes_alpha_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableL4IlbSubsetting".into(),
                    value: &enable_l4_ilb_subsetting_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableLegacyAbac".into(),
                    value: &enable_legacy_abac_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableMultiNetworking".into(),
                    value: &enable_multi_networking_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableShieldedNodes".into(),
                    value: &enable_shielded_nodes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableTpu".into(),
                    value: &enable_tpu_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enterpriseConfig".into(),
                    value: &enterprise_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fleet".into(),
                    value: &fleet_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayApiConfig".into(),
                    value: &gateway_api_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityServiceConfig".into(),
                    value: &identity_service_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "initialNodeCount".into(),
                    value: &initial_node_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipAllocationPolicy".into(),
                    value: &ip_allocation_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loggingConfig".into(),
                    value: &logging_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loggingService".into(),
                    value: &logging_service_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenancePolicy".into(),
                    value: &maintenance_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "masterAuth".into(),
                    value: &master_auth_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "masterAuthorizedNetworksConfig".into(),
                    value: &master_authorized_networks_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "meshCertificates".into(),
                    value: &mesh_certificates_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minMasterVersion".into(),
                    value: &min_master_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monitoringConfig".into(),
                    value: &monitoring_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monitoringService".into(),
                    value: &monitoring_service_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: &network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkPolicy".into(),
                    value: &network_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkingMode".into(),
                    value: &networking_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeConfig".into(),
                    value: &node_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeLocations".into(),
                    value: &node_locations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodePoolAutoConfig".into(),
                    value: &node_pool_auto_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodePoolDefaults".into(),
                    value: &node_pool_defaults_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodePools".into(),
                    value: &node_pools_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeVersion".into(),
                    value: &node_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationConfig".into(),
                    value: &notification_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "podSecurityPolicyConfig".into(),
                    value: &pod_security_policy_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateClusterConfig".into(),
                    value: &private_cluster_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateIpv6GoogleAccess".into(),
                    value: &private_ipv6_google_access_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protectConfig".into(),
                    value: &protect_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "releaseChannel".into(),
                    value: &release_channel_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "removeDefaultNodePool".into(),
                    value: &remove_default_node_pool_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceLabels".into(),
                    value: &resource_labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceUsageExportConfig".into(),
                    value: &resource_usage_export_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretManagerConfig".into(),
                    value: &secret_manager_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityPostureConfig".into(),
                    value: &security_posture_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceExternalIpsConfig".into(),
                    value: &service_external_ips_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetwork".into(),
                    value: &subnetwork_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tpuConfig".into(),
                    value: &tpu_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userManagedKeysConfig".into(),
                    value: &user_managed_keys_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "verticalPodAutoscaling".into(),
                    value: &vertical_pod_autoscaling_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workloadAltsConfig".into(),
                    value: &workload_alts_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workloadIdentityConfig".into(),
                    value: &workload_identity_config_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterResult {
            addons_config: o.get_field("addonsConfig"),
            allow_net_admin: o.get_field("allowNetAdmin"),
            authenticator_groups_config: o.get_field("authenticatorGroupsConfig"),
            binary_authorization: o.get_field("binaryAuthorization"),
            cluster_autoscaling: o.get_field("clusterAutoscaling"),
            cluster_ipv4_cidr: o.get_field("clusterIpv4Cidr"),
            cluster_telemetry: o.get_field("clusterTelemetry"),
            confidential_nodes: o.get_field("confidentialNodes"),
            control_plane_endpoints_config: o.get_field("controlPlaneEndpointsConfig"),
            cost_management_config: o.get_field("costManagementConfig"),
            database_encryption: o.get_field("databaseEncryption"),
            datapath_provider: o.get_field("datapathProvider"),
            default_max_pods_per_node: o.get_field("defaultMaxPodsPerNode"),
            default_snat_status: o.get_field("defaultSnatStatus"),
            deletion_protection: o.get_field("deletionProtection"),
            description: o.get_field("description"),
            dns_config: o.get_field("dnsConfig"),
            effective_labels: o.get_field("effectiveLabels"),
            enable_autopilot: o.get_field("enableAutopilot"),
            enable_cilium_clusterwide_network_policy: o
                .get_field("enableCiliumClusterwideNetworkPolicy"),
            enable_fqdn_network_policy: o.get_field("enableFqdnNetworkPolicy"),
            enable_intranode_visibility: o.get_field("enableIntranodeVisibility"),
            enable_k8s_beta_apis: o.get_field("enableK8sBetaApis"),
            enable_kubernetes_alpha: o.get_field("enableKubernetesAlpha"),
            enable_l4_ilb_subsetting: o.get_field("enableL4IlbSubsetting"),
            enable_legacy_abac: o.get_field("enableLegacyAbac"),
            enable_multi_networking: o.get_field("enableMultiNetworking"),
            enable_shielded_nodes: o.get_field("enableShieldedNodes"),
            enable_tpu: o.get_field("enableTpu"),
            endpoint: o.get_field("endpoint"),
            enterprise_config: o.get_field("enterpriseConfig"),
            fleet: o.get_field("fleet"),
            gateway_api_config: o.get_field("gatewayApiConfig"),
            identity_service_config: o.get_field("identityServiceConfig"),
            initial_node_count: o.get_field("initialNodeCount"),
            ip_allocation_policy: o.get_field("ipAllocationPolicy"),
            label_fingerprint: o.get_field("labelFingerprint"),
            location: o.get_field("location"),
            logging_config: o.get_field("loggingConfig"),
            logging_service: o.get_field("loggingService"),
            maintenance_policy: o.get_field("maintenancePolicy"),
            master_auth: o.get_field("masterAuth"),
            master_authorized_networks_config: o
                .get_field("masterAuthorizedNetworksConfig"),
            master_version: o.get_field("masterVersion"),
            mesh_certificates: o.get_field("meshCertificates"),
            min_master_version: o.get_field("minMasterVersion"),
            monitoring_config: o.get_field("monitoringConfig"),
            monitoring_service: o.get_field("monitoringService"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            network_policy: o.get_field("networkPolicy"),
            networking_mode: o.get_field("networkingMode"),
            node_config: o.get_field("nodeConfig"),
            node_locations: o.get_field("nodeLocations"),
            node_pool_auto_config: o.get_field("nodePoolAutoConfig"),
            node_pool_defaults: o.get_field("nodePoolDefaults"),
            node_pools: o.get_field("nodePools"),
            node_version: o.get_field("nodeVersion"),
            notification_config: o.get_field("notificationConfig"),
            operation: o.get_field("operation"),
            pod_security_policy_config: o.get_field("podSecurityPolicyConfig"),
            private_cluster_config: o.get_field("privateClusterConfig"),
            private_ipv6_google_access: o.get_field("privateIpv6GoogleAccess"),
            project: o.get_field("project"),
            protect_config: o.get_field("protectConfig"),
            pulumi_labels: o.get_field("pulumiLabels"),
            release_channel: o.get_field("releaseChannel"),
            remove_default_node_pool: o.get_field("removeDefaultNodePool"),
            resource_labels: o.get_field("resourceLabels"),
            resource_usage_export_config: o.get_field("resourceUsageExportConfig"),
            secret_manager_config: o.get_field("secretManagerConfig"),
            security_posture_config: o.get_field("securityPostureConfig"),
            self_link: o.get_field("selfLink"),
            service_external_ips_config: o.get_field("serviceExternalIpsConfig"),
            services_ipv4_cidr: o.get_field("servicesIpv4Cidr"),
            subnetwork: o.get_field("subnetwork"),
            tpu_config: o.get_field("tpuConfig"),
            tpu_ipv4_cidr_block: o.get_field("tpuIpv4CidrBlock"),
            user_managed_keys_config: o.get_field("userManagedKeysConfig"),
            vertical_pod_autoscaling: o.get_field("verticalPodAutoscaling"),
            workload_alts_config: o.get_field("workloadAltsConfig"),
            workload_identity_config: o.get_field("workloadIdentityConfig"),
        }
    }
}
