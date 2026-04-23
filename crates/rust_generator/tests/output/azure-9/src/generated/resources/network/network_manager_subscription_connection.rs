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
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod network_manager_subscription_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkManagerSubscriptionConnectionArgs {
        /// A description of the Network Manager Subscription Connection.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// Specifies the name which should be used for this Network Subscription Network Manager Connection. Changing this forces a new Network Subscription Network Manager Connection to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// Specifies the ID of the Network Manager which the Subscription is connected to.
        #[builder(into)]
        pub network_manager_id: pulumi_gestalt_rust::Input<String>,
        /// Specifies the ID of the target Subscription. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subscription_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkManagerSubscriptionConnectionResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
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
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkManagerSubscriptionConnectionArgs,
    ) -> NetworkManagerSubscriptionConnectionResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkManagerSubscriptionConnectionArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> NetworkManagerSubscriptionConnectionResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkManagerSubscriptionConnectionArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> NetworkManagerSubscriptionConnectionResult {
        let description_binding = args.description.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let network_manager_id_binding = args.network_manager_id.get_output(ctx);
        let subscription_id_binding = args.subscription_id.get_output(ctx);
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
            options,
        };
        let o = ctx.register_resource(request);
        NetworkManagerSubscriptionConnectionResult {
            id: o.get_id(),
            urn: o.get_urn(),
            connection_state: o.get_field("connectionState"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            network_manager_id: o.get_field("networkManagerId"),
            subscription_id: o.get_field("subscriptionId"),
        }
    }
}
