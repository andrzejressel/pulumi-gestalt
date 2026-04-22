/// Creates an Amazon CloudFront VPC origin.
///
/// For information about CloudFront VPC origins, see
/// [Amazon CloudFront Developer Guide - Restrict access with VPC origins][1].
///
/// ## Example Usage
///
/// ### Application Load Balancer
///
/// The following example below creates a CloudFront VPC origin for a Application Load Balancer.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let alb = vpc_origin::create(
///         "alb",
///         VpcOriginArgs::builder()
///             .vpc_origin_endpoint_config(
///                 VpcOriginVpcOriginEndpointConfig::builder()
///                     .arn("${this.arn}")
///                     .httpPort(8080)
///                     .httpsPort(8443)
///                     .name("Example VPC Origin")
///                     .originProtocolPolicy("https-only")
///                     .originSslProtocols(
///                         VpcOriginVpcOriginEndpointConfigOriginSslProtocols::builder()
///                             .items(vec!["TLSv1.2",])
///                             .quantity(1)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// terraform
///
/// import {
///
///   to = aws_cloudfront_vpc_origin.origin
///
///   id = vo_JQEa410sssUFoY6wMkx69j
///
/// }
///
/// Using `pulumi import`, import Cloudfront VPC origins using the `id`. For example:
///
/// console
///
/// % pulumi import aws_cloudfront_vpc_origin vo_JQEa410sssUFoY6wMkx69j
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod vpc_origin {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcOriginArgs {
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::Input<
            Option<super::super::types::cloudfront::VpcOriginTimeouts>,
        >,
        #[builder(into, default)]
        pub vpc_origin_endpoint_config: pulumi_gestalt_rust::Input<
            Option<super::super::types::cloudfront::VpcOriginVpcOriginEndpointConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcOriginResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The VPC origin ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The current version of the origin.
        pub etag: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudfront::VpcOriginTimeouts>,
        >,
        pub vpc_origin_endpoint_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudfront::VpcOriginVpcOriginEndpointConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcOriginArgs,
    ) -> VpcOriginResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcOriginArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> VpcOriginResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcOriginArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> VpcOriginResult {
        let tags_binding = args.tags.get_output(ctx);
        let timeouts_binding = args.timeouts.get_output(ctx);
        let vpc_origin_endpoint_config_binding = args
            .vpc_origin_endpoint_config
            .get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudfront/vpcOrigin:VpcOrigin".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcOriginEndpointConfig".into(),
                    value: &vpc_origin_endpoint_config_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        VpcOriginResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            etag: o.get_field("etag"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
            vpc_origin_endpoint_config: o.get_field("vpcOriginEndpointConfig"),
        }
    }
}
