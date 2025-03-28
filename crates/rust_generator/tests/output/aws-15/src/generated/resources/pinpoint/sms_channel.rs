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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sms_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SmsChannelArgs {
        /// ID of the application.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the channel is enabled or disabled. By default, it is set to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Identifier of the sender for your messages.
        #[builder(into, default)]
        pub sender_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Short Code registered with the phone provider.
        #[builder(into, default)]
        pub short_code: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SmsChannelResult {
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SmsChannelArgs,
    ) -> SmsChannelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_id_binding = args.application_id.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let sender_id_binding = args.sender_id.get_output(context);
        let short_code_binding = args.short_code.get_output(context);
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
        };
        let o = context.register_resource(request);
        SmsChannelResult {
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
