/// Resource for managing an AWS DevOps Guru Notification Channel.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = notification_channel::create(
///         "example",
///         NotificationChannelArgs::builder()
///             .sns(
///                 NotificationChannelSns::builder()
///                     .topicArn("${exampleAwsSnsTopic.arn}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Filters
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = notification_channel::create(
///         "example",
///         NotificationChannelArgs::builder()
///             .filters(
///                 NotificationChannelFilters::builder()
///                     .messageTypes(vec!["NEW_INSIGHT",])
///                     .severities(vec!["HIGH",])
///                     .build_struct(),
///             )
///             .sns(
///                 NotificationChannelSns::builder()
///                     .topicArn("${exampleAwsSnsTopic.arn}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DevOps Guru Notification Channel using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:devopsguru/notificationChannel:NotificationChannel example id-12345678
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod notification_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotificationChannelArgs {
        /// Filter configurations for the Amazon SNS notification topic. See the `filters` argument reference below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::Input<
            Option<super::super::types::devopsguru::NotificationChannelFilters>,
        >,
        /// SNS noficiation channel configurations. See the `sns` argument reference below.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub sns: pulumi_gestalt_rust::Input<
            Option<super::super::types::devopsguru::NotificationChannelSns>,
        >,
    }
    #[allow(dead_code)]
    pub struct NotificationChannelResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Filter configurations for the Amazon SNS notification topic. See the `filters` argument reference below.
        pub filters: pulumi_gestalt_rust::Output<
            Option<super::super::types::devopsguru::NotificationChannelFilters>,
        >,
        /// SNS noficiation channel configurations. See the `sns` argument reference below.
        ///
        /// The following arguments are optional:
        pub sns: pulumi_gestalt_rust::Output<
            Option<super::super::types::devopsguru::NotificationChannelSns>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NotificationChannelArgs,
    ) -> NotificationChannelResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NotificationChannelArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> NotificationChannelResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NotificationChannelArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> NotificationChannelResult {
        let filters_binding = args.filters.get_output(ctx);
        let sns_binding = args.sns.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:devopsguru/notificationChannel:NotificationChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sns".into(),
                    value: &sns_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        NotificationChannelResult {
            id: o.get_id(),
            urn: o.get_urn(),
            filters: o.get_field("filters"),
            sns: o.get_field("sns"),
        }
    }
}
