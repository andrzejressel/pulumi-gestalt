/// Resource for managing an AWS AccessAnalyzer Archive Rule.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:accessanalyzer:ArchiveRule
///     properties:
///       analyzerName: example-analyzer
///       ruleName: example-rule
///       filters:
///         - criteria: condition.aws:UserId
///           eqs:
///             - userid
///         - criteria: error
///           exists: true
///         - criteria: isPublic
///           eqs:
///             - 'false'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AccessAnalyzer ArchiveRule using the `analyzer_name/rule_name`. For example:
///
/// ```sh
/// $ pulumi import aws:accessanalyzer/archiveRule:ArchiveRule example example-analyzer/example-rule
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod archive_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ArchiveRuleArgs {
        /// Analyzer name.
        #[builder(into)]
        pub analyzer_name: pulumi_gestalt_rust::Input<String>,
        /// Filter criteria for the archive rule. See Filter for more details.
        #[builder(into)]
        pub filters: pulumi_gestalt_rust::Input<
            Vec<super::super::types::accessanalyzer::ArchiveRuleFilter>,
        >,
        /// Rule name.
        #[builder(into)]
        pub rule_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct ArchiveRuleResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Analyzer name.
        pub analyzer_name: pulumi_gestalt_rust::Output<String>,
        /// Filter criteria for the archive rule. See Filter for more details.
        pub filters: pulumi_gestalt_rust::Output<
            Vec<super::super::types::accessanalyzer::ArchiveRuleFilter>,
        >,
        /// Rule name.
        pub rule_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ArchiveRuleArgs,
    ) -> ArchiveRuleResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ArchiveRuleArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ArchiveRuleResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ArchiveRuleArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ArchiveRuleResult {
        let analyzer_name_binding = args.analyzer_name.get_output(ctx);
        let filters_binding = args.filters.get_output(ctx);
        let rule_name_binding = args.rule_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:accessanalyzer/archiveRule:ArchiveRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "analyzerName".into(),
                    value: &analyzer_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleName".into(),
                    value: &rule_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ArchiveRuleResult {
            id: o.get_id(),
            urn: o.get_urn(),
            analyzer_name: o.get_field("analyzerName"),
            filters: o.get_field("filters"),
            rule_name: o.get_field("ruleName"),
        }
    }
}
