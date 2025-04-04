#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_cluster_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterSnapshotArgs {
        /// Returns the list of snapshots created by the specific db_cluster
        #[builder(into, default)]
        pub db_cluster_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Returns information on a specific snapshot_id.
        #[builder(into, default)]
        pub db_cluster_snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Set this value to true to include manual DB Cluster Snapshots that are public and can be
        /// copied or restored by any AWS account, otherwise set this value to false. The default is `false`.
        #[builder(into, default)]
        pub include_public: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Set this value to true to include shared manual DB Cluster Snapshots from other
        /// AWS accounts that this AWS account has been given permission to copy or restore, otherwise set this value to false.
        /// The default is `false`.
        #[builder(into, default)]
        pub include_shared: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If more than one result is returned, use the most recent Snapshot.
        #[builder(into, default)]
        pub most_recent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Type of snapshots to be returned. If you don't specify a SnapshotType
        /// value, then both automated and manual DB cluster snapshots are returned. Shared and public DB Cluster Snapshots are not
        /// included in the returned results by default. Possible values are, `automated`, `manual`, `shared`, `public` and `awsbackup`.
        #[builder(into, default)]
        pub snapshot_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Mapping of tags, each pair of which must exactly match
        /// a pair on the desired DB cluster snapshot.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetClusterSnapshotResult {
        /// Allocated storage size in gigabytes (GB).
        pub allocated_storage: pulumi_gestalt_rust::Output<i32>,
        /// List of EC2 Availability Zones that instances in the DB cluster snapshot can be restored in.
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies the DB cluster identifier of the DB cluster that this DB cluster snapshot was created from.
        pub db_cluster_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN for the DB Cluster Snapshot.
        pub db_cluster_snapshot_arn: pulumi_gestalt_rust::Output<String>,
        pub db_cluster_snapshot_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the database engine.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// Version of the database engine for this DB cluster snapshot.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub include_public: pulumi_gestalt_rust::Output<Option<bool>>,
        pub include_shared: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If storage_encrypted is true, the AWS KMS key identifier for the encrypted DB cluster snapshot.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// License model information for the restored DB cluster.
        pub license_model: pulumi_gestalt_rust::Output<String>,
        pub most_recent: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Port that the DB cluster was listening on at the time of the snapshot.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// Time when the snapshot was taken, in Universal Coordinated Time (UTC).
        pub snapshot_create_time: pulumi_gestalt_rust::Output<String>,
        pub snapshot_type: pulumi_gestalt_rust::Output<Option<String>>,
        pub source_db_cluster_snapshot_arn: pulumi_gestalt_rust::Output<String>,
        /// Status of this DB Cluster Snapshot.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Whether the DB cluster snapshot is encrypted.
        pub storage_encrypted: pulumi_gestalt_rust::Output<bool>,
        /// Map of tags for the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// VPC ID associated with the DB cluster snapshot.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetClusterSnapshotArgs,
    ) -> GetClusterSnapshotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let db_cluster_identifier_binding = args
            .db_cluster_identifier
            .get_output(context);
        let db_cluster_snapshot_identifier_binding = args
            .db_cluster_snapshot_identifier
            .get_output(context);
        let include_public_binding = args.include_public.get_output(context);
        let include_shared_binding = args.include_shared.get_output(context);
        let most_recent_binding = args.most_recent.get_output(context);
        let snapshot_type_binding = args.snapshot_type.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:rds/getClusterSnapshot:getClusterSnapshot".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbClusterIdentifier".into(),
                    value: &db_cluster_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbClusterSnapshotIdentifier".into(),
                    value: &db_cluster_snapshot_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includePublic".into(),
                    value: &include_public_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeShared".into(),
                    value: &include_shared_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mostRecent".into(),
                    value: &most_recent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotType".into(),
                    value: &snapshot_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetClusterSnapshotResult {
            allocated_storage: o.get_field("allocatedStorage"),
            availability_zones: o.get_field("availabilityZones"),
            db_cluster_identifier: o.get_field("dbClusterIdentifier"),
            db_cluster_snapshot_arn: o.get_field("dbClusterSnapshotArn"),
            db_cluster_snapshot_identifier: o.get_field("dbClusterSnapshotIdentifier"),
            engine: o.get_field("engine"),
            engine_version: o.get_field("engineVersion"),
            id: o.get_field("id"),
            include_public: o.get_field("includePublic"),
            include_shared: o.get_field("includeShared"),
            kms_key_id: o.get_field("kmsKeyId"),
            license_model: o.get_field("licenseModel"),
            most_recent: o.get_field("mostRecent"),
            port: o.get_field("port"),
            snapshot_create_time: o.get_field("snapshotCreateTime"),
            snapshot_type: o.get_field("snapshotType"),
            source_db_cluster_snapshot_arn: o.get_field("sourceDbClusterSnapshotArn"),
            status: o.get_field("status"),
            storage_encrypted: o.get_field("storageEncrypted"),
            tags: o.get_field("tags"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
