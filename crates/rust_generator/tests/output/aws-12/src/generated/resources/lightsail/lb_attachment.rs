/// Attaches a Lightsail Instance to a Lightsail Load Balancer.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:lightsail:Lb
///     properties:
///       name: test-load-balancer
///       healthCheckPath: /
///       instancePort: '80'
///       tags:
///         foo: bar
///   testInstance:
///     type: aws:lightsail:Instance
///     name: test
///     properties:
///       name: test-instance
///       availabilityZone: ${available.names[0]}
///       blueprintId: amazon_linux_2
///       bundleId: nano_3_0
///   testLbAttachment:
///     type: aws:lightsail:LbAttachment
///     name: test
///     properties:
///       lbName: ${test.name}
///       instanceName: ${testInstance.name}
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
/// Using `pulumi import`, import `aws_lightsail_lb_attachment` using the name attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/lbAttachment:LbAttachment test example-load-balancer,example-instance
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod lb_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LbAttachmentArgs {
        /// The name of the instance to attach to the load balancer.
        #[builder(into)]
        pub instance_name: pulumi_gestalt_rust::Input<String>,
        /// The name of the Lightsail load balancer.
        #[builder(into)]
        pub lb_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct LbAttachmentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name of the instance to attach to the load balancer.
        pub instance_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Lightsail load balancer.
        pub lb_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LbAttachmentArgs,
    ) -> LbAttachmentResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LbAttachmentArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> LbAttachmentResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LbAttachmentArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> LbAttachmentResult {
        let instance_name_binding = args.instance_name.get_output(ctx);
        let lb_name_binding = args.lb_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/lbAttachment:LbAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceName".into(),
                    value: &instance_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lbName".into(),
                    value: &lb_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        LbAttachmentResult {
            id: o.get_id(),
            urn: o.get_urn(),
            instance_name: o.get_field("instanceName"),
            lb_name: o.get_field("lbName"),
        }
    }
}
