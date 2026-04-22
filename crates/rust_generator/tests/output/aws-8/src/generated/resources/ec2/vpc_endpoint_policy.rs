/// Provides a VPC Endpoint Policy resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleVpc:
///     type: aws:ec2:Vpc
///     name: example
///     properties:
///       cidrBlock: 10.0.0.0/16
///   exampleVpcEndpoint:
///     type: aws:ec2:VpcEndpoint
///     name: example
///     properties:
///       serviceName: ${example.serviceName}
///       vpcId: ${exampleVpc.id}
///   exampleVpcEndpointPolicy:
///     type: aws:ec2:VpcEndpointPolicy
///     name: example
///     properties:
///       vpcEndpointId: ${exampleVpcEndpoint.id}
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Sid: AllowAll
///               Effect: Allow
///               Principal:
///                 AWS: '*'
///               Action:
///                 - dynamodb:*
///               Resource: '*'
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ec2:getVpcEndpointService
///       arguments:
///         service: dynamodb
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Endpoint Policies using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcEndpointPolicy:VpcEndpointPolicy example vpce-3ecf2a57
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod vpc_endpoint_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointPolicyArgs {
        /// A policy to attach to the endpoint that controls access to the service. Defaults to full access. All `Gateway` and some `Interface` endpoints support policies - see the [relevant AWS documentation](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-endpoints-access.html) for more details.
        #[builder(into, default)]
        pub policy: pulumi_gestalt_rust::Input<Option<String>>,
        /// The VPC Endpoint ID.
        #[builder(into)]
        pub vpc_endpoint_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// A policy to attach to the endpoint that controls access to the service. Defaults to full access. All `Gateway` and some `Interface` endpoints support policies - see the [relevant AWS documentation](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-endpoints-access.html) for more details.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The VPC Endpoint ID.
        pub vpc_endpoint_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointPolicyArgs,
    ) -> VpcEndpointPolicyResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointPolicyArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> VpcEndpointPolicyResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointPolicyArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> VpcEndpointPolicyResult {
        let policy_binding = args.policy.get_output(ctx);
        let vpc_endpoint_id_binding = args.vpc_endpoint_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcEndpointPolicy:VpcEndpointPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcEndpointId".into(),
                    value: &vpc_endpoint_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        VpcEndpointPolicyResult {
            id: o.get_id(),
            urn: o.get_urn(),
            policy: o.get_field("policy"),
            vpc_endpoint_id: o.get_field("vpcEndpointId"),
        }
    }
}
