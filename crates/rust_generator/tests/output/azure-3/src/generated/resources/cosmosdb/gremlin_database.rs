/// Manages a Gremlin Database within a Cosmos DB Account.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleGremlinDatabase:
///     type: azure:cosmosdb:GremlinDatabase
///     name: example
///     properties:
///       name: tfex-cosmos-gremlin-db
///       resourceGroupName: ${example.resourceGroupName}
///       accountName: ${example.name}
///       throughput: 400
/// variables:
///   example:
///     fn::invoke:
///       function: azure:cosmosdb:getAccount
///       arguments:
///         name: tfex-cosmosdb-account
///         resourceGroupName: tfex-cosmosdb-account-rg
/// ```
///
/// ## Import
///
/// CosmosDB Gremlin Databases can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/gremlinDatabase:GremlinDatabase db1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.DocumentDB/databaseAccounts/account1/gremlinDatabases/db1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod gremlin_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GremlinDatabaseArgs {
        /// The name of the CosmosDB Account to create the Gremlin Database within. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `autoscale_settings` block as defined below. This must be set upon database creation otherwise it cannot be updated without a manual destroy-apply.
        ///
        /// > **Note:** Switching between autoscale and manual throughput is not supported via this provider and must be completed via the Azure Portal and refreshed.
        #[builder(into, default)]
        pub autoscale_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::GremlinDatabaseAutoscaleSettings>,
        >,
        /// Specifies the name of the Cosmos DB Gremlin Database. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Cosmos DB Gremlin Database is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The throughput of the Gremlin database (RU/s). Must be set in increments of `100`. The minimum value is `400`. This must be set upon database creation otherwise it cannot be updated without a manual resource destroy-apply.
        ///
        /// > **Note:** throughput has a maximum value of `1000000` unless a higher limit is requested via Azure Support
        #[builder(into, default)]
        pub throughput: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct GremlinDatabaseResult {
        /// The name of the CosmosDB Account to create the Gremlin Database within. Changing this forces a new resource to be created.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// An `autoscale_settings` block as defined below. This must be set upon database creation otherwise it cannot be updated without a manual destroy-apply.
        ///
        /// > **Note:** Switching between autoscale and manual throughput is not supported via this provider and must be completed via the Azure Portal and refreshed.
        pub autoscale_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::cosmosdb::GremlinDatabaseAutoscaleSettings>,
        >,
        /// Specifies the name of the Cosmos DB Gremlin Database. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the Cosmos DB Gremlin Database is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The throughput of the Gremlin database (RU/s). Must be set in increments of `100`. The minimum value is `400`. This must be set upon database creation otherwise it cannot be updated without a manual resource destroy-apply.
        ///
        /// > **Note:** throughput has a maximum value of `1000000` unless a higher limit is requested via Azure Support
        pub throughput: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GremlinDatabaseArgs,
    ) -> GremlinDatabaseResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let autoscale_settings_binding = args.autoscale_settings.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let throughput_binding = args.throughput.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cosmosdb/gremlinDatabase:GremlinDatabase".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoscaleSettings".into(),
                    value: &autoscale_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "throughput".into(),
                    value: &throughput_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GremlinDatabaseResult {
            account_name: o.get_field("accountName"),
            autoscale_settings: o.get_field("autoscaleSettings"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            throughput: o.get_field("throughput"),
        }
    }
}
