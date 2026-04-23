#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod delivery_pipeline_iam_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeliveryPipelineIamBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::Input<
            Option<super::super::types::clouddeploy::DeliveryPipelineIamBindingCondition>,
        >,
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::Input<Option<String>>,
        #[builder(into)]
        pub members: pulumi_gestalt_rust::Input<Vec<String>>,
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::Input<Option<String>>,
        #[builder(into)]
        pub role: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct DeliveryPipelineIamBindingResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::clouddeploy::DeliveryPipelineIamBindingCondition>,
        >,
        pub etag: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub members: pulumi_gestalt_rust::Output<Vec<String>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        pub role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeliveryPipelineIamBindingArgs,
    ) -> DeliveryPipelineIamBindingResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeliveryPipelineIamBindingArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DeliveryPipelineIamBindingResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeliveryPipelineIamBindingArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DeliveryPipelineIamBindingResult {
        let condition_binding = args.condition.get_output(ctx);
        let location_binding = args.location.get_output(ctx);
        let members_binding = args.members.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let project_binding = args.project.get_output(ctx);
        let role_binding = args.role.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:clouddeploy/deliveryPipelineIamBinding:DeliveryPipelineIamBinding"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: &condition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "members".into(),
                    value: &members_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: &role_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DeliveryPipelineIamBindingResult {
            id: o.get_id(),
            urn: o.get_urn(),
            condition: o.get_field("condition"),
            etag: o.get_field("etag"),
            location: o.get_field("location"),
            members: o.get_field("members"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            role: o.get_field("role"),
        }
    }
}
