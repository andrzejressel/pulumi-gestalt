#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_analytics_workspace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAnalyticsWorkspaceArgs {
        /// Specifies the name of the Log Analytics Workspace.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which the Log Analytics workspace is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAnalyticsWorkspaceResult {
        /// The workspace daily quota for ingestion in GB.
        pub daily_quota_gb: pulumi_gestalt_rust::Output<f64>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Primary shared key for the Log Analytics Workspace.
        pub primary_shared_key: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The workspace data retention in days.
        pub retention_in_days: pulumi_gestalt_rust::Output<i32>,
        /// The Secondary shared key for the Log Analytics Workspace.
        pub secondary_shared_key: pulumi_gestalt_rust::Output<String>,
        /// The SKU of the Log Analytics Workspace.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The Workspace (or Customer) ID for the Log Analytics Workspace.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAnalyticsWorkspaceArgs,
    ) -> GetAnalyticsWorkspaceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:operationalinsights/getAnalyticsWorkspace:getAnalyticsWorkspace"
                .into(),
            version: super::super::super::get_version(),
            object: &[
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
        let o = context.invoke_resource(request);
        GetAnalyticsWorkspaceResult {
            daily_quota_gb: o.get_field("dailyQuotaGb"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            primary_shared_key: o.get_field("primarySharedKey"),
            resource_group_name: o.get_field("resourceGroupName"),
            retention_in_days: o.get_field("retentionInDays"),
            secondary_shared_key: o.get_field("secondarySharedKey"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
