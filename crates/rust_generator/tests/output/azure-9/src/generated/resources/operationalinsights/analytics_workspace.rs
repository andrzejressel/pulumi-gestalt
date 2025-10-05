/// Manages a Log Analytics (formally Operational Insights) Workspace.
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
///             .name("acctest-01")
///             .resource_group_name("${example.name}")
///             .retention_in_days(30)
///             .sku("PerGB2018")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Log Analytics Workspaces can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:operationalinsights/analyticsWorkspace:AnalyticsWorkspace workspace1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.OperationalInsights/workspaces/workspace1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod analytics_workspace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AnalyticsWorkspaceArgs {
        /// Specifies if the log Analytics Workspace allow users accessing to data associated with resources they have permission to view, without permission to workspace. Defaults to `true`.
        #[builder(into, default)]
        pub allow_resource_only_permissions: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Is Customer Managed Storage mandatory for query management?
        #[builder(into, default)]
        pub cmk_for_query_forced: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The workspace daily quota for ingestion in GB. Defaults to -1 (unlimited) if omitted.
        ///
        /// > **NOTE:** When `sku` is set to `Free` this field should not be set and has a default value of `0.5`.
        #[builder(into, default)]
        pub daily_quota_gb: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// The ID of the Data Collection Rule to use for this workspace.
        #[builder(into, default)]
        pub data_collection_rule_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::operationalinsights::AnalyticsWorkspaceIdentity>,
        >,
        /// Whether to remove the data in the Log Analytics Workspace immediately after 30 days.
        #[builder(into, default)]
        pub immediate_data_purge_on30_days_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Should the Log Analytics Workspace support ingestion over the Public Internet? Defaults to `true`.
        #[builder(into, default)]
        pub internet_ingestion_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should the Log Analytics Workspace support querying over the Public Internet? Defaults to `true`.
        #[builder(into, default)]
        pub internet_query_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies if the log Analytics workspace should enforce authentication using Azure AD. Defaults to `false`.
        #[builder(into, default)]
        pub local_authentication_disabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Log Analytics Workspace. Workspace name should include 4-63 letters, digits or '-'. The '-' shouldn't be the first or the last symbol. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The capacity reservation level in GB for this workspace. Possible values are `100`, `200`, `300`, `400`, `500`, `1000`, `2000` and `5000`.
        ///
        /// > **NOTE:** `reservation_capacity_in_gb_per_day` can only be used when the `sku` is set to `CapacityReservation`.
        #[builder(into, default)]
        pub reservation_capacity_in_gb_per_day: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The name of the resource group in which the Log Analytics workspace is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The workspace data retention in days. Possible values are either 7 (Free Tier only) or range between 30 and 730.
        #[builder(into, default)]
        pub retention_in_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the SKU of the Log Analytics Workspace. Possible values are `PerNode`, `Premium`, `Standard`, `Standalone`, `Unlimited`, `CapacityReservation`, and `PerGB2018` (new SKU as of `2018-04-03`). Defaults to `PerGB2018`.
        ///
        /// > **NOTE:** A new pricing model took effect on `2018-04-03`, which requires the SKU `PerGB2018`. If you're provisioned resources before this date you have the option of remaining with the previous Pricing SKU and using the other SKUs defined above. More information about [the Pricing SKUs is available at the following URI](https://aka.ms/PricingTierWarning).
        ///
        /// > **NOTE:** Changing `sku` forces a new Log Analytics Workspace to be created, except when changing between `PerGB2018` and `CapacityReservation`. However, changing `sku` to `CapacityReservation` or changing `reservation_capacity_in_gb_per_day` to a higher tier will lead to a 31-days commitment period, during which the SKU cannot be changed to a lower one. Please refer to [official documentation](https://learn.microsoft.com/en-us/azure/azure-monitor/logs/cost-logs#commitment-tiers) for further information.
        #[builder(into, default)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        ///
        /// > **NOTE:** If a `azure.operationalinsights.AnalyticsWorkspace` is connected to a `azure.loganalytics.Cluster` via a `azure.loganalytics.LinkedService` you will not be able to modify the workspaces `sku` field until the link between the workspace and the cluster has been broken by deleting the `azure.loganalytics.LinkedService` resource. All other fields are modifiable while the workspace is linked to a cluster.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AnalyticsWorkspaceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Specifies if the log Analytics Workspace allow users accessing to data associated with resources they have permission to view, without permission to workspace. Defaults to `true`.
        pub allow_resource_only_permissions: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Is Customer Managed Storage mandatory for query management?
        pub cmk_for_query_forced: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The workspace daily quota for ingestion in GB. Defaults to -1 (unlimited) if omitted.
        ///
        /// > **NOTE:** When `sku` is set to `Free` this field should not be set and has a default value of `0.5`.
        pub daily_quota_gb: pulumi_gestalt_rust::Output<Option<f64>>,
        /// The ID of the Data Collection Rule to use for this workspace.
        pub data_collection_rule_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::operationalinsights::AnalyticsWorkspaceIdentity>,
        >,
        /// Whether to remove the data in the Log Analytics Workspace immediately after 30 days.
        pub immediate_data_purge_on30_days_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Should the Log Analytics Workspace support ingestion over the Public Internet? Defaults to `true`.
        pub internet_ingestion_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should the Log Analytics Workspace support querying over the Public Internet? Defaults to `true`.
        pub internet_query_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies if the log Analytics workspace should enforce authentication using Azure AD. Defaults to `false`.
        pub local_authentication_disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Log Analytics Workspace. Workspace name should include 4-63 letters, digits or '-'. The '-' shouldn't be the first or the last symbol. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Primary shared key for the Log Analytics Workspace.
        pub primary_shared_key: pulumi_gestalt_rust::Output<String>,
        /// The capacity reservation level in GB for this workspace. Possible values are `100`, `200`, `300`, `400`, `500`, `1000`, `2000` and `5000`.
        ///
        /// > **NOTE:** `reservation_capacity_in_gb_per_day` can only be used when the `sku` is set to `CapacityReservation`.
        pub reservation_capacity_in_gb_per_day: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the resource group in which the Log Analytics workspace is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The workspace data retention in days. Possible values are either 7 (Free Tier only) or range between 30 and 730.
        pub retention_in_days: pulumi_gestalt_rust::Output<i32>,
        /// The Secondary shared key for the Log Analytics Workspace.
        pub secondary_shared_key: pulumi_gestalt_rust::Output<String>,
        /// Specifies the SKU of the Log Analytics Workspace. Possible values are `PerNode`, `Premium`, `Standard`, `Standalone`, `Unlimited`, `CapacityReservation`, and `PerGB2018` (new SKU as of `2018-04-03`). Defaults to `PerGB2018`.
        ///
        /// > **NOTE:** A new pricing model took effect on `2018-04-03`, which requires the SKU `PerGB2018`. If you're provisioned resources before this date you have the option of remaining with the previous Pricing SKU and using the other SKUs defined above. More information about [the Pricing SKUs is available at the following URI](https://aka.ms/PricingTierWarning).
        ///
        /// > **NOTE:** Changing `sku` forces a new Log Analytics Workspace to be created, except when changing between `PerGB2018` and `CapacityReservation`. However, changing `sku` to `CapacityReservation` or changing `reservation_capacity_in_gb_per_day` to a higher tier will lead to a 31-days commitment period, during which the SKU cannot be changed to a lower one. Please refer to [official documentation](https://learn.microsoft.com/en-us/azure/azure-monitor/logs/cost-logs#commitment-tiers) for further information.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        ///
        /// > **NOTE:** If a `azure.operationalinsights.AnalyticsWorkspace` is connected to a `azure.loganalytics.Cluster` via a `azure.loganalytics.LinkedService` you will not be able to modify the workspaces `sku` field until the link between the workspace and the cluster has been broken by deleting the `azure.loganalytics.LinkedService` resource. All other fields are modifiable while the workspace is linked to a cluster.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Workspace (or Customer) ID for the Log Analytics Workspace.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AnalyticsWorkspaceArgs,
    ) -> AnalyticsWorkspaceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allow_resource_only_permissions_binding = args
            .allow_resource_only_permissions
            .get_output(context);
        let cmk_for_query_forced_binding = args.cmk_for_query_forced.get_output(context);
        let daily_quota_gb_binding = args.daily_quota_gb.get_output(context);
        let data_collection_rule_id_binding = args
            .data_collection_rule_id
            .get_output(context);
        let identity_binding = args.identity.get_output(context);
        let immediate_data_purge_on30_days_enabled_binding = args
            .immediate_data_purge_on30_days_enabled
            .get_output(context);
        let internet_ingestion_enabled_binding = args
            .internet_ingestion_enabled
            .get_output(context);
        let internet_query_enabled_binding = args
            .internet_query_enabled
            .get_output(context);
        let local_authentication_disabled_binding = args
            .local_authentication_disabled
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let reservation_capacity_in_gb_per_day_binding = args
            .reservation_capacity_in_gb_per_day
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let retention_in_days_binding = args.retention_in_days.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:operationalinsights/analyticsWorkspace:AnalyticsWorkspace"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowResourceOnlyPermissions".into(),
                    value: &allow_resource_only_permissions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cmkForQueryForced".into(),
                    value: &cmk_for_query_forced_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dailyQuotaGb".into(),
                    value: &daily_quota_gb_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataCollectionRuleId".into(),
                    value: &data_collection_rule_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "immediateDataPurgeOn30DaysEnabled".into(),
                    value: &immediate_data_purge_on30_days_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "internetIngestionEnabled".into(),
                    value: &internet_ingestion_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "internetQueryEnabled".into(),
                    value: &internet_query_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localAuthenticationDisabled".into(),
                    value: &local_authentication_disabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reservationCapacityInGbPerDay".into(),
                    value: &reservation_capacity_in_gb_per_day_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionInDays".into(),
                    value: &retention_in_days_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: &sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AnalyticsWorkspaceResult {
            id: o.get_field("id"),
            allow_resource_only_permissions: o.get_field("allowResourceOnlyPermissions"),
            cmk_for_query_forced: o.get_field("cmkForQueryForced"),
            daily_quota_gb: o.get_field("dailyQuotaGb"),
            data_collection_rule_id: o.get_field("dataCollectionRuleId"),
            identity: o.get_field("identity"),
            immediate_data_purge_on30_days_enabled: o
                .get_field("immediateDataPurgeOn30DaysEnabled"),
            internet_ingestion_enabled: o.get_field("internetIngestionEnabled"),
            internet_query_enabled: o.get_field("internetQueryEnabled"),
            local_authentication_disabled: o.get_field("localAuthenticationDisabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            primary_shared_key: o.get_field("primarySharedKey"),
            reservation_capacity_in_gb_per_day: o
                .get_field("reservationCapacityInGbPerDay"),
            resource_group_name: o.get_field("resourceGroupName"),
            retention_in_days: o.get_field("retentionInDays"),
            secondary_shared_key: o.get_field("secondarySharedKey"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
