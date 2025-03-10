/// An AlloyDB Backup.
///
///
/// To get more information about Backup, see:
///
/// * [API documentation](https://cloud.google.com/alloydb/docs/reference/rest/v1/projects.locations.backups/create)
/// * How-to Guides
///     * [AlloyDB](https://cloud.google.com/alloydb/docs/)
///
/// ## Example Usage
///
/// ### Alloydb Backup Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = backup::create(
///         "default",
///         BackupArgs::builder()
///             .backup_id("alloydb-backup")
///             .cluster_name("${defaultCluster.name}")
///             .location("us-central1")
///             .build_struct(),
///     );
///     let defaultCluster = cluster::create(
///         "defaultCluster",
///         ClusterArgs::builder()
///             .cluster_id("alloydb-cluster")
///             .location("us-central1")
///             .network_config(
///                 ClusterNetworkConfig::builder()
///                     .network("${defaultNetwork.id}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let defaultInstance = instance::create(
///         "defaultInstance",
///         InstanceArgs::builder()
///             .cluster("${defaultCluster.name}")
///             .instance_id("alloydb-instance")
///             .instance_type("PRIMARY")
///             .build_struct(),
///     );
///     let defaultNetwork = network::create(
///         "defaultNetwork",
///         NetworkArgs::builder().name("alloydb-network").build_struct(),
///     );
///     let privateIpAlloc = global_address::create(
///         "privateIpAlloc",
///         GlobalAddressArgs::builder()
///             .address_type("INTERNAL")
///             .name("alloydb-cluster")
///             .network("${defaultNetwork.id}")
///             .prefix_length(16)
///             .purpose("VPC_PEERING")
///             .build_struct(),
///     );
///     let vpcConnection = connection::create(
///         "vpcConnection",
///         ConnectionArgs::builder()
///             .network("${defaultNetwork.id}")
///             .reserved_peering_ranges(vec!["${privateIpAlloc.name}",])
///             .service("servicenetworking.googleapis.com")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Alloydb Backup Full
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:alloydb:Backup
///     properties:
///       location: us-central1
///       backupId: alloydb-backup
///       clusterName: ${defaultCluster.name}
///       description: example description
///       type: ON_DEMAND
///       labels:
///         label: key
///     options:
///       dependsOn:
///         - ${defaultInstance}
///   defaultCluster:
///     type: gcp:alloydb:Cluster
///     name: default
///     properties:
///       clusterId: alloydb-cluster
///       location: us-central1
///       networkConfig:
///         network: ${defaultNetwork.id}
///   defaultInstance:
///     type: gcp:alloydb:Instance
///     name: default
///     properties:
///       cluster: ${defaultCluster.name}
///       instanceId: alloydb-instance
///       instanceType: PRIMARY
///     options:
///       dependsOn:
///         - ${vpcConnection}
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: alloydb-cluster
///       addressType: INTERNAL
///       purpose: VPC_PEERING
///       prefixLength: 16
///       network: ${defaultNetwork.id}
///   vpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: vpc_connection
///     properties:
///       network: ${defaultNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAlloc.name}
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: alloydb-network
/// ```
///
/// ## Import
///
/// Backup can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/backups/{{backup_id}}`
///
/// * `{{project}}/{{location}}/{{backup_id}}`
///
/// * `{{location}}/{{backup_id}}`
///
/// When using the `pulumi import` command, Backup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:alloydb/backup:Backup default projects/{{project}}/locations/{{location}}/backups/{{backup_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:alloydb/backup:Backup default {{project}}/{{location}}/{{backup_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:alloydb/backup:Backup default {{location}}/{{backup_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backup {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupArgs {
        /// Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the alloydb backup.
        #[builder(into)]
        pub backup_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The full resource name of the backup source cluster (e.g., projects/{project}/locations/{location}/clusters/{clusterId}).
        #[builder(into)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// User-provided description of the backup.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User-settable and human-readable display name for the Backup.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// EncryptionConfig describes the encryption config of a cluster or a backup that is encrypted with a CMEK (customer-managed encryption key).
        /// Structure is documented below.
        #[builder(into, default)]
        pub encryption_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::BackupEncryptionConfig>,
        >,
        /// User-defined labels for the alloydb backup. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the alloydb backup should reside.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The backup type, which suggests the trigger for the backup.
        /// Possible values are: `TYPE_UNSPECIFIED`, `ON_DEMAND`, `AUTOMATED`, `CONTINUOUS`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BackupResult {
        /// Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the alloydb backup.
        pub backup_id: pulumi_gestalt_rust::Output<String>,
        /// The full resource name of the backup source cluster (e.g., projects/{project}/locations/{location}/clusters/{clusterId}).
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// Output only. The system-generated UID of the cluster which was used to create this resource.
        pub cluster_uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. Create time stamp. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Output only. Delete time stamp. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        /// User-provided description of the backup.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// User-settable and human-readable display name for the Backup.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// EncryptionConfig describes the encryption config of a cluster or a backup that is encrypted with a CMEK (customer-managed encryption key).
        /// Structure is documented below.
        pub encryption_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::alloydb::BackupEncryptionConfig>,
        >,
        /// EncryptionInfo describes the encryption information of a cluster or a backup.
        /// Structure is documented below.
        pub encryption_infos: pulumi_gestalt_rust::Output<
            Vec<super::super::types::alloydb::BackupEncryptionInfo>,
        >,
        /// For Resource freshness validation (https://google.aip.dev/154)
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Output only. The QuantityBasedExpiry of the backup, specified by the backup's retention policy.
        /// Once the expiry quantity is over retention, the backup is eligible to be garbage collected.
        /// Structure is documented below.
        pub expiry_quantities: pulumi_gestalt_rust::Output<
            Vec<super::super::types::alloydb::BackupExpiryQuantity>,
        >,
        /// Output only. The time at which after the backup is eligible to be garbage collected.
        /// It is the duration specified by the backup's retention policy, added to the backup's createTime.
        pub expiry_time: pulumi_gestalt_rust::Output<String>,
        /// User-defined labels for the alloydb backup. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the alloydb backup should reside.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Output only. The name of the backup resource with the format: * projects/{project}/locations/{region}/backups/{backupId}
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. Reconciling (https://google.aip.dev/128#reconciliation), if true, indicates that the service is actively updating the resource.
        /// This can happen due to user-triggered updates or system actions like failover or maintenance.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// Output only. The size of the backup in bytes.
        pub size_bytes: pulumi_gestalt_rust::Output<String>,
        /// Output only. The current state of the backup.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The backup type, which suggests the trigger for the backup.
        /// Possible values are: `TYPE_UNSPECIFIED`, `ON_DEMAND`, `AUTOMATED`, `CONTINUOUS`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. Update time stamp. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupArgs,
    ) -> BackupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let backup_id_binding = args.backup_id.get_output(context);
        let cluster_name_binding = args.cluster_name.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let encryption_config_binding = args.encryption_config.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:alloydb/backup:Backup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupId".into(),
                    value: &backup_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionConfig".into(),
                    value: &encryption_config_binding.drop_type(),
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
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BackupResult {
            annotations: o.get_field("annotations"),
            backup_id: o.get_field("backupId"),
            cluster_name: o.get_field("clusterName"),
            cluster_uid: o.get_field("clusterUid"),
            create_time: o.get_field("createTime"),
            delete_time: o.get_field("deleteTime"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            encryption_config: o.get_field("encryptionConfig"),
            encryption_infos: o.get_field("encryptionInfos"),
            etag: o.get_field("etag"),
            expiry_quantities: o.get_field("expiryQuantities"),
            expiry_time: o.get_field("expiryTime"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reconciling: o.get_field("reconciling"),
            size_bytes: o.get_field("sizeBytes"),
            state: o.get_field("state"),
            type_: o.get_field("type"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
