/// Manages a Resource Access Manager (RAM) Resource Association.
///
/// > *NOTE:* Certain AWS resources (e.g., EC2 Subnets) can only be shared in an AWS account that is a member of an AWS Organizations organization with organization-wide Resource Access Manager functionality enabled. See the [Resource Access Manager User Guide](https://docs.aws.amazon.com/ram/latest/userguide/what-is.html) and AWS service specific documentation for additional information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_association::create(
///         "example",
///         ResourceAssociationArgs::builder()
///             .resource_arn("${exampleAwsSubnet.arn}")
///             .resource_share_arn("${exampleAwsRamResourceShare.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import RAM Resource Associations using their Resource Share ARN and Resource ARN separated by a comma. For example:
///
/// ```sh
/// $ pulumi import aws:ram/resourceAssociation:ResourceAssociation example arn:aws:ram:eu-west-1:123456789012:resource-share/73da1ab9-b94a-4ba3-8eb4-45917f7f4b12,arn:aws:ec2:eu-west-1:123456789012:subnet/subnet-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceAssociationArgs {
        /// Amazon Resource Name (ARN) of the resource to associate with the RAM Resource Share.
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Amazon Resource Name (ARN) of the RAM Resource Share.
        #[builder(into)]
        pub resource_share_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceAssociationResult {
        /// Amazon Resource Name (ARN) of the resource to associate with the RAM Resource Share.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the RAM Resource Share.
        pub resource_share_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceAssociationArgs,
    ) -> ResourceAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let resource_arn_binding = args.resource_arn.get_output(context);
        let resource_share_arn_binding = args.resource_share_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ram/resourceAssociation:ResourceAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceShareArn".into(),
                    value: &resource_share_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceAssociationResult {
            resource_arn: o.get_field("resourceArn"),
            resource_share_arn: o.get_field("resourceShareArn"),
        }
    }
}
