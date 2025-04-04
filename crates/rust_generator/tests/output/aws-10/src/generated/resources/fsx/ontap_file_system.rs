/// Manages an Amazon FSx for NetApp ONTAP file system.
/// See the [FSx ONTAP User Guide](https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/what-is-fsx-ontap.html) for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = ontap_file_system::create(
///         "test",
///         OntapFileSystemArgs::builder()
///             .deployment_type("MULTI_AZ_1")
///             .preferred_subnet_id("${test1.id}")
///             .storage_capacity(1024)
///             .subnet_ids(vec!["${test1.id}", "${test2.id}",])
///             .throughput_capacity(512)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testhapairs = ontap_file_system::create(
///         "testhapairs",
///         OntapFileSystemArgs::builder()
///             .deployment_type("SINGLE_AZ_1")
///             .ha_pairs(2)
///             .preferred_subnet_id("${test1.id}")
///             .storage_capacity(2048)
///             .subnet_ids(vec!["${test1.id}",])
///             .throughput_capacity_per_ha_pair(128)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testsingleazgen2 = ontap_file_system::create(
///         "testsingleazgen2",
///         OntapFileSystemArgs::builder()
///             .deployment_type("SINGLE_AZ_2")
///             .ha_pairs(4)
///             .preferred_subnet_id("${test1.id}")
///             .storage_capacity(4096)
///             .subnet_ids(vec!["${test1.id}",])
///             .throughput_capacity_per_ha_pair(384)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testmultiazgen2 = ontap_file_system::create(
///         "testmultiazgen2",
///         OntapFileSystemArgs::builder()
///             .deployment_type("MULTI_AZ_2")
///             .ha_pairs(1)
///             .preferred_subnet_id("${test1.id}")
///             .storage_capacity(1024)
///             .subnet_ids(vec!["${test1.id}", "${test2.id}",])
///             .throughput_capacity_per_ha_pair(384)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import FSx File Systems using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:fsx/ontapFileSystem:OntapFileSystem example fs-543ab12b1ca672f33
/// ```
/// Certain resource arguments, like `security_group_ids`, do not have a FSx API method for reading the information after creation. If the argument is set in the Pulumi program on an imported resource, Pulumi will always show a difference. To workaround this behavior, either omit the argument from the Pulumi program or use `ignore_changes` to hide the difference. For example:
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ontap_file_system {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OntapFileSystemArgs {
        /// The number of days to retain automatic backups. Setting this to 0 disables automatic backups. You can retain automatic backups for a maximum of 90 days.
        #[builder(into, default)]
        pub automatic_backup_retention_days: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// A recurring daily time, in the format HH:MM. HH is the zero-padded hour of the day (0-23), and MM is the zero-padded minute of the hour. For example, 05:00 specifies 5 AM daily. Requires `automatic_backup_retention_days` to be set.
        #[builder(into, default)]
        pub daily_automatic_backup_start_time: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The filesystem deployment type. Supports `MULTI_AZ_1`, `MULTI_AZ_2`, `SINGLE_AZ_1`, and `SINGLE_AZ_2`.
        #[builder(into)]
        pub deployment_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SSD IOPS configuration for the Amazon FSx for NetApp ONTAP file system. See Disk Iops Configuration below.
        #[builder(into, default)]
        pub disk_iops_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::fsx::OntapFileSystemDiskIopsConfiguration>,
        >,
        /// Specifies the IP address range in which the endpoints to access your file system will be created. By default, Amazon FSx selects an unused IP address range for you from the 198.19.* range.
        #[builder(into, default)]
        pub endpoint_ip_address_range: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ONTAP administrative password for the fsxadmin user that you can use to administer your file system using the ONTAP CLI and REST API.
        #[builder(into, default)]
        pub fsx_admin_password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of ha_pairs to deploy for the file system. Valid value is 1 for `SINGLE_AZ_1` or `MULTI_AZ_1` and `MULTI_AZ_2`. Valid values are 1 through 12 for `SINGLE_AZ_2`.
        #[builder(into, default)]
        pub ha_pairs: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// ARN for the KMS Key to encrypt the file system at rest, Defaults to an AWS managed KMS Key.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID for a subnet. A subnet is a range of IP addresses in your virtual private cloud (VPC).
        #[builder(into)]
        pub preferred_subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the VPC route tables in which your file system's endpoints will be created. You should specify all VPC route tables associated with the subnets in which your clients are located. By default, Amazon FSx selects your VPC's default route table.
        #[builder(into, default)]
        pub route_table_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A list of IDs for the security groups that apply to the specified network interfaces created for file system access. These security groups will apply to all network interfaces.
        #[builder(into, default)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The storage capacity (GiB) of the file system. Valid values between `1024` and `196608` for file systems with deployment_type `SINGLE_AZ_1` and `MULTI_AZ_1`. Valid values are between `1024` and `524288` for `MULTI_AZ_2`. Valid values between `1024` (`1024` per ha pair) and `1048576` for file systems with deployment_type `SINGLE_AZ_2`. For `SINGLE_AZ_2`, the `1048576` (1PB) maximum is only supported when using 2 or more ha_pairs, the maximum is `524288` (512TB) when using 1 ha_pair.
        #[builder(into)]
        pub storage_capacity: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The filesystem storage type. defaults to `SSD`.
        #[builder(into, default)]
        pub storage_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of IDs for the subnets that the file system will be accessible from. Up to 2 subnets can be provided.
        #[builder(into)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// A map of tags to assign to the file system. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Sets the throughput capacity (in MBps) for the file system that you're creating. Valid values are `128`, `256`, `512`, `1024`, `2048`, and `4096`. This parameter is only supported when not using the ha_pairs parameter. Either throughput_capacity or throughput_capacity_per_ha_pair must be specified.
        #[builder(into, default)]
        pub throughput_capacity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Sets the per-HA-pair throughput capacity (in MBps) for the file system that you're creating, as opposed to `throughput_capacity` which specifies the total throughput capacity for the file system. Valid value for `MULTI_AZ_1` and `SINGLE_AZ_1` are `128`, `256`, `512`, `1024`, `2048`, and `4096`. Valid values for deployment type `MULTI_AZ_2` and `SINGLE_AZ_2` are `384`,`768`,`1536`,`3072`,`6144` where `ha_pairs` is `1`. Valid values for deployment type `SINGLE_AZ_2` are `1536`, `3072`, and `6144` where `ha_pairs` is greater than 1. This parameter is only supported when specifying the ha_pairs parameter. Either throughput_capacity or throughput_capacity_per_ha_pair must be specified.
        #[builder(into, default)]
        pub throughput_capacity_per_ha_pair: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The preferred start time (in `d:HH:MM` format) to perform weekly maintenance, in the UTC time zone.
        #[builder(into, default)]
        pub weekly_maintenance_start_time: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct OntapFileSystemResult {
        /// Amazon Resource Name of the file system.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The number of days to retain automatic backups. Setting this to 0 disables automatic backups. You can retain automatic backups for a maximum of 90 days.
        pub automatic_backup_retention_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A recurring daily time, in the format HH:MM. HH is the zero-padded hour of the day (0-23), and MM is the zero-padded minute of the hour. For example, 05:00 specifies 5 AM daily. Requires `automatic_backup_retention_days` to be set.
        pub daily_automatic_backup_start_time: pulumi_gestalt_rust::Output<String>,
        /// The filesystem deployment type. Supports `MULTI_AZ_1`, `MULTI_AZ_2`, `SINGLE_AZ_1`, and `SINGLE_AZ_2`.
        pub deployment_type: pulumi_gestalt_rust::Output<String>,
        /// The SSD IOPS configuration for the Amazon FSx for NetApp ONTAP file system. See Disk Iops Configuration below.
        pub disk_iops_configuration: pulumi_gestalt_rust::Output<
            super::super::types::fsx::OntapFileSystemDiskIopsConfiguration,
        >,
        /// The Domain Name Service (DNS) name for the file system. You can mount your file system using its DNS name.
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the IP address range in which the endpoints to access your file system will be created. By default, Amazon FSx selects an unused IP address range for you from the 198.19.* range.
        pub endpoint_ip_address_range: pulumi_gestalt_rust::Output<String>,
        /// The endpoints that are used to access data or to manage the file system using the NetApp ONTAP CLI, REST API, or NetApp SnapMirror. See Endpoints below.
        pub endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::fsx::OntapFileSystemEndpoint>,
        >,
        /// The ONTAP administrative password for the fsxadmin user that you can use to administer your file system using the ONTAP CLI and REST API.
        pub fsx_admin_password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The number of ha_pairs to deploy for the file system. Valid value is 1 for `SINGLE_AZ_1` or `MULTI_AZ_1` and `MULTI_AZ_2`. Valid values are 1 through 12 for `SINGLE_AZ_2`.
        pub ha_pairs: pulumi_gestalt_rust::Output<i32>,
        /// ARN for the KMS Key to encrypt the file system at rest, Defaults to an AWS managed KMS Key.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Set of Elastic Network Interface identifiers from which the file system is accessible The first network interface returned is the primary network interface.
        pub network_interface_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// AWS account identifier that created the file system.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// The ID for a subnet. A subnet is a range of IP addresses in your virtual private cloud (VPC).
        pub preferred_subnet_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the VPC route tables in which your file system's endpoints will be created. You should specify all VPC route tables associated with the subnets in which your clients are located. By default, Amazon FSx selects your VPC's default route table.
        pub route_table_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of IDs for the security groups that apply to the specified network interfaces created for file system access. These security groups will apply to all network interfaces.
        pub security_group_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The storage capacity (GiB) of the file system. Valid values between `1024` and `196608` for file systems with deployment_type `SINGLE_AZ_1` and `MULTI_AZ_1`. Valid values are between `1024` and `524288` for `MULTI_AZ_2`. Valid values between `1024` (`1024` per ha pair) and `1048576` for file systems with deployment_type `SINGLE_AZ_2`. For `SINGLE_AZ_2`, the `1048576` (1PB) maximum is only supported when using 2 or more ha_pairs, the maximum is `524288` (512TB) when using 1 ha_pair.
        pub storage_capacity: pulumi_gestalt_rust::Output<i32>,
        /// The filesystem storage type. defaults to `SSD`.
        pub storage_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of IDs for the subnets that the file system will be accessible from. Up to 2 subnets can be provided.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of tags to assign to the file system. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Sets the throughput capacity (in MBps) for the file system that you're creating. Valid values are `128`, `256`, `512`, `1024`, `2048`, and `4096`. This parameter is only supported when not using the ha_pairs parameter. Either throughput_capacity or throughput_capacity_per_ha_pair must be specified.
        pub throughput_capacity: pulumi_gestalt_rust::Output<i32>,
        /// Sets the per-HA-pair throughput capacity (in MBps) for the file system that you're creating, as opposed to `throughput_capacity` which specifies the total throughput capacity for the file system. Valid value for `MULTI_AZ_1` and `SINGLE_AZ_1` are `128`, `256`, `512`, `1024`, `2048`, and `4096`. Valid values for deployment type `MULTI_AZ_2` and `SINGLE_AZ_2` are `384`,`768`,`1536`,`3072`,`6144` where `ha_pairs` is `1`. Valid values for deployment type `SINGLE_AZ_2` are `1536`, `3072`, and `6144` where `ha_pairs` is greater than 1. This parameter is only supported when specifying the ha_pairs parameter. Either throughput_capacity or throughput_capacity_per_ha_pair must be specified.
        pub throughput_capacity_per_ha_pair: pulumi_gestalt_rust::Output<i32>,
        /// Identifier of the Virtual Private Cloud for the file system.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        /// The preferred start time (in `d:HH:MM` format) to perform weekly maintenance, in the UTC time zone.
        pub weekly_maintenance_start_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OntapFileSystemArgs,
    ) -> OntapFileSystemResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let automatic_backup_retention_days_binding = args
            .automatic_backup_retention_days
            .get_output(context);
        let daily_automatic_backup_start_time_binding = args
            .daily_automatic_backup_start_time
            .get_output(context);
        let deployment_type_binding = args.deployment_type.get_output(context);
        let disk_iops_configuration_binding = args
            .disk_iops_configuration
            .get_output(context);
        let endpoint_ip_address_range_binding = args
            .endpoint_ip_address_range
            .get_output(context);
        let fsx_admin_password_binding = args.fsx_admin_password.get_output(context);
        let ha_pairs_binding = args.ha_pairs.get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let preferred_subnet_id_binding = args.preferred_subnet_id.get_output(context);
        let route_table_ids_binding = args.route_table_ids.get_output(context);
        let security_group_ids_binding = args.security_group_ids.get_output(context);
        let storage_capacity_binding = args.storage_capacity.get_output(context);
        let storage_type_binding = args.storage_type.get_output(context);
        let subnet_ids_binding = args.subnet_ids.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let throughput_capacity_binding = args.throughput_capacity.get_output(context);
        let throughput_capacity_per_ha_pair_binding = args
            .throughput_capacity_per_ha_pair
            .get_output(context);
        let weekly_maintenance_start_time_binding = args
            .weekly_maintenance_start_time
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:fsx/ontapFileSystem:OntapFileSystem".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticBackupRetentionDays".into(),
                    value: &automatic_backup_retention_days_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dailyAutomaticBackupStartTime".into(),
                    value: &daily_automatic_backup_start_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deploymentType".into(),
                    value: &deployment_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskIopsConfiguration".into(),
                    value: &disk_iops_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointIpAddressRange".into(),
                    value: &endpoint_ip_address_range_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fsxAdminPassword".into(),
                    value: &fsx_admin_password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "haPairs".into(),
                    value: &ha_pairs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredSubnetId".into(),
                    value: &preferred_subnet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeTableIds".into(),
                    value: &route_table_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageCapacity".into(),
                    value: &storage_capacity_binding.drop_type(),
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
                    name: "throughputCapacity".into(),
                    value: &throughput_capacity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "throughputCapacityPerHaPair".into(),
                    value: &throughput_capacity_per_ha_pair_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "weeklyMaintenanceStartTime".into(),
                    value: &weekly_maintenance_start_time_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        OntapFileSystemResult {
            arn: o.get_field("arn"),
            automatic_backup_retention_days: o.get_field("automaticBackupRetentionDays"),
            daily_automatic_backup_start_time: o
                .get_field("dailyAutomaticBackupStartTime"),
            deployment_type: o.get_field("deploymentType"),
            disk_iops_configuration: o.get_field("diskIopsConfiguration"),
            dns_name: o.get_field("dnsName"),
            endpoint_ip_address_range: o.get_field("endpointIpAddressRange"),
            endpoints: o.get_field("endpoints"),
            fsx_admin_password: o.get_field("fsxAdminPassword"),
            ha_pairs: o.get_field("haPairs"),
            kms_key_id: o.get_field("kmsKeyId"),
            network_interface_ids: o.get_field("networkInterfaceIds"),
            owner_id: o.get_field("ownerId"),
            preferred_subnet_id: o.get_field("preferredSubnetId"),
            route_table_ids: o.get_field("routeTableIds"),
            security_group_ids: o.get_field("securityGroupIds"),
            storage_capacity: o.get_field("storageCapacity"),
            storage_type: o.get_field("storageType"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            throughput_capacity: o.get_field("throughputCapacity"),
            throughput_capacity_per_ha_pair: o.get_field("throughputCapacityPerHaPair"),
            vpc_id: o.get_field("vpcId"),
            weekly_maintenance_start_time: o.get_field("weeklyMaintenanceStartTime"),
        }
    }
}
