/// BranchRule is the protection rule to enforce pre-defined rules on designated branches within a repository.
///
///
/// To get more information about BranchRule, see:
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/secure-source-manager/docs/overview)
///
/// ## Example Usage
///
/// ### Secure Source Manager Branch Rule Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = branch_rule::create(
///         "basic",
///         BranchRuleArgs::builder()
///             .branch_rule_id("my-basic-branchrule")
///             .include_pattern("main")
///             .location("${repository.location}")
///             .repository_id("${repository.repositoryId}")
///             .build_struct(),
///     );
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .instance_id("my-basic-instance")
///             .location("us-central1")
///             .build_struct(),
///     );
///     let repository = repository::create(
///         "repository",
///         RepositoryArgs::builder()
///             .instance("${instance.name}")
///             .location("${instance.location}")
///             .repository_id("my-basic-repository")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Secure Source Manager Branch Rule With Fields
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = branch_rule::create(
///         "default",
///         BranchRuleArgs::builder()
///             .allow_stale_reviews(false)
///             .branch_rule_id("my-initial-branchrule")
///             .disabled(false)
///             .include_pattern("test")
///             .location("${repository.location}")
///             .minimum_approvals_count(2)
///             .minimum_reviews_count(2)
///             .repository_id("${repository.repositoryId}")
///             .require_comments_resolved(true)
///             .require_linear_history(true)
///             .require_pull_request(true)
///             .build_struct(),
///     );
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .instance_id("my-initial-instance")
///             .location("us-central1")
///             .build_struct(),
///     );
///     let repository = repository::create(
///         "repository",
///         RepositoryArgs::builder()
///             .instance("${instance.name}")
///             .location("${instance.location}")
///             .repository_id("my-initial-repository")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// BranchRule can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/repositories/{{repository_id}}/branchRules/{{branch_rule_id}}`
///
/// * `{{project}}/{{location}}/{{repository_id}}/{{branch_rule_id}}`
///
/// * `{{location}}/{{repository_id}}/{{branch_rule_id}}`
///
/// * `{{branch_rule_id}}`
///
/// When using the `pulumi import` command, BranchRule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/branchRule:BranchRule default projects/{{project}}/locations/{{location}}/repositories/{{repository_id}}/branchRules/{{branch_rule_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/branchRule:BranchRule default {{project}}/{{location}}/{{repository_id}}/{{branch_rule_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/branchRule:BranchRule default {{location}}/{{repository_id}}/{{branch_rule_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/branchRule:BranchRule default {{branch_rule_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod branch_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BranchRuleArgs {
        /// Determines if allow stale reviews or approvals before merging to the branch.
        #[builder(into, default)]
        pub allow_stale_reviews: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID for the BranchRule.
        #[builder(into)]
        pub branch_rule_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Determines if the branch rule is disabled or not.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The BranchRule matches branches based on the specified regular expression. Use .* to match all branches.
        #[builder(into)]
        pub include_pattern: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location for the Repository.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The minimum number of approvals required for the branch rule to be matched.
        #[builder(into, default)]
        pub minimum_approvals_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The minimum number of reviews required for the branch rule to be matched.
        #[builder(into, default)]
        pub minimum_reviews_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID for the Repository.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub repository_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Determines if require comments resolved before merging to the branch.
        #[builder(into, default)]
        pub require_comments_resolved: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Determines if require linear history before merging to the branch.
        #[builder(into, default)]
        pub require_linear_history: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Determines if the branch rule requires a pull request or not.
        #[builder(into, default)]
        pub require_pull_request: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct BranchRuleResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Determines if allow stale reviews or approvals before merging to the branch.
        pub allow_stale_reviews: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID for the BranchRule.
        pub branch_rule_id: pulumi_gestalt_rust::Output<String>,
        /// Time the BranchRule was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Determines if the branch rule is disabled or not.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The BranchRule matches branches based on the specified regular expression. Use .* to match all branches.
        pub include_pattern: pulumi_gestalt_rust::Output<String>,
        /// The location for the Repository.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The minimum number of approvals required for the branch rule to be matched.
        pub minimum_approvals_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The minimum number of reviews required for the branch rule to be matched.
        pub minimum_reviews_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The resource name for the BranchRule.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The ID for the Repository.
        ///
        ///
        /// - - -
        pub repository_id: pulumi_gestalt_rust::Output<String>,
        /// Determines if require comments resolved before merging to the branch.
        pub require_comments_resolved: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Determines if require linear history before merging to the branch.
        pub require_linear_history: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Determines if the branch rule requires a pull request or not.
        pub require_pull_request: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Unique identifier of the BranchRule.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Time the BranchRule was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BranchRuleArgs,
    ) -> BranchRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allow_stale_reviews_binding = args.allow_stale_reviews.get_output(context);
        let branch_rule_id_binding = args.branch_rule_id.get_output(context);
        let disabled_binding = args.disabled.get_output(context);
        let include_pattern_binding = args.include_pattern.get_output(context);
        let location_binding = args.location.get_output(context);
        let minimum_approvals_count_binding = args
            .minimum_approvals_count
            .get_output(context);
        let minimum_reviews_count_binding = args
            .minimum_reviews_count
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let repository_id_binding = args.repository_id.get_output(context);
        let require_comments_resolved_binding = args
            .require_comments_resolved
            .get_output(context);
        let require_linear_history_binding = args
            .require_linear_history
            .get_output(context);
        let require_pull_request_binding = args.require_pull_request.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:securesourcemanager/branchRule:BranchRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowStaleReviews".into(),
                    value: &allow_stale_reviews_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "branchRuleId".into(),
                    value: &branch_rule_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includePattern".into(),
                    value: &include_pattern_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minimumApprovalsCount".into(),
                    value: &minimum_approvals_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minimumReviewsCount".into(),
                    value: &minimum_reviews_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositoryId".into(),
                    value: &repository_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requireCommentsResolved".into(),
                    value: &require_comments_resolved_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requireLinearHistory".into(),
                    value: &require_linear_history_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requirePullRequest".into(),
                    value: &require_pull_request_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BranchRuleResult {
            id: o.get_field("id"),
            allow_stale_reviews: o.get_field("allowStaleReviews"),
            branch_rule_id: o.get_field("branchRuleId"),
            create_time: o.get_field("createTime"),
            disabled: o.get_field("disabled"),
            include_pattern: o.get_field("includePattern"),
            location: o.get_field("location"),
            minimum_approvals_count: o.get_field("minimumApprovalsCount"),
            minimum_reviews_count: o.get_field("minimumReviewsCount"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            repository_id: o.get_field("repositoryId"),
            require_comments_resolved: o.get_field("requireCommentsResolved"),
            require_linear_history: o.get_field("requireLinearHistory"),
            require_pull_request: o.get_field("requirePullRequest"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
