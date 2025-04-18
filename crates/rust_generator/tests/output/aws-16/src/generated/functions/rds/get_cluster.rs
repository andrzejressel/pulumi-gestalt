#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// Cluster identifier of the RDS cluster.
        #[builder(into)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        pub backtrack_window: pulumi_gestalt_rust::Output<i32>,
        pub backup_retention_period: pulumi_gestalt_rust::Output<i32>,
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        pub cluster_members: pulumi_gestalt_rust::Output<Vec<String>>,
        pub cluster_resource_id: pulumi_gestalt_rust::Output<String>,
        pub database_name: pulumi_gestalt_rust::Output<String>,
        pub db_cluster_parameter_group_name: pulumi_gestalt_rust::Output<String>,
        pub db_subnet_group_name: pulumi_gestalt_rust::Output<String>,
        pub db_system_id: pulumi_gestalt_rust::Output<String>,
        pub enabled_cloudwatch_logs_exports: pulumi_gestalt_rust::Output<Vec<String>>,
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        pub engine: pulumi_gestalt_rust::Output<String>,
        pub engine_mode: pulumi_gestalt_rust::Output<String>,
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        pub final_snapshot_identifier: pulumi_gestalt_rust::Output<String>,
        pub hosted_zone_id: pulumi_gestalt_rust::Output<String>,
        pub iam_database_authentication_enabled: pulumi_gestalt_rust::Output<bool>,
        pub iam_roles: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        pub master_user_secrets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::rds::GetClusterMasterUserSecret>,
        >,
        pub master_username: pulumi_gestalt_rust::Output<String>,
        pub network_type: pulumi_gestalt_rust::Output<String>,
        pub port: pulumi_gestalt_rust::Output<i32>,
        pub preferred_backup_window: pulumi_gestalt_rust::Output<String>,
        pub preferred_maintenance_window: pulumi_gestalt_rust::Output<String>,
        pub reader_endpoint: pulumi_gestalt_rust::Output<String>,
        pub replication_source_identifier: pulumi_gestalt_rust::Output<String>,
        pub storage_encrypted: pulumi_gestalt_rust::Output<bool>,
        /// A map of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetClusterArgs,
    ) -> GetClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_identifier_binding = args.cluster_identifier.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:rds/getCluster:getCluster".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetClusterResult {
            arn: o.get_field("arn"),
            availability_zones: o.get_field("availabilityZones"),
            backtrack_window: o.get_field("backtrackWindow"),
            backup_retention_period: o.get_field("backupRetentionPeriod"),
            cluster_identifier: o.get_field("clusterIdentifier"),
            cluster_members: o.get_field("clusterMembers"),
            cluster_resource_id: o.get_field("clusterResourceId"),
            database_name: o.get_field("databaseName"),
            db_cluster_parameter_group_name: o.get_field("dbClusterParameterGroupName"),
            db_subnet_group_name: o.get_field("dbSubnetGroupName"),
            db_system_id: o.get_field("dbSystemId"),
            enabled_cloudwatch_logs_exports: o.get_field("enabledCloudwatchLogsExports"),
            endpoint: o.get_field("endpoint"),
            engine: o.get_field("engine"),
            engine_mode: o.get_field("engineMode"),
            engine_version: o.get_field("engineVersion"),
            final_snapshot_identifier: o.get_field("finalSnapshotIdentifier"),
            hosted_zone_id: o.get_field("hostedZoneId"),
            iam_database_authentication_enabled: o
                .get_field("iamDatabaseAuthenticationEnabled"),
            iam_roles: o.get_field("iamRoles"),
            id: o.get_field("id"),
            kms_key_id: o.get_field("kmsKeyId"),
            master_user_secrets: o.get_field("masterUserSecrets"),
            master_username: o.get_field("masterUsername"),
            network_type: o.get_field("networkType"),
            port: o.get_field("port"),
            preferred_backup_window: o.get_field("preferredBackupWindow"),
            preferred_maintenance_window: o.get_field("preferredMaintenanceWindow"),
            reader_endpoint: o.get_field("readerEndpoint"),
            replication_source_identifier: o.get_field("replicationSourceIdentifier"),
            storage_encrypted: o.get_field("storageEncrypted"),
            tags: o.get_field("tags"),
            vpc_security_group_ids: o.get_field("vpcSecurityGroupIds"),
        }
    }
}
