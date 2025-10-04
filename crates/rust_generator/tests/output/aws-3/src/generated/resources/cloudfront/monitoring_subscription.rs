/// Provides a CloudFront real-time log configuration resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = monitoring_subscription::create(
///         "example",
///         MonitoringSubscriptionArgs::builder()
///             .distribution_id("${exampleAwsCloudfrontDistribution.id}")
///             .monitoring_subscription(
///                 MonitoringSubscriptionMonitoringSubscription::builder()
///                     .realtimeMetricsSubscriptionConfig(
///                         MonitoringSubscriptionMonitoringSubscriptionRealtimeMetricsSubscriptionConfig::builder()
///                             .realtimeMetricsSubscriptionStatus("Enabled")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudFront monitoring subscription using the id. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/monitoringSubscription:MonitoringSubscription example E3QYSUHO4VYRGB
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod monitoring_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitoringSubscriptionArgs {
        /// The ID of the distribution that you are enabling metrics for.
        #[builder(into)]
        pub distribution_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A monitoring subscription. This structure contains information about whether additional CloudWatch metrics are enabled for a given CloudFront distribution.
        #[builder(into)]
        pub monitoring_subscription: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cloudfront::MonitoringSubscriptionMonitoringSubscription,
        >,
    }
    #[allow(dead_code)]
    pub struct MonitoringSubscriptionResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the distribution that you are enabling metrics for.
        pub distribution_id: pulumi_gestalt_rust::Output<String>,
        /// A monitoring subscription. This structure contains information about whether additional CloudWatch metrics are enabled for a given CloudFront distribution.
        pub monitoring_subscription: pulumi_gestalt_rust::Output<
            super::super::types::cloudfront::MonitoringSubscriptionMonitoringSubscription,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MonitoringSubscriptionArgs,
    ) -> MonitoringSubscriptionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let distribution_id_binding = args.distribution_id.get_output(context);
        let monitoring_subscription_binding = args
            .monitoring_subscription
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudfront/monitoringSubscription:MonitoringSubscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "distributionId".into(),
                    value: &distribution_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monitoringSubscription".into(),
                    value: &monitoring_subscription_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MonitoringSubscriptionResult {
            id: o.get_field("id"),
            distribution_id: o.get_field("distributionId"),
            monitoring_subscription: o.get_field("monitoringSubscription"),
        }
    }
}
