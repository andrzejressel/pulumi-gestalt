#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod example_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExampleServerArgs {
        #[builder(into, default)]
        pub map_array_enum: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    std::collections::HashMap<
                        String,
                        super::types::AnnotationStoreSchemaValueType,
                    >,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ExampleServerResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub map_array_enum: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    std::collections::HashMap<
                        String,
                        super::types::AnnotationStoreSchemaValueType,
                    >,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExampleServerArgs,
    ) -> ExampleServerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let map_array_enum_binding = args.map_array_enum.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "example:index:ExampleServer".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mapArrayEnum".into(),
                    value: &map_array_enum_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExampleServerResult {
            id: o.get_field("id"),
            map_array_enum: o.get_field("mapArrayEnum"),
        }
    }
}
