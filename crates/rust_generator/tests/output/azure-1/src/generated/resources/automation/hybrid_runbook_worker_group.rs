/// Manages a Automation Hybrid Runbook Worker Group.
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
///             .location("${example.location}")
///             .name("example-account")
///             .resource_group_name("${example.name}")
///             .sku_name("Basic")
///             .build_struct(),
///     );
///     let exampleHybridRunbookWorkerGroup = hybrid_runbook_worker_group::create(
///         "exampleHybridRunbookWorkerGroup",
///         HybridRunbookWorkerGroupArgs::builder()
///             .automation_account_name("${exampleAccount.name}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Automations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/hybridRunbookWorkerGroup:HybridRunbookWorkerGroup example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/hybridRunbookWorkerGroups/grp1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod hybrid_runbook_worker_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HybridRunbookWorkerGroupArgs {
        /// The name of the Automation Account in which the Runbook Worker Group is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_gestalt_rust::Input<String>,
        /// The name of resource type `azure.automation.Credential` to use for hybrid worker.
        #[builder(into, default)]
        pub credential_name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name which should be used for this Automation Account Runbook Worker Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the Resource Group where the Automation should exist. Changing this forces a new Automation to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct HybridRunbookWorkerGroupResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name of the Automation Account in which the Runbook Worker Group is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_gestalt_rust::Output<String>,
        /// The name of resource type `azure.automation.Credential` to use for hybrid worker.
        pub credential_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Automation Account Runbook Worker Group. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Automation should exist. Changing this forces a new Automation to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HybridRunbookWorkerGroupArgs,
    ) -> HybridRunbookWorkerGroupResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HybridRunbookWorkerGroupArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> HybridRunbookWorkerGroupResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HybridRunbookWorkerGroupArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> HybridRunbookWorkerGroupResult {
        let automation_account_name_binding = args
            .automation_account_name
            .get_output(ctx);
        let credential_name_binding = args.credential_name.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:automation/hybridRunbookWorkerGroup:HybridRunbookWorkerGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "credentialName".into(),
                    value: &credential_name_binding.drop_type(),
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
            options,
        };
        let o = ctx.register_resource(request);
        HybridRunbookWorkerGroupResult {
            id: o.get_id(),
            urn: o.get_urn(),
            automation_account_name: o.get_field("automationAccountName"),
            credential_name: o.get_field("credentialName"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
