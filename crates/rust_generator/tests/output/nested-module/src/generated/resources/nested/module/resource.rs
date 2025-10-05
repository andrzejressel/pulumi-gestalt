#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceArgs {
        #[builder(into, default)]
        pub bar: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ResourceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub bar: pulumi_gestalt_rust::Output<Option<String>>,
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
        let bar_binding = args.bar.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "foo:nested/module:Resource".into(),
            name: name.to_string(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bar".into(),
                    value: &bar_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceResult {
            id: o.get_field("id"),
            bar: o.get_field("bar"),
        }
    }
}
