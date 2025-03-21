/// Manages an Azure SQL Managed Instance Failover Group.
///
/// ## Example Usage
///
/// > **Note:** For a more complete example, see the `./examples/sql-azure/managed_instance_failover_group` directory within the GitHub Repository.
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       addressSpaces:
///         - 10.0.0.0/16
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleNetworkSecurityGroup:
///     type: azure:network:NetworkSecurityGroup
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnetNetworkSecurityGroupAssociation:
///     type: azure:network:SubnetNetworkSecurityGroupAssociation
///     name: example
///     properties:
///       subnetId: ${exampleSubnet.id}
///       networkSecurityGroupId: ${exampleNetworkSecurityGroup.id}
///   exampleRouteTable:
///     type: azure:network:RouteTable
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnetRouteTableAssociation:
///     type: azure:network:SubnetRouteTableAssociation
///     name: example
///     properties:
///       subnetId: ${exampleSubnet.id}
///       routeTableId: ${exampleRouteTable.id}
///   primary:
///     type: azure:mssql:ManagedInstance
///     properties:
///       name: example-primary
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       administratorLogin: mradministrator
///       administratorLoginPassword: thisIsDog11
///       licenseType: BasePrice
///       subnetId: ${exampleSubnet.id}
///       skuName: GP_Gen5
///       vcores: 4
///       storageSizeInGb: 32
///       tags:
///         environment: prod
///     options:
///       dependsOn:
///         - ${exampleSubnetNetworkSecurityGroupAssociation}
///         - ${exampleSubnetRouteTableAssociation}
///   secondary:
///     type: azure:mssql:ManagedInstance
///     properties:
///       name: example-secondary
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       administratorLogin: mradministrator
///       administratorLoginPassword: thisIsDog11
///       licenseType: BasePrice
///       subnetId: ${exampleSubnet.id}
///       skuName: GP_Gen5
///       vcores: 4
///       storageSizeInGb: 32
///       tags:
///         environment: prod
///     options:
///       dependsOn:
///         - ${exampleSubnetNetworkSecurityGroupAssociation}
///         - ${exampleSubnetRouteTableAssociation}
///   exampleManagedInstanceFailoverGroup:
///     type: azure:mssql:ManagedInstanceFailoverGroup
///     name: example
///     properties:
///       name: example-failover-group
///       location: ${primary.location}
///       managedInstanceId: ${primary.id}
///       partnerManagedInstanceId: ${secondary.id}
///       readWriteEndpointFailoverPolicy:
///         mode: Automatic
///         graceMinutes: 60
/// ```
///
/// ## Import
///
/// SQL Instance Failover Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/managedInstanceFailoverGroup:ManagedInstanceFailoverGroup example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Sql/locations/Location/instanceFailoverGroups/failoverGroup1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod managed_instance_failover_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedInstanceFailoverGroupArgs {
        /// The Azure Region where the Managed Instance Failover Group should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Azure SQL Managed Instance which will be replicated using a Managed Instance Failover Group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub managed_instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Managed Instance Failover Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Azure SQL Managed Instance which will be replicated to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub partner_managed_instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `read_write_endpoint_failover_policy` block as defined below.
        #[builder(into)]
        pub read_write_endpoint_failover_policy: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::mssql::ManagedInstanceFailoverGroupReadWriteEndpointFailoverPolicy,
        >,
        /// Failover policy for the read-only endpoint. Defaults to `true`.
        #[builder(into, default)]
        pub readonly_endpoint_failover_policy_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct ManagedInstanceFailoverGroupResult {
        /// The Azure Region where the Managed Instance Failover Group should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Azure SQL Managed Instance which will be replicated using a Managed Instance Failover Group. Changing this forces a new resource to be created.
        pub managed_instance_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Managed Instance Failover Group. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Azure SQL Managed Instance which will be replicated to. Changing this forces a new resource to be created.
        pub partner_managed_instance_id: pulumi_gestalt_rust::Output<String>,
        /// A `partner_region` block as defined below.
        pub partner_regions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::mssql::ManagedInstanceFailoverGroupPartnerRegion>,
        >,
        /// A `read_write_endpoint_failover_policy` block as defined below.
        pub read_write_endpoint_failover_policy: pulumi_gestalt_rust::Output<
            super::super::types::mssql::ManagedInstanceFailoverGroupReadWriteEndpointFailoverPolicy,
        >,
        /// Failover policy for the read-only endpoint. Defaults to `true`.
        pub readonly_endpoint_failover_policy_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The partner replication role of the Managed Instance Failover Group.
        pub role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedInstanceFailoverGroupArgs,
    ) -> ManagedInstanceFailoverGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let managed_instance_id_binding = args.managed_instance_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let partner_managed_instance_id_binding = args
            .partner_managed_instance_id
            .get_output(context);
        let read_write_endpoint_failover_policy_binding = args
            .read_write_endpoint_failover_policy
            .get_output(context);
        let readonly_endpoint_failover_policy_enabled_binding = args
            .readonly_endpoint_failover_policy_enabled
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mssql/managedInstanceFailoverGroup:ManagedInstanceFailoverGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedInstanceId".into(),
                    value: &managed_instance_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partnerManagedInstanceId".into(),
                    value: &partner_managed_instance_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "readWriteEndpointFailoverPolicy".into(),
                    value: &read_write_endpoint_failover_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "readonlyEndpointFailoverPolicyEnabled".into(),
                    value: &readonly_endpoint_failover_policy_enabled_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ManagedInstanceFailoverGroupResult {
            location: o.get_field("location"),
            managed_instance_id: o.get_field("managedInstanceId"),
            name: o.get_field("name"),
            partner_managed_instance_id: o.get_field("partnerManagedInstanceId"),
            partner_regions: o.get_field("partnerRegions"),
            read_write_endpoint_failover_policy: o
                .get_field("readWriteEndpointFailoverPolicy"),
            readonly_endpoint_failover_policy_enabled: o
                .get_field("readonlyEndpointFailoverPolicyEnabled"),
            role: o.get_field("role"),
        }
    }
}
