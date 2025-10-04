/// Provides an EC2 Spot Instance Request resource. This allows instances to be
/// requested on the spot market.
///
/// By default this provider creates Spot Instance Requests with a `persistent` type,
/// which means that for the duration of their lifetime, AWS will launch an
/// instance with the configured details if and when the spot market will accept
/// the requested price.
///
/// On destruction, this provider will make an attempt to terminate the associated Spot
/// Instance if there is one present.
///
/// Spot Instances requests with a `one-time` type will close the spot request
/// when the instance is terminated either by the request being below the current spot
/// price availability or by a user.
///
/// > **NOTE:** Because their behavior depends on the live status of the spot
/// market, Spot Instance Requests have a unique lifecycle that makes them behave
/// differently than other resources. Most importantly: there is __no
/// guarantee__ that a Spot Instance exists to fulfill the request at any given
/// point in time. See the [AWS Spot Instance
/// documentation](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-spot-instances.html)
/// for more information.
///
/// > **NOTE [AWS strongly discourages](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/spot-best-practices.html#which-spot-request-method-to-use) the use of the legacy APIs called by this resource.
/// We recommend using the EC2 Instance resource with `instance_market_options` instead.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   # Request a spot instance at $0.03
///   cheapWorker:
///     type: aws:ec2:SpotInstanceRequest
///     name: cheap_worker
///     properties:
///       ami: ami-1234
///       spotPrice: '0.03'
///       instanceType: c4.xlarge
///       tags:
///         Name: CheapWorker
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spot_instance_request {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpotInstanceRequestArgs {
        /// AMI to use for the instance. Required unless `launch_template` is specified and the Launch Template specifes an AMI. If an AMI is specified in the Launch Template, setting `ami` will override the AMI specified in the Launch Template.
        #[builder(into, default)]
        pub ami: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to associate a public IP address with an instance in a VPC.
        #[builder(into, default)]
        pub associate_public_ip_address: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// AZ to start the instance in.
        #[builder(into, default)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The required duration for the Spot instances, in minutes. This value must be a multiple of 60 (60, 120, 180, 240, 300, or 360).
        /// The duration period starts as soon as your Spot instance receives its instance ID. At the end of the duration period, Amazon EC2 marks the Spot instance for termination and provides a Spot instance termination notice, which gives the instance a two-minute warning before it terminates.
        /// Note that you can't specify an Availability Zone group or a launch group if you specify a duration.
        #[builder(into, default)]
        pub block_duration_minutes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Describes an instance's Capacity Reservation targeting option. See Capacity Reservation Specification below for more details.
        ///
        /// > **NOTE:** Changing `cpu_core_count` and/or `cpu_threads_per_core` will cause the resource to be destroyed and re-created.
        #[builder(into, default)]
        pub capacity_reservation_specification: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::ec2::SpotInstanceRequestCapacityReservationSpecification,
            >,
        >,
        /// Sets the number of CPU cores for an instance. This option is only supported on creation of instance type that support CPU Options [CPU Cores and Threads Per CPU Core Per Instance Type](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-optimize-cpu.html#cpu-options-supported-instances-values) - specifying this option for unsupported instance types will return an error from the EC2 API.
        #[builder(into, default)]
        pub cpu_core_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The CPU options for the instance. See CPU Options below for more details.
        #[builder(into, default)]
        pub cpu_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::SpotInstanceRequestCpuOptions>,
        >,
        /// If set to 1, hyperthreading is disabled on the launched instance. Defaults to 2 if not set. See [Optimizing CPU Options](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-optimize-cpu.html) for more information.
        #[builder(into, default)]
        pub cpu_threads_per_core: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Configuration block for customizing the credit specification of the instance. See Credit Specification below for more details. This provider will only perform drift detection of its value when present in a configuration. Removing this configuration on existing instances will only stop managing it. It will not change the configuration back to the default for the instance type.
        #[builder(into, default)]
        pub credit_specification: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::SpotInstanceRequestCreditSpecification>,
        >,
        /// If true, enables [EC2 Instance Stop Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Stop_Start.html#Using_StopProtection).
        #[builder(into, default)]
        pub disable_api_stop: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If true, enables [EC2 Instance Termination Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/terminating-instances.html#Using_ChangingDisableAPITermination).
        #[builder(into, default)]
        pub disable_api_termination: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// One or more configuration blocks with additional EBS block devices to attach to the instance. Block device configurations only apply on resource creation. See Block Devices below for details on attributes and drift detection. When accessing this as an attribute reference, it is a set of objects.
        #[builder(into, default)]
        pub ebs_block_devices: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::SpotInstanceRequestEbsBlockDevice>>,
        >,
        /// If true, the launched EC2 instance will be EBS-optimized. Note that if this is not set on an instance type that is optimized by default then this will show as disabled but if the instance type is optimized by default then there is no need to set this and there is no effect to disabling it. See the [EBS Optimized section](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSOptimized.html) of the AWS User Guide for more information.
        #[builder(into, default)]
        pub ebs_optimized: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to assign a primary IPv6 Global Unicast Address (GUA) to the instance when launched in a dual-stack or IPv6-only subnet. A primary IPv6 address ensures a consistent IPv6 address for the instance and is automatically assigned by AWS to the ENI. Once enabled, the first IPv6 GUA becomes the primary IPv6 address and cannot be disabled. The primary IPv6 address remains until the instance is terminated or the ENI is detached. Disabling `enable_primary_ipv6` after it has been enabled forces recreation of the instance.
        #[builder(into, default)]
        pub enable_primary_ipv6: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Enable Nitro Enclaves on launched instances. See Enclave Options below for more details.
        #[builder(into, default)]
        pub enclave_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::SpotInstanceRequestEnclaveOptions>,
        >,
        /// One or more configuration blocks to customize Ephemeral (also known as "Instance Store") volumes on the instance. See Block Devices below for details. When accessing this as an attribute reference, it is a set of objects.
        #[builder(into, default)]
        pub ephemeral_block_devices: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::ec2::SpotInstanceRequestEphemeralBlockDevice>,
            >,
        >,
        /// If true, wait for password data to become available and retrieve it. Useful for getting the administrator password for instances running Microsoft Windows. The password data is exported to the `password_data` attribute. See [GetPasswordData](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_GetPasswordData.html) for more information.
        #[builder(into, default)]
        pub get_password_data: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If true, the launched EC2 instance will support hibernation.
        #[builder(into, default)]
        pub hibernation: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// ID of a dedicated host that the instance will be assigned to. Use when an instance is to be launched on a specific dedicated host.
        #[builder(into, default)]
        pub host_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the host resource group in which to launch the instances. If you specify an ARN, omit the `tenancy` parameter or set it to `host`.
        #[builder(into, default)]
        pub host_resource_group_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// IAM Instance Profile to launch the instance with. Specified as the name of the Instance Profile. Ensure your credentials have the correct permission to assign the instance profile according to the [EC2 documentation](http://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_switch-role-ec2.html#roles-usingrole-ec2instance-permissions), notably `iam:PassRole`.
        #[builder(into, default)]
        pub iam_instance_profile: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Shutdown behavior for the instance. Amazon defaults this to `stop` for EBS-backed instances and `terminate` for instance-store instances. Cannot be set on instance-store instances. See [Shutdown Behavior](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/terminating-instances.html#Using_ChangingInstanceInitiatedShutdownBehavior) for more information.
        #[builder(into, default)]
        pub instance_initiated_shutdown_behavior: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Indicates Spot instance behavior when it is interrupted. Valid values are `terminate`, `stop`, or `hibernate`. Default value is `terminate`.
        #[builder(into, default)]
        pub instance_interruption_behavior: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Instance type to use for the instance. Required unless `launch_template` is specified and the Launch Template specifies an instance type. If an instance type is specified in the Launch Template, setting `instance_type` will override the instance type specified in the Launch Template. Updates to this field will trigger a stop/start of the EC2 instance.
        #[builder(into, default)]
        pub instance_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Number of IPv6 addresses to associate with the primary network interface. Amazon EC2 chooses the IPv6 addresses from the range of your subnet.
        #[builder(into, default)]
        pub ipv6_address_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specify one or more IPv6 addresses from the range of the subnet to associate with the primary network interface
        #[builder(into, default)]
        pub ipv6_addresses: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Key name of the Key Pair to use for the instance; which can be managed using the `aws.ec2.KeyPair` resource.
        #[builder(into, default)]
        pub key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A launch group is a group of spot instances that launch together and terminate together.
        /// If left empty instances are launched and terminated individually.
        #[builder(into, default)]
        pub launch_group: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a Launch Template to configure the instance. Parameters configured on this resource will override the corresponding parameters in the Launch Template. See Launch Template Specification below for more details.
        #[builder(into, default)]
        pub launch_template: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::SpotInstanceRequestLaunchTemplate>,
        >,
        /// Maintenance and recovery options for the instance. See Maintenance Options below for more details.
        #[builder(into, default)]
        pub maintenance_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::SpotInstanceRequestMaintenanceOptions>,
        >,
        /// Customize the metadata options of the instance. See Metadata Options below for more details.
        #[builder(into, default)]
        pub metadata_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::SpotInstanceRequestMetadataOptions>,
        >,
        /// If true, the launched EC2 instance will have detailed monitoring enabled. (Available since v0.6.0)
        #[builder(into, default)]
        pub monitoring: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Customize network interfaces to be attached at instance boot time. See Network Interfaces below for more details.
        #[builder(into, default)]
        pub network_interfaces: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::SpotInstanceRequestNetworkInterface>>,
        >,
        /// Placement Group to start the instance in.
        #[builder(into, default)]
        pub placement_group: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Number of the partition the instance is in. Valid only if the `aws.ec2.PlacementGroup` resource's `strategy` argument is set to `"partition"`.
        #[builder(into, default)]
        pub placement_partition_number: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Options for the instance hostname. The default values are inherited from the subnet. See Private DNS Name Options below for more details.
        #[builder(into, default)]
        pub private_dns_name_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::SpotInstanceRequestPrivateDnsNameOptions>,
        >,
        /// Private IP address to associate with the instance in a VPC.
        #[builder(into, default)]
        pub private_ip: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block to customize details about the root block device of the instance. See Block Devices below for details. When accessing this as an attribute reference, it is a list containing one object.
        #[builder(into, default)]
        pub root_block_device: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::SpotInstanceRequestRootBlockDevice>,
        >,
        /// List of secondary private IPv4 addresses to assign to the instance's primary network interface (eth0) in a VPC. Can only be assigned to the primary network interface (eth0) attached at instance creation, not a pre-existing network interface i.e., referenced in a `network_interface` block. Refer to the [Elastic network interfaces documentation](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-eni.html#AvailableIpPerENI) to see the maximum number of private IP addresses allowed per instance type.
        #[builder(into, default)]
        pub secondary_private_ips: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// List of security group names to associate with.
        ///
        /// > **NOTE:** If you are creating Instances in a VPC, use `vpc_security_group_ids` instead.
        #[builder(into, default)]
        pub security_groups: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Controls if traffic is routed to the instance when the destination address does not match the instance. Used for NAT or VPNs. Defaults true.
        #[builder(into, default)]
        pub source_dest_check: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The maximum price to request on the spot market.
        #[builder(into, default)]
        pub spot_price: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If set to `one-time`, after
        /// the instance is terminated, the spot request will be closed.
        #[builder(into, default)]
        pub spot_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// VPC Subnet ID to launch in.
        #[builder(into, default)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the resource. Note that these tags apply to the instance and not block storage devices. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Tenancy of the instance (if the instance is running in a VPC). An instance with a tenancy of `dedicated` runs on single-tenant hardware. The `host` tenancy is not supported for the import-instance command. Valid values are `default`, `dedicated`, and `host`.
        #[builder(into, default)]
        pub tenancy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User data to provide when launching the instance. Do not pass gzip-compressed data via this argument; see `user_data_base64` instead. Updates to this field will trigger a stop/start of the EC2 instance by default. If the `user_data_replace_on_change` is set then updates to this field will trigger a destroy and recreate of the EC2 instance.
        #[builder(into, default)]
        pub user_data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Can be used instead of `user_data` to pass base64-encoded binary data directly. Use this instead of `user_data` whenever the value is not a valid UTF-8 string. For example, gzip-encoded user data must be base64-encoded and passed via this argument to avoid corruption. Updates to this field will trigger a stop/start of the EC2 instance by default. If the `user_data_replace_on_change` is set then updates to this field will trigger a destroy and recreate of the EC2 instance.
        #[builder(into, default)]
        pub user_data_base64: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// When used in combination with `user_data` or `user_data_base64` will trigger a destroy and recreate of the EC2 instance when set to `true`. Defaults to `false` if not set.
        #[builder(into, default)]
        pub user_data_replace_on_change: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The start date and time of the request, in UTC [RFC3339](https://tools.ietf.org/html/rfc3339#section-5.8) format(for example, YYYY-MM-DDTHH:MM:SSZ). The default is to start fulfilling the request immediately.
        #[builder(into, default)]
        pub valid_from: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The end date and time of the request, in UTC [RFC3339](https://tools.ietf.org/html/rfc3339#section-5.8) format(for example, YYYY-MM-DDTHH:MM:SSZ). At this point, no new Spot instance requests are placed or enabled to fulfill the request. The default end date is 7 days from the current date.
        #[builder(into, default)]
        pub valid_until: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign, at instance-creation time, to root and EBS volumes.
        ///
        /// > **NOTE:** Do not use `volume_tags` if you plan to manage block device tags outside the `aws.ec2.Instance` configuration, such as using `tags` in an `aws.ebs.Volume` resource attached via `aws.ec2.VolumeAttachment`. Doing so will result in resource cycling and inconsistent behavior.
        #[builder(into, default)]
        pub volume_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of security group IDs to associate with.
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// If set, this provider will
        /// wait for the Spot Request to be fulfilled, and will throw an error if the
        /// timeout of 10m is reached.
        #[builder(into, default)]
        pub wait_for_fulfillment: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SpotInstanceRequestResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// AMI to use for the instance. Required unless `launch_template` is specified and the Launch Template specifes an AMI. If an AMI is specified in the Launch Template, setting `ami` will override the AMI specified in the Launch Template.
        pub ami: pulumi_gestalt_rust::Output<String>,
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether to associate a public IP address with an instance in a VPC.
        pub associate_public_ip_address: pulumi_gestalt_rust::Output<bool>,
        /// AZ to start the instance in.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// The required duration for the Spot instances, in minutes. This value must be a multiple of 60 (60, 120, 180, 240, 300, or 360).
        /// The duration period starts as soon as your Spot instance receives its instance ID. At the end of the duration period, Amazon EC2 marks the Spot instance for termination and provides a Spot instance termination notice, which gives the instance a two-minute warning before it terminates.
        /// Note that you can't specify an Availability Zone group or a launch group if you specify a duration.
        pub block_duration_minutes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Describes an instance's Capacity Reservation targeting option. See Capacity Reservation Specification below for more details.
        ///
        /// > **NOTE:** Changing `cpu_core_count` and/or `cpu_threads_per_core` will cause the resource to be destroyed and re-created.
        pub capacity_reservation_specification: pulumi_gestalt_rust::Output<
            super::super::types::ec2::SpotInstanceRequestCapacityReservationSpecification,
        >,
        /// Sets the number of CPU cores for an instance. This option is only supported on creation of instance type that support CPU Options [CPU Cores and Threads Per CPU Core Per Instance Type](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-optimize-cpu.html#cpu-options-supported-instances-values) - specifying this option for unsupported instance types will return an error from the EC2 API.
        pub cpu_core_count: pulumi_gestalt_rust::Output<i32>,
        /// The CPU options for the instance. See CPU Options below for more details.
        pub cpu_options: pulumi_gestalt_rust::Output<
            super::super::types::ec2::SpotInstanceRequestCpuOptions,
        >,
        /// If set to 1, hyperthreading is disabled on the launched instance. Defaults to 2 if not set. See [Optimizing CPU Options](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-optimize-cpu.html) for more information.
        pub cpu_threads_per_core: pulumi_gestalt_rust::Output<i32>,
        /// Configuration block for customizing the credit specification of the instance. See Credit Specification below for more details. This provider will only perform drift detection of its value when present in a configuration. Removing this configuration on existing instances will only stop managing it. It will not change the configuration back to the default for the instance type.
        pub credit_specification: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::SpotInstanceRequestCreditSpecification>,
        >,
        /// If true, enables [EC2 Instance Stop Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Stop_Start.html#Using_StopProtection).
        pub disable_api_stop: pulumi_gestalt_rust::Output<bool>,
        /// If true, enables [EC2 Instance Termination Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/terminating-instances.html#Using_ChangingDisableAPITermination).
        pub disable_api_termination: pulumi_gestalt_rust::Output<bool>,
        /// One or more configuration blocks with additional EBS block devices to attach to the instance. Block device configurations only apply on resource creation. See Block Devices below for details on attributes and drift detection. When accessing this as an attribute reference, it is a set of objects.
        pub ebs_block_devices: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::SpotInstanceRequestEbsBlockDevice>,
        >,
        /// If true, the launched EC2 instance will be EBS-optimized. Note that if this is not set on an instance type that is optimized by default then this will show as disabled but if the instance type is optimized by default then there is no need to set this and there is no effect to disabling it. See the [EBS Optimized section](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSOptimized.html) of the AWS User Guide for more information.
        pub ebs_optimized: pulumi_gestalt_rust::Output<bool>,
        /// Whether to assign a primary IPv6 Global Unicast Address (GUA) to the instance when launched in a dual-stack or IPv6-only subnet. A primary IPv6 address ensures a consistent IPv6 address for the instance and is automatically assigned by AWS to the ENI. Once enabled, the first IPv6 GUA becomes the primary IPv6 address and cannot be disabled. The primary IPv6 address remains until the instance is terminated or the ENI is detached. Disabling `enable_primary_ipv6` after it has been enabled forces recreation of the instance.
        pub enable_primary_ipv6: pulumi_gestalt_rust::Output<bool>,
        /// Enable Nitro Enclaves on launched instances. See Enclave Options below for more details.
        pub enclave_options: pulumi_gestalt_rust::Output<
            super::super::types::ec2::SpotInstanceRequestEnclaveOptions,
        >,
        /// One or more configuration blocks to customize Ephemeral (also known as "Instance Store") volumes on the instance. See Block Devices below for details. When accessing this as an attribute reference, it is a set of objects.
        pub ephemeral_block_devices: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::SpotInstanceRequestEphemeralBlockDevice>,
        >,
        /// If true, wait for password data to become available and retrieve it. Useful for getting the administrator password for instances running Microsoft Windows. The password data is exported to the `password_data` attribute. See [GetPasswordData](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_GetPasswordData.html) for more information.
        pub get_password_data: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If true, the launched EC2 instance will support hibernation.
        pub hibernation: pulumi_gestalt_rust::Output<Option<bool>>,
        /// ID of a dedicated host that the instance will be assigned to. Use when an instance is to be launched on a specific dedicated host.
        pub host_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the host resource group in which to launch the instances. If you specify an ARN, omit the `tenancy` parameter or set it to `host`.
        pub host_resource_group_arn: pulumi_gestalt_rust::Output<String>,
        /// IAM Instance Profile to launch the instance with. Specified as the name of the Instance Profile. Ensure your credentials have the correct permission to assign the instance profile according to the [EC2 documentation](http://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_switch-role-ec2.html#roles-usingrole-ec2instance-permissions), notably `iam:PassRole`.
        pub iam_instance_profile: pulumi_gestalt_rust::Output<String>,
        /// Shutdown behavior for the instance. Amazon defaults this to `stop` for EBS-backed instances and `terminate` for instance-store instances. Cannot be set on instance-store instances. See [Shutdown Behavior](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/terminating-instances.html#Using_ChangingInstanceInitiatedShutdownBehavior) for more information.
        pub instance_initiated_shutdown_behavior: pulumi_gestalt_rust::Output<String>,
        /// Indicates Spot instance behavior when it is interrupted. Valid values are `terminate`, `stop`, or `hibernate`. Default value is `terminate`.
        pub instance_interruption_behavior: pulumi_gestalt_rust::Output<Option<String>>,
        pub instance_state: pulumi_gestalt_rust::Output<String>,
        /// Instance type to use for the instance. Required unless `launch_template` is specified and the Launch Template specifies an instance type. If an instance type is specified in the Launch Template, setting `instance_type` will override the instance type specified in the Launch Template. Updates to this field will trigger a stop/start of the EC2 instance.
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        /// Number of IPv6 addresses to associate with the primary network interface. Amazon EC2 chooses the IPv6 addresses from the range of your subnet.
        pub ipv6_address_count: pulumi_gestalt_rust::Output<i32>,
        /// Specify one or more IPv6 addresses from the range of the subnet to associate with the primary network interface
        pub ipv6_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Key name of the Key Pair to use for the instance; which can be managed using the `aws.ec2.KeyPair` resource.
        pub key_name: pulumi_gestalt_rust::Output<String>,
        /// A launch group is a group of spot instances that launch together and terminate together.
        /// If left empty instances are launched and terminated individually.
        pub launch_group: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies a Launch Template to configure the instance. Parameters configured on this resource will override the corresponding parameters in the Launch Template. See Launch Template Specification below for more details.
        pub launch_template: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::SpotInstanceRequestLaunchTemplate>,
        >,
        /// Maintenance and recovery options for the instance. See Maintenance Options below for more details.
        pub maintenance_options: pulumi_gestalt_rust::Output<
            super::super::types::ec2::SpotInstanceRequestMaintenanceOptions,
        >,
        /// Customize the metadata options of the instance. See Metadata Options below for more details.
        pub metadata_options: pulumi_gestalt_rust::Output<
            super::super::types::ec2::SpotInstanceRequestMetadataOptions,
        >,
        /// If true, the launched EC2 instance will have detailed monitoring enabled. (Available since v0.6.0)
        pub monitoring: pulumi_gestalt_rust::Output<bool>,
        /// Customize network interfaces to be attached at instance boot time. See Network Interfaces below for more details.
        pub network_interfaces: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::SpotInstanceRequestNetworkInterface>,
        >,
        pub outpost_arn: pulumi_gestalt_rust::Output<String>,
        pub password_data: pulumi_gestalt_rust::Output<String>,
        /// Placement Group to start the instance in.
        pub placement_group: pulumi_gestalt_rust::Output<String>,
        /// Number of the partition the instance is in. Valid only if the `aws.ec2.PlacementGroup` resource's `strategy` argument is set to `"partition"`.
        pub placement_partition_number: pulumi_gestalt_rust::Output<i32>,
        pub primary_network_interface_id: pulumi_gestalt_rust::Output<String>,
        /// The private DNS name assigned to the instance. Can only be
        /// used inside the Amazon EC2, and only available if you've enabled DNS hostnames
        /// for your VPC
        pub private_dns: pulumi_gestalt_rust::Output<String>,
        /// Options for the instance hostname. The default values are inherited from the subnet. See Private DNS Name Options below for more details.
        pub private_dns_name_options: pulumi_gestalt_rust::Output<
            super::super::types::ec2::SpotInstanceRequestPrivateDnsNameOptions,
        >,
        /// Private IP address to associate with the instance in a VPC.
        pub private_ip: pulumi_gestalt_rust::Output<String>,
        /// The public DNS name assigned to the instance. For EC2-VPC, this
        /// is only available if you've enabled DNS hostnames for your VPC
        pub public_dns: pulumi_gestalt_rust::Output<String>,
        /// The public IP address assigned to the instance, if applicable.
        pub public_ip: pulumi_gestalt_rust::Output<String>,
        /// Configuration block to customize details about the root block device of the instance. See Block Devices below for details. When accessing this as an attribute reference, it is a list containing one object.
        pub root_block_device: pulumi_gestalt_rust::Output<
            super::super::types::ec2::SpotInstanceRequestRootBlockDevice,
        >,
        /// List of secondary private IPv4 addresses to assign to the instance's primary network interface (eth0) in a VPC. Can only be assigned to the primary network interface (eth0) attached at instance creation, not a pre-existing network interface i.e., referenced in a `network_interface` block. Refer to the [Elastic network interfaces documentation](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-eni.html#AvailableIpPerENI) to see the maximum number of private IP addresses allowed per instance type.
        pub secondary_private_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// List of security group names to associate with.
        ///
        /// > **NOTE:** If you are creating Instances in a VPC, use `vpc_security_group_ids` instead.
        pub security_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Controls if traffic is routed to the instance when the destination address does not match the instance. Used for NAT or VPNs. Defaults true.
        pub source_dest_check: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The current [bid
        /// status](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/spot-bid-status.html)
        /// of the Spot Instance Request.
        pub spot_bid_status: pulumi_gestalt_rust::Output<String>,
        /// The Instance ID (if any) that is currently fulfilling
        /// the Spot Instance request.
        pub spot_instance_id: pulumi_gestalt_rust::Output<String>,
        /// The maximum price to request on the spot market.
        pub spot_price: pulumi_gestalt_rust::Output<String>,
        /// The current [request
        /// state](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/spot-requests.html#creating-spot-request-status)
        /// of the Spot Instance Request.
        pub spot_request_state: pulumi_gestalt_rust::Output<String>,
        /// If set to `one-time`, after
        /// the instance is terminated, the spot request will be closed.
        pub spot_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// VPC Subnet ID to launch in.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the resource. Note that these tags apply to the instance and not block storage devices. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Tenancy of the instance (if the instance is running in a VPC). An instance with a tenancy of `dedicated` runs on single-tenant hardware. The `host` tenancy is not supported for the import-instance command. Valid values are `default`, `dedicated`, and `host`.
        pub tenancy: pulumi_gestalt_rust::Output<String>,
        /// User data to provide when launching the instance. Do not pass gzip-compressed data via this argument; see `user_data_base64` instead. Updates to this field will trigger a stop/start of the EC2 instance by default. If the `user_data_replace_on_change` is set then updates to this field will trigger a destroy and recreate of the EC2 instance.
        pub user_data: pulumi_gestalt_rust::Output<String>,
        /// Can be used instead of `user_data` to pass base64-encoded binary data directly. Use this instead of `user_data` whenever the value is not a valid UTF-8 string. For example, gzip-encoded user data must be base64-encoded and passed via this argument to avoid corruption. Updates to this field will trigger a stop/start of the EC2 instance by default. If the `user_data_replace_on_change` is set then updates to this field will trigger a destroy and recreate of the EC2 instance.
        pub user_data_base64: pulumi_gestalt_rust::Output<String>,
        /// When used in combination with `user_data` or `user_data_base64` will trigger a destroy and recreate of the EC2 instance when set to `true`. Defaults to `false` if not set.
        pub user_data_replace_on_change: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The start date and time of the request, in UTC [RFC3339](https://tools.ietf.org/html/rfc3339#section-5.8) format(for example, YYYY-MM-DDTHH:MM:SSZ). The default is to start fulfilling the request immediately.
        pub valid_from: pulumi_gestalt_rust::Output<String>,
        /// The end date and time of the request, in UTC [RFC3339](https://tools.ietf.org/html/rfc3339#section-5.8) format(for example, YYYY-MM-DDTHH:MM:SSZ). At this point, no new Spot instance requests are placed or enabled to fulfill the request. The default end date is 7 days from the current date.
        pub valid_until: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign, at instance-creation time, to root and EBS volumes.
        ///
        /// > **NOTE:** Do not use `volume_tags` if you plan to manage block device tags outside the `aws.ec2.Instance` configuration, such as using `tags` in an `aws.ebs.Volume` resource attached via `aws.ec2.VolumeAttachment`. Doing so will result in resource cycling and inconsistent behavior.
        pub volume_tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of security group IDs to associate with.
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// If set, this provider will
        /// wait for the Spot Request to be fulfilled, and will throw an error if the
        /// timeout of 10m is reached.
        pub wait_for_fulfillment: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpotInstanceRequestArgs,
    ) -> SpotInstanceRequestResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let ami_binding = args.ami.get_output(context);
        let associate_public_ip_address_binding = args
            .associate_public_ip_address
            .get_output(context);
        let availability_zone_binding = args.availability_zone.get_output(context);
        let block_duration_minutes_binding = args
            .block_duration_minutes
            .get_output(context);
        let capacity_reservation_specification_binding = args
            .capacity_reservation_specification
            .get_output(context);
        let cpu_core_count_binding = args.cpu_core_count.get_output(context);
        let cpu_options_binding = args.cpu_options.get_output(context);
        let cpu_threads_per_core_binding = args.cpu_threads_per_core.get_output(context);
        let credit_specification_binding = args.credit_specification.get_output(context);
        let disable_api_stop_binding = args.disable_api_stop.get_output(context);
        let disable_api_termination_binding = args
            .disable_api_termination
            .get_output(context);
        let ebs_block_devices_binding = args.ebs_block_devices.get_output(context);
        let ebs_optimized_binding = args.ebs_optimized.get_output(context);
        let enable_primary_ipv6_binding = args.enable_primary_ipv6.get_output(context);
        let enclave_options_binding = args.enclave_options.get_output(context);
        let ephemeral_block_devices_binding = args
            .ephemeral_block_devices
            .get_output(context);
        let get_password_data_binding = args.get_password_data.get_output(context);
        let hibernation_binding = args.hibernation.get_output(context);
        let host_id_binding = args.host_id.get_output(context);
        let host_resource_group_arn_binding = args
            .host_resource_group_arn
            .get_output(context);
        let iam_instance_profile_binding = args.iam_instance_profile.get_output(context);
        let instance_initiated_shutdown_behavior_binding = args
            .instance_initiated_shutdown_behavior
            .get_output(context);
        let instance_interruption_behavior_binding = args
            .instance_interruption_behavior
            .get_output(context);
        let instance_type_binding = args.instance_type.get_output(context);
        let ipv6_address_count_binding = args.ipv6_address_count.get_output(context);
        let ipv6_addresses_binding = args.ipv6_addresses.get_output(context);
        let key_name_binding = args.key_name.get_output(context);
        let launch_group_binding = args.launch_group.get_output(context);
        let launch_template_binding = args.launch_template.get_output(context);
        let maintenance_options_binding = args.maintenance_options.get_output(context);
        let metadata_options_binding = args.metadata_options.get_output(context);
        let monitoring_binding = args.monitoring.get_output(context);
        let network_interfaces_binding = args.network_interfaces.get_output(context);
        let placement_group_binding = args.placement_group.get_output(context);
        let placement_partition_number_binding = args
            .placement_partition_number
            .get_output(context);
        let private_dns_name_options_binding = args
            .private_dns_name_options
            .get_output(context);
        let private_ip_binding = args.private_ip.get_output(context);
        let root_block_device_binding = args.root_block_device.get_output(context);
        let secondary_private_ips_binding = args
            .secondary_private_ips
            .get_output(context);
        let security_groups_binding = args.security_groups.get_output(context);
        let source_dest_check_binding = args.source_dest_check.get_output(context);
        let spot_price_binding = args.spot_price.get_output(context);
        let spot_type_binding = args.spot_type.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tenancy_binding = args.tenancy.get_output(context);
        let user_data_binding = args.user_data.get_output(context);
        let user_data_base64_binding = args.user_data_base64.get_output(context);
        let user_data_replace_on_change_binding = args
            .user_data_replace_on_change
            .get_output(context);
        let valid_from_binding = args.valid_from.get_output(context);
        let valid_until_binding = args.valid_until.get_output(context);
        let volume_tags_binding = args.volume_tags.get_output(context);
        let vpc_security_group_ids_binding = args
            .vpc_security_group_ids
            .get_output(context);
        let wait_for_fulfillment_binding = args.wait_for_fulfillment.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/spotInstanceRequest:SpotInstanceRequest".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ami".into(),
                    value: &ami_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "associatePublicIpAddress".into(),
                    value: &associate_public_ip_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blockDurationMinutes".into(),
                    value: &block_duration_minutes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacityReservationSpecification".into(),
                    value: &capacity_reservation_specification_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cpuCoreCount".into(),
                    value: &cpu_core_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cpuOptions".into(),
                    value: &cpu_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cpuThreadsPerCore".into(),
                    value: &cpu_threads_per_core_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "creditSpecification".into(),
                    value: &credit_specification_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableApiStop".into(),
                    value: &disable_api_stop_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableApiTermination".into(),
                    value: &disable_api_termination_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ebsBlockDevices".into(),
                    value: &ebs_block_devices_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ebsOptimized".into(),
                    value: &ebs_optimized_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enablePrimaryIpv6".into(),
                    value: &enable_primary_ipv6_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enclaveOptions".into(),
                    value: &enclave_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ephemeralBlockDevices".into(),
                    value: &ephemeral_block_devices_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "getPasswordData".into(),
                    value: &get_password_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hibernation".into(),
                    value: &hibernation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostId".into(),
                    value: &host_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostResourceGroupArn".into(),
                    value: &host_resource_group_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iamInstanceProfile".into(),
                    value: &iam_instance_profile_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceInitiatedShutdownBehavior".into(),
                    value: &instance_initiated_shutdown_behavior_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceInterruptionBehavior".into(),
                    value: &instance_interruption_behavior_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv6AddressCount".into(),
                    value: &ipv6_address_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv6Addresses".into(),
                    value: &ipv6_addresses_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyName".into(),
                    value: &key_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "launchGroup".into(),
                    value: &launch_group_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "launchTemplate".into(),
                    value: &launch_template_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenanceOptions".into(),
                    value: &maintenance_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadataOptions".into(),
                    value: &metadata_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monitoring".into(),
                    value: &monitoring_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInterfaces".into(),
                    value: &network_interfaces_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "placementGroup".into(),
                    value: &placement_group_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "placementPartitionNumber".into(),
                    value: &placement_partition_number_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateDnsNameOptions".into(),
                    value: &private_dns_name_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateIp".into(),
                    value: &private_ip_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rootBlockDevice".into(),
                    value: &root_block_device_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secondaryPrivateIps".into(),
                    value: &secondary_private_ips_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroups".into(),
                    value: &security_groups_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceDestCheck".into(),
                    value: &source_dest_check_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spotPrice".into(),
                    value: &spot_price_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spotType".into(),
                    value: &spot_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenancy".into(),
                    value: &tenancy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userData".into(),
                    value: &user_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userDataBase64".into(),
                    value: &user_data_base64_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userDataReplaceOnChange".into(),
                    value: &user_data_replace_on_change_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validFrom".into(),
                    value: &valid_from_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validUntil".into(),
                    value: &valid_until_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "volumeTags".into(),
                    value: &volume_tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: &vpc_security_group_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "waitForFulfillment".into(),
                    value: &wait_for_fulfillment_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpotInstanceRequestResult {
            id: o.get_field("id"),
            ami: o.get_field("ami"),
            arn: o.get_field("arn"),
            associate_public_ip_address: o.get_field("associatePublicIpAddress"),
            availability_zone: o.get_field("availabilityZone"),
            block_duration_minutes: o.get_field("blockDurationMinutes"),
            capacity_reservation_specification: o
                .get_field("capacityReservationSpecification"),
            cpu_core_count: o.get_field("cpuCoreCount"),
            cpu_options: o.get_field("cpuOptions"),
            cpu_threads_per_core: o.get_field("cpuThreadsPerCore"),
            credit_specification: o.get_field("creditSpecification"),
            disable_api_stop: o.get_field("disableApiStop"),
            disable_api_termination: o.get_field("disableApiTermination"),
            ebs_block_devices: o.get_field("ebsBlockDevices"),
            ebs_optimized: o.get_field("ebsOptimized"),
            enable_primary_ipv6: o.get_field("enablePrimaryIpv6"),
            enclave_options: o.get_field("enclaveOptions"),
            ephemeral_block_devices: o.get_field("ephemeralBlockDevices"),
            get_password_data: o.get_field("getPasswordData"),
            hibernation: o.get_field("hibernation"),
            host_id: o.get_field("hostId"),
            host_resource_group_arn: o.get_field("hostResourceGroupArn"),
            iam_instance_profile: o.get_field("iamInstanceProfile"),
            instance_initiated_shutdown_behavior: o
                .get_field("instanceInitiatedShutdownBehavior"),
            instance_interruption_behavior: o.get_field("instanceInterruptionBehavior"),
            instance_state: o.get_field("instanceState"),
            instance_type: o.get_field("instanceType"),
            ipv6_address_count: o.get_field("ipv6AddressCount"),
            ipv6_addresses: o.get_field("ipv6Addresses"),
            key_name: o.get_field("keyName"),
            launch_group: o.get_field("launchGroup"),
            launch_template: o.get_field("launchTemplate"),
            maintenance_options: o.get_field("maintenanceOptions"),
            metadata_options: o.get_field("metadataOptions"),
            monitoring: o.get_field("monitoring"),
            network_interfaces: o.get_field("networkInterfaces"),
            outpost_arn: o.get_field("outpostArn"),
            password_data: o.get_field("passwordData"),
            placement_group: o.get_field("placementGroup"),
            placement_partition_number: o.get_field("placementPartitionNumber"),
            primary_network_interface_id: o.get_field("primaryNetworkInterfaceId"),
            private_dns: o.get_field("privateDns"),
            private_dns_name_options: o.get_field("privateDnsNameOptions"),
            private_ip: o.get_field("privateIp"),
            public_dns: o.get_field("publicDns"),
            public_ip: o.get_field("publicIp"),
            root_block_device: o.get_field("rootBlockDevice"),
            secondary_private_ips: o.get_field("secondaryPrivateIps"),
            security_groups: o.get_field("securityGroups"),
            source_dest_check: o.get_field("sourceDestCheck"),
            spot_bid_status: o.get_field("spotBidStatus"),
            spot_instance_id: o.get_field("spotInstanceId"),
            spot_price: o.get_field("spotPrice"),
            spot_request_state: o.get_field("spotRequestState"),
            spot_type: o.get_field("spotType"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            tenancy: o.get_field("tenancy"),
            user_data: o.get_field("userData"),
            user_data_base64: o.get_field("userDataBase64"),
            user_data_replace_on_change: o.get_field("userDataReplaceOnChange"),
            valid_from: o.get_field("validFrom"),
            valid_until: o.get_field("validUntil"),
            volume_tags: o.get_field("volumeTags"),
            vpc_security_group_ids: o.get_field("vpcSecurityGroupIds"),
            wait_for_fulfillment: o.get_field("waitForFulfillment"),
        }
    }
}
