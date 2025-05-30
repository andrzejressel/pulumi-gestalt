/// Associates a Spring Cloud Application with a CosmosDB Account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .consistency_policy(
///                 AccountConsistencyPolicy::builder()
///                     .consistencyLevel("Strong")
///                     .build_struct(),
///             )
///             .geo_locations(
///                 vec![
///                     AccountGeoLocation::builder().failoverPriority(0)
///                     .location("${example.location}").build_struct(),
///                 ],
///             )
///             .kind("GlobalDocumentDB")
///             .location("${example.location}")
///             .name("example-cosmosdb-account")
///             .offer_type("Standard")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSpringCloudApp = spring_cloud_app::create(
///         "exampleSpringCloudApp",
///         SpringCloudAppArgs::builder()
///             .name("example-springcloudapp")
///             .resource_group_name("${example.name}")
///             .service_name("${exampleSpringCloudService.name}")
///             .build_struct(),
///     );
///     let exampleSpringCloudAppCosmosDBAssociation = spring_cloud_app_cosmos_db_association::create(
///         "exampleSpringCloudAppCosmosDBAssociation",
///         SpringCloudAppCosmosDbAssociationArgs::builder()
///             .api_type("table")
///             .cosmosdb_access_key("${exampleAccount.primaryKey}")
///             .cosmosdb_account_id("${exampleAccount.id}")
///             .name("example-bind")
///             .spring_cloud_app_id("${exampleSpringCloudApp.id}")
///             .build_struct(),
///     );
///     let exampleSpringCloudService = spring_cloud_service::create(
///         "exampleSpringCloudService",
///         SpringCloudServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-springcloud")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Spring Cloud Application CosmosDB Association can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudAppCosmosDBAssociation:SpringCloudAppCosmosDBAssociation example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourcegroup1/providers/Microsoft.AppPlatform/spring/service1/apps/app1/bindings/bind1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_app_cosmos_db_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudAppCosmosDBAssociationArgs {
        /// Specifies the API type which should be used when connecting to the CosmosDB Account. Possible values are `cassandra`, `gremlin`, `mongo`, `sql` or `table`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the CosmosDB Account access key.
        #[builder(into)]
        pub cosmosdb_access_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the ID of the CosmosDB Account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cosmosdb_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Cassandra Keyspace which the Spring Cloud App should be associated with. Should only be set when `api_type` is `cassandra`.
        #[builder(into, default)]
        pub cosmosdb_cassandra_keyspace_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the name of the Gremlin Database which the Spring Cloud App should be associated with. Should only be set when `api_type` is `gremlin`.
        #[builder(into, default)]
        pub cosmosdb_gremlin_database_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the name of the Gremlin Graph which the Spring Cloud App should be associated with. Should only be set when `api_type` is `gremlin`.
        #[builder(into, default)]
        pub cosmosdb_gremlin_graph_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the name of the Mongo Database which the Spring Cloud App should be associated with. Should only be set when `api_type` is `mongo`.
        #[builder(into, default)]
        pub cosmosdb_mongo_database_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the name of the SQL Database which the Spring Cloud App should be associated with. Should only be set when `api_type` is `sql`.
        #[builder(into, default)]
        pub cosmosdb_sql_database_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the name of the Spring Cloud Application Association. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Spring Cloud Application where this Association is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudAppCosmosDBAssociationResult {
        /// Specifies the API type which should be used when connecting to the CosmosDB Account. Possible values are `cassandra`, `gremlin`, `mongo`, `sql` or `table`. Changing this forces a new resource to be created.
        pub api_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the CosmosDB Account access key.
        pub cosmosdb_access_key: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the CosmosDB Account. Changing this forces a new resource to be created.
        pub cosmosdb_account_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Cassandra Keyspace which the Spring Cloud App should be associated with. Should only be set when `api_type` is `cassandra`.
        pub cosmosdb_cassandra_keyspace_name: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Specifies the name of the Gremlin Database which the Spring Cloud App should be associated with. Should only be set when `api_type` is `gremlin`.
        pub cosmosdb_gremlin_database_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Gremlin Graph which the Spring Cloud App should be associated with. Should only be set when `api_type` is `gremlin`.
        pub cosmosdb_gremlin_graph_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Mongo Database which the Spring Cloud App should be associated with. Should only be set when `api_type` is `mongo`.
        pub cosmosdb_mongo_database_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the SQL Database which the Spring Cloud App should be associated with. Should only be set when `api_type` is `sql`.
        pub cosmosdb_sql_database_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Spring Cloud Application Association. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Spring Cloud Application where this Association is created. Changing this forces a new resource to be created.
        pub spring_cloud_app_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudAppCosmosDBAssociationArgs,
    ) -> SpringCloudAppCosmosDBAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_type_binding = args.api_type.get_output(context);
        let cosmosdb_access_key_binding = args.cosmosdb_access_key.get_output(context);
        let cosmosdb_account_id_binding = args.cosmosdb_account_id.get_output(context);
        let cosmosdb_cassandra_keyspace_name_binding = args
            .cosmosdb_cassandra_keyspace_name
            .get_output(context);
        let cosmosdb_gremlin_database_name_binding = args
            .cosmosdb_gremlin_database_name
            .get_output(context);
        let cosmosdb_gremlin_graph_name_binding = args
            .cosmosdb_gremlin_graph_name
            .get_output(context);
        let cosmosdb_mongo_database_name_binding = args
            .cosmosdb_mongo_database_name
            .get_output(context);
        let cosmosdb_sql_database_name_binding = args
            .cosmosdb_sql_database_name
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let spring_cloud_app_id_binding = args.spring_cloud_app_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudAppCosmosDBAssociation:SpringCloudAppCosmosDBAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiType".into(),
                    value: &api_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cosmosdbAccessKey".into(),
                    value: &cosmosdb_access_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cosmosdbAccountId".into(),
                    value: &cosmosdb_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cosmosdbCassandraKeyspaceName".into(),
                    value: &cosmosdb_cassandra_keyspace_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cosmosdbGremlinDatabaseName".into(),
                    value: &cosmosdb_gremlin_database_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cosmosdbGremlinGraphName".into(),
                    value: &cosmosdb_gremlin_graph_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cosmosdbMongoDatabaseName".into(),
                    value: &cosmosdb_mongo_database_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cosmosdbSqlDatabaseName".into(),
                    value: &cosmosdb_sql_database_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "springCloudAppId".into(),
                    value: &spring_cloud_app_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudAppCosmosDBAssociationResult {
            api_type: o.get_field("apiType"),
            cosmosdb_access_key: o.get_field("cosmosdbAccessKey"),
            cosmosdb_account_id: o.get_field("cosmosdbAccountId"),
            cosmosdb_cassandra_keyspace_name: o
                .get_field("cosmosdbCassandraKeyspaceName"),
            cosmosdb_gremlin_database_name: o.get_field("cosmosdbGremlinDatabaseName"),
            cosmosdb_gremlin_graph_name: o.get_field("cosmosdbGremlinGraphName"),
            cosmosdb_mongo_database_name: o.get_field("cosmosdbMongoDatabaseName"),
            cosmosdb_sql_database_name: o.get_field("cosmosdbSqlDatabaseName"),
            name: o.get_field("name"),
            spring_cloud_app_id: o.get_field("springCloudAppId"),
        }
    }
}
