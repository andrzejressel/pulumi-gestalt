/// Manages a CodeBuild webhook, which is an endpoint accepted by the CodeBuild service to trigger builds from source code repositories. Depending on the source type of the CodeBuild project, the CodeBuild service may also automatically create and delete the actual repository webhook as well.
///
/// ## Example Usage
///
/// ### Bitbucket and GitHub
///
/// When working with [Bitbucket](https://bitbucket.org) and [GitHub](https://github.com) source CodeBuild webhooks, the CodeBuild service will automatically create (on `aws.codebuild.Webhook` resource creation) and delete (on `aws.codebuild.Webhook` resource deletion) the Bitbucket/GitHub repository webhook using its granted OAuth permissions. This behavior cannot be controlled by this provider.
///
/// > **Note:** The AWS account that this provider uses to create this resource *must* have authorized CodeBuild to access Bitbucket/GitHub's OAuth API in each applicable region. This is a manual step that must be done *before* creating webhooks with this resource. If OAuth is not configured, AWS will return an error similar to `ResourceNotFoundException: Could not find access token for server type github`. More information can be found in the CodeBuild User Guide for [Bitbucket](https://docs.aws.amazon.com/codebuild/latest/userguide/sample-bitbucket-pull-request.html) and [GitHub](https://docs.aws.amazon.com/codebuild/latest/userguide/sample-github-pull-request.html).
///
/// > **Note:** Further managing the automatically created Bitbucket/GitHub webhook with the `bitbucket_hook`/`github_repository_webhook` resource is only possible with importing that resource after creation of the `aws.codebuild.Webhook` resource. The CodeBuild API does not ever provide the `secret` attribute for the `aws.codebuild.Webhook` resource in this scenario.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = webhook::create(
///         "example",
///         WebhookArgs::builder()
///             .build_type("BUILD")
///             .filter_groups(
///                 vec![
///                     WebhookFilterGroup::builder()
///                     .filters(vec![WebhookFilterGroupFilter::builder().pattern("PUSH").
///                     type ("EVENT").build_struct(), WebhookFilterGroupFilter::builder()
///                     .pattern("master"). type ("BASE_REF").build_struct(),])
///                     .build_struct(),
///                 ],
///             )
///             .project_name("${exampleAwsCodebuildProject.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### GitHub Enterprise
///
/// When working with [GitHub Enterprise](https://enterprise.github.com/) source CodeBuild webhooks, the GHE repository webhook must be separately managed (e.g., manually or with the `github_repository_webhook` resource).
///
/// More information creating webhooks with GitHub Enterprise can be found in the [CodeBuild User Guide](https://docs.aws.amazon.com/codebuild/latest/userguide/sample-github-enterprise.html).
///
/// ```yaml
/// resources:
///   example:
///     type: aws:codebuild:Webhook
///     properties:
///       projectName: ${exampleAwsCodebuildProject.name}
///   exampleRepositoryWebhook:
///     type: github:RepositoryWebhook
///     name: example
///     properties:
///       active: true
///       events:
///         - push
///       name: example
///       repository: ${exampleGithubRepository.name}
///       configuration:
///         url: ${example.payloadUrl}
///         secret: ${example.secret}
///         contentType: json
///         insecureSsl: false
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeBuild Webhooks using the CodeBuild Project name. For example:
///
/// ```sh
/// $ pulumi import aws:codebuild/webhook:Webhook example MyProjectName
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod webhook {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebhookArgs {
        /// A regular expression used to determine which branches get built. Default is all branches are built. We recommend using `filter_group` over `branch_filter`.
        #[builder(into, default)]
        pub branch_filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of build this webhook will trigger. Valid values for this parameter are: `BUILD`, `BUILD_BATCH`.
        #[builder(into, default)]
        pub build_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Information about the webhook's trigger. Filter group blocks are documented below.
        #[builder(into, default)]
        pub filter_groups: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::codebuild::WebhookFilterGroup>>,
        >,
        /// The name of the build project.
        #[builder(into)]
        pub project_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Scope configuration for global or organization webhooks. Scope configuration blocks are documented below.
        #[builder(into, default)]
        pub scope_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codebuild::WebhookScopeConfiguration>,
        >,
    }
    #[allow(dead_code)]
    pub struct WebhookResult {
        /// A regular expression used to determine which branches get built. Default is all branches are built. We recommend using `filter_group` over `branch_filter`.
        pub branch_filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of build this webhook will trigger. Valid values for this parameter are: `BUILD`, `BUILD_BATCH`.
        pub build_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Information about the webhook's trigger. Filter group blocks are documented below.
        pub filter_groups: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::codebuild::WebhookFilterGroup>>,
        >,
        /// The CodeBuild endpoint where webhook events are sent.
        pub payload_url: pulumi_gestalt_rust::Output<String>,
        /// The name of the build project.
        pub project_name: pulumi_gestalt_rust::Output<String>,
        /// Scope configuration for global or organization webhooks. Scope configuration blocks are documented below.
        pub scope_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::codebuild::WebhookScopeConfiguration>,
        >,
        /// The secret token of the associated repository. Not returned by the CodeBuild API for all source types.
        pub secret: pulumi_gestalt_rust::Output<String>,
        /// The URL to the webhook.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WebhookArgs,
    ) -> WebhookResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let branch_filter_binding = args.branch_filter.get_output(context);
        let build_type_binding = args.build_type.get_output(context);
        let filter_groups_binding = args.filter_groups.get_output(context);
        let project_name_binding = args.project_name.get_output(context);
        let scope_configuration_binding = args.scope_configuration.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codebuild/webhook:Webhook".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "branchFilter".into(),
                    value: &branch_filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "buildType".into(),
                    value: &build_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filterGroups".into(),
                    value: &filter_groups_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "projectName".into(),
                    value: &project_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopeConfiguration".into(),
                    value: &scope_configuration_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WebhookResult {
            branch_filter: o.get_field("branchFilter"),
            build_type: o.get_field("buildType"),
            filter_groups: o.get_field("filterGroups"),
            payload_url: o.get_field("payloadUrl"),
            project_name: o.get_field("projectName"),
            scope_configuration: o.get_field("scopeConfiguration"),
            secret: o.get_field("secret"),
            url: o.get_field("url"),
        }
    }
}
