/// Manages Azure Batch Application instance.
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
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("examplesa")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleAccount2 = account::create(
///         "exampleAccount2",
///         AccountArgs::builder()
///             .location("${example.location}")
///             .name("exampleba")
///             .pool_allocation_mode("BatchService")
///             .resource_group_name("${example.name}")
///             .storage_account_authentication_mode("StorageKeys")
///             .storage_account_id("${exampleAccount.id}")
///             .build_struct(),
///     );
///     let exampleApplication = application::create(
///         "exampleApplication",
///         ApplicationArgs::builder()
///             .account_name("${exampleAccount2.name}")
///             .name("example-batch-application")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Batch Applications can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:batch/application:Application example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-rg/providers/Microsoft.Batch/batchAccounts/exampleba/applications/example-batch-application
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// The name of the Batch account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A value indicating whether packages within the application may be overwritten using the same version string. Defaults to `true`.
        #[builder(into, default)]
        pub allow_updates: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The package to use if a client requests the application but does not specify a version. This property can only be set to the name of an existing package.
        #[builder(into, default)]
        pub default_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name for the application.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the application. This must be unique within the account. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group that contains the Batch account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Batch account. Changing this forces a new resource to be created.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// A value indicating whether packages within the application may be overwritten using the same version string. Defaults to `true`.
        pub allow_updates: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The package to use if a client requests the application but does not specify a version. This property can only be set to the name of an existing package.
        pub default_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The display name for the application.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the application. This must be unique within the account. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group that contains the Batch account. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationArgs,
    ) -> ApplicationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let allow_updates_binding = args.allow_updates.get_output(context);
        let default_version_binding = args.default_version.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:batch/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowUpdates".into(),
                    value: &allow_updates_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultVersion".into(),
                    value: &default_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationResult {
            id: o.get_field("id"),
            account_name: o.get_field("accountName"),
            allow_updates: o.get_field("allowUpdates"),
            default_version: o.get_field("defaultVersion"),
            display_name: o.get_field("displayName"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
