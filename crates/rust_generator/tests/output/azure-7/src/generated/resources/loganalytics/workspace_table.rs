/// Manages a Table in a Log Analytics (formally Operational Insights) Workspace.
///
/// > **Note:** This resource does not create or destroy tables. This resource is used to update attributes (currently only retention_in_days) of the tables created when a Log Analytics Workspace is created. Deleting an azure.loganalytics.WorkspaceTable resource will not delete the table. Instead, the table's retention_in_days field will be set to the value of azure.operationalinsights.AnalyticsWorkspace retention_in_days
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
///             .name("example")
///             .resource_group_name("${example.name}")
///             .retention_in_days(30)
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleWorkspaceTable = workspace_table::create(
///         "exampleWorkspaceTable",
///         WorkspaceTableArgs::builder()
///             .name("AppMetrics")
///             .retention_in_days(60)
///             .total_retention_in_days(180)
///             .workspace_id("${exampleAnalyticsWorkspace.id}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace_table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceTableArgs {
        /// Specifies the name of a table in a Log Analytics Workspace.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specify the system how to handle and charge the logs ingested to the table. Possible values are `Analytics` and `Basic`. Defaults to `Analytics`.
        ///
        /// > **Note:** The `name` of tables currently supported by the `Basic` plan can be found [here](https://learn.microsoft.com/en-us/azure/azure-monitor/logs/basic-logs-configure?tabs=portal-1#supported-tables).
        #[builder(into, default)]
        pub plan: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The table's retention in days. Possible values are either `8` (Basic Tier only) or range between `4` and `730`.
        #[builder(into, default)]
        pub retention_in_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The table's total retention in days. Possible values range between `4` and `730`; or `1095`, `1460`, `1826`, `2191`, `2556`, `2922`, `3288`, `3653`, `4018`, or `4383`.
        ///
        /// > **Note:** `retention_in_days` and `total_retention_in_days` will revert back to the value of azure.operationalinsights.AnalyticsWorkspace retention_in_days when a azure.loganalytics.WorkspaceTable is deleted.
        ///
        /// > **Note:** The `retention_in_days` cannot be specified when `plan` is `Basic` because the retention is fixed at eight days.
        #[builder(into, default)]
        pub total_retention_in_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The object ID of the Log Analytics Workspace that contains the table.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceTableResult {
        /// Specifies the name of a table in a Log Analytics Workspace.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specify the system how to handle and charge the logs ingested to the table. Possible values are `Analytics` and `Basic`. Defaults to `Analytics`.
        ///
        /// > **Note:** The `name` of tables currently supported by the `Basic` plan can be found [here](https://learn.microsoft.com/en-us/azure/azure-monitor/logs/basic-logs-configure?tabs=portal-1#supported-tables).
        pub plan: pulumi_gestalt_rust::Output<Option<String>>,
        /// The table's retention in days. Possible values are either `8` (Basic Tier only) or range between `4` and `730`.
        pub retention_in_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The table's total retention in days. Possible values range between `4` and `730`; or `1095`, `1460`, `1826`, `2191`, `2556`, `2922`, `3288`, `3653`, `4018`, or `4383`.
        ///
        /// > **Note:** `retention_in_days` and `total_retention_in_days` will revert back to the value of azure.operationalinsights.AnalyticsWorkspace retention_in_days when a azure.loganalytics.WorkspaceTable is deleted.
        ///
        /// > **Note:** The `retention_in_days` cannot be specified when `plan` is `Basic` because the retention is fixed at eight days.
        pub total_retention_in_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The object ID of the Log Analytics Workspace that contains the table.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkspaceTableArgs,
    ) -> WorkspaceTableResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let plan_binding = args.plan.get_output(context);
        let retention_in_days_binding = args.retention_in_days.get_output(context);
        let total_retention_in_days_binding = args
            .total_retention_in_days
            .get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:loganalytics/workspaceTable:WorkspaceTable".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plan".into(),
                    value: &plan_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionInDays".into(),
                    value: &retention_in_days_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "totalRetentionInDays".into(),
                    value: &total_retention_in_days_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkspaceTableResult {
            name: o.get_field("name"),
            plan: o.get_field("plan"),
            retention_in_days: o.get_field("retentionInDays"),
            total_retention_in_days: o.get_field("totalRetentionInDays"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
