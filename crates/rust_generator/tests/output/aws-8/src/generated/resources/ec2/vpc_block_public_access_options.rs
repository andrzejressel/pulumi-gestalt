/// Resource for managing an AWS VPC Block Public Access Options.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vpc_block_public_access_options::create(
///         "example",
///         VpcBlockPublicAccessOptionsArgs::builder()
///             .internet_gateway_block_mode("block-bidirectional")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Block Public Access Options using the `aws_region`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcBlockPublicAccessOptions:VpcBlockPublicAccessOptions example us-east-1
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod vpc_block_public_access_options {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcBlockPublicAccessOptionsArgs {
        /// Block mode. Needs to be one of `block-bidirectional`, `block-ingress`, `off`. If this resource is deleted, then this value will be set to `off` in the AWS account and region.
        #[builder(into)]
        pub internet_gateway_block_mode: pulumi_gestalt_rust::Input<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::Input<
            Option<super::super::types::ec2::VpcBlockPublicAccessOptionsTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcBlockPublicAccessOptionsResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The AWS account id to which these options apply.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// The AWS region to which these options apply.
        pub aws_region: pulumi_gestalt_rust::Output<String>,
        /// Block mode. Needs to be one of `block-bidirectional`, `block-ingress`, `off`. If this resource is deleted, then this value will be set to `off` in the AWS account and region.
        pub internet_gateway_block_mode: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::VpcBlockPublicAccessOptionsTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcBlockPublicAccessOptionsArgs,
    ) -> VpcBlockPublicAccessOptionsResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcBlockPublicAccessOptionsArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> VpcBlockPublicAccessOptionsResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcBlockPublicAccessOptionsArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> VpcBlockPublicAccessOptionsResult {
        let internet_gateway_block_mode_binding = args
            .internet_gateway_block_mode
            .get_output(ctx);
        let timeouts_binding = args.timeouts.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcBlockPublicAccessOptions:VpcBlockPublicAccessOptions"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "internetGatewayBlockMode".into(),
                    value: &internet_gateway_block_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        VpcBlockPublicAccessOptionsResult {
            id: o.get_id(),
            urn: o.get_urn(),
            aws_account_id: o.get_field("awsAccountId"),
            aws_region: o.get_field("awsRegion"),
            internet_gateway_block_mode: o.get_field("internetGatewayBlockMode"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
