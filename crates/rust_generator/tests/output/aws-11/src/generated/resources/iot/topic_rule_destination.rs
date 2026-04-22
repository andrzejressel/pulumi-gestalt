/// ## Example Usage
///
///
/// ## Import
///
/// Using `pulumi import`, import IoT topic rule destinations using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:iot/topicRuleDestination:TopicRuleDestination example arn:aws:iot:us-west-2:123456789012:ruledestination/vpc/2ce781c8-68a6-4c52-9c62-63fe489ecc60
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod topic_rule_destination {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TopicRuleDestinationArgs {
        /// Whether or not to enable the destination. Default: `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::Input<Option<bool>>,
        /// Configuration of the virtual private cloud (VPC) connection. For more info, see the [AWS documentation](https://docs.aws.amazon.com/iot/latest/developerguide/vpc-rule-action.html).
        #[builder(into)]
        pub vpc_configuration: pulumi_gestalt_rust::Input<
            super::super::types::iot::TopicRuleDestinationVpcConfiguration,
        >,
    }
    #[allow(dead_code)]
    pub struct TopicRuleDestinationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the topic rule destination
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether or not to enable the destination. Default: `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Configuration of the virtual private cloud (VPC) connection. For more info, see the [AWS documentation](https://docs.aws.amazon.com/iot/latest/developerguide/vpc-rule-action.html).
        pub vpc_configuration: pulumi_gestalt_rust::Output<
            super::super::types::iot::TopicRuleDestinationVpcConfiguration,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TopicRuleDestinationArgs,
    ) -> TopicRuleDestinationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TopicRuleDestinationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> TopicRuleDestinationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TopicRuleDestinationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> TopicRuleDestinationResult {
        let enabled_binding = args.enabled.get_output(ctx);
        let vpc_configuration_binding = args.vpc_configuration.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iot/topicRuleDestination:TopicRuleDestination".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConfiguration".into(),
                    value: &vpc_configuration_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        TopicRuleDestinationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            enabled: o.get_field("enabled"),
            vpc_configuration: o.get_field("vpcConfiguration"),
        }
    }
}
