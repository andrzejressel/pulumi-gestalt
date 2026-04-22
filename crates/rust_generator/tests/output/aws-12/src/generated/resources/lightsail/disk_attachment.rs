/// Attaches a Lightsail disk to a Lightsail Instance
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:lightsail:Disk
///     properties:
///       name: test-disk
///       sizeInGb: 8
///       availabilityZone: ${available.names[0]}
///   testInstance:
///     type: aws:lightsail:Instance
///     name: test
///     properties:
///       name: test-instance
///       availabilityZone: ${available.names[0]}
///       blueprintId: amazon_linux_2
///       bundleId: nano_3_0
///   testDisk_attachment:
///     type: aws:lightsail:Disk_attachment
///     name: test
///     properties:
///       diskName: ${test.name}
///       instanceName: ${testInstance.name}
///       diskPath: /dev/xvdf
/// variables:
///   available:
///     fn::invoke:
///       function: aws:getAvailabilityZones
///       arguments:
///         state: available
///         filters:
///           - name: opt-in-status
///             values:
///               - opt-in-not-required
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_disk` using the id attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/disk_attachment:Disk_attachment test test-disk,test-instance
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod disk_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Disk_attachmentArgs {
        /// The name of the Lightsail Disk.
        #[builder(into)]
        pub disk_name: pulumi_gestalt_rust::Input<String>,
        /// The disk path to expose to the instance.
        #[builder(into)]
        pub disk_path: pulumi_gestalt_rust::Input<String>,
        /// The name of the Lightsail Instance to attach to.
        #[builder(into)]
        pub instance_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct Disk_attachmentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name of the Lightsail Disk.
        pub disk_name: pulumi_gestalt_rust::Output<String>,
        /// The disk path to expose to the instance.
        pub disk_path: pulumi_gestalt_rust::Output<String>,
        /// The name of the Lightsail Instance to attach to.
        pub instance_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: Disk_attachmentArgs,
    ) -> Disk_attachmentResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: Disk_attachmentArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> Disk_attachmentResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: Disk_attachmentArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> Disk_attachmentResult {
        let disk_name_binding = args.disk_name.get_output(ctx);
        let disk_path_binding = args.disk_path.get_output(ctx);
        let instance_name_binding = args.instance_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/disk_attachment:Disk_attachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskName".into(),
                    value: &disk_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskPath".into(),
                    value: &disk_path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceName".into(),
                    value: &instance_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        Disk_attachmentResult {
            id: o.get_id(),
            urn: o.get_urn(),
            disk_name: o.get_field("diskName"),
            disk_path: o.get_field("diskPath"),
            instance_name: o.get_field("instanceName"),
        }
    }
}
