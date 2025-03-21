/// Manages a Network Manager Subscription Connection which may cross tenants.
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
///   exampleNetworkManager:
///     type: azure:network:NetworkManager
///     name: example
///     properties:
///       name: example-networkmanager
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       scope:
///         subscriptionIds:
///           - ${current.id}
///       scopeAccesses:
///         - SecurityAdmin
///   exampleNetworkManagerSubscriptionConnection:
///     type: azure:network:NetworkManagerSubscriptionConnection
///     name: example
///     properties:
///       name: example-nsnmc
///       subscriptionId: ${current.id}
///       networkManagerId: ${exampleNetworkManager.id}
///       description: example
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
/// ## Import
///
/// Network Subscription Network Manager Connection can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkManagerSubscriptionConnection:NetworkManagerSubscriptionConnection example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Network/networkManagerConnections/networkManagerConnection1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_manager_subscription_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkManagerSubscriptionConnectionArgs {
        /// A description of the Network Manager Subscription Connection.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Network Subscription Network Manager Connection. Changing this forces a new Network Subscription Network Manager Connection to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Network Manager which the Subscription is connected to.
        #[builder(into)]
        pub network_manager_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the ID of the target Subscription. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subscription_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkManagerSubscriptionConnectionResult {
        /// The Connection state of the Network Manager Subscription Connection.
        pub connection_state: pulumi_gestalt_rust::Output<String>,
        /// A description of the Network Manager Subscription Connection.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Network Subscription Network Manager Connection. Changing this forces a new Network Subscription Network Manager Connection to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Network Manager which the Subscription is connected to.
        pub network_manager_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the target Subscription. Changing this forces a new resource to be created.
        pub subscription_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkManagerSubscriptionConnectionArgs,
    ) -> NetworkManagerSubscriptionConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_manager_id_binding = args.network_manager_id.get_output(context);
        let subscription_id_binding = args.subscription_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/networkManagerSubscriptionConnection:NetworkManagerSubscriptionConnection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkManagerId".into(),
                    value: &network_manager_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscriptionId".into(),
                    value: &subscription_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkManagerSubscriptionConnectionResult {
            connection_state: o.get_field("connectionState"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            network_manager_id: o.get_field("networkManagerId"),
            subscription_id: o.get_field("subscriptionId"),
        }
    }
}
