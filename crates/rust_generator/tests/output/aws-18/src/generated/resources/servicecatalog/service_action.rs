/// Manages a Service Catalog self-service action.
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
///     let example = service_action::create(
///         "example",
///         ServiceActionArgs::builder()
///             .definition(
///                 ServiceActionDefinition::builder()
///                     .name("AWS-RestartEC2Instance")
///                     .build_struct(),
///             )
///             .description("Motor generator unit")
///             .name("MGU")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_servicecatalog_service_action` using the service action ID. For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/serviceAction:ServiceAction example act-f1w12eperfslh
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod service_action {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceActionArgs {
        /// Language code. Valid values are `en` (English), `jp` (Japanese), and `zh` (Chinese). Default is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Self-service action definition configuration block. Detailed below.
        #[builder(into)]
        pub definition: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::servicecatalog::ServiceActionDefinition,
        >,
        /// Self-service action description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Self-service action name.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ServiceActionResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Language code. Valid values are `en` (English), `jp` (Japanese), and `zh` (Chinese). Default is `en`.
        pub accept_language: pulumi_gestalt_rust::Output<Option<String>>,
        /// Self-service action definition configuration block. Detailed below.
        pub definition: pulumi_gestalt_rust::Output<
            super::super::types::servicecatalog::ServiceActionDefinition,
        >,
        /// Self-service action description.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Self-service action name.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceActionArgs,
    ) -> ServiceActionResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceActionArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ServiceActionResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceActionArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ServiceActionResult {
        let accept_language_binding = args.accept_language.get_output(ctx);
        let definition_binding = args.definition.get_output(ctx);
        let description_binding = args.description.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:servicecatalog/serviceAction:ServiceAction".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acceptLanguage".into(),
                    value: &accept_language_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "definition".into(),
                    value: &definition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ServiceActionResult {
            id: o.get_id(),
            urn: o.get_urn(),
            accept_language: o.get_field("acceptLanguage"),
            definition: o.get_field("definition"),
            description: o.get_field("description"),
            name: o.get_field("name"),
        }
    }
}
