#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod random_terraform_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RandomTerraformConfigArgs {
        #[builder(into)]
        pub self_: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct RandomTerraformConfigResult {
        pub result: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        ctx: &pulumi_gestalt_rust::Context,
        args: RandomTerraformConfigArgs,
    ) -> RandomTerraformConfigResult {
        let self__binding = args.self_.get_output(ctx);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "pulumi:providers:random/terraformConfig".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "__self__".into(),
                    value: &self__binding.drop_type(),
                },
            ],
        };
        let o = ctx.invoke_resource(request);
        RandomTerraformConfigResult {
            result: o.get_field("result"),
        }
    }
}
