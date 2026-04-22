/// Provides a DAX Parameter Group resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = parameter_group::create(
///         "example",
///         ParameterGroupArgs::builder()
///             .name("example")
///             .parameters(
///                 vec![
///                     ParameterGroupParameter::builder().name("query-ttl-millis")
///                     .value("100000").build_struct(), ParameterGroupParameter::builder()
///                     .name("record-ttl-millis").value("100000").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DAX Parameter Group using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:dax/parameterGroup:ParameterGroup example my_dax_pg
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod parameter_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ParameterGroupArgs {
        /// A description of the parameter group.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the parameter group.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The parameters of the parameter group.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::Input<
            Option<Vec<super::super::types::dax::ParameterGroupParameter>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ParameterGroupResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// A description of the parameter group.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the parameter group.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parameters of the parameter group.
        pub parameters: pulumi_gestalt_rust::Output<
            Vec<super::super::types::dax::ParameterGroupParameter>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ParameterGroupArgs,
    ) -> ParameterGroupResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ParameterGroupArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ParameterGroupResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ParameterGroupArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ParameterGroupResult {
        let description_binding = args.description.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let parameters_binding = args.parameters.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:dax/parameterGroup:ParameterGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ParameterGroupResult {
            id: o.get_id(),
            urn: o.get_urn(),
            description: o.get_field("description"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
        }
    }
}
