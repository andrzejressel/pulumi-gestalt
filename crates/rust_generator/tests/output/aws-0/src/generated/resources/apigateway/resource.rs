/// Provides an API Gateway Resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myDemoAPI = rest_api::create(
///         "myDemoAPI",
///         RestApiArgs::builder()
///             .description("This is my API for demonstration purposes")
///             .name("MyDemoAPI")
///             .build_struct(),
///     );
///     let myDemoResource = resource::create(
///         "myDemoResource",
///         ResourceArgs::builder()
///             .parent_id("${myDemoAPI.rootResourceId}")
///             .path_part("mydemoresource")
///             .rest_api("${myDemoAPI.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_resource` using `REST-API-ID/RESOURCE-ID`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/resource:Resource example 12345abcde/67890fghij
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceArgs {
        /// ID of the parent API resource
        #[builder(into)]
        pub parent_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Last path segment of this API resource.
        #[builder(into)]
        pub path_part: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the associated REST API
        #[builder(into)]
        pub rest_api: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ID of the parent API resource
        pub parent_id: pulumi_gestalt_rust::Output<String>,
        /// Complete path for this API resource, including all parent paths.
        pub path: pulumi_gestalt_rust::Output<String>,
        /// Last path segment of this API resource.
        pub path_part: pulumi_gestalt_rust::Output<String>,
        /// ID of the associated REST API
        pub rest_api: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceArgs,
    ) -> ResourceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let parent_id_binding = args.parent_id.get_output(context);
        let path_part_binding = args.path_part.get_output(context);
        let rest_api_binding = args.rest_api.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/resource:Resource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentId".into(),
                    value: &parent_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pathPart".into(),
                    value: &path_part_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceResult {
            id: o.get_field("id"),
            parent_id: o.get_field("parentId"),
            path: o.get_field("path"),
            path_part: o.get_field("pathPart"),
            rest_api: o.get_field("restApi"),
        }
    }
}
