/// A Managed Service for Apache Kafka cluster. Apache Kafka is a trademark owned by the Apache Software Foundation.
///
///
///
/// ## Example Usage
///
/// ### Managedkafka Cluster Basic
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:managedkafka:Cluster
///     properties:
///       clusterId: my-cluster
///       location: us-central1
///       capacityConfig:
///         vcpuCount: 3
///         memoryBytes: 3.221225472e+09
///       gcpConfig:
///         accessConfig:
///           networkConfigs:
///             - subnet: projects/${project.number}/regions/us-central1/subnetworks/default
///       rebalanceConfig:
///         mode: NO_REBALANCE
///       labels:
///         key: value
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Managedkafka Cluster Cmek
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:managedkafka:Cluster
///     properties:
///       clusterId: my-cluster
///       location: us-central1
///       capacityConfig:
///         vcpuCount: 3
///         memoryBytes: 3.221225472e+09
///       gcpConfig:
///         accessConfig:
///           networkConfigs:
///             - subnet: projects/${project.number}/regions/us-central1/subnetworks/default
///         kmsKey: ${key.id}
///   kafkaServiceIdentity:
///     type: gcp:projects:ServiceIdentity
///     name: kafka_service_identity
///     properties:
///       project: ${project.projectId}
///       service: managedkafka.googleapis.com
///   key:
///     type: gcp:kms:CryptoKey
///     properties:
///       name: example-key
///       keyRing: ${keyRing.id}
///   keyRing:
///     type: gcp:kms:KeyRing
///     name: key_ring
///     properties:
///       name: example-key-ring
///       location: us-central1
///   cryptoKeyBinding:
///     type: gcp:kms:CryptoKeyIAMBinding
///     name: crypto_key_binding
///     properties:
///       cryptoKeyId: ${key.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       members:
///         - serviceAccount:service-${project.number}@gcp-sa-managedkafka.iam.gserviceaccount.com
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Cluster can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/clusters/{{cluster_id}}`
///
/// * `{{project}}/{{location}}/{{cluster_id}}`
///
/// * `{{location}}/{{cluster_id}}`
///
/// When using the `pulumi import` command, Cluster can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:managedkafka/cluster:Cluster default projects/{{project}}/locations/{{location}}/clusters/{{cluster_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:managedkafka/cluster:Cluster default {{project}}/{{location}}/{{cluster_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:managedkafka/cluster:Cluster default {{location}}/{{cluster_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// A capacity configuration of a Kafka cluster.
        /// Structure is documented below.
        #[builder(into)]
        pub capacity_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::managedkafka::ClusterCapacityConfig,
        >,
        /// The ID to use for the cluster, which will become the final component of the cluster's name. The ID must be 1-63 characters long, and match the regular expression `a-z?` to comply with RFC 1035. This value is structured like: `my-cluster-id`.
        #[builder(into)]
        pub cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration properties for a Kafka cluster deployed to Google Cloud Platform.
        /// Structure is documented below.
        #[builder(into)]
        pub gcp_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::managedkafka::ClusterGcpConfig,
        >,
        /// List of label KEY=VALUE pairs to add. Keys must start with a lowercase character and contain only hyphens (-),
        /// underscores ( ), lowercase characters, and numbers. Values must contain only hyphens (-), underscores ( ), lowercase
        /// characters, and numbers. **Note**: This field is non-authoritative, and will only manage the labels present in your
        /// configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the location of the Kafka resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Defines rebalancing behavior of a Kafka cluster.
        #[builder(into, default)]
        pub rebalance_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::managedkafka::ClusterRebalanceConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// A capacity configuration of a Kafka cluster.
        /// Structure is documented below.
        pub capacity_config: pulumi_gestalt_rust::Output<
            super::super::types::managedkafka::ClusterCapacityConfig,
        >,
        /// The ID to use for the cluster, which will become the final component of the cluster's name. The ID must be 1-63 characters long, and match the regular expression `a-z?` to comply with RFC 1035. This value is structured like: `my-cluster-id`.
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The time when the cluster was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration properties for a Kafka cluster deployed to Google Cloud Platform.
        /// Structure is documented below.
        pub gcp_config: pulumi_gestalt_rust::Output<
            super::super::types::managedkafka::ClusterGcpConfig,
        >,
        /// List of label KEY=VALUE pairs to add. Keys must start with a lowercase character and contain only hyphens (-),
        /// underscores ( ), lowercase characters, and numbers. Values must contain only hyphens (-), underscores ( ), lowercase
        /// characters, and numbers. **Note**: This field is non-authoritative, and will only manage the labels present in your
        /// configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the location of the Kafka resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the cluster. Structured like: `projects/PROJECT_ID/locations/LOCATION/clusters/CLUSTER_ID`.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Defines rebalancing behavior of a Kafka cluster.
        pub rebalance_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::managedkafka::ClusterRebalanceConfig>,
        >,
        /// The current state of the cluster. Possible values: `STATE_UNSPECIFIED`, `CREATING`, `ACTIVE`, `DELETING`.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The time when the cluster was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
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
        let capacity_config_binding = args.capacity_config.get_output(context);
        let cluster_id_binding = args.cluster_id.get_output(context);
        let gcp_config_binding = args.gcp_config.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let rebalance_config_binding = args.rebalance_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:managedkafka/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacityConfig".into(),
                    value: &capacity_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gcpConfig".into(),
                    value: &gcp_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rebalanceConfig".into(),
                    value: &rebalance_config_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterResult {
            capacity_config: o.get_field("capacityConfig"),
            cluster_id: o.get_field("clusterId"),
            create_time: o.get_field("createTime"),
            effective_labels: o.get_field("effectiveLabels"),
            gcp_config: o.get_field("gcpConfig"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            rebalance_config: o.get_field("rebalanceConfig"),
            state: o.get_field("state"),
            update_time: o.get_field("updateTime"),
        }
    }
}
