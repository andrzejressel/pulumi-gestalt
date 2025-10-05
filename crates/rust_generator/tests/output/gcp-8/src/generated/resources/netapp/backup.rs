/// NetApp Volumes supports volume backups, which are copies of your volumes
/// stored independently from the volume. Backups are stored in backup vaults,
/// which are containers for backups. If a volume is lost or deleted, you can
/// use backups to restore your data to a new volume.
///
/// When you create the first backup of a volume, all of the volume's used
/// data is sent to the backup vault. Subsequent backups of the same volume
/// only include data that has changed from the previous backup. This allows
/// for fast incremental-forever backups and reduces the required capacity
/// inside the backup vault.
///
/// You can create manual and scheduled backups. Manual backups can be taken
/// from a volume or from an existing volume snapshot. Scheduled backups
/// require a backup policy.
///
///
/// To get more information about Backup, see:
///
/// * [API documentation](https://cloud.google.com/netapp/volumes/docs/reference/rest/v1/projects.locations.backupVaults.backups)
/// * How-to Guides
///     * [Documentation](https://cloud.google.com/netapp/volumes/docs/protect-data/about-volume-backups)
///
/// ## Example Usage
///
/// ### Netapp Backup
///
///
/// ```yaml
/// resources:
///   defaultStoragePool:
///     type: gcp:netapp:StoragePool
///     name: default
///     properties:
///       name: backup-pool
///       location: us-central1
///       serviceLevel: PREMIUM
///       capacityGib: '2048'
///       network: ${default.id}
///   defaultVolume:
///     type: gcp:netapp:Volume
///     name: default
///     properties:
///       name: backup-volume
///       location: ${defaultStoragePool.location}
///       capacityGib: '100'
///       shareName: backup-volume
///       storagePool: ${defaultStoragePool.name}
///       protocols:
///         - NFSV3
///       deletionPolicy: FORCE
///       backupConfig:
///         backupVault: ${defaultBackupVault.id}
///   defaultBackupVault:
///     type: gcp:netapp:BackupVault
///     name: default
///     properties:
///       name: backup-vault
///       location: ${defaultStoragePool.location}
///   testBackup:
///     type: gcp:netapp:Backup
///     name: test_backup
///     properties:
///       name: test-backup
///       location: ${defaultBackupVault.location}
///       vaultName: ${defaultBackupVault.name}
///       sourceVolume: ${defaultVolume.id}
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:compute:getNetwork
///       arguments:
///         name: ""
/// ```
///
/// ## Import
///
/// Backup can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/backupVaults/{{vault_name}}/backups/{{name}}`
///
/// * `{{project}}/{{location}}/{{vault_name}}/{{name}}`
///
/// * `{{location}}/{{vault_name}}/{{name}}`
///
/// When using the `pulumi import` command, Backup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:netapp/backup:Backup default projects/{{project}}/locations/{{location}}/backupVaults/{{vault_name}}/backups/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:netapp/backup:Backup default {{project}}/{{location}}/{{vault_name}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:netapp/backup:Backup default {{location}}/{{vault_name}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backup {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupArgs {
        /// A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location of the backup.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource name of the backup. Needs to be unique per location.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If specified, backup will be created from the given snapshot. If not specified,
        /// there will be a new snapshot taken to initiate the backup creation.
        /// Format: `projects/{{projectId}}/locations/{{location}}/volumes/{{volumename}}/snapshots/{{snapshotname}}``
        #[builder(into, default)]
        pub source_snapshot: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of volumes this backup belongs to. Format: `projects/{{projects_id}}/locations/{{location}}/volumes/{{name}}``
        #[builder(into, default)]
        pub source_volume: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the backup vault to store the backup in.
        #[builder(into)]
        pub vault_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BackupResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Type of backup, manually created or created by a backup policy. Possible Values : [TYPE_UNSPECIFIED, MANUAL, SCHEDULED]
        pub backup_type: pulumi_gestalt_rust::Output<String>,
        /// Backups of a volume build incrementally on top of each other. They form a "backup chain".
        /// Total size of all backups in a chain in bytes = baseline backup size + sum(incremental backup size)
        pub chain_storage_bytes: pulumi_gestalt_rust::Output<String>,
        /// Create time of the backup. A timestamp in RFC3339 UTC "Zulu" format. Examples: "2023-06-22T09:13:01.617Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location of the backup.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the backup. Needs to be unique per location.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If specified, backup will be created from the given snapshot. If not specified,
        /// there will be a new snapshot taken to initiate the backup creation.
        /// Format: `projects/{{projectId}}/locations/{{location}}/volumes/{{volumename}}/snapshots/{{snapshotname}}``
        pub source_snapshot: pulumi_gestalt_rust::Output<Option<String>>,
        /// ID of volumes this backup belongs to. Format: `projects/{{projects_id}}/locations/{{location}}/volumes/{{name}}``
        pub source_volume: pulumi_gestalt_rust::Output<Option<String>>,
        /// The state of the Backup Vault. Possible Values : [STATE_UNSPECIFIED, CREATING, UPLOADING, READY, DELETING, ERROR, UPDATING]
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Name of the backup vault to store the backup in.
        pub vault_name: pulumi_gestalt_rust::Output<String>,
        /// Size of the file system when the backup was created. When creating a new volume from the backup, the volume capacity will have to be at least as big.
        pub volume_usage_bytes: pulumi_gestalt_rust::Output<String>,
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
        let description_binding = args.description.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let source_snapshot_binding = args.source_snapshot.get_output(context);
        let source_volume_binding = args.source_volume.get_output(context);
        let vault_name_binding = args.vault_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:netapp/backup:Backup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
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
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceSnapshot".into(),
                    value: &source_snapshot_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceVolume".into(),
                    value: &source_volume_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vaultName".into(),
                    value: &vault_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BackupResult {
            id: o.get_field("id"),
            backup_type: o.get_field("backupType"),
            chain_storage_bytes: o.get_field("chainStorageBytes"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            source_snapshot: o.get_field("sourceSnapshot"),
            source_volume: o.get_field("sourceVolume"),
            state: o.get_field("state"),
            vault_name: o.get_field("vaultName"),
            volume_usage_bytes: o.get_field("volumeUsageBytes"),
        }
    }
}
