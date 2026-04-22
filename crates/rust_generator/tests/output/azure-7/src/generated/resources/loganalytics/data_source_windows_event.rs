/// Manages a Log Analytics Windows Event DataSource.
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
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("example-law")
///             .resource_group_name("${example.name}")
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleDataSourceWindowsEvent = data_source_windows_event::create(
///         "exampleDataSourceWindowsEvent",
///         DataSourceWindowsEventArgs::builder()
///             .event_log_name("Application")
///             .event_types(vec!["Error",])
///             .name("example-lad-wpc")
///             .resource_group_name("${example.name}")
///             .workspace_name("${exampleAnalyticsWorkspace.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Log Analytics Windows Event DataSources can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:loganalytics/dataSourceWindowsEvent:DataSourceWindowsEvent example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/dataSources/datasource1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod data_source_windows_event {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataSourceWindowsEventArgs {
        /// Specifies the name of the Windows Event Log to collect events from.
        #[builder(into)]
        pub event_log_name: pulumi_gestalt_rust::Input<String>,
        /// Specifies an array of event types applied to the specified event log. Possible values include `Error`, `Warning` and `Information`.
        #[builder(into)]
        pub event_types: pulumi_gestalt_rust::Input<Vec<String>>,
        /// The name which should be used for this Log Analytics Windows Event DataSource. Changing this forces a new Log Analytics Windows Event DataSource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the Resource Group where the Log Analytics Windows Event DataSource should exist. Changing this forces a new Log Analytics Windows Event DataSource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        /// The name of the Log Analytics Workspace where the Log Analytics Windows Event DataSource should exist. Changing this forces a new Log Analytics Windows Event DataSource to be created.
        #[builder(into)]
        pub workspace_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct DataSourceWindowsEventResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Windows Event Log to collect events from.
        pub event_log_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies an array of event types applied to the specified event log. Possible values include `Error`, `Warning` and `Information`.
        pub event_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name which should be used for this Log Analytics Windows Event DataSource. Changing this forces a new Log Analytics Windows Event DataSource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Log Analytics Windows Event DataSource should exist. Changing this forces a new Log Analytics Windows Event DataSource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Log Analytics Workspace where the Log Analytics Windows Event DataSource should exist. Changing this forces a new Log Analytics Windows Event DataSource to be created.
        pub workspace_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataSourceWindowsEventArgs,
    ) -> DataSourceWindowsEventResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataSourceWindowsEventArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DataSourceWindowsEventResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataSourceWindowsEventArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DataSourceWindowsEventResult {
        let event_log_name_binding = args.event_log_name.get_output(ctx);
        let event_types_binding = args.event_types.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let workspace_name_binding = args.workspace_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:loganalytics/dataSourceWindowsEvent:DataSourceWindowsEvent"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventLogName".into(),
                    value: &event_log_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventTypes".into(),
                    value: &event_types_binding.drop_type(),
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
                    name: "workspaceName".into(),
                    value: &workspace_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DataSourceWindowsEventResult {
            id: o.get_id(),
            urn: o.get_urn(),
            event_log_name: o.get_field("eventLogName"),
            event_types: o.get_field("eventTypes"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            workspace_name: o.get_field("workspaceName"),
        }
    }
}
