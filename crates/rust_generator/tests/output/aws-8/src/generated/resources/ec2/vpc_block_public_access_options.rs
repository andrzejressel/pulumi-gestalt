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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_block_public_access_options {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcBlockPublicAccessOptionsArgs {
        /// Block mode. Needs to be one of `block-bidirectional`, `block-ingress`, `off`. If this resource is deleted, then this value will be set to `off` in the AWS account and region.
        #[builder(into)]
        pub internet_gateway_block_mode: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::VpcBlockPublicAccessOptionsTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcBlockPublicAccessOptionsResult {
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcBlockPublicAccessOptionsArgs,
    ) -> VpcBlockPublicAccessOptionsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let internet_gateway_block_mode_binding = args
            .internet_gateway_block_mode
            .get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
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
        };
        let o = context.register_resource(request);
        VpcBlockPublicAccessOptionsResult {
            aws_account_id: o.get_field("awsAccountId"),
            aws_region: o.get_field("awsRegion"),
            internet_gateway_block_mode: o.get_field("internetGatewayBlockMode"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
