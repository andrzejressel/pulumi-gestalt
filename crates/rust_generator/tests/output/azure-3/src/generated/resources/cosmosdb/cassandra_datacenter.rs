/// Manages a Cassandra Datacenter.
///
/// > **NOTE:** In order for the `Azure Managed Instances for Apache Cassandra` to work properly the product requires the `Azure Cosmos DB` Application ID to be present and working in your tenant. If the `Azure Cosmos DB` Application ID is missing in your environment you will need to have an administrator of your tenant run the following command to add the `Azure Cosmos DB` Application ID to your tenant:
///
/// ```powershell
/// New-AzADServicePrincipal -ApplicationId a232010e-820c-4083-83bb-3ace5fc29d0b
/// ```
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: accexample-rg
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       addressSpaces:
///         - 10.0.0.0/16
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example-subnet
///       resourceGroupName: ${exampleResourceGroup.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${exampleVirtualNetwork.id}
///       roleDefinitionName: Network Contributor
///       principalId: ${example.objectId}
///   exampleCassandraCluster:
///     type: azure:cosmosdb:CassandraCluster
///     name: example
///     properties:
///       name: example-cluster
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       delegatedManagementSubnetId: ${exampleSubnet.id}
///       defaultAdminPassword: Password1234
///     options:
///       dependsOn:
///         - ${exampleAssignment}
///   exampleCassandraDatacenter:
///     type: azure:cosmosdb:CassandraDatacenter
///     name: example
///     properties:
///       name: example-datacenter
///       location: ${exampleCassandraCluster.location}
///       cassandraClusterId: ${exampleCassandraCluster.id}
///       delegatedManagementSubnetId: ${exampleSubnet.id}
///       nodeCount: 3
///       diskCount: 4
///       skuName: Standard_DS14_v2
///       availabilityZonesEnabled: false
/// variables:
///   example:
///     fn::invoke:
///       function: azuread:getServicePrincipal
///       arguments:
///         displayName: Azure Cosmos DB
/// ```
///
/// ## Import
///
/// Cassandra Datacenters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/cassandraDatacenter:CassandraDatacenter example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.DocumentDB/cassandraClusters/cluster1/dataCenters/dc1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cassandra_datacenter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CassandraDatacenterArgs {
        /// Determines whether availability zones are enabled. Defaults to `true`.
        #[builder(into, default)]
        pub availability_zones_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The key URI of the customer key to use for the encryption of the backup Storage Account.
        #[builder(into, default)]
        pub backup_storage_customer_key_uri: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The fragment of the cassandra.yaml configuration file to be included in the cassandra.yaml for all nodes in this Cassandra Datacenter. The fragment should be Base64 encoded and only a subset of keys is allowed.
        #[builder(into, default)]
        pub base64_encoded_yaml_fragment: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the Cassandra Cluster. Changing this forces a new Cassandra Datacenter to be created.
        #[builder(into)]
        pub cassandra_cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the delegated management subnet for this Cassandra Datacenter. Changing this forces a new Cassandra Datacenter to be created.
        #[builder(into)]
        pub delegated_management_subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Determines the number of p30 disks that are attached to each node.
        #[builder(into, default)]
        pub disk_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The Disk SKU that is used for this Cassandra Datacenter. Defaults to `P30`.
        #[builder(into, default)]
        pub disk_sku: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Azure Region where the Cassandra Datacenter should exist. Changing this forces a new Cassandra Datacenter to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The key URI of the customer key to use for the encryption of the Managed Disk.
        #[builder(into, default)]
        pub managed_disk_customer_key_uri: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name which should be used for this Cassandra Datacenter. Changing this forces a new Cassandra Datacenter to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of nodes the Cassandra Datacenter should have. The number should be equal or greater than `3`. Defaults to `3`.
        #[builder(into, default)]
        pub node_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Determines the selected sku.
        ///
        /// > **NOTE:** In v4.0 of the provider the `sku_name` will have a default value of `Standard_E16s_v5`.
        #[builder(into, default)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CassandraDatacenterResult {
        /// Determines whether availability zones are enabled. Defaults to `true`.
        pub availability_zones_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The key URI of the customer key to use for the encryption of the backup Storage Account.
        pub backup_storage_customer_key_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// The fragment of the cassandra.yaml configuration file to be included in the cassandra.yaml for all nodes in this Cassandra Datacenter. The fragment should be Base64 encoded and only a subset of keys is allowed.
        pub base64_encoded_yaml_fragment: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Cassandra Cluster. Changing this forces a new Cassandra Datacenter to be created.
        pub cassandra_cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the delegated management subnet for this Cassandra Datacenter. Changing this forces a new Cassandra Datacenter to be created.
        pub delegated_management_subnet_id: pulumi_gestalt_rust::Output<String>,
        /// Determines the number of p30 disks that are attached to each node.
        pub disk_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The Disk SKU that is used for this Cassandra Datacenter. Defaults to `P30`.
        pub disk_sku: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Azure Region where the Cassandra Datacenter should exist. Changing this forces a new Cassandra Datacenter to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The key URI of the customer key to use for the encryption of the Managed Disk.
        pub managed_disk_customer_key_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Cassandra Datacenter. Changing this forces a new Cassandra Datacenter to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of nodes the Cassandra Datacenter should have. The number should be equal or greater than `3`. Defaults to `3`.
        pub node_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A list of IP Address for the seed nodes in this Cassandra Datacenter.
        pub seed_node_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Determines the selected sku.
        ///
        /// > **NOTE:** In v4.0 of the provider the `sku_name` will have a default value of `Standard_E16s_v5`.
        pub sku_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CassandraDatacenterArgs,
    ) -> CassandraDatacenterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let availability_zones_enabled_binding = args
            .availability_zones_enabled
            .get_output(context);
        let backup_storage_customer_key_uri_binding = args
            .backup_storage_customer_key_uri
            .get_output(context);
        let base64_encoded_yaml_fragment_binding = args
            .base64_encoded_yaml_fragment
            .get_output(context);
        let cassandra_cluster_id_binding = args.cassandra_cluster_id.get_output(context);
        let delegated_management_subnet_id_binding = args
            .delegated_management_subnet_id
            .get_output(context);
        let disk_count_binding = args.disk_count.get_output(context);
        let disk_sku_binding = args.disk_sku.get_output(context);
        let location_binding = args.location.get_output(context);
        let managed_disk_customer_key_uri_binding = args
            .managed_disk_customer_key_uri
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let node_count_binding = args.node_count.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cosmosdb/cassandraDatacenter:CassandraDatacenter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZonesEnabled".into(),
                    value: &availability_zones_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupStorageCustomerKeyUri".into(),
                    value: &backup_storage_customer_key_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "base64EncodedYamlFragment".into(),
                    value: &base64_encoded_yaml_fragment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cassandraClusterId".into(),
                    value: &cassandra_cluster_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "delegatedManagementSubnetId".into(),
                    value: &delegated_management_subnet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskCount".into(),
                    value: &disk_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskSku".into(),
                    value: &disk_sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedDiskCustomerKeyUri".into(),
                    value: &managed_disk_customer_key_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeCount".into(),
                    value: &node_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CassandraDatacenterResult {
            availability_zones_enabled: o.get_field("availabilityZonesEnabled"),
            backup_storage_customer_key_uri: o.get_field("backupStorageCustomerKeyUri"),
            base64_encoded_yaml_fragment: o.get_field("base64EncodedYamlFragment"),
            cassandra_cluster_id: o.get_field("cassandraClusterId"),
            delegated_management_subnet_id: o.get_field("delegatedManagementSubnetId"),
            disk_count: o.get_field("diskCount"),
            disk_sku: o.get_field("diskSku"),
            location: o.get_field("location"),
            managed_disk_customer_key_uri: o.get_field("managedDiskCustomerKeyUri"),
            name: o.get_field("name"),
            node_count: o.get_field("nodeCount"),
            seed_node_ip_addresses: o.get_field("seedNodeIpAddresses"),
            sku_name: o.get_field("skuName"),
        }
    }
}
