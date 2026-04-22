/// Provides a Model for a REST API Gateway.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   myDemoAPI:
///     type: aws:apigateway:RestApi
///     name: MyDemoAPI
///     properties:
///       name: MyDemoAPI
///       description: This is my API for demonstration purposes
///   myDemoModel:
///     type: aws:apigateway:Model
///     name: MyDemoModel
///     properties:
///       restApi: ${myDemoAPI.id}
///       name: user
///       description: a JSON schema
///       contentType: application/json
///       schema:
///         fn::toJSON:
///           type: object
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_model` using `REST-API-ID/NAME`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/model:Model example 12345abcde/example
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod model {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ModelArgs {
        /// Content type of the model
        #[builder(into)]
        pub content_type: pulumi_gestalt_rust::Input<String>,
        /// Description of the model
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// Name of the model
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// ID of the associated REST API
        #[builder(into)]
        pub rest_api: pulumi_gestalt_rust::Input<String>,
        /// Schema of the model in a JSON form
        #[builder(into, default)]
        pub schema: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ModelResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Content type of the model
        pub content_type: pulumi_gestalt_rust::Output<String>,
        /// Description of the model
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the model
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ID of the associated REST API
        pub rest_api: pulumi_gestalt_rust::Output<String>,
        /// Schema of the model in a JSON form
        pub schema: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ModelArgs,
    ) -> ModelResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ModelArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ModelResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ModelArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ModelResult {
        let content_type_binding = args.content_type.get_output(ctx);
        let description_binding = args.description.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let rest_api_binding = args.rest_api.get_output(ctx);
        let schema_binding = args.schema.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/model:Model".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentType".into(),
                    value: &content_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schema".into(),
                    value: &schema_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ModelResult {
            id: o.get_id(),
            urn: o.get_urn(),
            content_type: o.get_field("contentType"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            rest_api: o.get_field("restApi"),
            schema: o.get_field("schema"),
        }
    }
}
