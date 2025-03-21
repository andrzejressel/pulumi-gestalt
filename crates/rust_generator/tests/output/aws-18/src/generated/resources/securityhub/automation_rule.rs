/// Resource for managing an AWS Security Hub Automation Rule.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:securityhub:AutomationRule
///     properties:
///       description: Elevate finding severity to CRITICAL when specific resources such as an S3 bucket is at risk
///       ruleName: Elevate severity of findings that relate to important resources
///       ruleOrder: 1
///       actions:
///         - findingFieldsUpdate:
///             severity:
///               label: CRITICAL
///               product: '0.0'
///             note:
///               text: This is a critical resource. Please review ASAP.
///               updatedBy: sechub-automation
///             types:
///               - Software and Configuration Checks/Industry and Regulatory Standards
///             userDefinedFields:
///               key: value
///           type: FINDING_FIELDS_UPDATE
///       criteria:
///         resourceIds:
///           - comparison: EQUALS
///             value: arn:aws:s3:::examplebucket/*
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Security Hub automation rule using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:securityhub/automationRule:AutomationRule example arn:aws:securityhub:us-west-2:123456789012:automation-rule/473eddde-f5c4-4ae5-85c7-e922f271fffc
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod automation_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutomationRuleArgs {
        /// A block that specifies one or more actions to update finding fields if a finding matches the conditions specified in `Criteria`. Documented below.
        #[builder(into, default)]
        pub actions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::securityhub::AutomationRuleAction>>,
        >,
        /// A block that specifies a set of ASFF finding field attributes and corresponding expected values that Security Hub uses to filter findings. Documented below.
        #[builder(into, default)]
        pub criteria: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::securityhub::AutomationRuleCriteria>,
        >,
        /// The description of the rule.
        #[builder(into)]
        pub description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies whether a rule is the last to be applied with respect to a finding that matches the rule criteria. Defaults to `false`.
        #[builder(into, default)]
        pub is_terminal: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the rule.
        #[builder(into)]
        pub rule_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An integer ranging from 1 to 1000 that represents the order in which the rule action is applied to findings. Security Hub applies rules with lower values for this parameter first.
        #[builder(into)]
        pub rule_order: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Whether the rule is active after it is created.
        #[builder(into, default)]
        pub rule_status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AutomationRuleResult {
        /// A block that specifies one or more actions to update finding fields if a finding matches the conditions specified in `Criteria`. Documented below.
        pub actions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::securityhub::AutomationRuleAction>>,
        >,
        /// The ARN of the Security Hub automation rule.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A block that specifies a set of ASFF finding field attributes and corresponding expected values that Security Hub uses to filter findings. Documented below.
        pub criteria: pulumi_gestalt_rust::Output<
            Option<super::super::types::securityhub::AutomationRuleCriteria>,
        >,
        /// The description of the rule.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether a rule is the last to be applied with respect to a finding that matches the rule criteria. Defaults to `false`.
        pub is_terminal: pulumi_gestalt_rust::Output<bool>,
        /// The name of the rule.
        pub rule_name: pulumi_gestalt_rust::Output<String>,
        /// An integer ranging from 1 to 1000 that represents the order in which the rule action is applied to findings. Security Hub applies rules with lower values for this parameter first.
        pub rule_order: pulumi_gestalt_rust::Output<i32>,
        /// Whether the rule is active after it is created.
        pub rule_status: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AutomationRuleArgs,
    ) -> AutomationRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let actions_binding = args.actions.get_output(context);
        let criteria_binding = args.criteria.get_output(context);
        let description_binding = args.description.get_output(context);
        let is_terminal_binding = args.is_terminal.get_output(context);
        let rule_name_binding = args.rule_name.get_output(context);
        let rule_order_binding = args.rule_order.get_output(context);
        let rule_status_binding = args.rule_status.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:securityhub/automationRule:AutomationRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actions".into(),
                    value: &actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "criteria".into(),
                    value: &criteria_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isTerminal".into(),
                    value: &is_terminal_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleName".into(),
                    value: &rule_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleOrder".into(),
                    value: &rule_order_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleStatus".into(),
                    value: &rule_status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AutomationRuleResult {
            actions: o.get_field("actions"),
            arn: o.get_field("arn"),
            criteria: o.get_field("criteria"),
            description: o.get_field("description"),
            is_terminal: o.get_field("isTerminal"),
            rule_name: o.get_field("ruleName"),
            rule_order: o.get_field("ruleOrder"),
            rule_status: o.get_field("ruleStatus"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
