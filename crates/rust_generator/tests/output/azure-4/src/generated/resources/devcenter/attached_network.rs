/// Manages a Dev Center Attached Network.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-dcan
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: internal
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleDevCenter:
///     type: azure:devcenter:DevCenter
///     name: example
///     properties:
///       name: example-dc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       identity:
///         type: SystemAssigned
///   exampleNetworkConnection:
///     type: azure:devcenter:NetworkConnection
///     name: example
///     properties:
///       name: example-dcnc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       domainJoinType: AzureADJoin
///       subnetId: ${exampleSubnet.id}
///   exampleAttachedNetwork:
///     type: azure:devcenter:AttachedNetwork
///     name: example
///     properties:
///       name: example-dcet
///       devCenterId: ${exampleDevCenter.id}
///       networkConnectionId: ${exampleNetworkConnection.id}
/// ```
///
/// ## Import
///
/// An existing Dev Center Attached Network can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devcenter/attachedNetwork:AttachedNetwork example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DevCenter/devCenters/dc1/attachedNetworks/et1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod attached_network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AttachedNetworkArgs {
        /// The ID of the associated Dev Center. Changing this forces a new resource to be created.
        #[builder(into)]
        pub dev_center_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of this Dev Center Attached Network. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Dev Center Network Connection you want to attach. Changing this forces a new resource to be created.
        #[builder(into)]
        pub network_connection_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AttachedNetworkResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the associated Dev Center. Changing this forces a new resource to be created.
        pub dev_center_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of this Dev Center Attached Network. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Dev Center Network Connection you want to attach. Changing this forces a new resource to be created.
        pub network_connection_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AttachedNetworkArgs,
    ) -> AttachedNetworkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dev_center_id_binding = args.dev_center_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_connection_id_binding = args
            .network_connection_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:devcenter/attachedNetwork:AttachedNetwork".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "devCenterId".into(),
                    value: &dev_center_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkConnectionId".into(),
                    value: &network_connection_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AttachedNetworkResult {
            id: o.get_field("id"),
            dev_center_id: o.get_field("devCenterId"),
            name: o.get_field("name"),
            network_connection_id: o.get_field("networkConnectionId"),
        }
    }
}
