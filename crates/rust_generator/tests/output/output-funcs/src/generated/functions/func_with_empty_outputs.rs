#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod func_with_empty_outputs {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FuncWithEmptyOutputsArgs {
        /// The Name of the FeatureGroup.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::Input<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(ctx: &pulumi_gestalt_rust::Context, args: FuncWithEmptyOutputsArgs) {
        let name_binding = args.name.get_output(ctx);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "mypkg::funcWithEmptyOutputs".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        ctx.invoke_resource(request);
    }
}
