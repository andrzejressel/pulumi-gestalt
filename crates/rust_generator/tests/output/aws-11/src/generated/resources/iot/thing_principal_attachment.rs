/// Attaches Principal to AWS IoT Thing.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iot:Thing
///     properties:
///       name: example
///   cert:
///     type: aws:iot:Certificate
///     properties:
///       csr:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: csr.pem
///           return: result
///       active: true
///   att:
///     type: aws:iot:ThingPrincipalAttachment
///     properties:
///       principal: ${cert.arn}
///       thing: ${example.name}
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod thing_principal_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ThingPrincipalAttachmentArgs {
        /// The AWS IoT Certificate ARN or Amazon Cognito Identity ID.
        #[builder(into)]
        pub principal: pulumi_gestalt_rust::Input<String>,
        /// The name of the thing.
        #[builder(into)]
        pub thing: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct ThingPrincipalAttachmentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The AWS IoT Certificate ARN or Amazon Cognito Identity ID.
        pub principal: pulumi_gestalt_rust::Output<String>,
        /// The name of the thing.
        pub thing: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ThingPrincipalAttachmentArgs,
    ) -> ThingPrincipalAttachmentResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ThingPrincipalAttachmentArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ThingPrincipalAttachmentResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ThingPrincipalAttachmentArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ThingPrincipalAttachmentResult {
        let principal_binding = args.principal.get_output(ctx);
        let thing_binding = args.thing.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iot/thingPrincipalAttachment:ThingPrincipalAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principal".into(),
                    value: &principal_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "thing".into(),
                    value: &thing_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ThingPrincipalAttachmentResult {
            id: o.get_id(),
            urn: o.get_urn(),
            principal: o.get_field("principal"),
            thing: o.get_field("thing"),
        }
    }
}
