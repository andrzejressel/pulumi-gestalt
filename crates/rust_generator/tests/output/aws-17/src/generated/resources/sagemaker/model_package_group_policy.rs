/// Provides a SageMaker Model Package Group Policy resource.
///
/// ## Example Usage
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Model Package Groups using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/modelPackageGroupPolicy:ModelPackageGroupPolicy example example
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod model_package_group_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ModelPackageGroupPolicyArgs {
        /// The name of the model package group.
        #[builder(into)]
        pub model_package_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub resource_policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ModelPackageGroupPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name of the model package group.
        pub model_package_group_name: pulumi_gestalt_rust::Output<String>,
        pub resource_policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ModelPackageGroupPolicyArgs,
    ) -> ModelPackageGroupPolicyResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ModelPackageGroupPolicyArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ModelPackageGroupPolicyResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ModelPackageGroupPolicyArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ModelPackageGroupPolicyResult {
        let model_package_group_name_binding = args
            .model_package_group_name
            .get_output(ctx);
        let resource_policy_binding = args.resource_policy.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/modelPackageGroupPolicy:ModelPackageGroupPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "modelPackageGroupName".into(),
                    value: &model_package_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourcePolicy".into(),
                    value: &resource_policy_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ModelPackageGroupPolicyResult {
            id: o.get_id(),
            urn: o.get_urn(),
            model_package_group_name: o.get_field("modelPackageGroupName"),
            resource_policy: o.get_field("resourcePolicy"),
        }
    }
}
