/// ## Example Usage
///
/// ## Import
///
/// RepositoryGroup can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/codeRepositoryIndexes/{{code_repository_index}}/repositoryGroups/{{repository_group_id}}`
///
/// * `{{project}}/{{location}}/{{code_repository_index}}/{{repository_group_id}}`
///
/// * `{{location}}/{{code_repository_index}}/{{repository_group_id}}`
///
/// When using the `pulumi import` command, RepositoryGroup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gemini/repositoryGroup:RepositoryGroup default projects/{{project}}/locations/{{location}}/codeRepositoryIndexes/{{code_repository_index}}/repositoryGroups/{{repository_group_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gemini/repositoryGroup:RepositoryGroup default {{project}}/{{location}}/{{code_repository_index}}/{{repository_group_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gemini/repositoryGroup:RepositoryGroup default {{location}}/{{code_repository_index}}/{{repository_group_id}}
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod repository_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryGroupArgs {
        /// Required. Id of the Code Repository Index.
        #[builder(into)]
        pub code_repository_index: pulumi_gestalt_rust::Input<String>,
        /// Optional. Labels as key value pairs **Note**: This field is non-authoritative, and will only manage the labels present
        /// in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the Code Repository Index, for example `us-central1`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::Input<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::Input<Option<String>>,
        /// Required. List of repositories to group
        /// Structure is documented below.
        #[builder(into)]
        pub repositories: pulumi_gestalt_rust::Input<
            Vec<super::super::types::gemini::RepositoryGroupRepository>,
        >,
        /// Required. Id of the Repository Group.
        #[builder(into)]
        pub repository_group_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct RepositoryGroupResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Required. Id of the Code Repository Index.
        pub code_repository_index: pulumi_gestalt_rust::Output<String>,
        /// Output only. Create time stamp
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. Labels as key value pairs **Note**: This field is non-authoritative, and will only manage the labels present
        /// in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the Code Repository Index, for example `us-central1`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Immutable. Identifier. name of resource
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Required. List of repositories to group
        /// Structure is documented below.
        pub repositories: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gemini::RepositoryGroupRepository>,
        >,
        /// Required. Id of the Repository Group.
        pub repository_group_id: pulumi_gestalt_rust::Output<String>,
        /// Output only. Update time stamp
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RepositoryGroupArgs,
    ) -> RepositoryGroupResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RepositoryGroupArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> RepositoryGroupResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RepositoryGroupArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> RepositoryGroupResult {
        let code_repository_index_binding = args.code_repository_index.get_output(ctx);
        let labels_binding = args.labels.get_output(ctx);
        let location_binding = args.location.get_output(ctx);
        let project_binding = args.project.get_output(ctx);
        let repositories_binding = args.repositories.get_output(ctx);
        let repository_group_id_binding = args.repository_group_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:gemini/repositoryGroup:RepositoryGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "codeRepositoryIndex".into(),
                    value: &code_repository_index_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositories".into(),
                    value: &repositories_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositoryGroupId".into(),
                    value: &repository_group_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        RepositoryGroupResult {
            id: o.get_id(),
            urn: o.get_urn(),
            code_repository_index: o.get_field("codeRepositoryIndex"),
            create_time: o.get_field("createTime"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            repositories: o.get_field("repositories"),
            repository_group_id: o.get_field("repositoryGroupId"),
            update_time: o.get_field("updateTime"),
        }
    }
}
