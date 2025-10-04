/// Manages a Storage Blob Inventory Policy.
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
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .blob_properties(
///                 AccountBlobProperties::builder().versioningEnabled(true).build_struct(),
///             )
///             .location("${example.location}")
///             .name("examplestoracc")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleBlobInventoryPolicy = blob_inventory_policy::create(
///         "exampleBlobInventoryPolicy",
///         BlobInventoryPolicyArgs::builder()
///             .rules(
///                 vec![
///                     BlobInventoryPolicyRule::builder().format("Csv").name("rule1")
///                     .schedule("Daily").schemaFields(vec!["Name", "Last-Modified",])
///                     .scope("Container").storageContainerName("${exampleContainer.name}")
///                     .build_struct(),
///                 ],
///             )
///             .storage_account_id("${exampleAccount.id}")
///             .build_struct(),
///     );
///     let exampleContainer = container::create(
///         "exampleContainer",
///         ContainerArgs::builder()
///             .container_access_type("private")
///             .name("examplecontainer")
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Storage Blob Inventory Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/blobInventoryPolicy:BlobInventoryPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Storage/storageAccounts/storageAccount1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod blob_inventory_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BlobInventoryPolicyArgs {
        /// One or more `rules` blocks as defined below.
        #[builder(into)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::storage::BlobInventoryPolicyRule>,
        >,
        /// The ID of the storage account to apply this Blob Inventory Policy to. Changing this forces a new Storage Blob Inventory Policy to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BlobInventoryPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// One or more `rules` blocks as defined below.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::storage::BlobInventoryPolicyRule>,
        >,
        /// The ID of the storage account to apply this Blob Inventory Policy to. Changing this forces a new Storage Blob Inventory Policy to be created.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BlobInventoryPolicyArgs,
    ) -> BlobInventoryPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let rules_binding = args.rules.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:storage/blobInventoryPolicy:BlobInventoryPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: &rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BlobInventoryPolicyResult {
            id: o.get_field("id"),
            rules: o.get_field("rules"),
            storage_account_id: o.get_field("storageAccountId"),
        }
    }
}
