/// Manages a Line integration for a Bot Channel
///
/// > **Note** A bot can only have a single Line Channel associated with it.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleChannelsRegistration:
///     type: azure:bot:ChannelsRegistration
///     name: example
///     properties:
///       name: example-bcr
///       location: global
///       resourceGroupName: ${example.name}
///       sku: F0
///       microsoftAppId: ${current.clientId}
///   exampleChannelLine:
///     type: azure:bot:ChannelLine
///     name: example
///     properties:
///       botName: ${exampleChannelsRegistration.name}
///       location: ${exampleChannelsRegistration.location}
///       resourceGroupName: ${example.name}
///       lineChannels:
///         - accessToken: asdfdsdfTYUIOIoj1231hkjhk
///           secret: aagfdgfd123567
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// The Line Integration for a Bot Channel can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:bot/channelLine:ChannelLine example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.BotService/botServices/botService1/channels/LineChannel
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod channel_line {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelLineArgs {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub bot_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `line_channel` blocks as defined below.
        #[builder(into)]
        pub line_channels: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::bot::ChannelLineLineChannel>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group where the Line Channel should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ChannelLineResult {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        pub bot_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `line_channel` blocks as defined below.
        pub line_channels: pulumi_gestalt_rust::Output<
            Vec<super::super::types::bot::ChannelLineLineChannel>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group where the Line Channel should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ChannelLineArgs,
    ) -> ChannelLineResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bot_name_binding = args.bot_name.get_output(context);
        let line_channels_binding = args.line_channels.get_output(context);
        let location_binding = args.location.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:bot/channelLine:ChannelLine".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "botName".into(),
                    value: &bot_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lineChannels".into(),
                    value: &line_channels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ChannelLineResult {
            bot_name: o.get_field("botName"),
            line_channels: o.get_field("lineChannels"),
            location: o.get_field("location"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
