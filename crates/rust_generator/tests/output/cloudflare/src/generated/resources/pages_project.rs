/// Provides a resource which manages Cloudflare Pages projects.
///
/// > If you are using a `source` block configuration, you must first have a
///    connected GitHub or GitLab account connected to Cloudflare. See the
///    [Getting Started with Pages] documentation on how to link your accounts.
///
/// ## Import
///
/// !> It is not possible to import a pages project with secret environment variables. If you have a secret environment variable, you must remove it from your project before importing it.
///
/// ```sh
/// $ pulumi import cloudflare:index/pagesProject:PagesProject example <account_id>/<project_name>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod pages_project {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PagesProjectArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration for the project build process. Read more about the build configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/build-configuration).
        #[builder(into, default)]
        pub build_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::PagesProjectBuildConfig>,
        >,
        /// Configuration for deployments in a project.
        #[builder(into, default)]
        pub deployment_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::PagesProjectDeploymentConfigs>,
        >,
        /// Name of the project.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the branch that is used for the production environment.
        #[builder(into)]
        pub production_branch: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration for the project source. Read more about the source configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/branch-build-controls/).
        #[builder(into, default)]
        pub source: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::PagesProjectSource>,
        >,
    }
    #[allow(dead_code)]
    pub struct PagesProjectResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Configuration for the project build process. Read more about the build configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/build-configuration).
        pub build_config: pulumi_gestalt_rust::Output<
            Option<super::types::PagesProjectBuildConfig>,
        >,
        /// When the project was created.
        pub created_on: pulumi_gestalt_rust::Output<String>,
        /// Configuration for deployments in a project.
        pub deployment_configs: pulumi_gestalt_rust::Output<
            super::types::PagesProjectDeploymentConfigs,
        >,
        /// A list of associated custom domains for the project.
        pub domains: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Name of the project.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the branch that is used for the production environment.
        pub production_branch: pulumi_gestalt_rust::Output<String>,
        /// Configuration for the project source. Read more about the source configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/branch-build-controls/).
        pub source: pulumi_gestalt_rust::Output<
            Option<super::types::PagesProjectSource>,
        >,
        /// The Cloudflare subdomain associated with the project.
        pub subdomain: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PagesProjectArgs,
    ) -> PagesProjectResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let build_config_binding = args.build_config.get_output(context);
        let deployment_configs_binding = args.deployment_configs.get_output(context);
        let name_binding = args.name.get_output(context);
        let production_branch_binding = args.production_branch.get_output(context);
        let source_binding = args.source.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/pagesProject:PagesProject".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "buildConfig".into(),
                    value: &build_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deploymentConfigs".into(),
                    value: &deployment_configs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "productionBranch".into(),
                    value: &production_branch_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: &source_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PagesProjectResult {
            id: o.get_field("id"),
            account_id: o.get_field("accountId"),
            build_config: o.get_field("buildConfig"),
            created_on: o.get_field("createdOn"),
            deployment_configs: o.get_field("deploymentConfigs"),
            domains: o.get_field("domains"),
            name: o.get_field("name"),
            production_branch: o.get_field("productionBranch"),
            source: o.get_field("source"),
            subdomain: o.get_field("subdomain"),
        }
    }
}
