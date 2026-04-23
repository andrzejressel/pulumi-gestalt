/// Use the `aws.pinpoint.SmsChannel` resource to manage Pinpoint SMS Channels.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let app = app::create("app", AppArgs::builder().build_struct());
///     let sms = sms_channel::create(
///         "sms",
///         SmsChannelArgs::builder().application_id("${app.applicationId}").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the Pinpoint SMS Channel using the `application_id`. For example:
///
/// ```sh
/// $ pulumi import aws:pinpoint/smsChannel:SmsChannel sms application-id
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod sms_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SmsChannelArgs {
        /// ID of the application.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::Input<String>,
        /// Whether the channel is enabled or disabled. By default, it is set to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::Input<Option<bool>>,
        /// Identifier of the sender for your messages.
        #[builder(into, default)]
        pub sender_id: pulumi_gestalt_rust::Input<Option<String>>,
        /// Short Code registered with the phone provider.
        #[builder(into, default)]
        pub short_code: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SmsChannelResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// ID of the application.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// Whether the channel is enabled or disabled. By default, it is set to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Maximum number of promotional messages that can be sent per second.
        pub promotional_messages_per_second: pulumi_gestalt_rust::Output<i32>,
        /// Identifier of the sender for your messages.
        pub sender_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Short Code registered with the phone provider.
        pub short_code: pulumi_gestalt_rust::Output<Option<String>>,
        /// Maximum number of transactional messages per second that can be sent.
        pub transactional_messages_per_second: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SmsChannelArgs,
    ) -> SmsChannelResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SmsChannelArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> SmsChannelResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SmsChannelArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> SmsChannelResult {
        let application_id_binding = args.application_id.get_output(ctx);
        let enabled_binding = args.enabled.get_output(ctx);
        let sender_id_binding = args.sender_id.get_output(ctx);
        let short_code_binding = args.short_code.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:pinpoint/smsChannel:SmsChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "senderId".into(),
                    value: &sender_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shortCode".into(),
                    value: &short_code_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        SmsChannelResult {
            id: o.get_id(),
            urn: o.get_urn(),
            application_id: o.get_field("applicationId"),
            enabled: o.get_field("enabled"),
            promotional_messages_per_second: o.get_field("promotionalMessagesPerSecond"),
            sender_id: o.get_field("senderId"),
            short_code: o.get_field("shortCode"),
            transactional_messages_per_second: o
                .get_field("transactionalMessagesPerSecond"),
        }
    }
}
