/// Manages a Kusto (also known as Azure Data Explorer) Database Principal Assignment.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: KustoRG
///       location: West Europe
///   exampleCluster:
///     type: azure:kusto:Cluster
///     name: example
///     properties:
///       name: kustocluster
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku:
///         name: Standard_D13_v2
///         capacity: 2
///   exampleDatabase:
///     type: azure:kusto:Database
///     name: example
///     properties:
///       name: KustoDatabase
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       clusterName: ${exampleCluster.name}
///       hotCachePeriod: P7D
///       softDeletePeriod: P31D
///   exampleDatabasePrincipalAssignment:
///     type: azure:kusto:DatabasePrincipalAssignment
///     name: example
///     properties:
///       name: KustoPrincipalAssignment
///       resourceGroupName: ${example.name}
///       clusterName: ${exampleCluster.name}
///       databaseName: ${exampleDatabase.name}
///       tenantId: ${current.tenantId}
///       principalId: ${current.clientId}
///       principalType: App
///       role: Viewer
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Kusto Database Principal Assignment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:kusto/databasePrincipalAssignment:DatabasePrincipalAssignment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Kusto/clusters/cluster1/databases/database1/principalAssignments/assignment1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod database_principal_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabasePrincipalAssignmentArgs {
        /// The name of the cluster in which to create the resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the database in which to create the resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the kusto principal assignment. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The object id of the principal. Changing this forces a new resource to be created.
        #[builder(into)]
        pub principal_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of the principal. Valid values include `App`, `Group`, `User`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub principal_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which to create the resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The database role assigned to the principal. Valid values include `Admin`, `Ingestor`, `Monitor`, `UnrestrictedViewer`, `User` and `Viewer`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The tenant id in which the principal resides. Changing this forces a new resource to be created.
        #[builder(into)]
        pub tenant_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DatabasePrincipalAssignmentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The name of the cluster in which to create the resource. Changing this forces a new resource to be created.
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the database in which to create the resource. Changing this forces a new resource to be created.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the kusto principal assignment. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The object id of the principal. Changing this forces a new resource to be created.
        pub principal_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the principal.
        pub principal_name: pulumi_gestalt_rust::Output<String>,
        /// The type of the principal. Valid values include `App`, `Group`, `User`. Changing this forces a new resource to be created.
        pub principal_type: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the resource. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The database role assigned to the principal. Valid values include `Admin`, `Ingestor`, `Monitor`, `UnrestrictedViewer`, `User` and `Viewer`. Changing this forces a new resource to be created.
        pub role: pulumi_gestalt_rust::Output<String>,
        /// The tenant id in which the principal resides. Changing this forces a new resource to be created.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the tenant.
        pub tenant_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DatabasePrincipalAssignmentArgs,
    ) -> DatabasePrincipalAssignmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_name_binding = args.cluster_name.get_output(context);
        let database_name_binding = args.database_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let principal_id_binding = args.principal_id.get_output(context);
        let principal_type_binding = args.principal_type.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let role_binding = args.role.get_output(context);
        let tenant_id_binding = args.tenant_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:kusto/databasePrincipalAssignment:DatabasePrincipalAssignment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalId".into(),
                    value: &principal_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalType".into(),
                    value: &principal_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: &role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenantId".into(),
                    value: &tenant_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DatabasePrincipalAssignmentResult {
            id: o.get_field("id"),
            cluster_name: o.get_field("clusterName"),
            database_name: o.get_field("databaseName"),
            name: o.get_field("name"),
            principal_id: o.get_field("principalId"),
            principal_name: o.get_field("principalName"),
            principal_type: o.get_field("principalType"),
            resource_group_name: o.get_field("resourceGroupName"),
            role: o.get_field("role"),
            tenant_id: o.get_field("tenantId"),
            tenant_name: o.get_field("tenantName"),
        }
    }
}
