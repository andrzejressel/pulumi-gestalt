/// Manages a Log Analytics (formally Operational Insights) Saved Search.
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
///     let exampleSavedSearch = saved_search::create(
///         "exampleSavedSearch",
///         SavedSearchArgs::builder()
///             .category("exampleCategory")
///             .display_name("exampleDisplayName")
///             .log_analytics_workspace_id("${exampleAnalyticsWorkspace.id}")
///             .name("exampleSavedSearch")
///             .query("exampleQuery")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Log Analytics Saved Searches can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:loganalytics/savedSearch:SavedSearch search1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.OperationalInsights/workspaces/workspace1/savedSearches/search1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod saved_search {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SavedSearchArgs {
        /// The category that the Saved Search will be listed under. Changing this forces a new resource to be created.
        #[builder(into)]
        pub category: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name that Saved Search will be displayed as. Changing this forces a new resource to be created.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The function alias if the query serves as a function. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub function_alias: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The function parameters if the query serves as a function. Changing this forces a new resource to be created. For more examples and proper syntax please refer to [this document](https://learn.microsoft.com/en-us/azure/data-explorer/kusto/query/functions/user-defined-functions).
        #[builder(into, default)]
        pub function_parameters: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the ID of the Log Analytics Workspace that the Saved Search will be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Log Analytics Saved Search. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The query expression for the saved search. Changing this forces a new resource to be created.
        #[builder(into)]
        pub query: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Logs Analytics Saved Search. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SavedSearchResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The category that the Saved Search will be listed under. Changing this forces a new resource to be created.
        pub category: pulumi_gestalt_rust::Output<String>,
        /// The name that Saved Search will be displayed as. Changing this forces a new resource to be created.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The function alias if the query serves as a function. Changing this forces a new resource to be created.
        pub function_alias: pulumi_gestalt_rust::Output<Option<String>>,
        /// The function parameters if the query serves as a function. Changing this forces a new resource to be created. For more examples and proper syntax please refer to [this document](https://learn.microsoft.com/en-us/azure/data-explorer/kusto/query/functions/user-defined-functions).
        pub function_parameters: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the ID of the Log Analytics Workspace that the Saved Search will be associated with. Changing this forces a new resource to be created.
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Log Analytics Saved Search. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The query expression for the saved search. Changing this forces a new resource to be created.
        pub query: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Logs Analytics Saved Search. Changing this forces a new resource to be created.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SavedSearchArgs,
    ) -> SavedSearchResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let category_binding = args.category.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let function_alias_binding = args.function_alias.get_output(context);
        let function_parameters_binding = args.function_parameters.get_output(context);
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let query_binding = args.query.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:loganalytics/savedSearch:SavedSearch".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "category".into(),
                    value: &category_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "functionAlias".into(),
                    value: &function_alias_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "functionParameters".into(),
                    value: &function_parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "query".into(),
                    value: &query_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SavedSearchResult {
            id: o.get_field("id"),
            category: o.get_field("category"),
            display_name: o.get_field("displayName"),
            function_alias: o.get_field("functionAlias"),
            function_parameters: o.get_field("functionParameters"),
            log_analytics_workspace_id: o.get_field("logAnalyticsWorkspaceId"),
            name: o.get_field("name"),
            query: o.get_field("query"),
            tags: o.get_field("tags"),
        }
    }
}
