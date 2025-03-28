/// Manages the Shared Private Link Resource for a Web Pubsub service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: terraform-webpubsub
///       location: east us
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: examplekeyvault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: standard
///       softDeleteRetentionDays: 7
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           certificatePermissions:
///             - managecontacts
///           keyPermissions:
///             - create
///           secretPermissions:
///             - set
///   exampleService:
///     type: azure:webpubsub:Service
///     name: example
///     properties:
///       name: tfex-webpubsub
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard_S1
///       capacity: 1
///   exampleSharedPrivateLinkResource:
///     type: azure:webpubsub:SharedPrivateLinkResource
///     name: example
///     properties:
///       name: tfex-webpubsub-splr
///       webPubsubId: ${exampleService.id}
///       subresourceName: vault
///       targetResourceId: ${exampleKeyVault.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Web Pubsub Shared Private Link Resource can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:webpubsub/sharedPrivateLinkResource:SharedPrivateLinkResource example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.SignalRService/webPubSub/webPubSub1/sharedPrivateLinkResources/resource1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod shared_private_link_resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedPrivateLinkResourceArgs {
        /// Specify the name of the Web Pubsub Shared Private Link Resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specify the request message for requesting approval of the Shared Private Link Enabled Remote Resource.
        #[builder(into, default)]
        pub request_message: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specify the sub resource name which the Web Pubsub Private Endpoint is able to connect to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The available sub resource can be retrieved by using `azure.webpubsub.getPrivateLinkResource` data source.
        #[builder(into)]
        pub subresource_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specify the ID of the Shared Private Link Enabled Remote Resource which this Web Pubsub Private Endpoint should be connected to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The sub resource name should match with the type of the target resource id that's being specified.
        #[builder(into)]
        pub target_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specify the id of the Web Pubsub. Changing this forces a new resource to be created.
        #[builder(into)]
        pub web_pubsub_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SharedPrivateLinkResourceResult {
        /// Specify the name of the Web Pubsub Shared Private Link Resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specify the request message for requesting approval of the Shared Private Link Enabled Remote Resource.
        pub request_message: pulumi_gestalt_rust::Output<Option<String>>,
        /// The status of a private endpoint connection. Possible values are Pending, Approved, Rejected or Disconnected.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Specify the sub resource name which the Web Pubsub Private Endpoint is able to connect to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The available sub resource can be retrieved by using `azure.webpubsub.getPrivateLinkResource` data source.
        pub subresource_name: pulumi_gestalt_rust::Output<String>,
        /// Specify the ID of the Shared Private Link Enabled Remote Resource which this Web Pubsub Private Endpoint should be connected to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The sub resource name should match with the type of the target resource id that's being specified.
        pub target_resource_id: pulumi_gestalt_rust::Output<String>,
        /// Specify the id of the Web Pubsub. Changing this forces a new resource to be created.
        pub web_pubsub_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SharedPrivateLinkResourceArgs,
    ) -> SharedPrivateLinkResourceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let request_message_binding = args.request_message.get_output(context);
        let subresource_name_binding = args.subresource_name.get_output(context);
        let target_resource_id_binding = args.target_resource_id.get_output(context);
        let web_pubsub_id_binding = args.web_pubsub_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:webpubsub/sharedPrivateLinkResource:SharedPrivateLinkResource"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestMessage".into(),
                    value: &request_message_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subresourceName".into(),
                    value: &subresource_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetResourceId".into(),
                    value: &target_resource_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "webPubsubId".into(),
                    value: &web_pubsub_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SharedPrivateLinkResourceResult {
            name: o.get_field("name"),
            request_message: o.get_field("requestMessage"),
            status: o.get_field("status"),
            subresource_name: o.get_field("subresourceName"),
            target_resource_id: o.get_field("targetResourceId"),
            web_pubsub_id: o.get_field("webPubsubId"),
        }
    }
}
