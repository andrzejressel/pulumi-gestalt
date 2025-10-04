/// Manages an AWS Storage Gateway file, tape, or volume gateway in the provider region.
///
/// > **NOTE:** The Storage Gateway API requires the gateway to be connected to properly return information after activation. If you are receiving `The specified gateway is not connected` errors during resource creation (gateway activation), ensure your gateway instance meets the [Storage Gateway requirements](https://docs.aws.amazon.com/storagegateway/latest/userguide/Requirements.html).
///
/// ## Example Usage
///
/// ### Local Cache
///
/// ```yaml
/// resources:
///   testVolumeAttachment:
///     type: aws:ec2:VolumeAttachment
///     name: test
///     properties:
///       deviceName: /dev/xvdb
///       volumeId: ${testAwsEbsVolume.id}
///       instanceId: ${testAwsInstance.id}
///   testCache:
///     type: aws:storagegateway:Cache
///     name: test
///     properties:
///       diskId: ${test.diskId}
///       gatewayArn: ${testAwsStoragegatewayGateway.arn}
/// variables:
///   test:
///     fn::invoke:
///       function: aws:storagegateway:getLocalDisk
///       arguments:
///         diskNode: ${testAwsVolumeAttachment.deviceName}
///         gatewayArn: ${testAwsStoragegatewayGateway.arn}
/// ```
///
/// ### FSx File Gateway
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = gateway::create(
///         "example",
///         GatewayArgs::builder()
///             .gateway_ip_address("1.2.3.4")
///             .gateway_name("example")
///             .gateway_timezone("GMT")
///             .gateway_type("FILE_FSX_SMB")
///             .smb_active_directory_settings(
///                 GatewaySmbActiveDirectorySettings::builder()
///                     .domainName("corp.example.com")
///                     .password("avoid-plaintext-passwords")
///                     .username("Admin")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### S3 File Gateway
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = gateway::create(
///         "example",
///         GatewayArgs::builder()
///             .gateway_ip_address("1.2.3.4")
///             .gateway_name("example")
///             .gateway_timezone("GMT")
///             .gateway_type("FILE_S3")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Tape Gateway
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = gateway::create(
///         "example",
///         GatewayArgs::builder()
///             .gateway_ip_address("1.2.3.4")
///             .gateway_name("example")
///             .gateway_timezone("GMT")
///             .gateway_type("VTL")
///             .medium_changer_type("AWS-Gateway-VTL")
///             .tape_drive_type("IBM-ULT3580-TD5")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Volume Gateway (Cached)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = gateway::create(
///         "example",
///         GatewayArgs::builder()
///             .gateway_ip_address("1.2.3.4")
///             .gateway_name("example")
///             .gateway_timezone("GMT")
///             .gateway_type("CACHED")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Volume Gateway (Stored)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = gateway::create(
///         "example",
///         GatewayArgs::builder()
///             .gateway_ip_address("1.2.3.4")
///             .gateway_name("example")
///             .gateway_timezone("GMT")
///             .gateway_type("STORED")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_storagegateway_gateway` using the gateway Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:storagegateway/gateway:Gateway example arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678
/// ```
/// Certain resource arguments, like `gateway_ip_address` do not have a Storage Gateway API method for reading the information after creation, either omit the argument from the Pulumi program or use `ignore_changes` to hide the difference. For example:
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GatewayArgs {
        /// Gateway activation key during resource creation. Conflicts with `gateway_ip_address`. Additional information is available in the [Storage Gateway User Guide](https://docs.aws.amazon.com/storagegateway/latest/userguide/get-activation-key.html).
        #[builder(into, default)]
        pub activation_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The average download bandwidth rate limit in bits per second. This is supported for the `CACHED`, `STORED`, and `VTL` gateway types.
        #[builder(into, default)]
        pub average_download_rate_limit_in_bits_per_sec: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The average upload bandwidth rate limit in bits per second. This is supported for the `CACHED`, `STORED`, and `VTL` gateway types.
        #[builder(into, default)]
        pub average_upload_rate_limit_in_bits_per_sec: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The Amazon Resource Name (ARN) of the Amazon CloudWatch log group to use to monitor and log events in the gateway.
        #[builder(into, default)]
        pub cloudwatch_log_group_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Gateway IP address to retrieve activation key during resource creation. Conflicts with `activation_key`. Gateway must be accessible on port 80 from where this provider is running. Additional information is available in the [Storage Gateway User Guide](https://docs.aws.amazon.com/storagegateway/latest/userguide/get-activation-key.html).
        #[builder(into, default)]
        pub gateway_ip_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the gateway.
        #[builder(into)]
        pub gateway_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Time zone for the gateway. The time zone is of the format "GMT", "GMT-hr:mm", or "GMT+hr:mm". For example, `GMT-4:00` indicates the time is 4 hours behind GMT. The time zone is used, for example, for scheduling snapshots and your gateway's maintenance schedule.
        #[builder(into)]
        pub gateway_timezone: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of the gateway. The default value is `STORED`. Valid values: `CACHED`, `FILE_FSX_SMB`, `FILE_S3`, `STORED`, `VTL`.
        #[builder(into, default)]
        pub gateway_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// VPC endpoint address to be used when activating your gateway. This should be used when your instance is in a private subnet. Requires HTTP access from client computer running this provider. More info on what ports are required by your VPC Endpoint Security group in [Activating a Gateway in a Virtual Private Cloud](https://docs.aws.amazon.com/storagegateway/latest/userguide/gateway-private-link.html).
        #[builder(into, default)]
        pub gateway_vpc_endpoint: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The gateway's weekly maintenance start time information, including day and time of the week. The maintenance time is the time in your gateway's time zone. More details below.
        #[builder(into, default)]
        pub maintenance_start_time: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storagegateway::GatewayMaintenanceStartTime>,
        >,
        /// Type of medium changer to use for tape gateway. This provider cannot detect drift of this argument. Valid values: `STK-L700`, `AWS-Gateway-VTL`, `IBM-03584L32-0402`.
        #[builder(into, default)]
        pub medium_changer_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Nested argument with Active Directory domain join information for Server Message Block (SMB) file shares. Only valid for `FILE_S3` and `FILE_FSX_SMB` gateway types. Must be set before creating `ActiveDirectory` authentication SMB file shares. More details below.
        #[builder(into, default)]
        pub smb_active_directory_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::storagegateway::GatewaySmbActiveDirectorySettings,
            >,
        >,
        /// Specifies whether the shares on this gateway appear when listing shares.
        #[builder(into, default)]
        pub smb_file_share_visibility: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Guest password for Server Message Block (SMB) file shares. Only valid for `FILE_S3` and `FILE_FSX_SMB` gateway types. Must be set before creating `GuestAccess` authentication SMB file shares. This provider can only detect drift of the existence of a guest password, not its actual value from the gateway. This provider can however update the password with changing the argument.
        #[builder(into, default)]
        pub smb_guest_password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the type of security strategy. Valid values are: `ClientSpecified`, `MandatorySigning`, and `MandatoryEncryption`. See [Setting a Security Level for Your Gateway](https://docs.aws.amazon.com/storagegateway/latest/userguide/managing-gateway-file.html#security-strategy) for more information.
        #[builder(into, default)]
        pub smb_security_strategy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of tape drive to use for tape gateway. This provider cannot detect drift of this argument. Valid values: `IBM-ULT3580-TD5`.
        #[builder(into, default)]
        pub tape_drive_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GatewayResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Gateway activation key during resource creation. Conflicts with `gateway_ip_address`. Additional information is available in the [Storage Gateway User Guide](https://docs.aws.amazon.com/storagegateway/latest/userguide/get-activation-key.html).
        pub activation_key: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the gateway.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The average download bandwidth rate limit in bits per second. This is supported for the `CACHED`, `STORED`, and `VTL` gateway types.
        pub average_download_rate_limit_in_bits_per_sec: pulumi_gestalt_rust::Output<
            Option<i32>,
        >,
        /// The average upload bandwidth rate limit in bits per second. This is supported for the `CACHED`, `STORED`, and `VTL` gateway types.
        pub average_upload_rate_limit_in_bits_per_sec: pulumi_gestalt_rust::Output<
            Option<i32>,
        >,
        /// The Amazon Resource Name (ARN) of the Amazon CloudWatch log group to use to monitor and log events in the gateway.
        pub cloudwatch_log_group_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Amazon EC2 instance that was used to launch the gateway.
        pub ec2_instance_id: pulumi_gestalt_rust::Output<String>,
        /// The type of endpoint for your gateway.
        pub endpoint_type: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the gateway.
        pub gateway_id: pulumi_gestalt_rust::Output<String>,
        /// Gateway IP address to retrieve activation key during resource creation. Conflicts with `activation_key`. Gateway must be accessible on port 80 from where this provider is running. Additional information is available in the [Storage Gateway User Guide](https://docs.aws.amazon.com/storagegateway/latest/userguide/get-activation-key.html).
        pub gateway_ip_address: pulumi_gestalt_rust::Output<String>,
        /// Name of the gateway.
        pub gateway_name: pulumi_gestalt_rust::Output<String>,
        /// An array that contains descriptions of the gateway network interfaces. See Gateway Network Interface.
        pub gateway_network_interfaces: pulumi_gestalt_rust::Output<
            Vec<super::super::types::storagegateway::GatewayGatewayNetworkInterface>,
        >,
        /// Time zone for the gateway. The time zone is of the format "GMT", "GMT-hr:mm", or "GMT+hr:mm". For example, `GMT-4:00` indicates the time is 4 hours behind GMT. The time zone is used, for example, for scheduling snapshots and your gateway's maintenance schedule.
        pub gateway_timezone: pulumi_gestalt_rust::Output<String>,
        /// Type of the gateway. The default value is `STORED`. Valid values: `CACHED`, `FILE_FSX_SMB`, `FILE_S3`, `STORED`, `VTL`.
        pub gateway_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// VPC endpoint address to be used when activating your gateway. This should be used when your instance is in a private subnet. Requires HTTP access from client computer running this provider. More info on what ports are required by your VPC Endpoint Security group in [Activating a Gateway in a Virtual Private Cloud](https://docs.aws.amazon.com/storagegateway/latest/userguide/gateway-private-link.html).
        pub gateway_vpc_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of hypervisor environment used by the host.
        pub host_environment: pulumi_gestalt_rust::Output<String>,
        /// The gateway's weekly maintenance start time information, including day and time of the week. The maintenance time is the time in your gateway's time zone. More details below.
        pub maintenance_start_time: pulumi_gestalt_rust::Output<
            super::super::types::storagegateway::GatewayMaintenanceStartTime,
        >,
        /// Type of medium changer to use for tape gateway. This provider cannot detect drift of this argument. Valid values: `STK-L700`, `AWS-Gateway-VTL`, `IBM-03584L32-0402`.
        pub medium_changer_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Nested argument with Active Directory domain join information for Server Message Block (SMB) file shares. Only valid for `FILE_S3` and `FILE_FSX_SMB` gateway types. Must be set before creating `ActiveDirectory` authentication SMB file shares. More details below.
        pub smb_active_directory_settings: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::storagegateway::GatewaySmbActiveDirectorySettings,
            >,
        >,
        /// Specifies whether the shares on this gateway appear when listing shares.
        pub smb_file_share_visibility: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Guest password for Server Message Block (SMB) file shares. Only valid for `FILE_S3` and `FILE_FSX_SMB` gateway types. Must be set before creating `GuestAccess` authentication SMB file shares. This provider can only detect drift of the existence of a guest password, not its actual value from the gateway. This provider can however update the password with changing the argument.
        pub smb_guest_password: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the type of security strategy. Valid values are: `ClientSpecified`, `MandatorySigning`, and `MandatoryEncryption`. See [Setting a Security Level for Your Gateway](https://docs.aws.amazon.com/storagegateway/latest/userguide/managing-gateway-file.html#security-strategy) for more information.
        pub smb_security_strategy: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Type of tape drive to use for tape gateway. This provider cannot detect drift of this argument. Valid values: `IBM-ULT3580-TD5`.
        pub tape_drive_type: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GatewayArgs,
    ) -> GatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let activation_key_binding = args.activation_key.get_output(context);
        let average_download_rate_limit_in_bits_per_sec_binding = args
            .average_download_rate_limit_in_bits_per_sec
            .get_output(context);
        let average_upload_rate_limit_in_bits_per_sec_binding = args
            .average_upload_rate_limit_in_bits_per_sec
            .get_output(context);
        let cloudwatch_log_group_arn_binding = args
            .cloudwatch_log_group_arn
            .get_output(context);
        let gateway_ip_address_binding = args.gateway_ip_address.get_output(context);
        let gateway_name_binding = args.gateway_name.get_output(context);
        let gateway_timezone_binding = args.gateway_timezone.get_output(context);
        let gateway_type_binding = args.gateway_type.get_output(context);
        let gateway_vpc_endpoint_binding = args.gateway_vpc_endpoint.get_output(context);
        let maintenance_start_time_binding = args
            .maintenance_start_time
            .get_output(context);
        let medium_changer_type_binding = args.medium_changer_type.get_output(context);
        let smb_active_directory_settings_binding = args
            .smb_active_directory_settings
            .get_output(context);
        let smb_file_share_visibility_binding = args
            .smb_file_share_visibility
            .get_output(context);
        let smb_guest_password_binding = args.smb_guest_password.get_output(context);
        let smb_security_strategy_binding = args
            .smb_security_strategy
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tape_drive_type_binding = args.tape_drive_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:storagegateway/gateway:Gateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "activationKey".into(),
                    value: &activation_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "averageDownloadRateLimitInBitsPerSec".into(),
                    value: &average_download_rate_limit_in_bits_per_sec_binding
                        .drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "averageUploadRateLimitInBitsPerSec".into(),
                    value: &average_upload_rate_limit_in_bits_per_sec_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudwatchLogGroupArn".into(),
                    value: &cloudwatch_log_group_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayIpAddress".into(),
                    value: &gateway_ip_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayName".into(),
                    value: &gateway_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayTimezone".into(),
                    value: &gateway_timezone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayType".into(),
                    value: &gateway_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayVpcEndpoint".into(),
                    value: &gateway_vpc_endpoint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenanceStartTime".into(),
                    value: &maintenance_start_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mediumChangerType".into(),
                    value: &medium_changer_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smbActiveDirectorySettings".into(),
                    value: &smb_active_directory_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smbFileShareVisibility".into(),
                    value: &smb_file_share_visibility_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smbGuestPassword".into(),
                    value: &smb_guest_password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smbSecurityStrategy".into(),
                    value: &smb_security_strategy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tapeDriveType".into(),
                    value: &tape_drive_type_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GatewayResult {
            id: o.get_field("id"),
            activation_key: o.get_field("activationKey"),
            arn: o.get_field("arn"),
            average_download_rate_limit_in_bits_per_sec: o
                .get_field("averageDownloadRateLimitInBitsPerSec"),
            average_upload_rate_limit_in_bits_per_sec: o
                .get_field("averageUploadRateLimitInBitsPerSec"),
            cloudwatch_log_group_arn: o.get_field("cloudwatchLogGroupArn"),
            ec2_instance_id: o.get_field("ec2InstanceId"),
            endpoint_type: o.get_field("endpointType"),
            gateway_id: o.get_field("gatewayId"),
            gateway_ip_address: o.get_field("gatewayIpAddress"),
            gateway_name: o.get_field("gatewayName"),
            gateway_network_interfaces: o.get_field("gatewayNetworkInterfaces"),
            gateway_timezone: o.get_field("gatewayTimezone"),
            gateway_type: o.get_field("gatewayType"),
            gateway_vpc_endpoint: o.get_field("gatewayVpcEndpoint"),
            host_environment: o.get_field("hostEnvironment"),
            maintenance_start_time: o.get_field("maintenanceStartTime"),
            medium_changer_type: o.get_field("mediumChangerType"),
            smb_active_directory_settings: o.get_field("smbActiveDirectorySettings"),
            smb_file_share_visibility: o.get_field("smbFileShareVisibility"),
            smb_guest_password: o.get_field("smbGuestPassword"),
            smb_security_strategy: o.get_field("smbSecurityStrategy"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            tape_drive_type: o.get_field("tapeDriveType"),
        }
    }
}
