/// Provides an Amazon MQ broker resource. This resources also manages users for the broker.
///
/// > For more information on Amazon MQ, see [Amazon MQ documentation](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/welcome.html).
///
/// > **NOTE:** Amazon MQ currently places limits on **RabbitMQ** brokers. For example, a RabbitMQ broker cannot have: instances with an associated IP address of an ENI attached to the broker, an associated LDAP server to authenticate and authorize broker connections, storage type `EFS`, or audit logging. Although this resource allows you to create RabbitMQ users, RabbitMQ users cannot have console access or groups. Also, Amazon MQ does not return information about RabbitMQ users so drift detection is not possible.
///
/// > **NOTE:** Changes to an MQ Broker can occur when you change a parameter, such as `configuration` or `user`, and are reflected in the next maintenance window. Because of this, the provider may report a difference in its planning phase because a modification has not yet taken place. You can use the `apply_immediately` flag to instruct the service to apply the change immediately (see documentation below). Using `apply_immediately` can result in a brief downtime as the broker reboots.
///
///
/// ## Example Usage
///
/// ### Basic Example
///
/// ```yaml
/// resources:
///   example:
///     type: aws:mq:Broker
///     properties:
///       brokerName: example
///       configuration:
///         id: ${test.id}
///         revision: ${test.latestRevision}
///       engineType: ActiveMQ
///       engineVersion: 5.17.6
///       hostInstanceType: mq.t2.micro
///       securityGroups:
///         - ${testAwsSecurityGroup.id}
///       users:
///         - username: ExampleUser
///           password: MindTheGap
/// ```
///
/// ### High-throughput Optimized Example
///
/// This example shows the use of EBS storage for high-throughput optimized performance.
///
/// ```yaml
/// resources:
///   example:
///     type: aws:mq:Broker
///     properties:
///       brokerName: example
///       configuration:
///         id: ${test.id}
///         revision: ${test.latestRevision}
///       engineType: ActiveMQ
///       engineVersion: 5.17.6
///       storageType: ebs
///       hostInstanceType: mq.m5.large
///       securityGroups:
///         - ${testAwsSecurityGroup.id}
///       users:
///         - username: ExampleUser
///           password: MindTheGap
/// ```
///
/// ### Cross-Region Data Replication
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = broker::create(
///         "example",
///         BrokerArgs::builder()
///             .apply_immediately(true)
///             .broker_name("example")
///             .data_replication_mode("CRDR")
///             .data_replication_primary_broker_arn("${primary.arn}")
///             .deployment_mode("ACTIVE_STANDBY_MULTI_AZ")
///             .engine_type("ActiveMQ")
///             .engine_version("5.17.6")
///             .host_instance_type("mq.m5.large")
///             .security_groups(vec!["${exampleAwsSecurityGroup.id}",])
///             .users(
///                 vec![
///                     BrokerUser::builder().password("MindTheGap").username("ExampleUser")
///                     .build_struct(), BrokerUser::builder().password("Example12345")
///                     .replicationUser(true).username("ExampleReplicationUser")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let examplePrimary = broker::create(
///         "examplePrimary",
///         BrokerArgs::builder()
///             .apply_immediately(true)
///             .broker_name("example_primary")
///             .deployment_mode("ACTIVE_STANDBY_MULTI_AZ")
///             .engine_type("ActiveMQ")
///             .engine_version("5.17.6")
///             .host_instance_type("mq.m5.large")
///             .security_groups(vec!["${examplePrimaryAwsSecurityGroup.id}",])
///             .users(
///                 vec![
///                     BrokerUser::builder().password("MindTheGap").username("ExampleUser")
///                     .build_struct(), BrokerUser::builder().password("Example12345")
///                     .replicationUser(true).username("ExampleReplicationUser")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// See the [AWS MQ documentation](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/crdr-for-active-mq.html) on cross-region data replication for additional details.
///
/// ## Import
///
/// Using `pulumi import`, import MQ Brokers using their broker id. For example:
///
/// ```sh
/// $ pulumi import aws:mq/broker:Broker example a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod broker {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BrokerArgs {
        /// Specifies whether any broker modifications are applied immediately, or during the next maintenance window. Default is `false`.
        #[builder(into, default)]
        pub apply_immediately: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Authentication strategy used to secure the broker. Valid values are `simple` and `ldap`. `ldap` is not supported for `engine_type` `RabbitMQ`.
        #[builder(into, default)]
        pub authentication_strategy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to automatically upgrade to new minor versions of brokers as Amazon MQ makes releases available.
        #[builder(into, default)]
        pub auto_minor_version_upgrade: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the broker.
        #[builder(into, default)]
        pub broker_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for broker configuration. Applies to `engine_type` of `ActiveMQ` and `RabbitMQ` only. Detailed below.
        #[builder(into, default)]
        pub configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mq::BrokerConfiguration>,
        >,
        /// Defines whether this broker is a part of a data replication pair. Valid values are `CRDR` and `NONE`.
        #[builder(into, default)]
        pub data_replication_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of the primary broker that is used to replicate data from in a data replication pair, and is applied to the replica broker. Must be set when `data_replication_mode` is `CRDR`.
        #[builder(into, default)]
        pub data_replication_primary_broker_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Deployment mode of the broker. Valid values are `SINGLE_INSTANCE`, `ACTIVE_STANDBY_MULTI_AZ`, and `CLUSTER_MULTI_AZ`. Default is `SINGLE_INSTANCE`.
        #[builder(into, default)]
        pub deployment_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block containing encryption options. Detailed below.
        #[builder(into, default)]
        pub encryption_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mq::BrokerEncryptionOptions>,
        >,
        /// Type of broker engine. Valid values are `ActiveMQ` and `RabbitMQ`.
        #[builder(into)]
        pub engine_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version of the broker engine. See the [AmazonMQ Broker Engine docs](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/broker-engine.html) for supported versions. For example, `5.17.6`.
        #[builder(into)]
        pub engine_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Broker's instance type. For example, `mq.t3.micro`, `mq.m5.large`.
        #[builder(into)]
        pub host_instance_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block for the LDAP server used to authenticate and authorize connections to the broker. Not supported for `engine_type` `RabbitMQ`. Detailed below. (Currently, AWS may not process changes to LDAP server metadata.)
        #[builder(into, default)]
        pub ldap_server_metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mq::BrokerLdapServerMetadata>,
        >,
        /// Configuration block for the logging configuration of the broker. Detailed below.
        #[builder(into, default)]
        pub logs: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mq::BrokerLogs>,
        >,
        /// Configuration block for the maintenance window start time. Detailed below.
        #[builder(into, default)]
        pub maintenance_window_start_time: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mq::BrokerMaintenanceWindowStartTime>,
        >,
        /// Whether to enable connections from applications outside of the VPC that hosts the broker's subnets.
        #[builder(into, default)]
        pub publicly_accessible: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of security group IDs assigned to the broker.
        #[builder(into, default)]
        pub security_groups: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Storage type of the broker. For `engine_type` `ActiveMQ`, the valid values are `efs` and `ebs`, and the AWS-default is `efs`. For `engine_type` `RabbitMQ`, only `ebs` is supported. When using `ebs`, only the `mq.m5` broker instance type family is supported.
        #[builder(into, default)]
        pub storage_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of subnet IDs in which to launch the broker. A `SINGLE_INSTANCE` deployment requires one subnet. An `ACTIVE_STANDBY_MULTI_AZ` deployment requires multiple subnets.
        #[builder(into, default)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Map of tags to assign to the broker. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for broker users. For `engine_type` of `RabbitMQ`, Amazon MQ does not return broker users preventing this resource from making user updates and drift detection. Detailed below.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub users: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::mq::BrokerUser>,
        >,
    }
    #[allow(dead_code)]
    pub struct BrokerResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether any broker modifications are applied immediately, or during the next maintenance window. Default is `false`.
        pub apply_immediately: pulumi_gestalt_rust::Output<Option<bool>>,
        /// ARN of the broker.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Authentication strategy used to secure the broker. Valid values are `simple` and `ldap`. `ldap` is not supported for `engine_type` `RabbitMQ`.
        pub authentication_strategy: pulumi_gestalt_rust::Output<String>,
        /// Whether to automatically upgrade to new minor versions of brokers as Amazon MQ makes releases available.
        pub auto_minor_version_upgrade: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the broker.
        pub broker_name: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for broker configuration. Applies to `engine_type` of `ActiveMQ` and `RabbitMQ` only. Detailed below.
        pub configuration: pulumi_gestalt_rust::Output<
            super::super::types::mq::BrokerConfiguration,
        >,
        /// Defines whether this broker is a part of a data replication pair. Valid values are `CRDR` and `NONE`.
        pub data_replication_mode: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the primary broker that is used to replicate data from in a data replication pair, and is applied to the replica broker. Must be set when `data_replication_mode` is `CRDR`.
        pub data_replication_primary_broker_arn: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Deployment mode of the broker. Valid values are `SINGLE_INSTANCE`, `ACTIVE_STANDBY_MULTI_AZ`, and `CLUSTER_MULTI_AZ`. Default is `SINGLE_INSTANCE`.
        pub deployment_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block containing encryption options. Detailed below.
        pub encryption_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::mq::BrokerEncryptionOptions>,
        >,
        /// Type of broker engine. Valid values are `ActiveMQ` and `RabbitMQ`.
        pub engine_type: pulumi_gestalt_rust::Output<String>,
        /// Version of the broker engine. See the [AmazonMQ Broker Engine docs](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/broker-engine.html) for supported versions. For example, `5.17.6`.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// Broker's instance type. For example, `mq.t3.micro`, `mq.m5.large`.
        pub host_instance_type: pulumi_gestalt_rust::Output<String>,
        /// List of information about allocated brokers (both active & standby).
        pub instances: pulumi_gestalt_rust::Output<
            Vec<super::super::types::mq::BrokerInstance>,
        >,
        /// Configuration block for the LDAP server used to authenticate and authorize connections to the broker. Not supported for `engine_type` `RabbitMQ`. Detailed below. (Currently, AWS may not process changes to LDAP server metadata.)
        pub ldap_server_metadata: pulumi_gestalt_rust::Output<
            Option<super::super::types::mq::BrokerLdapServerMetadata>,
        >,
        /// Configuration block for the logging configuration of the broker. Detailed below.
        pub logs: pulumi_gestalt_rust::Output<
            Option<super::super::types::mq::BrokerLogs>,
        >,
        /// Configuration block for the maintenance window start time. Detailed below.
        pub maintenance_window_start_time: pulumi_gestalt_rust::Output<
            super::super::types::mq::BrokerMaintenanceWindowStartTime,
        >,
        /// (Optional) The data replication mode that will be applied after reboot.
        pub pending_data_replication_mode: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable connections from applications outside of the VPC that hosts the broker's subnets.
        pub publicly_accessible: pulumi_gestalt_rust::Output<Option<bool>>,
        /// List of security group IDs assigned to the broker.
        pub security_groups: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Storage type of the broker. For `engine_type` `ActiveMQ`, the valid values are `efs` and `ebs`, and the AWS-default is `efs`. For `engine_type` `RabbitMQ`, only `ebs` is supported. When using `ebs`, only the `mq.m5` broker instance type family is supported.
        pub storage_type: pulumi_gestalt_rust::Output<String>,
        /// List of subnet IDs in which to launch the broker. A `SINGLE_INSTANCE` deployment requires one subnet. An `ACTIVE_STANDBY_MULTI_AZ` deployment requires multiple subnets.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Map of tags to assign to the broker. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for broker users. For `engine_type` of `RabbitMQ`, Amazon MQ does not return broker users preventing this resource from making user updates and drift detection. Detailed below.
        ///
        /// The following arguments are optional:
        pub users: pulumi_gestalt_rust::Output<Vec<super::super::types::mq::BrokerUser>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BrokerArgs,
    ) -> BrokerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let apply_immediately_binding = args.apply_immediately.get_output(context);
        let authentication_strategy_binding = args
            .authentication_strategy
            .get_output(context);
        let auto_minor_version_upgrade_binding = args
            .auto_minor_version_upgrade
            .get_output(context);
        let broker_name_binding = args.broker_name.get_output(context);
        let configuration_binding = args.configuration.get_output(context);
        let data_replication_mode_binding = args
            .data_replication_mode
            .get_output(context);
        let data_replication_primary_broker_arn_binding = args
            .data_replication_primary_broker_arn
            .get_output(context);
        let deployment_mode_binding = args.deployment_mode.get_output(context);
        let encryption_options_binding = args.encryption_options.get_output(context);
        let engine_type_binding = args.engine_type.get_output(context);
        let engine_version_binding = args.engine_version.get_output(context);
        let host_instance_type_binding = args.host_instance_type.get_output(context);
        let ldap_server_metadata_binding = args.ldap_server_metadata.get_output(context);
        let logs_binding = args.logs.get_output(context);
        let maintenance_window_start_time_binding = args
            .maintenance_window_start_time
            .get_output(context);
        let publicly_accessible_binding = args.publicly_accessible.get_output(context);
        let security_groups_binding = args.security_groups.get_output(context);
        let storage_type_binding = args.storage_type.get_output(context);
        let subnet_ids_binding = args.subnet_ids.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let users_binding = args.users.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:mq/broker:Broker".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applyImmediately".into(),
                    value: &apply_immediately_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationStrategy".into(),
                    value: &authentication_strategy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoMinorVersionUpgrade".into(),
                    value: &auto_minor_version_upgrade_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "brokerName".into(),
                    value: &broker_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataReplicationMode".into(),
                    value: &data_replication_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataReplicationPrimaryBrokerArn".into(),
                    value: &data_replication_primary_broker_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deploymentMode".into(),
                    value: &deployment_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionOptions".into(),
                    value: &encryption_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineType".into(),
                    value: &engine_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineVersion".into(),
                    value: &engine_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostInstanceType".into(),
                    value: &host_instance_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ldapServerMetadata".into(),
                    value: &ldap_server_metadata_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logs".into(),
                    value: &logs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenanceWindowStartTime".into(),
                    value: &maintenance_window_start_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publiclyAccessible".into(),
                    value: &publicly_accessible_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroups".into(),
                    value: &security_groups_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageType".into(),
                    value: &storage_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "users".into(),
                    value: &users_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BrokerResult {
            id: o.get_field("id"),
            apply_immediately: o.get_field("applyImmediately"),
            arn: o.get_field("arn"),
            authentication_strategy: o.get_field("authenticationStrategy"),
            auto_minor_version_upgrade: o.get_field("autoMinorVersionUpgrade"),
            broker_name: o.get_field("brokerName"),
            configuration: o.get_field("configuration"),
            data_replication_mode: o.get_field("dataReplicationMode"),
            data_replication_primary_broker_arn: o
                .get_field("dataReplicationPrimaryBrokerArn"),
            deployment_mode: o.get_field("deploymentMode"),
            encryption_options: o.get_field("encryptionOptions"),
            engine_type: o.get_field("engineType"),
            engine_version: o.get_field("engineVersion"),
            host_instance_type: o.get_field("hostInstanceType"),
            instances: o.get_field("instances"),
            ldap_server_metadata: o.get_field("ldapServerMetadata"),
            logs: o.get_field("logs"),
            maintenance_window_start_time: o.get_field("maintenanceWindowStartTime"),
            pending_data_replication_mode: o.get_field("pendingDataReplicationMode"),
            publicly_accessible: o.get_field("publiclyAccessible"),
            security_groups: o.get_field("securityGroups"),
            storage_type: o.get_field("storageType"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            users: o.get_field("users"),
        }
    }
}
