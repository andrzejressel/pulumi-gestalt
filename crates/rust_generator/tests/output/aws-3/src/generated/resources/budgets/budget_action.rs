/// Provides a budget action resource. Budget actions are cost savings controls that run either automatically on your behalf or by using a workflow approval process.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleBudgetAction:
///     type: aws:budgets:BudgetAction
///     name: example
///     properties:
///       budgetName: ${exampleBudget.name}
///       actionType: APPLY_IAM_POLICY
///       approvalModel: AUTOMATIC
///       notificationType: ACTUAL
///       executionRoleArn: ${exampleRole.arn}
///       actionThreshold:
///         actionThresholdType: ABSOLUTE_VALUE
///         actionThresholdValue: 100
///       definition:
///         iamActionDefinition:
///           policyArn: ${examplePolicy.arn}
///           roles:
///             - ${exampleRole.name}
///       subscribers:
///         - address: example@example.example
///           subscriptionType: EMAIL
///       tags:
///         Tag1: Value1
///         Tag2: Value2
///   examplePolicy:
///     type: aws:iam:Policy
///     name: example
///     properties:
///       name: example
///       description: My example policy
///       policy: ${example.json}
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example
///       assumeRolePolicy: ${assumeRole.json}
///   exampleBudget:
///     type: aws:budgets:Budget
///     name: example
///     properties:
///       name: example
///       budgetType: USAGE
///       limitAmount: '10.0'
///       limitUnit: dollars
///       timePeriodStart: 2006-01-02_15:04
///       timeUnit: MONTHLY
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - ec2:Describe*
///             resources:
///               - '*'
///   current:
///     fn::invoke:
///       function: aws:getPartition
///       arguments: {}
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - budgets.${current.dnsSuffix}
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import budget actions using `AccountID:ActionID:BudgetName`. For example:
///
/// ```sh
/// $ pulumi import aws:budgets/budgetAction:BudgetAction myBudget 123456789012:some-id:myBudget
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod budget_action {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BudgetActionArgs {
        /// The ID of the target account for budget. Will use current user's account_id by default if omitted.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The trigger threshold of the action. See Action Threshold.
        #[builder(into)]
        pub action_threshold: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::budgets::BudgetActionActionThreshold,
        >,
        /// The type of action. This defines the type of tasks that can be carried out by this action. This field also determines the format for definition. Valid values are `APPLY_IAM_POLICY`, `APPLY_SCP_POLICY`, and `RUN_SSM_DOCUMENTS`.
        #[builder(into)]
        pub action_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// This specifies if the action needs manual or automatic approval. Valid values are `AUTOMATIC` and `MANUAL`.
        #[builder(into)]
        pub approval_model: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of a budget.
        #[builder(into)]
        pub budget_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies all of the type-specific parameters. See Definition.
        #[builder(into)]
        pub definition: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::budgets::BudgetActionDefinition,
        >,
        /// The role passed for action execution and reversion. Roles and actions must be in the same account.
        #[builder(into)]
        pub execution_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of a notification. Valid values are `ACTUAL` or `FORECASTED`.
        #[builder(into)]
        pub notification_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of subscribers. See Subscriber.
        #[builder(into)]
        pub subscribers: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::budgets::BudgetActionSubscriber>,
        >,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct BudgetActionResult {
        /// The ID of the target account for budget. Will use current user's account_id by default if omitted.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The id of the budget action.
        pub action_id: pulumi_gestalt_rust::Output<String>,
        /// The trigger threshold of the action. See Action Threshold.
        pub action_threshold: pulumi_gestalt_rust::Output<
            super::super::types::budgets::BudgetActionActionThreshold,
        >,
        /// The type of action. This defines the type of tasks that can be carried out by this action. This field also determines the format for definition. Valid values are `APPLY_IAM_POLICY`, `APPLY_SCP_POLICY`, and `RUN_SSM_DOCUMENTS`.
        pub action_type: pulumi_gestalt_rust::Output<String>,
        /// This specifies if the action needs manual or automatic approval. Valid values are `AUTOMATIC` and `MANUAL`.
        pub approval_model: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the budget action.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of a budget.
        pub budget_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies all of the type-specific parameters. See Definition.
        pub definition: pulumi_gestalt_rust::Output<
            super::super::types::budgets::BudgetActionDefinition,
        >,
        /// The role passed for action execution and reversion. Roles and actions must be in the same account.
        pub execution_role_arn: pulumi_gestalt_rust::Output<String>,
        /// The type of a notification. Valid values are `ACTUAL` or `FORECASTED`.
        pub notification_type: pulumi_gestalt_rust::Output<String>,
        /// The status of the budget action.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A list of subscribers. See Subscriber.
        pub subscribers: pulumi_gestalt_rust::Output<
            Vec<super::super::types::budgets::BudgetActionSubscriber>,
        >,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: BudgetActionArgs,
    ) -> BudgetActionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let action_threshold_binding = args.action_threshold.get_output(context);
        let action_type_binding = args.action_type.get_output(context);
        let approval_model_binding = args.approval_model.get_output(context);
        let budget_name_binding = args.budget_name.get_output(context);
        let definition_binding = args.definition.get_output(context);
        let execution_role_arn_binding = args.execution_role_arn.get_output(context);
        let notification_type_binding = args.notification_type.get_output(context);
        let subscribers_binding = args.subscribers.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:budgets/budgetAction:BudgetAction".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actionThreshold".into(),
                    value: &action_threshold_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actionType".into(),
                    value: &action_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "approvalModel".into(),
                    value: &approval_model_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "budgetName".into(),
                    value: &budget_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "definition".into(),
                    value: &definition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "executionRoleArn".into(),
                    value: &execution_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationType".into(),
                    value: &notification_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscribers".into(),
                    value: &subscribers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BudgetActionResult {
            account_id: o.get_field("accountId"),
            action_id: o.get_field("actionId"),
            action_threshold: o.get_field("actionThreshold"),
            action_type: o.get_field("actionType"),
            approval_model: o.get_field("approvalModel"),
            arn: o.get_field("arn"),
            budget_name: o.get_field("budgetName"),
            definition: o.get_field("definition"),
            execution_role_arn: o.get_field("executionRoleArn"),
            notification_type: o.get_field("notificationType"),
            status: o.get_field("status"),
            subscribers: o.get_field("subscribers"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
