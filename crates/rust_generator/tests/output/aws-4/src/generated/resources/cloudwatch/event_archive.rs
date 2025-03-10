/// Provides an EventBridge event archive resource.
///
/// > **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let order = event_bus::create(
///         "order",
///         EventBusArgs::builder().name("orders").build_struct(),
///     );
///     let orderEventArchive = event_archive::create(
///         "orderEventArchive",
///         EventArchiveArgs::builder()
///             .event_source_arn("${order.arn}")
///             .name("order-archive")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Example all optional arguments
///
/// ```yaml
/// resources:
///   order:
///     type: aws:cloudwatch:EventBus
///     properties:
///       name: orders
///   orderEventArchive:
///     type: aws:cloudwatch:EventArchive
///     name: order
///     properties:
///       name: order-archive
///       description: Archived events from order service
///       eventSourceArn: ${order.arn}
///       retentionDays: 7
///       eventPattern:
///         fn::toJSON:
///           source:
///             - company.team.order
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an EventBridge archive using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/eventArchive:EventArchive imported_event_archive order-archive
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_archive {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventArchiveArgs {
        /// The description of the new event archive.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Instructs the new event archive to only capture events matched by this pattern. By default, it attempts to archive every event received in the `event_source_arn`.
        #[builder(into, default)]
        pub event_pattern: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Event bus source ARN from where these events should be archived.
        #[builder(into)]
        pub event_source_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the new event archive. The archive name cannot exceed 48 characters.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum number of days to retain events in the new event archive. By default, it archives indefinitely.
        #[builder(into, default)]
        pub retention_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct EventArchiveResult {
        /// The Amazon Resource Name (ARN) of the event archive.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The description of the new event archive.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Instructs the new event archive to only capture events matched by this pattern. By default, it attempts to archive every event received in the `event_source_arn`.
        pub event_pattern: pulumi_gestalt_rust::Output<Option<String>>,
        /// Event bus source ARN from where these events should be archived.
        pub event_source_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the new event archive. The archive name cannot exceed 48 characters.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The maximum number of days to retain events in the new event archive. By default, it archives indefinitely.
        pub retention_days: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventArchiveArgs,
    ) -> EventArchiveResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let event_pattern_binding = args.event_pattern.get_output(context);
        let event_source_arn_binding = args.event_source_arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let retention_days_binding = args.retention_days.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/eventArchive:EventArchive".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventPattern".into(),
                    value: &event_pattern_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventSourceArn".into(),
                    value: &event_source_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionDays".into(),
                    value: &retention_days_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventArchiveResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            event_pattern: o.get_field("eventPattern"),
            event_source_arn: o.get_field("eventSourceArn"),
            name: o.get_field("name"),
            retention_days: o.get_field("retentionDays"),
        }
    }
}
