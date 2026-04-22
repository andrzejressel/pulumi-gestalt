/// Manages a Queue within an Azure Storage Account.
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
///             .location("${example.location}")
///             .name("examplestorageacc")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleQueue = queue::create(
///         "exampleQueue",
///         QueueArgs::builder()
///             .name("mysamplequeue")
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Storage Queue's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/queue:Queue queue1 https://example.queue.core.windows.net/queue1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod queue {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QueueArgs {
        /// A mapping of MetaData which should be assigned to this Storage Queue.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Queue which should be created within the Storage Account. Must be unique within the storage account the queue is located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// Specifies the Storage Account in which the Storage Queue should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct QueueResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// A mapping of MetaData which should be assigned to this Storage Queue.
        pub metadata: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Queue which should be created within the Storage Account. Must be unique within the storage account the queue is located. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Resource Manager ID of this Storage Queue.
        pub resource_manager_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Storage Account in which the Storage Queue should exist. Changing this forces a new resource to be created.
        pub storage_account_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: QueueArgs,
    ) -> QueueResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: QueueArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> QueueResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: QueueArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> QueueResult {
        let metadata_binding = args.metadata.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let storage_account_name_binding = args.storage_account_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:storage/queue:Queue".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        QueueResult {
            id: o.get_id(),
            urn: o.get_urn(),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            resource_manager_id: o.get_field("resourceManagerId"),
            storage_account_name: o.get_field("storageAccountName"),
        }
    }
}
