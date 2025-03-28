#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_replication_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReplicationInstanceArgs {
        /// The replication instance identifier.
        #[builder(into)]
        pub replication_instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetReplicationInstanceResult {
        /// The amount of storage (in gigabytes) to be initially allocated for the replication instance.
        pub allocated_storage: pulumi_gestalt_rust::Output<i32>,
        /// Indicates that minor engine upgrades will be applied automatically to the replication instance during the maintenance window.
        pub auto_minor_version_upgrade: pulumi_gestalt_rust::Output<bool>,
        /// The EC2 Availability Zone that the replication instance will be created in.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// The engine version number of the replication instance.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the KMS key used to encrypt the connection parameters.
        pub kms_key_arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies if the replication instance is a multi-az deployment.
        pub multi_az: pulumi_gestalt_rust::Output<bool>,
        /// The type of IP address protocol used by the replication instance.
        pub network_type: pulumi_gestalt_rust::Output<String>,
        /// The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).
        pub preferred_maintenance_window: pulumi_gestalt_rust::Output<String>,
        /// Specifies the accessibility options for the replication instance. A value of true represents an instance with a public IP address. A value of false represents an instance with a private IP address.
        pub publicly_accessible: pulumi_gestalt_rust::Output<bool>,
        /// The Amazon Resource Name (ARN) of the replication instance.
        pub replication_instance_arn: pulumi_gestalt_rust::Output<String>,
        /// The compute and memory capacity of the replication instance as specified by the replication instance class. See [AWS DMS User Guide](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_ReplicationInstance.Types.html) for information on instance classes.
        pub replication_instance_class: pulumi_gestalt_rust::Output<String>,
        pub replication_instance_id: pulumi_gestalt_rust::Output<String>,
        /// A list of the private IP addresses of the replication instance.
        pub replication_instance_private_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of the public IP addresses of the replication instance.
        pub replication_instance_public_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A subnet group to associate with the replication instance.
        pub replication_subnet_group_id: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// A set of VPC security group IDs that are used with the replication instance.
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetReplicationInstanceArgs,
    ) -> GetReplicationInstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let replication_instance_id_binding = args
            .replication_instance_id
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:dms/getReplicationInstance:getReplicationInstance".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationInstanceId".into(),
                    value: &replication_instance_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetReplicationInstanceResult {
            allocated_storage: o.get_field("allocatedStorage"),
            auto_minor_version_upgrade: o.get_field("autoMinorVersionUpgrade"),
            availability_zone: o.get_field("availabilityZone"),
            engine_version: o.get_field("engineVersion"),
            id: o.get_field("id"),
            kms_key_arn: o.get_field("kmsKeyArn"),
            multi_az: o.get_field("multiAz"),
            network_type: o.get_field("networkType"),
            preferred_maintenance_window: o.get_field("preferredMaintenanceWindow"),
            publicly_accessible: o.get_field("publiclyAccessible"),
            replication_instance_arn: o.get_field("replicationInstanceArn"),
            replication_instance_class: o.get_field("replicationInstanceClass"),
            replication_instance_id: o.get_field("replicationInstanceId"),
            replication_instance_private_ips: o
                .get_field("replicationInstancePrivateIps"),
            replication_instance_public_ips: o.get_field("replicationInstancePublicIps"),
            replication_subnet_group_id: o.get_field("replicationSubnetGroupId"),
            tags: o.get_field("tags"),
            vpc_security_group_ids: o.get_field("vpcSecurityGroupIds"),
        }
    }
}
