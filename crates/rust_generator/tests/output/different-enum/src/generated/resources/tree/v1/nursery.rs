#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod nursery {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NurseryArgs {
        /// The sizes of trees available
        #[builder(into, default)]
        pub sizes: pulumi_gestalt_rust::InputOrOutput<
            Option<
                std::collections::HashMap<
                    String,
                    super::super::super::types::tree::v1::TreeSize,
                >,
            >,
        >,
        /// The varieties available
        #[builder(into)]
        pub varieties: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::super::types::tree::v1::RubberTreeVariety>,
        >,
    }
    #[allow(dead_code)]
    pub struct NurseryResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NurseryArgs,
    ) -> NurseryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let sizes_binding = args.sizes.get_output(context);
        let varieties_binding = args.varieties.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "plant:tree/v1:Nursery".into(),
            name: name.to_string(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sizes".into(),
                    value: &sizes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "varieties".into(),
                    value: &varieties_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NurseryResult {
            id: o.get_field("id"),
        }
    }
}
