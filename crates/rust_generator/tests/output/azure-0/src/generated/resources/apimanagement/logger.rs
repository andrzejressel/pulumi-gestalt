/// Manages a Logger within an API Management Service.
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
///     let exampleInsights = insights::create(
///         "exampleInsights",
///         InsightsArgs::builder()
///             .application_type("other")
///             .location("${example.location}")
///             .name("example-appinsights")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLogger = logger::create(
///         "exampleLogger",
///         LoggerArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .application_insights(
///                 LoggerApplicationInsights::builder()
///                     .instrumentationKey("${exampleInsights.instrumentationKey}")
///                     .build_struct(),
///             )
///             .name("example-logger")
///             .resource_group_name("${example.name}")
///             .resource_id("${exampleInsights.id}")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-apim")
///             .publisher_email("company@exmaple.com")
///             .publisher_name("My Company")
///             .resource_group_name("${example.name}")
///             .sku_name("Developer_1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// API Management Loggers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/logger:Logger example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-rg/providers/Microsoft.ApiManagement/service/example-apim/loggers/example-logger
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod logger {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoggerArgs {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `application_insights` block as documented below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub application_insights: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::LoggerApplicationInsights>,
        >,
        /// Specifies whether records should be buffered in the Logger prior to publishing. Defaults to `true`.
        #[builder(into, default)]
        pub buffered: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A description of this Logger.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `eventhub` block as documented below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub eventhub: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::LoggerEventhub>,
        >,
        /// The name of this Logger, which must be unique within the API Management Service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The target resource id which will be linked in the API-Management portal page. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub resource_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LoggerResult {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// An `application_insights` block as documented below. Changing this forces a new resource to be created.
        pub application_insights: pulumi_gestalt_rust::Output<
            Option<super::super::types::apimanagement::LoggerApplicationInsights>,
        >,
        /// Specifies whether records should be buffered in the Logger prior to publishing. Defaults to `true`.
        pub buffered: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A description of this Logger.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `eventhub` block as documented below. Changing this forces a new resource to be created.
        pub eventhub: pulumi_gestalt_rust::Output<
            Option<super::super::types::apimanagement::LoggerEventhub>,
        >,
        /// The name of this Logger, which must be unique within the API Management Service. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The target resource id which will be linked in the API-Management portal page. Changing this forces a new resource to be created.
        pub resource_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LoggerArgs,
    ) -> LoggerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_name_binding = args.api_management_name.get_output(context);
        let application_insights_binding = args.application_insights.get_output(context);
        let buffered_binding = args.buffered.get_output(context);
        let description_binding = args.description.get_output(context);
        let eventhub_binding = args.eventhub.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let resource_id_binding = args.resource_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/logger:Logger".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationInsights".into(),
                    value: &application_insights_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "buffered".into(),
                    value: &buffered_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhub".into(),
                    value: &eventhub_binding.drop_type(),
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
                    name: "resourceId".into(),
                    value: &resource_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LoggerResult {
            api_management_name: o.get_field("apiManagementName"),
            application_insights: o.get_field("applicationInsights"),
            buffered: o.get_field("buffered"),
            description: o.get_field("description"),
            eventhub: o.get_field("eventhub"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            resource_id: o.get_field("resourceId"),
        }
    }
}
