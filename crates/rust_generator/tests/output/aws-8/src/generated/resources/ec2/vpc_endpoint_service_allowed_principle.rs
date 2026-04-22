/// Provides a resource to allow a principal to discover a VPC endpoint service.
///
/// > **NOTE on VPC Endpoint Services and VPC Endpoint Service Allowed Principals:** This provider provides
/// both a standalone VPC Endpoint Service Allowed Principal resource
/// and a VPC Endpoint Service resource with an `allowed_principals` attribute. Do not use the same principal ARN in both
/// a VPC Endpoint Service resource and a VPC Endpoint Service Allowed Principal resource. Doing so will cause a conflict
/// and will overwrite the association.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   allowMeToFoo:
///     type: aws:ec2:VpcEndpointServiceAllowedPrinciple
///     name: allow_me_to_foo
///     properties:
///       vpcEndpointServiceId: ${foo.id}
///       principalArn: ${current.arn}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod vpc_endpoint_service_allowed_principle {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointServiceAllowedPrincipleArgs {
        /// The ARN of the principal to allow permissions.
        #[builder(into)]
        pub principal_arn: pulumi_gestalt_rust::Input<String>,
        /// The ID of the VPC endpoint service to allow permission.
        #[builder(into)]
        pub vpc_endpoint_service_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointServiceAllowedPrincipleResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the principal to allow permissions.
        pub principal_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VPC endpoint service to allow permission.
        pub vpc_endpoint_service_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointServiceAllowedPrincipleArgs,
    ) -> VpcEndpointServiceAllowedPrincipleResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointServiceAllowedPrincipleArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> VpcEndpointServiceAllowedPrincipleResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointServiceAllowedPrincipleArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> VpcEndpointServiceAllowedPrincipleResult {
        let principal_arn_binding = args.principal_arn.get_output(ctx);
        let vpc_endpoint_service_id_binding = args
            .vpc_endpoint_service_id
            .get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcEndpointServiceAllowedPrinciple:VpcEndpointServiceAllowedPrinciple"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalArn".into(),
                    value: &principal_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcEndpointServiceId".into(),
                    value: &vpc_endpoint_service_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        VpcEndpointServiceAllowedPrincipleResult {
            id: o.get_id(),
            urn: o.get_urn(),
            principal_arn: o.get_field("principalArn"),
            vpc_endpoint_service_id: o.get_field("vpcEndpointServiceId"),
        }
    }
}
