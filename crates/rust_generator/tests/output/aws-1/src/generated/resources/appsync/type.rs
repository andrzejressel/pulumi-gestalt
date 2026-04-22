/// Provides an AppSync Type.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appsync:GraphQLApi
///     properties:
///       authenticationType: API_KEY
///       name: example
///   exampleType:
///     type: aws:appsync:Type
///     name: example
///     properties:
///       apiId: ${example.id}
///       format: SDL
///       definition: |
///         type Mutation
///
///         {
///         putPost(id: ID!,title: String! ): Post
///
///         }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Appsync Types using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:appsync/type:Type example api-id:format:name
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod type_ {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TypeArgs {
        /// GraphQL API ID.
        #[builder(into)]
        pub api_id: pulumi_gestalt_rust::Input<String>,
        /// The type definition.
        #[builder(into)]
        pub definition: pulumi_gestalt_rust::Input<String>,
        /// The type format: `SDL` or `JSON`.
        #[builder(into)]
        pub format: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct TypeResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// GraphQL API ID.
        pub api_id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the type.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The type definition.
        pub definition: pulumi_gestalt_rust::Output<String>,
        /// The type description.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The type format: `SDL` or `JSON`.
        pub format: pulumi_gestalt_rust::Output<String>,
        /// The type name.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TypeArgs,
    ) -> TypeResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TypeArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> TypeResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TypeArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> TypeResult {
        let api_id_binding = args.api_id.get_output(ctx);
        let definition_binding = args.definition.get_output(ctx);
        let format_binding = args.format.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appsync/type:Type".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "definition".into(),
                    value: &definition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "format".into(),
                    value: &format_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        TypeResult {
            id: o.get_id(),
            urn: o.get_urn(),
            api_id: o.get_field("apiId"),
            arn: o.get_field("arn"),
            definition: o.get_field("definition"),
            description: o.get_field("description"),
            format: o.get_field("format"),
            name: o.get_field("name"),
        }
    }
}
