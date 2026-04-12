/// Manages an outbound (egress) rule for a security group.
///
/// When specifying an outbound rule for your security group in a VPC, the configuration must include a destination for the traffic.
///
/// > **NOTE:** Using `aws.vpc.SecurityGroupEgressRule` and `aws.vpc.SecurityGroupIngressRule` resources is the current best practice. Avoid using the `aws.ec2.SecurityGroupRule` resource and the `ingress` and `egress` arguments of the `aws.ec2.SecurityGroup` resource for configuring in-line rules, as they struggle with managing multiple CIDR blocks, and tags and descriptions due to the historical lack of unique IDs.
///
/// !> **WARNING:** You should not use the `aws.vpc.SecurityGroupEgressRule` and `aws.vpc.SecurityGroupIngressRule` resources in conjunction with the `aws.ec2.SecurityGroup` resource with _in-line rules_ (using the `ingress` and `egress` arguments of `aws.ec2.SecurityGroup`) or the `aws.ec2.SecurityGroupRule` resource. Doing so may cause rule conflicts, perpetual differences, and result in rules being overwritten.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = security_group_egress_rule::create(
///         "example",
///         SecurityGroupEgressRuleArgs::builder()
///             .cidr_ipv_4("10.0.0.0/8")
///             .from_port(80)
///             .ip_protocol("tcp")
///             .security_group_id("${exampleAwsSecurityGroup.id}")
///             .to_port(80)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import security group egress rules using the `security_group_rule_id`. For example:
///
/// ```sh
/// $ pulumi import aws:vpc/securityGroupEgressRule:SecurityGroupEgressRule example sgr-02108b27edd666983
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod security_group_egress_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityGroupEgressRuleArgs {
        /// The destination IPv4 CIDR range.
        #[builder(into, default)]
        pub cidr_ipv4: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The destination IPv6 CIDR range.
        #[builder(into, default)]
        pub cidr_ipv6: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The security group rule description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The start of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 type.
        #[builder(into, default)]
        pub from_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The IP protocol name or number. Use `-1` to specify all protocols. Note that if `ip_protocol` is set to `-1`, it translates to all protocols, all port ranges, and `from_port` and `to_port` values should not be defined.
        #[builder(into)]
        pub ip_protocol: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the destination prefix list.
        #[builder(into, default)]
        pub prefix_list_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The destination security group that is referenced in the rule.
        #[builder(into, default)]
        pub referenced_security_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the security group.
        #[builder(into)]
        pub security_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The end of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 code.
        #[builder(into, default)]
        pub to_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct SecurityGroupEgressRuleResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the security group rule.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The destination IPv4 CIDR range.
        pub cidr_ipv4: pulumi_gestalt_rust::Output<Option<String>>,
        /// The destination IPv6 CIDR range.
        pub cidr_ipv6: pulumi_gestalt_rust::Output<Option<String>>,
        /// The security group rule description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The start of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 type.
        pub from_port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The IP protocol name or number. Use `-1` to specify all protocols. Note that if `ip_protocol` is set to `-1`, it translates to all protocols, all port ranges, and `from_port` and `to_port` values should not be defined.
        pub ip_protocol: pulumi_gestalt_rust::Output<String>,
        /// The ID of the destination prefix list.
        pub prefix_list_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The destination security group that is referenced in the rule.
        pub referenced_security_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the security group.
        pub security_group_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the security group rule.
        pub security_group_rule_id: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The end of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 code.
        pub to_port: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityGroupEgressRuleArgs,
    ) -> SecurityGroupEgressRuleResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityGroupEgressRuleArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> SecurityGroupEgressRuleResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityGroupEgressRuleArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> SecurityGroupEgressRuleResult {
        let cidr_ipv4_binding = args.cidr_ipv4.get_output(ctx);
        let cidr_ipv6_binding = args.cidr_ipv6.get_output(ctx);
        let description_binding = args.description.get_output(ctx);
        let from_port_binding = args.from_port.get_output(ctx);
        let ip_protocol_binding = args.ip_protocol.get_output(ctx);
        let prefix_list_id_binding = args.prefix_list_id.get_output(ctx);
        let referenced_security_group_id_binding = args
            .referenced_security_group_id
            .get_output(ctx);
        let security_group_id_binding = args.security_group_id.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let to_port_binding = args.to_port.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:vpc/securityGroupEgressRule:SecurityGroupEgressRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidrIpv4".into(),
                    value: &cidr_ipv4_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidrIpv6".into(),
                    value: &cidr_ipv6_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fromPort".into(),
                    value: &from_port_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipProtocol".into(),
                    value: &ip_protocol_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prefixListId".into(),
                    value: &prefix_list_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "referencedSecurityGroupId".into(),
                    value: &referenced_security_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupId".into(),
                    value: &security_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "toPort".into(),
                    value: &to_port_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        SecurityGroupEgressRuleResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            cidr_ipv4: o.get_field("cidrIpv4"),
            cidr_ipv6: o.get_field("cidrIpv6"),
            description: o.get_field("description"),
            from_port: o.get_field("fromPort"),
            ip_protocol: o.get_field("ipProtocol"),
            prefix_list_id: o.get_field("prefixListId"),
            referenced_security_group_id: o.get_field("referencedSecurityGroupId"),
            security_group_id: o.get_field("securityGroupId"),
            security_group_rule_id: o.get_field("securityGroupRuleId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            to_port: o.get_field("toPort"),
        }
    }
}
