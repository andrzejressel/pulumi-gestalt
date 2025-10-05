/// Creates a new Amazon Redshift Serverless Usage Limit.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workgroup::create(
///         "example",
///         WorkgroupArgs::builder()
///             .namespace_name("${exampleAwsRedshiftserverlessNamespace.namespaceName}")
///             .workgroup_name("example")
///             .build_struct(),
///     );
///     let exampleUsageLimit = usage_limit::create(
///         "exampleUsageLimit",
///         UsageLimitArgs::builder()
///             .amount(60)
///             .resource_arn("${example.arn}")
///             .usage_type("serverless-compute")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Serverless Usage Limits using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:redshiftserverless/usageLimit:UsageLimit example example-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod usage_limit {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UsageLimitArgs {
        /// The limit amount. If time-based, this amount is in Redshift Processing Units (RPU) consumed per hour. If data-based, this amount is in terabytes (TB) of data transferred between Regions in cross-account sharing. The value must be a positive number.
        #[builder(into)]
        pub amount: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The action that Amazon Redshift Serverless takes when the limit is reached. Valid values are `log`, `emit-metric`, and `deactivate`. The default is `log`.
        #[builder(into, default)]
        pub breach_action: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The time period that the amount applies to. A weekly period begins on Sunday. Valid values are `daily`, `weekly`, and `monthly`. The default is `monthly`.
        #[builder(into, default)]
        pub period: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of the Amazon Redshift Serverless resource to create the usage limit for.
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of Amazon Redshift Serverless usage to create a usage limit for. Valid values are `serverless-compute` or `cross-region-datasharing`.
        #[builder(into)]
        pub usage_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UsageLimitResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The limit amount. If time-based, this amount is in Redshift Processing Units (RPU) consumed per hour. If data-based, this amount is in terabytes (TB) of data transferred between Regions in cross-account sharing. The value must be a positive number.
        pub amount: pulumi_gestalt_rust::Output<i32>,
        /// Amazon Resource Name (ARN) of the Redshift Serverless Usage Limit.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The action that Amazon Redshift Serverless takes when the limit is reached. Valid values are `log`, `emit-metric`, and `deactivate`. The default is `log`.
        pub breach_action: pulumi_gestalt_rust::Output<Option<String>>,
        /// The time period that the amount applies to. A weekly period begins on Sunday. Valid values are `daily`, `weekly`, and `monthly`. The default is `monthly`.
        pub period: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the Amazon Redshift Serverless resource to create the usage limit for.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
        /// The type of Amazon Redshift Serverless usage to create a usage limit for. Valid values are `serverless-compute` or `cross-region-datasharing`.
        pub usage_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UsageLimitArgs,
    ) -> UsageLimitResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let amount_binding = args.amount.get_output(context);
        let breach_action_binding = args.breach_action.get_output(context);
        let period_binding = args.period.get_output(context);
        let resource_arn_binding = args.resource_arn.get_output(context);
        let usage_type_binding = args.usage_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshiftserverless/usageLimit:UsageLimit".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "amount".into(),
                    value: &amount_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "breachAction".into(),
                    value: &breach_action_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "period".into(),
                    value: &period_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "usageType".into(),
                    value: &usage_type_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        UsageLimitResult {
            id: o.get_field("id"),
            amount: o.get_field("amount"),
            arn: o.get_field("arn"),
            breach_action: o.get_field("breachAction"),
            period: o.get_field("period"),
            resource_arn: o.get_field("resourceArn"),
            usage_type: o.get_field("usageType"),
        }
    }
}
