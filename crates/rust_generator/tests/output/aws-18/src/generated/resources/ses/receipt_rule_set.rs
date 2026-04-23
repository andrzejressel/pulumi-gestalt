/// Provides an SES receipt rule set resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let main = receipt_rule_set::create(
///         "main",
///         ReceiptRuleSetArgs::builder().rule_set_name("primary-rules").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SES receipt rule sets using the rule set name. For example:
///
/// ```sh
/// $ pulumi import aws:ses/receiptRuleSet:ReceiptRuleSet my_rule_set my_rule_set_name
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod receipt_rule_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReceiptRuleSetArgs {
        /// Name of the rule set.
        #[builder(into)]
        pub rule_set_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct ReceiptRuleSetResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// SES receipt rule set ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the rule set.
        pub rule_set_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReceiptRuleSetArgs,
    ) -> ReceiptRuleSetResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReceiptRuleSetArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ReceiptRuleSetResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReceiptRuleSetArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ReceiptRuleSetResult {
        let rule_set_name_binding = args.rule_set_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ses/receiptRuleSet:ReceiptRuleSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleSetName".into(),
                    value: &rule_set_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ReceiptRuleSetResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            rule_set_name: o.get_field("ruleSetName"),
        }
    }
}
