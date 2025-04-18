/// Provides an Elastic Disaster Recovery replication configuration template resource. Before using DRS, your account must be [initialized](https://docs.aws.amazon.com/drs/latest/userguide/getting-started-initializing.html).
///
/// > **NOTE:** Your configuration must use the PIT policy shown in the basic configuration due to AWS rules. The only value that you can change is the `retention_duration` of `rule_id` 3.
///
/// ## Example Usage
///
/// ## Import
///
/// Using `pulumi import`, import DRS Replication Configuration Template using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:drs/replicationConfigurationTemplate:ReplicationConfigurationTemplate example templateid
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod replication_configuration_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicationConfigurationTemplateArgs {
        /// Whether to associate the default Elastic Disaster Recovery Security group with the Replication Configuration Template.
        #[builder(into)]
        pub associate_default_security_group: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Whether to allow the AWS replication agent to automatically replicate newly added disks.
        #[builder(into, default)]
        pub auto_replicate_new_disks: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configure bandwidth throttling for the outbound data transfer rate of the Source Server in Mbps.
        #[builder(into)]
        pub bandwidth_throttling: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Whether to create a Public IP for the Recovery Instance by default.
        #[builder(into)]
        pub create_public_ip: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Data plane routing mechanism that will be used for replication. Valid values are `PUBLIC_IP` and `PRIVATE_IP`.
        #[builder(into)]
        pub data_plane_routing: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Staging Disk EBS volume type to be used during replication. Valid values are `GP2`, `GP3`, `ST1`, or `AUTO`.
        #[builder(into)]
        pub default_large_staging_disk_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of EBS encryption to be used during replication. Valid values are `DEFAULT` and `CUSTOM`.
        #[builder(into)]
        pub ebs_encryption: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the EBS encryption key to be used during replication.
        #[builder(into, default)]
        pub ebs_encryption_key_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for Point in time (PIT) policy to manage snapshots taken during replication. See below.
        #[builder(into, default)]
        pub pit_policies: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::drs::ReplicationConfigurationTemplatePitPolicy>,
            >,
        >,
        /// Instance type to be used for the replication server.
        #[builder(into)]
        pub replication_server_instance_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Security group IDs that will be used by the replication server.
        #[builder(into)]
        pub replication_servers_security_groups_ids: pulumi_gestalt_rust::InputOrOutput<
            Vec<String>,
        >,
        /// Subnet to be used by the replication staging area.
        #[builder(into)]
        pub staging_area_subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Set of tags to be associated with all resources created in the replication staging area: EC2 replication server, EBS volumes, EBS snapshots, etc.
        #[builder(into)]
        pub staging_area_tags: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, String>,
        >,
        /// Set of tags to be associated with the Replication Configuration Template resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::drs::ReplicationConfigurationTemplateTimeouts>,
        >,
        /// Whether to use a dedicated Replication Server in the replication staging area.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub use_dedicated_replication_server: pulumi_gestalt_rust::InputOrOutput<bool>,
    }
    #[allow(dead_code)]
    pub struct ReplicationConfigurationTemplateResult {
        /// Replication configuration template ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether to associate the default Elastic Disaster Recovery Security group with the Replication Configuration Template.
        pub associate_default_security_group: pulumi_gestalt_rust::Output<bool>,
        /// Whether to allow the AWS replication agent to automatically replicate newly added disks.
        pub auto_replicate_new_disks: pulumi_gestalt_rust::Output<bool>,
        /// Configure bandwidth throttling for the outbound data transfer rate of the Source Server in Mbps.
        pub bandwidth_throttling: pulumi_gestalt_rust::Output<i32>,
        /// Whether to create a Public IP for the Recovery Instance by default.
        pub create_public_ip: pulumi_gestalt_rust::Output<bool>,
        /// Data plane routing mechanism that will be used for replication. Valid values are `PUBLIC_IP` and `PRIVATE_IP`.
        pub data_plane_routing: pulumi_gestalt_rust::Output<String>,
        /// Staging Disk EBS volume type to be used during replication. Valid values are `GP2`, `GP3`, `ST1`, or `AUTO`.
        pub default_large_staging_disk_type: pulumi_gestalt_rust::Output<String>,
        /// Type of EBS encryption to be used during replication. Valid values are `DEFAULT` and `CUSTOM`.
        pub ebs_encryption: pulumi_gestalt_rust::Output<String>,
        /// ARN of the EBS encryption key to be used during replication.
        pub ebs_encryption_key_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block for Point in time (PIT) policy to manage snapshots taken during replication. See below.
        pub pit_policies: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::drs::ReplicationConfigurationTemplatePitPolicy>,
            >,
        >,
        /// Instance type to be used for the replication server.
        pub replication_server_instance_type: pulumi_gestalt_rust::Output<String>,
        /// Security group IDs that will be used by the replication server.
        pub replication_servers_security_groups_ids: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        /// Subnet to be used by the replication staging area.
        pub staging_area_subnet_id: pulumi_gestalt_rust::Output<String>,
        /// Set of tags to be associated with all resources created in the replication staging area: EC2 replication server, EBS volumes, EBS snapshots, etc.
        pub staging_area_tags: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of tags to be associated with the Replication Configuration Template resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::drs::ReplicationConfigurationTemplateTimeouts>,
        >,
        /// Whether to use a dedicated Replication Server in the replication staging area.
        ///
        /// The following arguments are optional:
        pub use_dedicated_replication_server: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReplicationConfigurationTemplateArgs,
    ) -> ReplicationConfigurationTemplateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let associate_default_security_group_binding = args
            .associate_default_security_group
            .get_output(context);
        let auto_replicate_new_disks_binding = args
            .auto_replicate_new_disks
            .get_output(context);
        let bandwidth_throttling_binding = args.bandwidth_throttling.get_output(context);
        let create_public_ip_binding = args.create_public_ip.get_output(context);
        let data_plane_routing_binding = args.data_plane_routing.get_output(context);
        let default_large_staging_disk_type_binding = args
            .default_large_staging_disk_type
            .get_output(context);
        let ebs_encryption_binding = args.ebs_encryption.get_output(context);
        let ebs_encryption_key_arn_binding = args
            .ebs_encryption_key_arn
            .get_output(context);
        let pit_policies_binding = args.pit_policies.get_output(context);
        let replication_server_instance_type_binding = args
            .replication_server_instance_type
            .get_output(context);
        let replication_servers_security_groups_ids_binding = args
            .replication_servers_security_groups_ids
            .get_output(context);
        let staging_area_subnet_id_binding = args
            .staging_area_subnet_id
            .get_output(context);
        let staging_area_tags_binding = args.staging_area_tags.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let use_dedicated_replication_server_binding = args
            .use_dedicated_replication_server
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:drs/replicationConfigurationTemplate:ReplicationConfigurationTemplate"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "associateDefaultSecurityGroup".into(),
                    value: &associate_default_security_group_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoReplicateNewDisks".into(),
                    value: &auto_replicate_new_disks_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bandwidthThrottling".into(),
                    value: &bandwidth_throttling_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createPublicIp".into(),
                    value: &create_public_ip_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataPlaneRouting".into(),
                    value: &data_plane_routing_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultLargeStagingDiskType".into(),
                    value: &default_large_staging_disk_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ebsEncryption".into(),
                    value: &ebs_encryption_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ebsEncryptionKeyArn".into(),
                    value: &ebs_encryption_key_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pitPolicies".into(),
                    value: &pit_policies_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationServerInstanceType".into(),
                    value: &replication_server_instance_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationServersSecurityGroupsIds".into(),
                    value: &replication_servers_security_groups_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stagingAreaSubnetId".into(),
                    value: &staging_area_subnet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stagingAreaTags".into(),
                    value: &staging_area_tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "useDedicatedReplicationServer".into(),
                    value: &use_dedicated_replication_server_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReplicationConfigurationTemplateResult {
            arn: o.get_field("arn"),
            associate_default_security_group: o
                .get_field("associateDefaultSecurityGroup"),
            auto_replicate_new_disks: o.get_field("autoReplicateNewDisks"),
            bandwidth_throttling: o.get_field("bandwidthThrottling"),
            create_public_ip: o.get_field("createPublicIp"),
            data_plane_routing: o.get_field("dataPlaneRouting"),
            default_large_staging_disk_type: o.get_field("defaultLargeStagingDiskType"),
            ebs_encryption: o.get_field("ebsEncryption"),
            ebs_encryption_key_arn: o.get_field("ebsEncryptionKeyArn"),
            pit_policies: o.get_field("pitPolicies"),
            replication_server_instance_type: o
                .get_field("replicationServerInstanceType"),
            replication_servers_security_groups_ids: o
                .get_field("replicationServersSecurityGroupsIds"),
            staging_area_subnet_id: o.get_field("stagingAreaSubnetId"),
            staging_area_tags: o.get_field("stagingAreaTags"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
            use_dedicated_replication_server: o
                .get_field("useDedicatedReplicationServer"),
        }
    }
}
