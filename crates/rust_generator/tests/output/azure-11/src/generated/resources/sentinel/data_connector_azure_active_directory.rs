/// Manages a Azure Active Directory Data Connector.
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
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("example-workspace")
///             .resource_group_name("${example.name}")
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleDataConnectorAzureActiveDirectory = data_connector_azure_active_directory::create(
///         "exampleDataConnectorAzureActiveDirectory",
///         DataConnectorAzureActiveDirectoryArgs::builder()
///             .log_analytics_workspace_id(
///                 "${exampleLogAnalyticsWorkspaceOnboarding.workspaceId}",
///             )
///             .name("example")
///             .build_struct(),
///     );
///     let exampleLogAnalyticsWorkspaceOnboarding = log_analytics_workspace_onboarding::create(
///         "exampleLogAnalyticsWorkspaceOnboarding",
///         LogAnalyticsWorkspaceOnboardingArgs::builder()
///             .workspace_id("${exampleAnalyticsWorkspace.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure Active Directory Data Connectors can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/dataConnectorAzureActiveDirectory:DataConnectorAzureActiveDirectory example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/dataConnectors/dc1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_connector_azure_active_directory {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataConnectorAzureActiveDirectoryArgs {
        /// The ID of the Log Analytics Workspace that this Azure Active Directory Data Connector resides in. Changing this forces a new Azure Active Directory Data Connector to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Azure Active Directory Data Connector. Changing this forces a new Azure Active Directory Data Connector to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the tenant that this Azure Active Directory Data Connector connects to. Changing this forces a new Azure Active Directory Data Connector to be created.
        ///
        /// > **NOTE** Currently, only the same tenant as the running account is allowed. Cross-tenant scenario is not supported yet.
        #[builder(into, default)]
        pub tenant_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DataConnectorAzureActiveDirectoryResult {
        /// The ID of the Log Analytics Workspace that this Azure Active Directory Data Connector resides in. Changing this forces a new Azure Active Directory Data Connector to be created.
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Azure Active Directory Data Connector. Changing this forces a new Azure Active Directory Data Connector to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the tenant that this Azure Active Directory Data Connector connects to. Changing this forces a new Azure Active Directory Data Connector to be created.
        ///
        /// > **NOTE** Currently, only the same tenant as the running account is allowed. Cross-tenant scenario is not supported yet.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataConnectorAzureActiveDirectoryArgs,
    ) -> DataConnectorAzureActiveDirectoryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let tenant_id_binding = args.tenant_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:sentinel/dataConnectorAzureActiveDirectory:DataConnectorAzureActiveDirectory"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenantId".into(),
                    value: &tenant_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DataConnectorAzureActiveDirectoryResult {
            log_analytics_workspace_id: o.get_field("logAnalyticsWorkspaceId"),
            name: o.get_field("name"),
            tenant_id: o.get_field("tenantId"),
        }
    }
}
