/// ## Example Usage
///
/// ### Alloydb Cluster Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:alloydb:Cluster
///     properties:
///       clusterId: alloydb-cluster
///       location: us-central1
///       networkConfig:
///         network: ${defaultNetwork.id}
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: alloydb-cluster
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Alloydb Cluster Full
///
///
/// ```yaml
/// resources:
///   full:
///     type: gcp:alloydb:Cluster
///     properties:
///       clusterId: alloydb-cluster-full
///       location: us-central1
///       networkConfig:
///         network: ${default.id}
///       databaseVersion: POSTGRES_15
///       initialUser:
///         user: alloydb-cluster-full
///         password: alloydb-cluster-full
///       continuousBackupConfig:
///         enabled: true
///         recoveryWindowDays: 14
///       automatedBackupPolicy:
///         location: us-central1
///         backupWindow: 1800s
///         enabled: true
///         weeklySchedule:
///           daysOfWeeks:
///             - MONDAY
///           startTimes:
///             - hours: 23
///               minutes: 0
///               seconds: 0
///               nanos: 0
///         quantityBasedRetention:
///           count: 1
///         labels:
///           test: alloydb-cluster-full
///       labels:
///         test: alloydb-cluster-full
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: alloydb-cluster-full
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Alloydb Cluster Restore
///
///
/// ```yaml
/// resources:
///   source:
///     type: gcp:alloydb:Cluster
///     properties:
///       clusterId: alloydb-source-cluster
///       location: us-central1
///       network: ${default.id}
///       initialUser:
///         password: alloydb-source-cluster
///   sourceInstance:
///     type: gcp:alloydb:Instance
///     name: source
///     properties:
///       cluster: ${source.name}
///       instanceId: alloydb-instance
///       instanceType: PRIMARY
///       machineConfig:
///         cpuCount: 2
///     options:
///       dependsOn:
///         - ${vpcConnection}
///   sourceBackup:
///     type: gcp:alloydb:Backup
///     name: source
///     properties:
///       backupId: alloydb-backup
///       location: us-central1
///       clusterName: ${source.name}
///     options:
///       dependsOn:
///         - ${sourceInstance}
///   restoredFromBackup:
///     type: gcp:alloydb:Cluster
///     name: restored_from_backup
///     properties:
///       clusterId: alloydb-backup-restored
///       location: us-central1
///       networkConfig:
///         network: ${default.id}
///       restoreBackupSource:
///         backupName: ${sourceBackup.name}
///   restoredViaPitr:
///     type: gcp:alloydb:Cluster
///     name: restored_via_pitr
///     properties:
///       clusterId: alloydb-pitr-restored
///       location: us-central1
///       networkConfig:
///         network: ${default.id}
///       restoreContinuousBackupSource:
///         cluster: ${source.name}
///         pointInTime: 2023-08-03T19:19:00.094Z
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: alloydb-source-cluster
///       addressType: INTERNAL
///       purpose: VPC_PEERING
///       prefixLength: 16
///       network: ${default.id}
///   vpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: vpc_connection
///     properties:
///       network: ${default.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAlloc.name}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
///   default:
///     fn::invoke:
///       function: gcp:compute:getNetwork
///       arguments:
///         name: alloydb-network
/// ```
/// ### Alloydb Secondary Cluster Basic
///
///
/// ```yaml
/// resources:
///   primary:
///     type: gcp:alloydb:Cluster
///     properties:
///       clusterId: alloydb-primary-cluster
///       location: us-central1
///       networkConfig:
///         network: ${default.id}
///   primaryInstance:
///     type: gcp:alloydb:Instance
///     name: primary
///     properties:
///       cluster: ${primary.name}
///       instanceId: alloydb-primary-instance
///       instanceType: PRIMARY
///       machineConfig:
///         cpuCount: 2
///     options:
///       dependsOn:
///         - ${vpcConnection}
///   secondary:
///     type: gcp:alloydb:Cluster
///     properties:
///       clusterId: alloydb-secondary-cluster
///       location: us-east1
///       networkConfig:
///         network: ${default.id}
///       clusterType: SECONDARY
///       continuousBackupConfig:
///         enabled: false
///       secondaryConfig:
///         primaryClusterName: ${primary.name}
///     options:
///       dependsOn:
///         - ${primaryInstance}
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: alloydb-secondary-cluster
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: alloydb-secondary-cluster
///       addressType: INTERNAL
///       purpose: VPC_PEERING
///       prefixLength: 16
///       network: ${default.id}
///   vpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: vpc_connection
///     properties:
///       network: ${default.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAlloc.name}
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
/// * `{{cluster_id}}`
///
/// When using the `pulumi import` command, Cluster can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:alloydb/cluster:Cluster default projects/{{project}}/locations/{{location}}/clusters/{{cluster_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:alloydb/cluster:Cluster default {{project}}/{{location}}/{{cluster_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:alloydb/cluster:Cluster default {{location}}/{{cluster_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:alloydb/cluster:Cluster default {{cluster_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The automated backup policy for this cluster. AutomatedBackupPolicy is disabled by default.
        /// Structure is documented below.
        #[builder(into, default)]
        pub automated_backup_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::ClusterAutomatedBackupPolicy>,
        >,
        /// The ID of the alloydb cluster.
        #[builder(into)]
        pub cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of cluster. If not set, defaults to PRIMARY.
        /// Default value is `PRIMARY`.
        /// Possible values are: `PRIMARY`, `SECONDARY`.
        #[builder(into, default)]
        pub cluster_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The continuous backup config for this cluster.
        /// If no policy is provided then the default policy will be used. The default policy takes one backup a day and retains backups for 14 days.
        /// Structure is documented below.
        #[builder(into, default)]
        pub continuous_backup_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::ClusterContinuousBackupConfig>,
        >,
        /// The database engine major version. This is an optional field and it's populated at the Cluster creation time. This field cannot be changed after cluster creation.
        #[builder(into, default)]
        pub database_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Policy to determine if the cluster should be deleted forcefully.
        /// Deleting a cluster forcefully, deletes the cluster and all its associated instances within the cluster.
        /// Deleting a Secondary cluster with a secondary instance REQUIRES setting deletion_policy = "FORCE" otherwise an error is returned. This is needed as there is no support to delete just the secondary instance, and the only way to delete secondary instance is to delete the associated secondary cluster forcefully which also deletes the secondary instance.
        /// Possible values: DEFAULT, FORCE
        #[builder(into, default)]
        pub deletion_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User-settable and human-readable display name for the Cluster.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// EncryptionConfig describes the encryption config of a cluster or a backup that is encrypted with a CMEK (customer-managed encryption key).
        /// Structure is documented below.
        #[builder(into, default)]
        pub encryption_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::ClusterEncryptionConfig>,
        >,
        /// For Resource freshness validation (https://google.aip.dev/154)
        #[builder(into, default)]
        pub etag: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Initial user to setup during cluster creation.
        /// Structure is documented below.
        #[builder(into, default)]
        pub initial_user: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::ClusterInitialUser>,
        >,
        /// User-defined labels for the alloydb cluster.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the alloydb cluster should reside.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// MaintenanceUpdatePolicy defines the policy for system updates.
        /// Structure is documented below.
        #[builder(into, default)]
        pub maintenance_update_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::ClusterMaintenanceUpdatePolicy>,
        >,
        /// Metadata related to network configuration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub network_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::ClusterNetworkConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration for Private Service Connect (PSC) for the cluster.
        /// Structure is documented below.
        #[builder(into, default)]
        pub psc_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::ClusterPscConfig>,
        >,
        /// The source when restoring from a backup. Conflicts with 'restore_continuous_backup_source', both can't be set together.
        /// Structure is documented below.
        #[builder(into, default)]
        pub restore_backup_source: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::ClusterRestoreBackupSource>,
        >,
        /// The source when restoring via point in time recovery (PITR). Conflicts with 'restore_backup_source', both can't be set together.
        /// Structure is documented below.
        #[builder(into, default)]
        pub restore_continuous_backup_source: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::ClusterRestoreContinuousBackupSource>,
        >,
        /// Configuration of the secondary cluster for Cross Region Replication. This should be set if and only if the cluster is of type SECONDARY.
        /// Structure is documented below.
        #[builder(into, default)]
        pub secondary_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::ClusterSecondaryConfig>,
        >,
        /// The subscrition type of cluster.
        /// Possible values are: `TRIAL`, `STANDARD`.
        #[builder(into, default)]
        pub subscription_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The automated backup policy for this cluster. AutomatedBackupPolicy is disabled by default.
        /// Structure is documented below.
        pub automated_backup_policy: pulumi_gestalt_rust::Output<
            super::super::types::alloydb::ClusterAutomatedBackupPolicy,
        >,
        /// Cluster created from backup.
        /// Structure is documented below.
        pub backup_sources: pulumi_gestalt_rust::Output<
            Vec<super::super::types::alloydb::ClusterBackupSource>,
        >,
        /// The ID of the alloydb cluster.
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The type of cluster. If not set, defaults to PRIMARY.
        /// Default value is `PRIMARY`.
        /// Possible values are: `PRIMARY`, `SECONDARY`.
        pub cluster_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The continuous backup config for this cluster.
        /// If no policy is provided then the default policy will be used. The default policy takes one backup a day and retains backups for 14 days.
        /// Structure is documented below.
        pub continuous_backup_config: pulumi_gestalt_rust::Output<
            super::super::types::alloydb::ClusterContinuousBackupConfig,
        >,
        /// ContinuousBackupInfo describes the continuous backup properties of a cluster.
        /// Structure is documented below.
        pub continuous_backup_infos: pulumi_gestalt_rust::Output<
            Vec<super::super::types::alloydb::ClusterContinuousBackupInfo>,
        >,
        /// The database engine major version. This is an optional field and it's populated at the Cluster creation time. This field cannot be changed after cluster creation.
        pub database_version: pulumi_gestalt_rust::Output<String>,
        /// Policy to determine if the cluster should be deleted forcefully.
        /// Deleting a cluster forcefully, deletes the cluster and all its associated instances within the cluster.
        /// Deleting a Secondary cluster with a secondary instance REQUIRES setting deletion_policy = "FORCE" otherwise an error is returned. This is needed as there is no support to delete just the secondary instance, and the only way to delete secondary instance is to delete the associated secondary cluster forcefully which also deletes the secondary instance.
        /// Possible values: DEFAULT, FORCE
        pub deletion_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// User-settable and human-readable display name for the Cluster.
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
            Option<super::super::types::alloydb::ClusterEncryptionConfig>,
        >,
        /// (Output)
        /// Output only. The encryption information for the WALs and backups required for ContinuousBackup.
        /// Structure is documented below.
        pub encryption_infos: pulumi_gestalt_rust::Output<
            Vec<super::super::types::alloydb::ClusterEncryptionInfo>,
        >,
        /// For Resource freshness validation (https://google.aip.dev/154)
        pub etag: pulumi_gestalt_rust::Output<Option<String>>,
        /// Initial user to setup during cluster creation.
        /// Structure is documented below.
        pub initial_user: pulumi_gestalt_rust::Output<
            Option<super::super::types::alloydb::ClusterInitialUser>,
        >,
        /// User-defined labels for the alloydb cluster.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the alloydb cluster should reside.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// MaintenanceUpdatePolicy defines the policy for system updates.
        /// Structure is documented below.
        pub maintenance_update_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::alloydb::ClusterMaintenanceUpdatePolicy>,
        >,
        /// Cluster created via DMS migration.
        /// Structure is documented below.
        pub migration_sources: pulumi_gestalt_rust::Output<
            Vec<super::super::types::alloydb::ClusterMigrationSource>,
        >,
        /// The name of the cluster resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Metadata related to network configuration.
        /// Structure is documented below.
        pub network_config: pulumi_gestalt_rust::Output<
            super::super::types::alloydb::ClusterNetworkConfig,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Configuration for Private Service Connect (PSC) for the cluster.
        /// Structure is documented below.
        pub psc_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::alloydb::ClusterPscConfig>,
        >,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. Reconciling (https://google.aip.dev/128#reconciliation).
        /// Set to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them.
        /// This can happen due to user-triggered updates or system actions like failover or maintenance.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// The source when restoring from a backup. Conflicts with 'restore_continuous_backup_source', both can't be set together.
        /// Structure is documented below.
        pub restore_backup_source: pulumi_gestalt_rust::Output<
            Option<super::super::types::alloydb::ClusterRestoreBackupSource>,
        >,
        /// The source when restoring via point in time recovery (PITR). Conflicts with 'restore_backup_source', both can't be set together.
        /// Structure is documented below.
        pub restore_continuous_backup_source: pulumi_gestalt_rust::Output<
            Option<super::super::types::alloydb::ClusterRestoreContinuousBackupSource>,
        >,
        /// Configuration of the secondary cluster for Cross Region Replication. This should be set if and only if the cluster is of type SECONDARY.
        /// Structure is documented below.
        pub secondary_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::alloydb::ClusterSecondaryConfig>,
        >,
        /// Output only. The current serving state of the cluster.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The subscrition type of cluster.
        /// Possible values are: `TRIAL`, `STANDARD`.
        pub subscription_type: pulumi_gestalt_rust::Output<String>,
        /// Contains information and all metadata related to TRIAL clusters.
        /// Structure is documented below.
        pub trial_metadatas: pulumi_gestalt_rust::Output<
            Vec<super::super::types::alloydb::ClusterTrialMetadata>,
        >,
        /// The system-generated UID of the resource.
        pub uid: pulumi_gestalt_rust::Output<String>,
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
        let annotations_binding = args.annotations.get_output(context);
        let automated_backup_policy_binding = args
            .automated_backup_policy
            .get_output(context);
        let cluster_id_binding = args.cluster_id.get_output(context);
        let cluster_type_binding = args.cluster_type.get_output(context);
        let continuous_backup_config_binding = args
            .continuous_backup_config
            .get_output(context);
        let database_version_binding = args.database_version.get_output(context);
        let deletion_policy_binding = args.deletion_policy.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let encryption_config_binding = args.encryption_config.get_output(context);
        let etag_binding = args.etag.get_output(context);
        let initial_user_binding = args.initial_user.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let maintenance_update_policy_binding = args
            .maintenance_update_policy
            .get_output(context);
        let network_config_binding = args.network_config.get_output(context);
        let project_binding = args.project.get_output(context);
        let psc_config_binding = args.psc_config.get_output(context);
        let restore_backup_source_binding = args
            .restore_backup_source
            .get_output(context);
        let restore_continuous_backup_source_binding = args
            .restore_continuous_backup_source
            .get_output(context);
        let secondary_config_binding = args.secondary_config.get_output(context);
        let subscription_type_binding = args.subscription_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:alloydb/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automatedBackupPolicy".into(),
                    value: &automated_backup_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterType".into(),
                    value: &cluster_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "continuousBackupConfig".into(),
                    value: &continuous_backup_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseVersion".into(),
                    value: &database_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionPolicy".into(),
                    value: &deletion_policy_binding.drop_type(),
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
                    name: "etag".into(),
                    value: &etag_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "initialUser".into(),
                    value: &initial_user_binding.drop_type(),
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
                    name: "maintenanceUpdatePolicy".into(),
                    value: &maintenance_update_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkConfig".into(),
                    value: &network_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pscConfig".into(),
                    value: &psc_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restoreBackupSource".into(),
                    value: &restore_backup_source_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restoreContinuousBackupSource".into(),
                    value: &restore_continuous_backup_source_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secondaryConfig".into(),
                    value: &secondary_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscriptionType".into(),
                    value: &subscription_type_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterResult {
            id: o.get_field("id"),
            annotations: o.get_field("annotations"),
            automated_backup_policy: o.get_field("automatedBackupPolicy"),
            backup_sources: o.get_field("backupSources"),
            cluster_id: o.get_field("clusterId"),
            cluster_type: o.get_field("clusterType"),
            continuous_backup_config: o.get_field("continuousBackupConfig"),
            continuous_backup_infos: o.get_field("continuousBackupInfos"),
            database_version: o.get_field("databaseVersion"),
            deletion_policy: o.get_field("deletionPolicy"),
            display_name: o.get_field("displayName"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            encryption_config: o.get_field("encryptionConfig"),
            encryption_infos: o.get_field("encryptionInfos"),
            etag: o.get_field("etag"),
            initial_user: o.get_field("initialUser"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            maintenance_update_policy: o.get_field("maintenanceUpdatePolicy"),
            migration_sources: o.get_field("migrationSources"),
            name: o.get_field("name"),
            network_config: o.get_field("networkConfig"),
            project: o.get_field("project"),
            psc_config: o.get_field("pscConfig"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reconciling: o.get_field("reconciling"),
            restore_backup_source: o.get_field("restoreBackupSource"),
            restore_continuous_backup_source: o
                .get_field("restoreContinuousBackupSource"),
            secondary_config: o.get_field("secondaryConfig"),
            state: o.get_field("state"),
            subscription_type: o.get_field("subscriptionType"),
            trial_metadatas: o.get_field("trialMetadatas"),
            uid: o.get_field("uid"),
        }
    }
}
