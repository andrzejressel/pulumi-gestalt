#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod instance_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceIamPolicyArgs {
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::Input<String>,
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::Input<Option<String>>,
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::Input<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceIamPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub etag: pulumi_gestalt_rust::Output<String>,
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceIamPolicyArgs,
    ) -> InstanceIamPolicyResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceIamPolicyArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> InstanceIamPolicyResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceIamPolicyArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> InstanceIamPolicyResult {
        let instance_id_binding = args.instance_id.get_output(ctx);
        let location_binding = args.location.get_output(ctx);
        let policy_data_binding = args.policy_data.get_output(ctx);
        let project_binding = args.project.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:securesourcemanager/instanceIamPolicy:InstanceIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        InstanceIamPolicyResult {
            id: o.get_id(),
            urn: o.get_urn(),
            etag: o.get_field("etag"),
            instance_id: o.get_field("instanceId"),
            location: o.get_field("location"),
            policy_data: o.get_field("policyData"),
            project: o.get_field("project"),
        }
    }
}
