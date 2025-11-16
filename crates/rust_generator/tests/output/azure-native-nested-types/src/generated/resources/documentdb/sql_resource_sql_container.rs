/// An Azure Cosmos DB container.
/// API Version: 2021-03-15.
///
/// ## Example Usage
/// ### CosmosDBSqlContainerCreateUpdate
///
///
///
///
///
/// ## Import
///
/// An existing resource can be imported using its type token, name, and identifier, e.g.
///
/// ```sh
/// $ pulumi import azure-native:documentdb:SqlResourceSqlContainer containerName /subscriptions/subid/resourceGroups/rg1/providers/Microsoft.DocumentDB/databaseAccounts/ddb1/sqlDatabases/databaseName/sqlContainers/containerName
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod sql_resource_sql_container {
    #[allow(dead_code)]
    pub struct SqlResourceSqlContainerResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub resource: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::documentdb::SqlContainerGetPropertiesResponseResource,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
    ) -> SqlResourceSqlContainerResult {
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure-native:documentdb:SqlResourceSqlContainer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[],
        };
        let o = context.register_resource(request);
        SqlResourceSqlContainerResult {
            id: o.get_field("id"),
            resource: o.get_field("resource"),
        }
    }
}
