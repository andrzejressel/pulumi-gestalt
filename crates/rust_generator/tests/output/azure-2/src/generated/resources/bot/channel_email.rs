/// Manages a Email integration for a Bot Channel
///
/// > **Note** A bot can only have a single Email Channel associated with it.
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
///       name: example
///       location: global
///       resourceGroupName: ${example.name}
///       sku: F0
///       microsoftAppId: ${current.clientId}
///   exampleChannelEmail:
///     type: azure:bot:ChannelEmail
///     name: example
///     properties:
///       botName: ${exampleChannelsRegistration.name}
///       location: ${exampleChannelsRegistration.location}
///       resourceGroupName: ${example.name}
///       emailAddress: example.com
///       emailPassword: '123456'
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// The Email Integration for a Bot Channel can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:bot/channelEmail:ChannelEmail example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.BotService/botServices/example/channels/EmailChannel
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod channel_email {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelEmailArgs {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub bot_name: pulumi_gestalt_rust::Input<String>,
        /// The email address that the Bot will authenticate with.
        #[builder(into)]
        pub email_address: pulumi_gestalt_rust::Input<String>,
        /// The email password that the Bot will authenticate with.
        #[builder(into, default)]
        pub email_password: pulumi_gestalt_rust::Input<Option<String>>,
        /// The supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::Input<Option<String>>,
        /// The magic code used to set up OAUTH authentication.
        #[builder(into, default)]
        pub magic_code: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the resource group in which to create the Bot Channel. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct ChannelEmailResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        pub bot_name: pulumi_gestalt_rust::Output<String>,
        /// The email address that the Bot will authenticate with.
        pub email_address: pulumi_gestalt_rust::Output<String>,
        /// The email password that the Bot will authenticate with.
        pub email_password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The magic code used to set up OAUTH authentication.
        pub magic_code: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Bot Channel. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ChannelEmailArgs,
    ) -> ChannelEmailResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ChannelEmailArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ChannelEmailResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ChannelEmailArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ChannelEmailResult {
        let bot_name_binding = args.bot_name.get_output(ctx);
        let email_address_binding = args.email_address.get_output(ctx);
        let email_password_binding = args.email_password.get_output(ctx);
        let location_binding = args.location.get_output(ctx);
        let magic_code_binding = args.magic_code.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:bot/channelEmail:ChannelEmail".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "botName".into(),
                    value: &bot_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailAddress".into(),
                    value: &email_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailPassword".into(),
                    value: &email_password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "magicCode".into(),
                    value: &magic_code_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ChannelEmailResult {
            id: o.get_id(),
            urn: o.get_urn(),
            bot_name: o.get_field("botName"),
            email_address: o.get_field("emailAddress"),
            email_password: o.get_field("emailPassword"),
            location: o.get_field("location"),
            magic_code: o.get_field("magicCode"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
