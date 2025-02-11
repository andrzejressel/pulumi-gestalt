/// The AMI resource allows the creation and management of a completely-custom
/// *Amazon Machine Image* (AMI).
///
/// If you just want to duplicate an existing AMI, possibly copying it to another
/// region, it's better to use `aws.ec2.AmiCopy` instead.
///
/// If you just want to share an existing AMI with another AWS account,
/// it's better to use `aws.ec2.AmiLaunchPermission` instead.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = ami::create(
///         "example",
///         AmiArgs::builder()
///             .ebs_block_devices(
///                 vec![
///                     AmiEbsBlockDevice::builder().deviceName("/dev/xvda")
///                     .snapshotId("snap-xxxxxxxx").volumeSize(8).build_struct(),
///                 ],
///             )
///             .imds_support("v2.0")
///             .name("example")
///             .root_device_name("/dev/xvda")
///             .virtualization_type("hvm")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ami` using the ID of the AMI. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/ami:Ami example ami-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ami {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AmiArgs {
        /// Machine architecture for created instances. Defaults to "x86_64".
        #[builder(into, default)]
        pub architecture: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boot mode of the AMI. For more information, see [Boot modes](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ami-boot.html) in the Amazon Elastic Compute Cloud User Guide.
        #[builder(into, default)]
        pub boot_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Date and time to deprecate the AMI. If you specified a value for seconds, Amazon EC2 rounds the seconds to the nearest minute. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        #[builder(into, default)]
        pub deprecation_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Longer, human-readable description for the AMI.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Nested block describing an EBS block device that should be
        /// attached to created instances. The structure of this block is described below.
        #[builder(into, default)]
        pub ebs_block_devices: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::AmiEbsBlockDevice>>,
        >,
        /// Whether enhanced networking with ENA is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub ena_support: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Nested block describing an ephemeral block device that
        /// should be attached to created instances. The structure of this block is described below.
        #[builder(into, default)]
        pub ephemeral_block_devices: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::AmiEphemeralBlockDevice>>,
        >,
        #[builder(into, default)]
        pub image_location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If EC2 instances started from this image should require the use of the Instance Metadata Service V2 (IMDSv2), set this argument to `v2.0`. For more information, see [Configure instance metadata options for new instances](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-IMDS-new-instances.html#configure-IMDS-new-instances-ami-configuration).
        #[builder(into, default)]
        pub imds_support: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub kernel_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Region-unique name for the AMI.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub ramdisk_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the root device (for example, `/dev/sda1`, or `/dev/xvda`).
        #[builder(into, default)]
        pub root_device_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub sriov_net_support: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If the image is configured for NitroTPM support, the value is `v2.0`. For more information, see [NitroTPM](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/nitrotpm.html) in the Amazon Elastic Compute Cloud User Guide.
        #[builder(into, default)]
        pub tpm_support: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Keyword to choose what virtualization mode created instances
        /// will use. Can be either "paravirtual" (the default) or "hvm". The choice of virtualization type
        /// changes the set of further arguments that are required, as described below.
        #[builder(into, default)]
        pub virtualization_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AmiResult {
        /// Machine architecture for created instances. Defaults to "x86_64".
        pub architecture: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the AMI.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Boot mode of the AMI. For more information, see [Boot modes](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ami-boot.html) in the Amazon Elastic Compute Cloud User Guide.
        pub boot_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// Date and time to deprecate the AMI. If you specified a value for seconds, Amazon EC2 rounds the seconds to the nearest minute. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        pub deprecation_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// Longer, human-readable description for the AMI.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Nested block describing an EBS block device that should be
        /// attached to created instances. The structure of this block is described below.
        pub ebs_block_devices: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::AmiEbsBlockDevice>,
        >,
        /// Whether enhanced networking with ENA is enabled. Defaults to `false`.
        pub ena_support: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Nested block describing an ephemeral block device that
        /// should be attached to created instances. The structure of this block is described below.
        pub ephemeral_block_devices: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::AmiEphemeralBlockDevice>,
        >,
        /// Hypervisor type of the image.
        pub hypervisor: pulumi_gestalt_rust::Output<String>,
        pub image_location: pulumi_gestalt_rust::Output<String>,
        /// AWS account alias (for example, amazon, self) or the AWS account ID of the AMI owner.
        pub image_owner_alias: pulumi_gestalt_rust::Output<String>,
        /// Type of image.
        pub image_type: pulumi_gestalt_rust::Output<String>,
        /// If EC2 instances started from this image should require the use of the Instance Metadata Service V2 (IMDSv2), set this argument to `v2.0`. For more information, see [Configure instance metadata options for new instances](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-IMDS-new-instances.html#configure-IMDS-new-instances-ami-configuration).
        pub imds_support: pulumi_gestalt_rust::Output<Option<String>>,
        pub kernel_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub manage_ebs_snapshots: pulumi_gestalt_rust::Output<bool>,
        /// Region-unique name for the AMI.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID of the image owner.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// This value is set to windows for Windows AMIs; otherwise, it is blank.
        pub platform: pulumi_gestalt_rust::Output<String>,
        /// Platform details associated with the billing code of the AMI.
        pub platform_details: pulumi_gestalt_rust::Output<String>,
        /// Whether the image has public launch permissions.
        pub public: pulumi_gestalt_rust::Output<bool>,
        pub ramdisk_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the root device (for example, `/dev/sda1`, or `/dev/xvda`).
        pub root_device_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Snapshot ID for the root volume (for EBS-backed AMIs)
        pub root_snapshot_id: pulumi_gestalt_rust::Output<String>,
        pub sriov_net_support: pulumi_gestalt_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If the image is configured for NitroTPM support, the value is `v2.0`. For more information, see [NitroTPM](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/nitrotpm.html) in the Amazon Elastic Compute Cloud User Guide.
        pub tpm_support: pulumi_gestalt_rust::Output<Option<String>>,
        /// Operation of the Amazon EC2 instance and the billing code that is associated with the AMI.
        pub usage_operation: pulumi_gestalt_rust::Output<String>,
        /// Keyword to choose what virtualization mode created instances
        /// will use. Can be either "paravirtual" (the default) or "hvm". The choice of virtualization type
        /// changes the set of further arguments that are required, as described below.
        pub virtualization_type: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AmiArgs,
    ) -> AmiResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let architecture_binding = args.architecture.get_output(context);
        let boot_mode_binding = args.boot_mode.get_output(context);
        let deprecation_time_binding = args.deprecation_time.get_output(context);
        let description_binding = args.description.get_output(context);
        let ebs_block_devices_binding = args.ebs_block_devices.get_output(context);
        let ena_support_binding = args.ena_support.get_output(context);
        let ephemeral_block_devices_binding = args
            .ephemeral_block_devices
            .get_output(context);
        let image_location_binding = args.image_location.get_output(context);
        let imds_support_binding = args.imds_support.get_output(context);
        let kernel_id_binding = args.kernel_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let ramdisk_id_binding = args.ramdisk_id.get_output(context);
        let root_device_name_binding = args.root_device_name.get_output(context);
        let sriov_net_support_binding = args.sriov_net_support.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tpm_support_binding = args.tpm_support.get_output(context);
        let virtualization_type_binding = args.virtualization_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/ami:Ami".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "architecture".into(),
                    value: &architecture_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bootMode".into(),
                    value: &boot_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deprecationTime".into(),
                    value: &deprecation_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ebsBlockDevices".into(),
                    value: &ebs_block_devices_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enaSupport".into(),
                    value: &ena_support_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ephemeralBlockDevices".into(),
                    value: &ephemeral_block_devices_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageLocation".into(),
                    value: &image_location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imdsSupport".into(),
                    value: &imds_support_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kernelId".into(),
                    value: &kernel_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ramdiskId".into(),
                    value: &ramdisk_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rootDeviceName".into(),
                    value: &root_device_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sriovNetSupport".into(),
                    value: &sriov_net_support_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tpmSupport".into(),
                    value: &tpm_support_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualizationType".into(),
                    value: &virtualization_type_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AmiResult {
            architecture: o.get_field("architecture"),
            arn: o.get_field("arn"),
            boot_mode: o.get_field("bootMode"),
            deprecation_time: o.get_field("deprecationTime"),
            description: o.get_field("description"),
            ebs_block_devices: o.get_field("ebsBlockDevices"),
            ena_support: o.get_field("enaSupport"),
            ephemeral_block_devices: o.get_field("ephemeralBlockDevices"),
            hypervisor: o.get_field("hypervisor"),
            image_location: o.get_field("imageLocation"),
            image_owner_alias: o.get_field("imageOwnerAlias"),
            image_type: o.get_field("imageType"),
            imds_support: o.get_field("imdsSupport"),
            kernel_id: o.get_field("kernelId"),
            manage_ebs_snapshots: o.get_field("manageEbsSnapshots"),
            name: o.get_field("name"),
            owner_id: o.get_field("ownerId"),
            platform: o.get_field("platform"),
            platform_details: o.get_field("platformDetails"),
            public: o.get_field("public"),
            ramdisk_id: o.get_field("ramdiskId"),
            root_device_name: o.get_field("rootDeviceName"),
            root_snapshot_id: o.get_field("rootSnapshotId"),
            sriov_net_support: o.get_field("sriovNetSupport"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            tpm_support: o.get_field("tpmSupport"),
            usage_operation: o.get_field("usageOperation"),
            virtualization_type: o.get_field("virtualizationType"),
        }
    }
}
