/// Manages a Cloud Dataproc cluster resource within GCP.
///
/// * [API documentation](https://cloud.google.com/dataproc/docs/reference/rest/v1/projects.regions.clusters)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dataproc/docs)
///
///
/// !> **Warning:** Due to limitations of the API, all arguments except
/// `labels`,`cluster_config.worker_config.num_instances` and `cluster_config.preemptible_worker_config.num_instances` are non-updatable. Changing `cluster_config.worker_config.min_num_instances` will be ignored. Changing others will cause recreation of the
/// whole cluster!
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let simplecluster = cluster::create(
///         "simplecluster",
///         ClusterArgs::builder().name("simplecluster").region("us-central1").build_struct(),
///     );
/// }
/// ```
///
/// ### Advanced
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: service-account-id
///       displayName: Service Account
///   mycluster:
///     type: gcp:dataproc:Cluster
///     properties:
///       name: mycluster
///       region: us-central1
///       gracefulDecommissionTimeout: 120s
///       labels:
///         foo: bar
///       clusterConfig:
///         stagingBucket: dataproc-staging-bucket
///         masterConfig:
///           numInstances: 1
///           machineType: e2-medium
///           diskConfig:
///             bootDiskType: pd-ssd
///             bootDiskSizeGb: 30
///         workerConfig:
///           numInstances: 2
///           machineType: e2-medium
///           minCpuPlatform: Intel Skylake
///           diskConfig:
///             bootDiskSizeGb: 30
///             numLocalSsds: 1
///         preemptibleWorkerConfig:
///           numInstances: 0
///         softwareConfig:
///           imageVersion: 2.0.35-debian10
///           overrideProperties:
///             dataproc:dataproc.allow.zero.workers: 'true'
///         gceClusterConfig:
///           tags:
///             - foo
///             - bar
///           serviceAccount: ${default.email}
///           serviceAccountScopes:
///             - cloud-platform
///         initializationActions:
///           - script: gs://dataproc-initialization-actions/stackdriver/stackdriver.sh
///             timeoutSec: 500
/// ```
///
/// ### Using A GPU Accelerator
///
/// ```yaml
/// resources:
///   acceleratedCluster:
///     type: gcp:dataproc:Cluster
///     name: accelerated_cluster
///     properties:
///       name: my-cluster-with-gpu
///       region: us-central1
///       clusterConfig:
///         gceClusterConfig:
///           zone: us-central1-a
///         masterConfig:
///           accelerators:
///             - acceleratorType: nvidia-tesla-k80
///               acceleratorCount: '1'
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// Allows you to configure various aspects of the cluster.
        /// Structure defined below.
        #[builder(into, default)]
        pub cluster_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::ClusterClusterConfig>,
        >,
        #[builder(into, default)]
        pub graceful_decommission_timeout: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The list of the labels (key/value pairs) configured on the resource and to be applied to instances in the cluster.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration. Please refer
        /// to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the cluster, unique within the project and
        /// zone.
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the `cluster` will exist. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region in which the cluster and associated nodes will be created in.
        /// Defaults to `global`.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Allows you to configure a virtual Dataproc on GKE cluster.
        /// Structure defined below.
        #[builder(into, default)]
        pub virtual_cluster_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::ClusterVirtualClusterConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// Allows you to configure various aspects of the cluster.
        /// Structure defined below.
        pub cluster_config: pulumi_gestalt_rust::Output<
            super::super::types::dataproc::ClusterClusterConfig,
        >,
        /// The list of labels (key/value pairs) to be applied to
        /// instances in the cluster. GCP generates some itself including `goog-dataproc-cluster-name`
        /// which is the name of the cluster.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub graceful_decommission_timeout: pulumi_gestalt_rust::Output<Option<String>>,
        /// The list of the labels (key/value pairs) configured on the resource and to be applied to instances in the cluster.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration. Please refer
        /// to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the cluster, unique within the project and
        /// zone.
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the `cluster` will exist. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region in which the cluster and associated nodes will be created in.
        /// Defaults to `global`.
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// Allows you to configure a virtual Dataproc on GKE cluster.
        /// Structure defined below.
        pub virtual_cluster_config: pulumi_gestalt_rust::Output<
            super::super::types::dataproc::ClusterVirtualClusterConfig,
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
        let cluster_config_binding = args.cluster_config.get_output(context);
        let graceful_decommission_timeout_binding = args
            .graceful_decommission_timeout
            .get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let virtual_cluster_config_binding = args
            .virtual_cluster_config
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataproc/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterConfig".into(),
                    value: &cluster_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gracefulDecommissionTimeout".into(),
                    value: &graceful_decommission_timeout_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualClusterConfig".into(),
                    value: &virtual_cluster_config_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterResult {
            cluster_config: o.get_field("clusterConfig"),
            effective_labels: o.get_field("effectiveLabels"),
            graceful_decommission_timeout: o.get_field("gracefulDecommissionTimeout"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            virtual_cluster_config: o.get_field("virtualClusterConfig"),
        }
    }
}
