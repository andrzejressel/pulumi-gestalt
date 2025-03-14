/// Manages a [RDS Aurora Cluster](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/CHAP_Aurora.html) or a [RDS Multi-AZ DB Cluster](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/multi-az-db-clusters-concepts.html). To manage cluster instances that inherit configuration from the cluster (when not running the cluster in `serverless` engine mode), see the `aws.rds.ClusterInstance` resource. To manage non-Aurora DB instances (e.g., MySQL, PostgreSQL, SQL Server, etc.), see the `aws.rds.Instance` resource.
///
/// For information on the difference between the available Aurora MySQL engines see [Comparison between Aurora MySQL 1 and Aurora MySQL 2](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/AuroraMySQL.Updates.20180206.html) in the Amazon RDS User Guide.
///
/// Changes to an RDS Cluster can occur when you manually change a parameter, such as `port`, and are reflected in the next maintenance window. Because of this, this provider may report a difference in its planning phase because a modification has not yet taken place. You can use the `apply_immediately` flag to instruct the service to apply the change immediately (see documentation below).
///
/// > **Note:** Multi-AZ DB clusters are supported only for the MySQL and PostgreSQL DB engines.
///
/// > **Note:** `ca_certificate_identifier` is only supported for Multi-AZ DB clusters.
///
/// > **Note:** using `apply_immediately` can result in a brief downtime as the server reboots. See the AWS Docs on [RDS Maintenance](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_UpgradeDBInstance.Maintenance.html) for more information.
///
/// > **Note:** All arguments including the username and password will be stored in the raw state as plain-text.
/// > **NOTE on RDS Clusters and RDS Cluster Role Associations:** Pulumi provides both a standalone RDS Cluster Role Association - (an association between an RDS Cluster and a single IAM Role) and an RDS Cluster resource with `iam_roles` attributes. Use one resource or the other to associate IAM Roles and RDS Clusters. Not doing so will cause a conflict of associations and will result in the association being overwritten.
///
/// ## Example Usage
///
/// ### Aurora MySQL 2.x (MySQL 5.7)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = cluster::create(
///         "default",
///         ClusterArgs::builder()
///             .availability_zones(vec!["us-west-2a", "us-west-2b", "us-west-2c",])
///             .backup_retention_period(5)
///             .cluster_identifier("aurora-cluster-demo")
///             .database_name("mydb")
///             .engine("aurora-mysql")
///             .engine_version("5.7.mysql_aurora.2.03.2")
///             .master_password("must_be_eight_characters")
///             .master_username("foo")
///             .preferred_backup_window("07:00-09:00")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Aurora MySQL 1.x (MySQL 5.6)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = cluster::create(
///         "default",
///         ClusterArgs::builder()
///             .availability_zones(vec!["us-west-2a", "us-west-2b", "us-west-2c",])
///             .backup_retention_period(5)
///             .cluster_identifier("aurora-cluster-demo")
///             .database_name("mydb")
///             .master_password("must_be_eight_characters")
///             .master_username("foo")
///             .preferred_backup_window("07:00-09:00")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Aurora with PostgreSQL engine
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let postgresql = cluster::create(
///         "postgresql",
///         ClusterArgs::builder()
///             .availability_zones(vec!["us-west-2a", "us-west-2b", "us-west-2c",])
///             .backup_retention_period(5)
///             .cluster_identifier("aurora-cluster-demo")
///             .database_name("mydb")
///             .engine("aurora-postgresql")
///             .master_password("must_be_eight_characters")
///             .master_username("foo")
///             .preferred_backup_window("07:00-09:00")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### RDS Multi-AZ Cluster
///
/// > More information about RDS Multi-AZ Clusters can be found in the [RDS User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/multi-az-db-clusters-concepts.html).
///
/// To create a Multi-AZ RDS cluster, you must additionally specify the `engine`, `storage_type`, `allocated_storage`, `iops` and `db_cluster_instance_class` attributes.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster::create(
///         "example",
///         ClusterArgs::builder()
///             .allocated_storage(100)
///             .availability_zones(vec!["us-west-2a", "us-west-2b", "us-west-2c",])
///             .cluster_identifier("example")
///             .db_cluster_instance_class("db.r6gd.xlarge")
///             .engine("mysql")
///             .iops(1000)
///             .master_password("mustbeeightcharaters")
///             .master_username("test")
///             .storage_type("io1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### RDS Serverless v2 Cluster
///
/// > More information about RDS Serverless v2 Clusters can be found in the [RDS User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-serverless-v2.html).
///
/// > **Note:** Unlike Serverless v1, in Serverless v2 the `storage_encrypted` value is set to `false` by default.
/// This is because Serverless v1 uses the `serverless` `engine_mode`, but Serverless v2 uses the `provisioned` `engine_mode`.
///
/// To create a Serverless v2 RDS cluster, you must additionally specify the `engine_mode` and `serverlessv2_scaling_configuration` attributes. An `aws.rds.ClusterInstance` resource must also be added to the cluster with the `instance_class` attribute specified.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster::create(
///         "example",
///         ClusterArgs::builder()
///             .cluster_identifier("example")
///             .database_name("test")
///             .engine("aurora-postgresql")
///             .engine_mode("provisioned")
///             .engine_version("13.6")
///             .master_password("must_be_eight_characters")
///             .master_username("test")
///             .serverlessv_2_scaling_configuration(
///                 ClusterServerlessv2ScalingConfiguration::builder()
///                     .maxCapacity(1)
///                     .minCapacity(0)
///                     .secondsUntilAutoPause(3600)
///                     .build_struct(),
///             )
///             .storage_encrypted(true)
///             .build_struct(),
///     );
///     let exampleClusterInstance = cluster_instance::create(
///         "exampleClusterInstance",
///         ClusterInstanceArgs::builder()
///             .cluster_identifier("${example.id}")
///             .engine("${example.engine}")
///             .engine_version("${example.engineVersion}")
///             .instance_class("db.serverless")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### RDS/Aurora Managed Master Passwords via Secrets Manager, default KMS Key
///
/// > More information about RDS/Aurora Aurora integrates with Secrets Manager to manage master user passwords for your DB clusters can be found in the [RDS User Guide](https://aws.amazon.com/about-aws/whats-new/2022/12/amazon-rds-integration-aws-secrets-manager/) and [Aurora User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/rds-secrets-manager.html).
///
/// You can specify the `manage_master_user_password` attribute to enable managing the master password with Secrets Manager. You can also update an existing cluster to use Secrets Manager by specify the `manage_master_user_password` attribute and removing the `master_password` attribute (removal is required).
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = cluster::create(
///         "test",
///         ClusterArgs::builder()
///             .cluster_identifier("example")
///             .database_name("test")
///             .manage_master_user_password(true)
///             .master_username("test")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### RDS/Aurora Managed Master Passwords via Secrets Manager, specific KMS Key
///
/// > More information about RDS/Aurora Aurora integrates with Secrets Manager to manage master user passwords for your DB clusters can be found in the [RDS User Guide](https://aws.amazon.com/about-aws/whats-new/2022/12/amazon-rds-integration-aws-secrets-manager/) and [Aurora User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/rds-secrets-manager.html).
///
/// You can specify the `master_user_secret_kms_key_id` attribute to specify a specific KMS Key.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = key::create(
///         "example",
///         KeyArgs::builder().description("Example KMS Key").build_struct(),
///     );
///     let test = cluster::create(
///         "test",
///         ClusterArgs::builder()
///             .cluster_identifier("example")
///             .database_name("test")
///             .manage_master_user_password(true)
///             .master_user_secret_kms_key_id("${example.keyId}")
///             .master_username("test")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Global Cluster Restored From Snapshot
///
/// ```yaml
/// resources:
///   exampleCluster:
///     type: aws:rds:Cluster
///     name: example
///     properties:
///       engine: aurora
///       engineVersion: 5.6.mysql_aurora.1.22.4
///       clusterIdentifier: example
///       snapshotIdentifier: ${example.id}
///   exampleGlobalCluster:
///     type: aws:rds:GlobalCluster
///     name: example
///     properties:
///       globalClusterIdentifier: example
///       sourceDbClusterIdentifier: ${exampleCluster.arn}
///       forceDestroy: true
/// variables:
///   example:
///     fn::invoke:
///       function: aws:rds:getClusterSnapshot
///       arguments:
///         dbClusterIdentifier: example-original-cluster
///         mostRecent: true
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import RDS Clusters using the `cluster_identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/cluster:Cluster aurora_cluster aurora-prod-cluster
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// The amount of storage in gibibytes (GiB) to allocate to each DB instance in the Multi-AZ DB cluster.
        #[builder(into, default)]
        pub allocated_storage: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Enable to allow major engine version upgrades when changing engine versions. Defaults to `false`.
        #[builder(into, default)]
        pub allow_major_version_upgrade: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies whether any cluster modifications are applied immediately, or during the next maintenance window. Default is `false`. See [Amazon RDS Documentation for more information.](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Overview.DBInstance.Modifying.html)
        #[builder(into, default)]
        pub apply_immediately: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of EC2 Availability Zones for the DB cluster storage where DB cluster instances can be created.
        /// RDS automatically assigns 3 AZs if less than 3 AZs are configured, which will show as a difference requiring resource recreation next pulumi up.
        /// We recommend specifying 3 AZs or using the `lifecycle` configuration block `ignore_changes` argument if necessary.
        /// A maximum of 3 AZs can be configured.
        #[builder(into, default)]
        pub availability_zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Target backtrack window, in seconds. Only available for `aurora` and `aurora-mysql` engines currently. To disable backtracking, set this value to `0`. Defaults to `0`. Must be between `0` and `259200` (72 hours)
        #[builder(into, default)]
        pub backtrack_window: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Days to retain backups for. Default `1`
        #[builder(into, default)]
        pub backup_retention_period: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The CA certificate identifier to use for the DB cluster's server certificate.
        #[builder(into, default)]
        pub ca_certificate_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The cluster identifier. If omitted, this provider will assign a random, unique identifier.
        #[builder(into, default)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique cluster identifier beginning with the specified prefix. Conflicts with `cluster_identifier`.
        #[builder(into, default)]
        pub cluster_identifier_prefix: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// List of RDS Instances that are a part of this cluster
        #[builder(into, default)]
        pub cluster_members: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Copy all Cluster `tags` to snapshots. Default is `false`.
        #[builder(into, default)]
        pub copy_tags_to_snapshot: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name for an automatically created database on cluster creation. There are different naming restrictions per database engine: [RDS Naming Constraints](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_Limits.html#RDS_Limits.Constraints)
        #[builder(into, default)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The compute and memory capacity of each DB instance in the Multi-AZ DB cluster, for example `db.m6g.xlarge`. Not all DB instance classes are available in all AWS Regions, or for all database engines. For the full list of DB instance classes and availability for your engine, see [DB instance class](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Concepts.DBInstanceClass.html) in the Amazon RDS User Guide.
        #[builder(into, default)]
        pub db_cluster_instance_class: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A cluster parameter group to associate with the cluster.
        #[builder(into, default)]
        pub db_cluster_parameter_group_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Instance parameter group to associate with all instances of the DB cluster. The `db_instance_parameter_group_name` parameter is only valid in combination with the `allow_major_version_upgrade` parameter.
        #[builder(into, default)]
        pub db_instance_parameter_group_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// DB subnet group to associate with this DB cluster.
        /// **NOTE:** This must match the `db_subnet_group_name` specified on every `aws.rds.ClusterInstance` in the cluster.
        #[builder(into, default)]
        pub db_subnet_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// For use with RDS Custom.
        #[builder(into, default)]
        pub db_system_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether to remove automated backups immediately after the DB cluster is deleted. Default is `true`.
        #[builder(into, default)]
        pub delete_automated_backups: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If the DB cluster should have deletion protection enabled.
        /// The database can't be deleted when this value is set to `true`.
        /// The default is `false`.
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Directory Service Active Directory domain to create the cluster in.
        #[builder(into, default)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the IAM role to be used when making API calls to the Directory Service.
        #[builder(into, default)]
        pub domain_iam_role_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether cluster should forward writes to an associated global cluster. Applied to secondary clusters to enable them to forward writes to an `aws.rds.GlobalCluster`'s primary cluster. See the [User Guide for Aurora](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-global-database-write-forwarding.html) for more information.
        #[builder(into, default)]
        pub enable_global_write_forwarding: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Enable HTTP endpoint (data API). Only valid for some combinations of `engine_mode`, `engine` and `engine_version` and only available in some regions. See the [Region and version availability](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/data-api.html#data-api.regions) section of the documentation. This option also does not work with any of these options specified: `snapshot_identifier`, `replication_source_identifier`, `s3_import`.
        #[builder(into, default)]
        pub enable_http_endpoint: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether read replicas can forward write operations to the writer DB instance in the DB cluster. By default, write operations aren't allowed on reader DB instances.. See the [User Guide for Aurora](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-mysql-write-forwarding.html) for more information. **NOTE:** Local write forwarding requires Aurora MySQL version 3.04 or higher.
        #[builder(into, default)]
        pub enable_local_write_forwarding: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Set of log types to export to cloudwatch. If omitted, no logs will be exported. The following log types are supported: `audit`, `error`, `general`, `slowquery`, `postgresql` (PostgreSQL).
        #[builder(into, default)]
        pub enabled_cloudwatch_logs_exports: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Name of the database engine to be used for this DB cluster. Valid Values: `aurora-mysql`, `aurora-postgresql`, `mysql`, `postgres`. (Note that `mysql` and `postgres` are Multi-AZ RDS clusters).
        #[builder(into)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The life cycle type for this DB instance. This setting is valid for cluster types Aurora DB clusters and Multi-AZ DB clusters. Valid values are `open-source-rds-extended-support`, `open-source-rds-extended-support-disabled`. Default value is `open-source-rds-extended-support`. [Using Amazon RDS Extended Support]: https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/extended-support.html
        #[builder(into, default)]
        pub engine_lifecycle_support: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Database engine mode. Valid values: `global` (only valid for Aurora MySQL 1.21 and earlier), `parallelquery`, `provisioned`, `serverless`. Defaults to: `provisioned`. See the [RDS User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-serverless.html) for limitations when using `serverless`.
        #[builder(into, default)]
        pub engine_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Database engine version. Updating this argument results in an outage. See the [Aurora MySQL](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Updates.html) and [Aurora Postgres](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraPostgreSQL.Updates.html) documentation for your configured engine to determine this value, or by running `aws rds describe-db-engine-versions`. For example with Aurora MySQL 2, a potential value for this argument is `5.7.mysql_aurora.2.03.2`. The value can contain a partial version where supported by the API. The actual engine version used is returned in the attribute `engine_version_actual`, , see Attribute Reference below.
        #[builder(into, default)]
        pub engine_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of your final DB snapshot when this DB cluster is deleted. If omitted, no final snapshot will be made.
        #[builder(into, default)]
        pub final_snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Global cluster identifier specified on `aws.rds.GlobalCluster`.
        #[builder(into, default)]
        pub global_cluster_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies whether or not mappings of AWS Identity and Access Management (IAM) accounts to database accounts is enabled. Please see [AWS Documentation](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/UsingWithRDS.IAMDBAuth.html) for availability and limitations.
        #[builder(into, default)]
        pub iam_database_authentication_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// List of ARNs for the IAM roles to associate to the RDS Cluster.
        #[builder(into, default)]
        pub iam_roles: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Amount of Provisioned IOPS (input/output operations per second) to be initially allocated for each DB instance in the Multi-AZ DB cluster. For information about valid Iops values, see [Amazon RDS Provisioned IOPS storage to improve performance](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_Storage.html#USER_PIOPS) in the Amazon RDS User Guide. (This setting is required to create a Multi-AZ DB cluster). Must be a multiple between .5 and 50 of the storage amount for the DB cluster.
        #[builder(into, default)]
        pub iops: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// ARN for the KMS encryption key. When specifying `kms_key_id`, `storage_encrypted` needs to be set to true.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set to true to allow RDS to manage the master user password in Secrets Manager. Cannot be set if `master_password` is provided.
        #[builder(into, default)]
        pub manage_master_user_password: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Password for the master DB user. Note that this may show up in logs, and it will be stored in the state file. Please refer to the [RDS Naming Constraints](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_Limits.html#RDS_Limits.Constraints). Cannot be set if `manage_master_user_password` is set to `true`.
        #[builder(into, default)]
        pub master_password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key. To use a KMS key in a different Amazon Web Services account, specify the key ARN or alias ARN. If not specified, the default KMS key for your Amazon Web Services account is used.
        #[builder(into, default)]
        pub master_user_secret_kms_key_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Username for the master DB user. Please refer to the [RDS Naming Constraints](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_Limits.html#RDS_Limits.Constraints). This argument does not support in-place updates and cannot be changed during a restore from snapshot.
        #[builder(into, default)]
        pub master_username: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Network type of the cluster. Valid values: `IPV4`, `DUAL`.
        #[builder(into, default)]
        pub network_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enables Performance Insights for the RDS Cluster
        #[builder(into, default)]
        pub performance_insights_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the KMS Key ID to encrypt Performance Insights data. If not specified, the default RDS KMS key will be used (`aws/rds`).
        #[builder(into, default)]
        pub performance_insights_kms_key_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the amount of time to retain performance insights data for. Defaults to 7 days if Performance Insights are enabled. Valid values are `7`, `month * 31` (where month is a number of months from 1-23), and `731`. See [here](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PerfInsights.Overview.cost.html) for more information on retention periods.
        #[builder(into, default)]
        pub performance_insights_retention_period: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Port on which the DB accepts connections.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Daily time range during which automated backups are created if automated backups are enabled using the BackupRetentionPeriod parameter.Time in UTC. Default: A 30-minute window selected at random from an 8-hour block of time per region, e.g. `04:00-09:00`.
        #[builder(into, default)]
        pub preferred_backup_window: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Weekly time range during which system maintenance can occur, in (UTC) e.g., `wed:04:00-wed:04:30`
        #[builder(into, default)]
        pub preferred_maintenance_window: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// ARN of a source DB cluster or DB instance if this DB cluster is to be created as a Read Replica. **Note:** Removing this attribute after creation will promote the read replica to a standalone cluster. If DB Cluster is part of a Global Cluster, use the `ignoreChanges` resource option to prevent Pulumi from showing differences for this argument instead of configuring this value.
        #[builder(into, default)]
        pub replication_source_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Nested attribute for [point in time restore](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-pitr.html). More details below.
        #[builder(into, default)]
        pub restore_to_point_in_time: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rds::ClusterRestoreToPointInTime>,
        >,
        #[builder(into, default)]
        pub s3_import: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rds::ClusterS3Import>,
        >,
        /// Nested attribute with scaling properties. Only valid when `engine_mode` is set to `serverless`. More details below.
        #[builder(into, default)]
        pub scaling_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rds::ClusterScalingConfiguration>,
        >,
        /// Nested attribute with scaling properties for ServerlessV2. Only valid when `engine_mode` is set to `provisioned`. More details below.
        #[builder(into, default)]
        pub serverlessv2_scaling_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rds::ClusterServerlessv2ScalingConfiguration>,
        >,
        /// Determines whether a final DB snapshot is created before the DB cluster is deleted. If true is specified, no DB snapshot is created. If false is specified, a DB snapshot is created before the DB cluster is deleted, using the value from `final_snapshot_identifier`. Default is `false`.
        #[builder(into, default)]
        pub skip_final_snapshot: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether or not to create this cluster from a snapshot. You can use either the name or ARN when specifying a DB cluster snapshot, or the ARN when specifying a DB snapshot. Conflicts with `global_cluster_identifier`. Clusters cannot be restored from snapshot **and** joined to an existing global cluster in a single operation. See the [AWS documentation](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-global-database-getting-started.html#aurora-global-database.use-snapshot) or the Global Cluster Restored From Snapshot example for instructions on building a global cluster starting with a snapshot.
        #[builder(into, default)]
        pub snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The source region for an encrypted replica DB cluster.
        #[builder(into, default)]
        pub source_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the DB cluster is encrypted. The default is `false` for `provisioned` `engine_mode` and `true` for `serverless` `engine_mode`. When restoring an unencrypted `snapshot_identifier`, the `kms_key_id` argument must be provided to encrypt the restored cluster. The provider will only perform drift detection if a configuration value is provided.
        #[builder(into, default)]
        pub storage_encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// (Forces new for Multi-AZ DB clusters) Specifies the storage type to be associated with the DB cluster. For Aurora DB clusters, `storage_type` modifications can be done in-place. For Multi-AZ DB Clusters, the `iops` argument must also be set. Valid values are: `""`, `aurora-iopt1` (Aurora DB Clusters); `io1`, `io2` (Multi-AZ DB Clusters). Default: `""` (Aurora DB Clusters); `io1` (Multi-AZ DB Clusters).
        #[builder(into, default)]
        pub storage_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the DB cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of VPC security groups to associate with the Cluster
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// The amount of storage in gibibytes (GiB) to allocate to each DB instance in the Multi-AZ DB cluster.
        pub allocated_storage: pulumi_gestalt_rust::Output<i32>,
        /// Enable to allow major engine version upgrades when changing engine versions. Defaults to `false`.
        pub allow_major_version_upgrade: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether any cluster modifications are applied immediately, or during the next maintenance window. Default is `false`. See [Amazon RDS Documentation for more information.](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Overview.DBInstance.Modifying.html)
        pub apply_immediately: pulumi_gestalt_rust::Output<bool>,
        /// Amazon Resource Name (ARN) of cluster
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// List of EC2 Availability Zones for the DB cluster storage where DB cluster instances can be created.
        /// RDS automatically assigns 3 AZs if less than 3 AZs are configured, which will show as a difference requiring resource recreation next pulumi up.
        /// We recommend specifying 3 AZs or using the `lifecycle` configuration block `ignore_changes` argument if necessary.
        /// A maximum of 3 AZs can be configured.
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Target backtrack window, in seconds. Only available for `aurora` and `aurora-mysql` engines currently. To disable backtracking, set this value to `0`. Defaults to `0`. Must be between `0` and `259200` (72 hours)
        pub backtrack_window: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Days to retain backups for. Default `1`
        pub backup_retention_period: pulumi_gestalt_rust::Output<i32>,
        /// The CA certificate identifier to use for the DB cluster's server certificate.
        pub ca_certificate_identifier: pulumi_gestalt_rust::Output<String>,
        /// Expiration date of the DB instance’s server certificate
        pub ca_certificate_valid_till: pulumi_gestalt_rust::Output<String>,
        /// The cluster identifier. If omitted, this provider will assign a random, unique identifier.
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique cluster identifier beginning with the specified prefix. Conflicts with `cluster_identifier`.
        pub cluster_identifier_prefix: pulumi_gestalt_rust::Output<String>,
        /// List of RDS Instances that are a part of this cluster
        pub cluster_members: pulumi_gestalt_rust::Output<Vec<String>>,
        /// RDS Cluster Resource ID
        pub cluster_resource_id: pulumi_gestalt_rust::Output<String>,
        /// Copy all Cluster `tags` to snapshots. Default is `false`.
        pub copy_tags_to_snapshot: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name for an automatically created database on cluster creation. There are different naming restrictions per database engine: [RDS Naming Constraints](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_Limits.html#RDS_Limits.Constraints)
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// The compute and memory capacity of each DB instance in the Multi-AZ DB cluster, for example `db.m6g.xlarge`. Not all DB instance classes are available in all AWS Regions, or for all database engines. For the full list of DB instance classes and availability for your engine, see [DB instance class](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Concepts.DBInstanceClass.html) in the Amazon RDS User Guide.
        pub db_cluster_instance_class: pulumi_gestalt_rust::Output<Option<String>>,
        /// A cluster parameter group to associate with the cluster.
        pub db_cluster_parameter_group_name: pulumi_gestalt_rust::Output<String>,
        /// Instance parameter group to associate with all instances of the DB cluster. The `db_instance_parameter_group_name` parameter is only valid in combination with the `allow_major_version_upgrade` parameter.
        pub db_instance_parameter_group_name: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// DB subnet group to associate with this DB cluster.
        /// **NOTE:** This must match the `db_subnet_group_name` specified on every `aws.rds.ClusterInstance` in the cluster.
        pub db_subnet_group_name: pulumi_gestalt_rust::Output<String>,
        /// For use with RDS Custom.
        pub db_system_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether to remove automated backups immediately after the DB cluster is deleted. Default is `true`.
        pub delete_automated_backups: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If the DB cluster should have deletion protection enabled.
        /// The database can't be deleted when this value is set to `true`.
        /// The default is `false`.
        pub deletion_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the Directory Service Active Directory domain to create the cluster in.
        pub domain: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the IAM role to be used when making API calls to the Directory Service.
        pub domain_iam_role_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether cluster should forward writes to an associated global cluster. Applied to secondary clusters to enable them to forward writes to an `aws.rds.GlobalCluster`'s primary cluster. See the [User Guide for Aurora](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-global-database-write-forwarding.html) for more information.
        pub enable_global_write_forwarding: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Enable HTTP endpoint (data API). Only valid for some combinations of `engine_mode`, `engine` and `engine_version` and only available in some regions. See the [Region and version availability](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/data-api.html#data-api.regions) section of the documentation. This option also does not work with any of these options specified: `snapshot_identifier`, `replication_source_identifier`, `s3_import`.
        pub enable_http_endpoint: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether read replicas can forward write operations to the writer DB instance in the DB cluster. By default, write operations aren't allowed on reader DB instances.. See the [User Guide for Aurora](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-mysql-write-forwarding.html) for more information. **NOTE:** Local write forwarding requires Aurora MySQL version 3.04 or higher.
        pub enable_local_write_forwarding: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Set of log types to export to cloudwatch. If omitted, no logs will be exported. The following log types are supported: `audit`, `error`, `general`, `slowquery`, `postgresql` (PostgreSQL).
        pub enabled_cloudwatch_logs_exports: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// DNS address of the RDS instance
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Name of the database engine to be used for this DB cluster. Valid Values: `aurora-mysql`, `aurora-postgresql`, `mysql`, `postgres`. (Note that `mysql` and `postgres` are Multi-AZ RDS clusters).
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// The life cycle type for this DB instance. This setting is valid for cluster types Aurora DB clusters and Multi-AZ DB clusters. Valid values are `open-source-rds-extended-support`, `open-source-rds-extended-support-disabled`. Default value is `open-source-rds-extended-support`. [Using Amazon RDS Extended Support]: https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/extended-support.html
        pub engine_lifecycle_support: pulumi_gestalt_rust::Output<String>,
        /// Database engine mode. Valid values: `global` (only valid for Aurora MySQL 1.21 and earlier), `parallelquery`, `provisioned`, `serverless`. Defaults to: `provisioned`. See the [RDS User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-serverless.html) for limitations when using `serverless`.
        pub engine_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// Database engine version. Updating this argument results in an outage. See the [Aurora MySQL](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Updates.html) and [Aurora Postgres](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraPostgreSQL.Updates.html) documentation for your configured engine to determine this value, or by running `aws rds describe-db-engine-versions`. For example with Aurora MySQL 2, a potential value for this argument is `5.7.mysql_aurora.2.03.2`. The value can contain a partial version where supported by the API. The actual engine version used is returned in the attribute `engine_version_actual`, , see Attribute Reference below.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// Running version of the database.
        pub engine_version_actual: pulumi_gestalt_rust::Output<String>,
        /// Name of your final DB snapshot when this DB cluster is deleted. If omitted, no final snapshot will be made.
        pub final_snapshot_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// Global cluster identifier specified on `aws.rds.GlobalCluster`.
        pub global_cluster_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// Route53 Hosted Zone ID of the endpoint
        pub hosted_zone_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether or not mappings of AWS Identity and Access Management (IAM) accounts to database accounts is enabled. Please see [AWS Documentation](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/UsingWithRDS.IAMDBAuth.html) for availability and limitations.
        pub iam_database_authentication_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// List of ARNs for the IAM roles to associate to the RDS Cluster.
        pub iam_roles: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Amount of Provisioned IOPS (input/output operations per second) to be initially allocated for each DB instance in the Multi-AZ DB cluster. For information about valid Iops values, see [Amazon RDS Provisioned IOPS storage to improve performance](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_Storage.html#USER_PIOPS) in the Amazon RDS User Guide. (This setting is required to create a Multi-AZ DB cluster). Must be a multiple between .5 and 50 of the storage amount for the DB cluster.
        pub iops: pulumi_gestalt_rust::Output<Option<i32>>,
        /// ARN for the KMS encryption key. When specifying `kms_key_id`, `storage_encrypted` needs to be set to true.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Set to true to allow RDS to manage the master user password in Secrets Manager. Cannot be set if `master_password` is provided.
        pub manage_master_user_password: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Password for the master DB user. Note that this may show up in logs, and it will be stored in the state file. Please refer to the [RDS Naming Constraints](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_Limits.html#RDS_Limits.Constraints). Cannot be set if `manage_master_user_password` is set to `true`.
        pub master_password: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key. To use a KMS key in a different Amazon Web Services account, specify the key ARN or alias ARN. If not specified, the default KMS key for your Amazon Web Services account is used.
        pub master_user_secret_kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Block that specifies the master user secret. Only available when `manage_master_user_password` is set to true. Documented below.
        pub master_user_secrets: pulumi_gestalt_rust::Output<
            Vec<super::super::types::rds::ClusterMasterUserSecret>,
        >,
        /// Username for the master DB user. Please refer to the [RDS Naming Constraints](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_Limits.html#RDS_Limits.Constraints). This argument does not support in-place updates and cannot be changed during a restore from snapshot.
        pub master_username: pulumi_gestalt_rust::Output<String>,
        /// Network type of the cluster. Valid values: `IPV4`, `DUAL`.
        pub network_type: pulumi_gestalt_rust::Output<String>,
        /// Enables Performance Insights for the RDS Cluster
        pub performance_insights_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the KMS Key ID to encrypt Performance Insights data. If not specified, the default RDS KMS key will be used (`aws/rds`).
        pub performance_insights_kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the amount of time to retain performance insights data for. Defaults to 7 days if Performance Insights are enabled. Valid values are `7`, `month * 31` (where month is a number of months from 1-23), and `731`. See [here](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PerfInsights.Overview.cost.html) for more information on retention periods.
        pub performance_insights_retention_period: pulumi_gestalt_rust::Output<i32>,
        /// Port on which the DB accepts connections.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// Daily time range during which automated backups are created if automated backups are enabled using the BackupRetentionPeriod parameter.Time in UTC. Default: A 30-minute window selected at random from an 8-hour block of time per region, e.g. `04:00-09:00`.
        pub preferred_backup_window: pulumi_gestalt_rust::Output<String>,
        /// Weekly time range during which system maintenance can occur, in (UTC) e.g., `wed:04:00-wed:04:30`
        pub preferred_maintenance_window: pulumi_gestalt_rust::Output<String>,
        /// Read-only endpoint for the Aurora cluster, automatically
        /// load-balanced across replicas
        pub reader_endpoint: pulumi_gestalt_rust::Output<String>,
        /// ARN of a source DB cluster or DB instance if this DB cluster is to be created as a Read Replica. **Note:** Removing this attribute after creation will promote the read replica to a standalone cluster. If DB Cluster is part of a Global Cluster, use the `ignoreChanges` resource option to prevent Pulumi from showing differences for this argument instead of configuring this value.
        pub replication_source_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// Nested attribute for [point in time restore](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-pitr.html). More details below.
        pub restore_to_point_in_time: pulumi_gestalt_rust::Output<
            Option<super::super::types::rds::ClusterRestoreToPointInTime>,
        >,
        pub s3_import: pulumi_gestalt_rust::Output<
            Option<super::super::types::rds::ClusterS3Import>,
        >,
        /// Nested attribute with scaling properties. Only valid when `engine_mode` is set to `serverless`. More details below.
        pub scaling_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::rds::ClusterScalingConfiguration>,
        >,
        /// Nested attribute with scaling properties for ServerlessV2. Only valid when `engine_mode` is set to `provisioned`. More details below.
        pub serverlessv2_scaling_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::rds::ClusterServerlessv2ScalingConfiguration>,
        >,
        /// Determines whether a final DB snapshot is created before the DB cluster is deleted. If true is specified, no DB snapshot is created. If false is specified, a DB snapshot is created before the DB cluster is deleted, using the value from `final_snapshot_identifier`. Default is `false`.
        pub skip_final_snapshot: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether or not to create this cluster from a snapshot. You can use either the name or ARN when specifying a DB cluster snapshot, or the ARN when specifying a DB snapshot. Conflicts with `global_cluster_identifier`. Clusters cannot be restored from snapshot **and** joined to an existing global cluster in a single operation. See the [AWS documentation](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-global-database-getting-started.html#aurora-global-database.use-snapshot) or the Global Cluster Restored From Snapshot example for instructions on building a global cluster starting with a snapshot.
        pub snapshot_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// The source region for an encrypted replica DB cluster.
        pub source_region: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether the DB cluster is encrypted. The default is `false` for `provisioned` `engine_mode` and `true` for `serverless` `engine_mode`. When restoring an unencrypted `snapshot_identifier`, the `kms_key_id` argument must be provided to encrypt the restored cluster. The provider will only perform drift detection if a configuration value is provided.
        pub storage_encrypted: pulumi_gestalt_rust::Output<bool>,
        /// (Forces new for Multi-AZ DB clusters) Specifies the storage type to be associated with the DB cluster. For Aurora DB clusters, `storage_type` modifications can be done in-place. For Multi-AZ DB Clusters, the `iops` argument must also be set. Valid values are: `""`, `aurora-iopt1` (Aurora DB Clusters); `io1`, `io2` (Multi-AZ DB Clusters). Default: `""` (Aurora DB Clusters); `io1` (Multi-AZ DB Clusters).
        pub storage_type: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the DB cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// List of VPC security groups to associate with the Cluster
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
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
        let allocated_storage_binding = args.allocated_storage.get_output(context);
        let allow_major_version_upgrade_binding = args
            .allow_major_version_upgrade
            .get_output(context);
        let apply_immediately_binding = args.apply_immediately.get_output(context);
        let availability_zones_binding = args.availability_zones.get_output(context);
        let backtrack_window_binding = args.backtrack_window.get_output(context);
        let backup_retention_period_binding = args
            .backup_retention_period
            .get_output(context);
        let ca_certificate_identifier_binding = args
            .ca_certificate_identifier
            .get_output(context);
        let cluster_identifier_binding = args.cluster_identifier.get_output(context);
        let cluster_identifier_prefix_binding = args
            .cluster_identifier_prefix
            .get_output(context);
        let cluster_members_binding = args.cluster_members.get_output(context);
        let copy_tags_to_snapshot_binding = args
            .copy_tags_to_snapshot
            .get_output(context);
        let database_name_binding = args.database_name.get_output(context);
        let db_cluster_instance_class_binding = args
            .db_cluster_instance_class
            .get_output(context);
        let db_cluster_parameter_group_name_binding = args
            .db_cluster_parameter_group_name
            .get_output(context);
        let db_instance_parameter_group_name_binding = args
            .db_instance_parameter_group_name
            .get_output(context);
        let db_subnet_group_name_binding = args.db_subnet_group_name.get_output(context);
        let db_system_id_binding = args.db_system_id.get_output(context);
        let delete_automated_backups_binding = args
            .delete_automated_backups
            .get_output(context);
        let deletion_protection_binding = args.deletion_protection.get_output(context);
        let domain_binding = args.domain.get_output(context);
        let domain_iam_role_name_binding = args.domain_iam_role_name.get_output(context);
        let enable_global_write_forwarding_binding = args
            .enable_global_write_forwarding
            .get_output(context);
        let enable_http_endpoint_binding = args.enable_http_endpoint.get_output(context);
        let enable_local_write_forwarding_binding = args
            .enable_local_write_forwarding
            .get_output(context);
        let enabled_cloudwatch_logs_exports_binding = args
            .enabled_cloudwatch_logs_exports
            .get_output(context);
        let engine_binding = args.engine.get_output(context);
        let engine_lifecycle_support_binding = args
            .engine_lifecycle_support
            .get_output(context);
        let engine_mode_binding = args.engine_mode.get_output(context);
        let engine_version_binding = args.engine_version.get_output(context);
        let final_snapshot_identifier_binding = args
            .final_snapshot_identifier
            .get_output(context);
        let global_cluster_identifier_binding = args
            .global_cluster_identifier
            .get_output(context);
        let iam_database_authentication_enabled_binding = args
            .iam_database_authentication_enabled
            .get_output(context);
        let iam_roles_binding = args.iam_roles.get_output(context);
        let iops_binding = args.iops.get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let manage_master_user_password_binding = args
            .manage_master_user_password
            .get_output(context);
        let master_password_binding = args.master_password.get_output(context);
        let master_user_secret_kms_key_id_binding = args
            .master_user_secret_kms_key_id
            .get_output(context);
        let master_username_binding = args.master_username.get_output(context);
        let network_type_binding = args.network_type.get_output(context);
        let performance_insights_enabled_binding = args
            .performance_insights_enabled
            .get_output(context);
        let performance_insights_kms_key_id_binding = args
            .performance_insights_kms_key_id
            .get_output(context);
        let performance_insights_retention_period_binding = args
            .performance_insights_retention_period
            .get_output(context);
        let port_binding = args.port.get_output(context);
        let preferred_backup_window_binding = args
            .preferred_backup_window
            .get_output(context);
        let preferred_maintenance_window_binding = args
            .preferred_maintenance_window
            .get_output(context);
        let replication_source_identifier_binding = args
            .replication_source_identifier
            .get_output(context);
        let restore_to_point_in_time_binding = args
            .restore_to_point_in_time
            .get_output(context);
        let s3_import_binding = args.s3_import.get_output(context);
        let scaling_configuration_binding = args
            .scaling_configuration
            .get_output(context);
        let serverlessv2_scaling_configuration_binding = args
            .serverlessv2_scaling_configuration
            .get_output(context);
        let skip_final_snapshot_binding = args.skip_final_snapshot.get_output(context);
        let snapshot_identifier_binding = args.snapshot_identifier.get_output(context);
        let source_region_binding = args.source_region.get_output(context);
        let storage_encrypted_binding = args.storage_encrypted.get_output(context);
        let storage_type_binding = args.storage_type.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_security_group_ids_binding = args
            .vpc_security_group_ids
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rds/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allocatedStorage".into(),
                    value: &allocated_storage_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowMajorVersionUpgrade".into(),
                    value: &allow_major_version_upgrade_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applyImmediately".into(),
                    value: &apply_immediately_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZones".into(),
                    value: &availability_zones_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backtrackWindow".into(),
                    value: &backtrack_window_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupRetentionPeriod".into(),
                    value: &backup_retention_period_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "caCertificateIdentifier".into(),
                    value: &ca_certificate_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIdentifierPrefix".into(),
                    value: &cluster_identifier_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterMembers".into(),
                    value: &cluster_members_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "copyTagsToSnapshot".into(),
                    value: &copy_tags_to_snapshot_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbClusterInstanceClass".into(),
                    value: &db_cluster_instance_class_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbClusterParameterGroupName".into(),
                    value: &db_cluster_parameter_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbInstanceParameterGroupName".into(),
                    value: &db_instance_parameter_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbSubnetGroupName".into(),
                    value: &db_subnet_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbSystemId".into(),
                    value: &db_system_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deleteAutomatedBackups".into(),
                    value: &delete_automated_backups_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: &domain_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainIamRoleName".into(),
                    value: &domain_iam_role_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableGlobalWriteForwarding".into(),
                    value: &enable_global_write_forwarding_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableHttpEndpoint".into(),
                    value: &enable_http_endpoint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableLocalWriteForwarding".into(),
                    value: &enable_local_write_forwarding_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabledCloudwatchLogsExports".into(),
                    value: &enabled_cloudwatch_logs_exports_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engine".into(),
                    value: &engine_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineLifecycleSupport".into(),
                    value: &engine_lifecycle_support_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineMode".into(),
                    value: &engine_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineVersion".into(),
                    value: &engine_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "finalSnapshotIdentifier".into(),
                    value: &final_snapshot_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalClusterIdentifier".into(),
                    value: &global_cluster_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iamDatabaseAuthenticationEnabled".into(),
                    value: &iam_database_authentication_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iamRoles".into(),
                    value: &iam_roles_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iops".into(),
                    value: &iops_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "manageMasterUserPassword".into(),
                    value: &manage_master_user_password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "masterPassword".into(),
                    value: &master_password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "masterUserSecretKmsKeyId".into(),
                    value: &master_user_secret_kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "masterUsername".into(),
                    value: &master_username_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkType".into(),
                    value: &network_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "performanceInsightsEnabled".into(),
                    value: &performance_insights_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "performanceInsightsKmsKeyId".into(),
                    value: &performance_insights_kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "performanceInsightsRetentionPeriod".into(),
                    value: &performance_insights_retention_period_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: &port_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredBackupWindow".into(),
                    value: &preferred_backup_window_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredMaintenanceWindow".into(),
                    value: &preferred_maintenance_window_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationSourceIdentifier".into(),
                    value: &replication_source_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restoreToPointInTime".into(),
                    value: &restore_to_point_in_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3Import".into(),
                    value: &s3_import_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scalingConfiguration".into(),
                    value: &scaling_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverlessv2ScalingConfiguration".into(),
                    value: &serverlessv2_scaling_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipFinalSnapshot".into(),
                    value: &skip_final_snapshot_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotIdentifier".into(),
                    value: &snapshot_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceRegion".into(),
                    value: &source_region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageEncrypted".into(),
                    value: &storage_encrypted_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageType".into(),
                    value: &storage_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: &vpc_security_group_ids_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterResult {
            allocated_storage: o.get_field("allocatedStorage"),
            allow_major_version_upgrade: o.get_field("allowMajorVersionUpgrade"),
            apply_immediately: o.get_field("applyImmediately"),
            arn: o.get_field("arn"),
            availability_zones: o.get_field("availabilityZones"),
            backtrack_window: o.get_field("backtrackWindow"),
            backup_retention_period: o.get_field("backupRetentionPeriod"),
            ca_certificate_identifier: o.get_field("caCertificateIdentifier"),
            ca_certificate_valid_till: o.get_field("caCertificateValidTill"),
            cluster_identifier: o.get_field("clusterIdentifier"),
            cluster_identifier_prefix: o.get_field("clusterIdentifierPrefix"),
            cluster_members: o.get_field("clusterMembers"),
            cluster_resource_id: o.get_field("clusterResourceId"),
            copy_tags_to_snapshot: o.get_field("copyTagsToSnapshot"),
            database_name: o.get_field("databaseName"),
            db_cluster_instance_class: o.get_field("dbClusterInstanceClass"),
            db_cluster_parameter_group_name: o.get_field("dbClusterParameterGroupName"),
            db_instance_parameter_group_name: o
                .get_field("dbInstanceParameterGroupName"),
            db_subnet_group_name: o.get_field("dbSubnetGroupName"),
            db_system_id: o.get_field("dbSystemId"),
            delete_automated_backups: o.get_field("deleteAutomatedBackups"),
            deletion_protection: o.get_field("deletionProtection"),
            domain: o.get_field("domain"),
            domain_iam_role_name: o.get_field("domainIamRoleName"),
            enable_global_write_forwarding: o.get_field("enableGlobalWriteForwarding"),
            enable_http_endpoint: o.get_field("enableHttpEndpoint"),
            enable_local_write_forwarding: o.get_field("enableLocalWriteForwarding"),
            enabled_cloudwatch_logs_exports: o.get_field("enabledCloudwatchLogsExports"),
            endpoint: o.get_field("endpoint"),
            engine: o.get_field("engine"),
            engine_lifecycle_support: o.get_field("engineLifecycleSupport"),
            engine_mode: o.get_field("engineMode"),
            engine_version: o.get_field("engineVersion"),
            engine_version_actual: o.get_field("engineVersionActual"),
            final_snapshot_identifier: o.get_field("finalSnapshotIdentifier"),
            global_cluster_identifier: o.get_field("globalClusterIdentifier"),
            hosted_zone_id: o.get_field("hostedZoneId"),
            iam_database_authentication_enabled: o
                .get_field("iamDatabaseAuthenticationEnabled"),
            iam_roles: o.get_field("iamRoles"),
            iops: o.get_field("iops"),
            kms_key_id: o.get_field("kmsKeyId"),
            manage_master_user_password: o.get_field("manageMasterUserPassword"),
            master_password: o.get_field("masterPassword"),
            master_user_secret_kms_key_id: o.get_field("masterUserSecretKmsKeyId"),
            master_user_secrets: o.get_field("masterUserSecrets"),
            master_username: o.get_field("masterUsername"),
            network_type: o.get_field("networkType"),
            performance_insights_enabled: o.get_field("performanceInsightsEnabled"),
            performance_insights_kms_key_id: o.get_field("performanceInsightsKmsKeyId"),
            performance_insights_retention_period: o
                .get_field("performanceInsightsRetentionPeriod"),
            port: o.get_field("port"),
            preferred_backup_window: o.get_field("preferredBackupWindow"),
            preferred_maintenance_window: o.get_field("preferredMaintenanceWindow"),
            reader_endpoint: o.get_field("readerEndpoint"),
            replication_source_identifier: o.get_field("replicationSourceIdentifier"),
            restore_to_point_in_time: o.get_field("restoreToPointInTime"),
            s3_import: o.get_field("s3Import"),
            scaling_configuration: o.get_field("scalingConfiguration"),
            serverlessv2_scaling_configuration: o
                .get_field("serverlessv2ScalingConfiguration"),
            skip_final_snapshot: o.get_field("skipFinalSnapshot"),
            snapshot_identifier: o.get_field("snapshotIdentifier"),
            source_region: o.get_field("sourceRegion"),
            storage_encrypted: o.get_field("storageEncrypted"),
            storage_type: o.get_field("storageType"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_security_group_ids: o.get_field("vpcSecurityGroupIds"),
        }
    }
}
