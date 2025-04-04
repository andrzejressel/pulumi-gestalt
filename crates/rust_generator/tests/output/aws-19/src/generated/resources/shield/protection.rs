/// Enables AWS Shield Advanced for a specific AWS resource.
/// The resource can be an Amazon CloudFront distribution, Elastic Load Balancing load balancer, AWS Global Accelerator accelerator, Elastic IP Address, or an Amazon Route 53 hosted zone.
///
/// ## Example Usage
///
/// ### Create protection
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:Eip
///     properties:
///       domain: vpc
///   exampleProtection:
///     type: aws:shield:Protection
///     name: example
///     properties:
///       name: example
///       resourceArn: arn:aws:ec2:${current.name}:${currentGetCallerIdentity.accountId}:eip-allocation/${example.id}
///       tags:
///         Environment: Dev
/// variables:
///   available:
///     fn::invoke:
///       function: aws:getAvailabilityZones
///       arguments: {}
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
///   currentGetCallerIdentity:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Shield protection resources using specifying their ID. For example:
///
/// ```sh
/// $ pulumi import aws:shield/protection:Protection example ff9592dc-22f3-4e88-afa1-7b29fde9669a
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod protection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProtectionArgs {
        /// A friendly name for the Protection you are creating.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN (Amazon Resource Name) of the resource to be protected.
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProtectionResult {
        /// The ARN of the Protection.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A friendly name for the Protection you are creating.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ARN (Amazon Resource Name) of the resource to be protected.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProtectionArgs,
    ) -> ProtectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_arn_binding = args.resource_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:shield/protection:Protection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProtectionResult {
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            resource_arn: o.get_field("resourceArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
