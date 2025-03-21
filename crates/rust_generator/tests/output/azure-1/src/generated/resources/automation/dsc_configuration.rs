/// Manages a Automation DSC Configuration.
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
///             .name("account1")
///             .resource_group_name("${example.name}")
///             .sku_name("Basic")
///             .build_struct(),
///     );
///     let exampleDscConfiguration = dsc_configuration::create(
///         "exampleDscConfiguration",
///         DscConfigurationArgs::builder()
///             .automation_account_name("${exampleAccount.name}")
///             .content_embedded("configuration test {}")
///             .location("${example.location}")
///             .name("test")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Automation DSC Configuration's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/dscConfiguration:DscConfiguration configuration1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/configurations/configuration1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dsc_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DscConfigurationArgs {
        /// The name of the automation account in which the DSC Configuration is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The PowerShell DSC Configuration script.
        #[builder(into)]
        pub content_embedded: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description to go with DSC Configuration.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Must be the same location as the Automation Account. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Verbose log option.
        #[builder(into, default)]
        pub log_verbose: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the name of the DSC Configuration. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the DSC Configuration is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DscConfigurationResult {
        /// The name of the automation account in which the DSC Configuration is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_gestalt_rust::Output<String>,
        /// The PowerShell DSC Configuration script.
        pub content_embedded: pulumi_gestalt_rust::Output<String>,
        /// Description to go with DSC Configuration.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Must be the same location as the Automation Account. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Verbose log option.
        pub log_verbose: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the DSC Configuration. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the DSC Configuration is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub state: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
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
        args: DscConfigurationArgs,
    ) -> DscConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let automation_account_name_binding = args
            .automation_account_name
            .get_output(context);
        let content_embedded_binding = args.content_embedded.get_output(context);
        let description_binding = args.description.get_output(context);
        let location_binding = args.location.get_output(context);
        let log_verbose_binding = args.log_verbose.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:automation/dscConfiguration:DscConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentEmbedded".into(),
                    value: &content_embedded_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logVerbose".into(),
                    value: &log_verbose_binding.drop_type(),
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
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DscConfigurationResult {
            automation_account_name: o.get_field("automationAccountName"),
            content_embedded: o.get_field("contentEmbedded"),
            description: o.get_field("description"),
            location: o.get_field("location"),
            log_verbose: o.get_field("logVerbose"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
        }
    }
}
