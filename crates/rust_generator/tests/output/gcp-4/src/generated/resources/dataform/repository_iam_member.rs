#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod repository_iam_member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryIamMemberArgs {
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::Input<
            Option<super::super::types::dataform::RepositoryIamMemberCondition>,
        >,
        #[builder(into)]
        pub member: pulumi_gestalt_rust::Input<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::Input<Option<String>>,
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::Input<Option<String>>,
        #[builder(into)]
        pub repository: pulumi_gestalt_rust::Input<String>,
        #[builder(into)]
        pub role: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct RepositoryIamMemberResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataform::RepositoryIamMemberCondition>,
        >,
        pub etag: pulumi_gestalt_rust::Output<String>,
        pub member: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        pub region: pulumi_gestalt_rust::Output<String>,
        pub repository: pulumi_gestalt_rust::Output<String>,
        pub role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RepositoryIamMemberArgs,
    ) -> RepositoryIamMemberResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RepositoryIamMemberArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> RepositoryIamMemberResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RepositoryIamMemberArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> RepositoryIamMemberResult {
        let condition_binding = args.condition.get_output(ctx);
        let member_binding = args.member.get_output(ctx);
        let project_binding = args.project.get_output(ctx);
        let region_binding = args.region.get_output(ctx);
        let repository_binding = args.repository.get_output(ctx);
        let role_binding = args.role.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataform/repositoryIamMember:RepositoryIamMember".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: &condition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "member".into(),
                    value: &member_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repository".into(),
                    value: &repository_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: &role_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        RepositoryIamMemberResult {
            id: o.get_id(),
            urn: o.get_urn(),
            condition: o.get_field("condition"),
            etag: o.get_field("etag"),
            member: o.get_field("member"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            repository: o.get_field("repository"),
            role: o.get_field("role"),
        }
    }
}
