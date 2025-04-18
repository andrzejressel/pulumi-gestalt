/// Provides an SES receipt rule resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let store = receipt_rule::create(
///         "store",
///         ReceiptRuleArgs::builder()
///             .add_header_actions(
///                 vec![
///                     ReceiptRuleAddHeaderAction::builder().headerName("Custom-Header")
///                     .headerValue("Added by SES").position(1).build_struct(),
///                 ],
///             )
///             .enabled(true)
///             .name("store")
///             .recipients(vec!["karen@example.com",])
///             .rule_set_name("default-rule-set")
///             .s_3_actions(
///                 vec![
///                     ReceiptRuleS3Action::builder().bucketName("emails").position(2)
///                     .build_struct(),
///                 ],
///             )
///             .scan_enabled(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SES receipt rules using the ruleset name and rule name separated by `:`. For example:
///
/// ```sh
/// $ pulumi import aws:ses/receiptRule:ReceiptRule my_rule my_rule_set:my_rule
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod receipt_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReceiptRuleArgs {
        /// A list of Add Header Action blocks. Documented below.
        #[builder(into, default)]
        pub add_header_actions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ses::ReceiptRuleAddHeaderAction>>,
        >,
        /// The name of the rule to place this rule after
        #[builder(into, default)]
        pub after: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of Bounce Action blocks. Documented below.
        #[builder(into, default)]
        pub bounce_actions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ses::ReceiptRuleBounceAction>>,
        >,
        /// If true, the rule will be enabled
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A list of Lambda Action blocks. Documented below.
        #[builder(into, default)]
        pub lambda_actions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ses::ReceiptRuleLambdaAction>>,
        >,
        /// The name of the rule
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of email addresses
        #[builder(into, default)]
        pub recipients: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the rule set
        #[builder(into)]
        pub rule_set_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of S3 Action blocks. Documented below.
        #[builder(into, default)]
        pub s3_actions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ses::ReceiptRuleS3Action>>,
        >,
        /// If true, incoming emails will be scanned for spam and viruses
        #[builder(into, default)]
        pub scan_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A list of SNS Action blocks. Documented below.
        #[builder(into, default)]
        pub sns_actions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ses::ReceiptRuleSnsAction>>,
        >,
        /// A list of Stop Action blocks. Documented below.
        #[builder(into, default)]
        pub stop_actions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ses::ReceiptRuleStopAction>>,
        >,
        /// `Require` or `Optional`
        #[builder(into, default)]
        pub tls_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of WorkMail Action blocks. Documented below.
        #[builder(into, default)]
        pub workmail_actions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ses::ReceiptRuleWorkmailAction>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ReceiptRuleResult {
        /// A list of Add Header Action blocks. Documented below.
        pub add_header_actions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleAddHeaderAction>>,
        >,
        /// The name of the rule to place this rule after
        pub after: pulumi_gestalt_rust::Output<Option<String>>,
        /// The SES receipt rule ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A list of Bounce Action blocks. Documented below.
        pub bounce_actions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleBounceAction>>,
        >,
        /// If true, the rule will be enabled
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list of Lambda Action blocks. Documented below.
        pub lambda_actions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleLambdaAction>>,
        >,
        /// The name of the rule
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of email addresses
        pub recipients: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The name of the rule set
        pub rule_set_name: pulumi_gestalt_rust::Output<String>,
        /// A list of S3 Action blocks. Documented below.
        pub s3_actions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleS3Action>>,
        >,
        /// If true, incoming emails will be scanned for spam and viruses
        pub scan_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list of SNS Action blocks. Documented below.
        pub sns_actions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleSnsAction>>,
        >,
        /// A list of Stop Action blocks. Documented below.
        pub stop_actions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleStopAction>>,
        >,
        /// `Require` or `Optional`
        pub tls_policy: pulumi_gestalt_rust::Output<String>,
        /// A list of WorkMail Action blocks. Documented below.
        pub workmail_actions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ses::ReceiptRuleWorkmailAction>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReceiptRuleArgs,
    ) -> ReceiptRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let add_header_actions_binding = args.add_header_actions.get_output(context);
        let after_binding = args.after.get_output(context);
        let bounce_actions_binding = args.bounce_actions.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let lambda_actions_binding = args.lambda_actions.get_output(context);
        let name_binding = args.name.get_output(context);
        let recipients_binding = args.recipients.get_output(context);
        let rule_set_name_binding = args.rule_set_name.get_output(context);
        let s3_actions_binding = args.s3_actions.get_output(context);
        let scan_enabled_binding = args.scan_enabled.get_output(context);
        let sns_actions_binding = args.sns_actions.get_output(context);
        let stop_actions_binding = args.stop_actions.get_output(context);
        let tls_policy_binding = args.tls_policy.get_output(context);
        let workmail_actions_binding = args.workmail_actions.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ses/receiptRule:ReceiptRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addHeaderActions".into(),
                    value: &add_header_actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "after".into(),
                    value: &after_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bounceActions".into(),
                    value: &bounce_actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lambdaActions".into(),
                    value: &lambda_actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recipients".into(),
                    value: &recipients_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleSetName".into(),
                    value: &rule_set_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3Actions".into(),
                    value: &s3_actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scanEnabled".into(),
                    value: &scan_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snsActions".into(),
                    value: &sns_actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stopActions".into(),
                    value: &stop_actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tlsPolicy".into(),
                    value: &tls_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workmailActions".into(),
                    value: &workmail_actions_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReceiptRuleResult {
            add_header_actions: o.get_field("addHeaderActions"),
            after: o.get_field("after"),
            arn: o.get_field("arn"),
            bounce_actions: o.get_field("bounceActions"),
            enabled: o.get_field("enabled"),
            lambda_actions: o.get_field("lambdaActions"),
            name: o.get_field("name"),
            recipients: o.get_field("recipients"),
            rule_set_name: o.get_field("ruleSetName"),
            s3_actions: o.get_field("s3Actions"),
            scan_enabled: o.get_field("scanEnabled"),
            sns_actions: o.get_field("snsActions"),
            stop_actions: o.get_field("stopActions"),
            tls_policy: o.get_field("tlsPolicy"),
            workmail_actions: o.get_field("workmailActions"),
        }
    }
}
