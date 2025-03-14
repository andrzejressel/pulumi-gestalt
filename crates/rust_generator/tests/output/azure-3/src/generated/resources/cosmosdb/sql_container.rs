/// Manages a SQL Container within a Cosmos DB Account.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleSqlDatabase:
///     type: azure:cosmosdb:SqlDatabase
///     name: example
///     properties:
///       name: example-acsd
///       resourceGroupName: ${example.resourceGroupName}
///       accountName: ${example.name}
///   exampleSqlContainer:
///     type: azure:cosmosdb:SqlContainer
///     name: example
///     properties:
///       name: example-container
///       resourceGroupName: ${example.resourceGroupName}
///       accountName: ${example.name}
///       databaseName: ${exampleSqlDatabase.name}
///       partitionKeyPaths:
///         - /definition/id
///       partitionKeyVersion: 1
///       throughput: 400
///       indexingPolicy:
///         indexingMode: consistent
///         includedPaths:
///           - path: /*
///           - path: /included/?
///         excludedPaths:
///           - path: /excluded/?
///       uniqueKeys:
///         - paths:
///             - /definition/idlong
///             - /definition/idshort
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
/// Cosmos SQL Containers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/sqlContainer:SqlContainer example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DocumentDB/databaseAccounts/account1/sqlDatabases/database1/containers/container1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sql_container {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlContainerArgs {
        /// The name of the Cosmos DB Account to create the container within. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The default time to live of Analytical Storage for this SQL container. If present and the value is set to `-1`, it is equal to infinity, and items don’t expire by default. If present and the value is set to some number `n` – items will expire `n` seconds after their last modified time.
        #[builder(into, default)]
        pub analytical_storage_ttl: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// An `autoscale_settings` block as defined below. This must be set upon database creation otherwise it cannot be updated without a manual destroy-apply.
        ///
        /// > **Note:** Switching between autoscale and manual throughput is not supported via this provider and must be completed via the Azure Portal and refreshed.
        #[builder(into, default)]
        pub autoscale_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::SqlContainerAutoscaleSettings>,
        >,
        /// A `conflict_resolution_policy` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub conflict_resolution_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::SqlContainerConflictResolutionPolicy>,
        >,
        /// The name of the Cosmos DB SQL Database to create the container within. Changing this forces a new resource to be created.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The default time to live of SQL container. If missing, items are not expired automatically. If present and the value is set to `-1`, it is equal to infinity, and items don’t expire by default. If present and the value is set to some number `n` – items will expire `n` seconds after their last modified time.
        #[builder(into, default)]
        pub default_ttl: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// An `indexing_policy` block as defined below.
        #[builder(into, default)]
        pub indexing_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::SqlContainerIndexingPolicy>,
        >,
        /// Specifies the name of the Cosmos DB SQL Container. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Define a partition key kind. Possible values are `Hash` and `MultiHash`. Defaults to `Hash`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub partition_key_kind: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of partition key paths. Changing this forces a new resource to be created.
        #[builder(into)]
        pub partition_key_paths: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Define a partition key version. Possible values are `1`and `2`. This should be set to `2` in order to use large partition keys.
        ///
        /// > **Note:** If `partition_key_version` is not specified when creating a new resource, you can update `partition_key_version` to `1`, updating to `2` forces a new resource to be created.
        #[builder(into, default)]
        pub partition_key_version: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the resource group in which the Cosmos DB SQL Container is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The throughput of SQL container (RU/s). Must be set in increments of `100`. The minimum value is `400`. This must be set upon container creation otherwise it cannot be updated without a manual resource destroy-apply.
        #[builder(into, default)]
        pub throughput: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// One or more `unique_key` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub unique_keys: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cosmosdb::SqlContainerUniqueKey>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SqlContainerResult {
        /// The name of the Cosmos DB Account to create the container within. Changing this forces a new resource to be created.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// The default time to live of Analytical Storage for this SQL container. If present and the value is set to `-1`, it is equal to infinity, and items don’t expire by default. If present and the value is set to some number `n` – items will expire `n` seconds after their last modified time.
        pub analytical_storage_ttl: pulumi_gestalt_rust::Output<Option<i32>>,
        /// An `autoscale_settings` block as defined below. This must be set upon database creation otherwise it cannot be updated without a manual destroy-apply.
        ///
        /// > **Note:** Switching between autoscale and manual throughput is not supported via this provider and must be completed via the Azure Portal and refreshed.
        pub autoscale_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::cosmosdb::SqlContainerAutoscaleSettings>,
        >,
        /// A `conflict_resolution_policy` blocks as defined below. Changing this forces a new resource to be created.
        pub conflict_resolution_policy: pulumi_gestalt_rust::Output<
            super::super::types::cosmosdb::SqlContainerConflictResolutionPolicy,
        >,
        /// The name of the Cosmos DB SQL Database to create the container within. Changing this forces a new resource to be created.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// The default time to live of SQL container. If missing, items are not expired automatically. If present and the value is set to `-1`, it is equal to infinity, and items don’t expire by default. If present and the value is set to some number `n` – items will expire `n` seconds after their last modified time.
        pub default_ttl: pulumi_gestalt_rust::Output<Option<i32>>,
        /// An `indexing_policy` block as defined below.
        pub indexing_policy: pulumi_gestalt_rust::Output<
            super::super::types::cosmosdb::SqlContainerIndexingPolicy,
        >,
        /// Specifies the name of the Cosmos DB SQL Container. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Define a partition key kind. Possible values are `Hash` and `MultiHash`. Defaults to `Hash`. Changing this forces a new resource to be created.
        pub partition_key_kind: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of partition key paths. Changing this forces a new resource to be created.
        pub partition_key_paths: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Define a partition key version. Possible values are `1`and `2`. This should be set to `2` in order to use large partition keys.
        ///
        /// > **Note:** If `partition_key_version` is not specified when creating a new resource, you can update `partition_key_version` to `1`, updating to `2` forces a new resource to be created.
        pub partition_key_version: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the resource group in which the Cosmos DB SQL Container is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The throughput of SQL container (RU/s). Must be set in increments of `100`. The minimum value is `400`. This must be set upon container creation otherwise it cannot be updated without a manual resource destroy-apply.
        pub throughput: pulumi_gestalt_rust::Output<i32>,
        /// One or more `unique_key` blocks as defined below. Changing this forces a new resource to be created.
        pub unique_keys: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cosmosdb::SqlContainerUniqueKey>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SqlContainerArgs,
    ) -> SqlContainerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let analytical_storage_ttl_binding = args
            .analytical_storage_ttl
            .get_output(context);
        let autoscale_settings_binding = args.autoscale_settings.get_output(context);
        let conflict_resolution_policy_binding = args
            .conflict_resolution_policy
            .get_output(context);
        let database_name_binding = args.database_name.get_output(context);
        let default_ttl_binding = args.default_ttl.get_output(context);
        let indexing_policy_binding = args.indexing_policy.get_output(context);
        let name_binding = args.name.get_output(context);
        let partition_key_kind_binding = args.partition_key_kind.get_output(context);
        let partition_key_paths_binding = args.partition_key_paths.get_output(context);
        let partition_key_version_binding = args
            .partition_key_version
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let throughput_binding = args.throughput.get_output(context);
        let unique_keys_binding = args.unique_keys.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cosmosdb/sqlContainer:SqlContainer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "analyticalStorageTtl".into(),
                    value: &analytical_storage_ttl_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoscaleSettings".into(),
                    value: &autoscale_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "conflictResolutionPolicy".into(),
                    value: &conflict_resolution_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultTtl".into(),
                    value: &default_ttl_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "indexingPolicy".into(),
                    value: &indexing_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partitionKeyKind".into(),
                    value: &partition_key_kind_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partitionKeyPaths".into(),
                    value: &partition_key_paths_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partitionKeyVersion".into(),
                    value: &partition_key_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "throughput".into(),
                    value: &throughput_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "uniqueKeys".into(),
                    value: &unique_keys_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SqlContainerResult {
            account_name: o.get_field("accountName"),
            analytical_storage_ttl: o.get_field("analyticalStorageTtl"),
            autoscale_settings: o.get_field("autoscaleSettings"),
            conflict_resolution_policy: o.get_field("conflictResolutionPolicy"),
            database_name: o.get_field("databaseName"),
            default_ttl: o.get_field("defaultTtl"),
            indexing_policy: o.get_field("indexingPolicy"),
            name: o.get_field("name"),
            partition_key_kind: o.get_field("partitionKeyKind"),
            partition_key_paths: o.get_field("partitionKeyPaths"),
            partition_key_version: o.get_field("partitionKeyVersion"),
            resource_group_name: o.get_field("resourceGroupName"),
            throughput: o.get_field("throughput"),
            unique_keys: o.get_field("uniqueKeys"),
        }
    }
}
