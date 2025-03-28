/// A ChimeSDKVoice SIP Media Application is a managed object that passes values from a SIP rule to a target AWS Lambda function.
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
///     let example = sdkvoice_sip_media_application::create(
///         "example",
///         SdkvoiceSipMediaApplicationArgs::builder()
///             .aws_region("us-east-1")
///             .endpoints(
///                 SdkvoiceSipMediaApplicationEndpoints::builder()
///                     .lambdaArn("${test.arn}")
///                     .build_struct(),
///             )
///             .name("example-sip-media-application")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a ChimeSDKVoice SIP Media Application using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:chime/sdkvoiceSipMediaApplication:SdkvoiceSipMediaApplication example abcdef123456
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sdkvoice_sip_media_application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SdkvoiceSipMediaApplicationArgs {
        /// The AWS Region in which the AWS Chime SDK Voice Sip Media Application is created.
        #[builder(into)]
        pub aws_region: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of endpoints (Lambda Amazon Resource Names) specified for the SIP media application. Currently, only one endpoint is supported. See `endpoints`.
        #[builder(into)]
        pub endpoints: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::chime::SdkvoiceSipMediaApplicationEndpoints,
        >,
        /// The name of the AWS Chime SDK Voice Sip Media Application.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SdkvoiceSipMediaApplicationResult {
        /// ARN (Amazon Resource Name) of the AWS Chime SDK Voice Sip Media Application
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The AWS Region in which the AWS Chime SDK Voice Sip Media Application is created.
        pub aws_region: pulumi_gestalt_rust::Output<String>,
        /// List of endpoints (Lambda Amazon Resource Names) specified for the SIP media application. Currently, only one endpoint is supported. See `endpoints`.
        pub endpoints: pulumi_gestalt_rust::Output<
            super::super::types::chime::SdkvoiceSipMediaApplicationEndpoints,
        >,
        /// The name of the AWS Chime SDK Voice Sip Media Application.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: SdkvoiceSipMediaApplicationArgs,
    ) -> SdkvoiceSipMediaApplicationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let aws_region_binding = args.aws_region.get_output(context);
        let endpoints_binding = args.endpoints.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:chime/sdkvoiceSipMediaApplication:SdkvoiceSipMediaApplication"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsRegion".into(),
                    value: &aws_region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpoints".into(),
                    value: &endpoints_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SdkvoiceSipMediaApplicationResult {
            arn: o.get_field("arn"),
            aws_region: o.get_field("awsRegion"),
            endpoints: o.get_field("endpoints"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
