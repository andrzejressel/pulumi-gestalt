/// Manages a SQL Dedicated Gateway within a Cosmos DB Account.
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
///             .name("example-resource-group")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .consistency_policy(
///                 AccountConsistencyPolicy::builder()
///                     .consistencyLevel("BoundedStaleness")
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
///             .name("example-ca")
///             .offer_type("Standard")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSqlDedicatedGateway = sql_dedicated_gateway::create(
///         "exampleSqlDedicatedGateway",
///         SqlDedicatedGatewayArgs::builder()
///             .cosmosdb_account_id("${exampleAccount.id}")
///             .instance_count(1)
///             .instance_size("Cosmos.D4s")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// CosmosDB SQL Dedicated Gateways can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/sqlDedicatedGateway:SqlDedicatedGateway example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.DocumentDB/databaseAccounts/account1/services/SqlDedicatedGateway
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod sql_dedicated_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlDedicatedGatewayArgs {
        /// The resource ID of the CosmosDB Account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cosmosdb_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The instance count for the CosmosDB SQL Dedicated Gateway. Possible value is between `1` and `5`.
        #[builder(into)]
        pub instance_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The instance size for the CosmosDB SQL Dedicated Gateway. Changing this forces a new resource to be created. Possible values are `Cosmos.D4s`, `Cosmos.D8s` and `Cosmos.D16s`.
        #[builder(into)]
        pub instance_size: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SqlDedicatedGatewayResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the CosmosDB Account. Changing this forces a new resource to be created.
        pub cosmosdb_account_id: pulumi_gestalt_rust::Output<String>,
        /// The instance count for the CosmosDB SQL Dedicated Gateway. Possible value is between `1` and `5`.
        pub instance_count: pulumi_gestalt_rust::Output<i32>,
        /// The instance size for the CosmosDB SQL Dedicated Gateway. Changing this forces a new resource to be created. Possible values are `Cosmos.D4s`, `Cosmos.D8s` and `Cosmos.D16s`.
        pub instance_size: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SqlDedicatedGatewayArgs,
    ) -> SqlDedicatedGatewayResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SqlDedicatedGatewayArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> SqlDedicatedGatewayResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SqlDedicatedGatewayArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> SqlDedicatedGatewayResult {
        let cosmosdb_account_id_binding = args.cosmosdb_account_id.get_output(ctx);
        let instance_count_binding = args.instance_count.get_output(ctx);
        let instance_size_binding = args.instance_size.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cosmosdb/sqlDedicatedGateway:SqlDedicatedGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cosmosdbAccountId".into(),
                    value: &cosmosdb_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceCount".into(),
                    value: &instance_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceSize".into(),
                    value: &instance_size_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        SqlDedicatedGatewayResult {
            id: o.get_id(),
            urn: o.get_urn(),
            cosmosdb_account_id: o.get_field("cosmosdbAccountId"),
            instance_count: o.get_field("instanceCount"),
            instance_size: o.get_field("instanceSize"),
        }
    }
}
