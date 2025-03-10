/// Manages a Resource Group Template Deployment.
///
/// > **Note:** This resource will automatically attempt to delete resources deployed by the ARM Template when it is deleted. This behavior can be disabled in the provider `features` block by setting the `delete_nested_items_during_deletion` field to `false` within the `template_deployment` block.
///
/// ## Import
///
/// Resource Group Template Deployments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:core/resourceGroupTemplateDeployment:ResourceGroupTemplateDeployment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Resources/deployments/template1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_group_template_deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceGroupTemplateDeploymentArgs {
        /// The Debug Level which should be used for this Resource Group Template Deployment. Possible values are `none`, `requestContent`, `responseContent` and `requestContent, responseContent`.
        #[builder(into, default)]
        pub debug_level: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Deployment Mode for this Resource Group Template Deployment. Possible values are `Complete` (where resources in the Resource Group not specified in the ARM Template will be destroyed) and `Incremental` (where resources are additive only).
        ///
        /// > **Note:** If `deployment_mode` is set to `Complete` then resources within this Resource Group which are not defined in the ARM Template will be deleted.
        #[builder(into)]
        pub deployment_mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Resource Group Template Deployment. Changing this forces a new Resource Group Template Deployment to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The contents of the ARM Template parameters file - containing a JSON list of parameters.
        ///
        /// > An example of how to pass variables into an ARM Template can be seen in the example.
        #[builder(into, default)]
        pub parameters_content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Resource Group Template Deployment should exist. Changing this forces a new Resource Group Template Deployment to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Resource Group Template Deployment.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The contents of the ARM Template which should be deployed into this Resource Group. Cannot be specified with `template_spec_version_id`.
        #[builder(into, default)]
        pub template_content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Template Spec Version to deploy. Cannot be specified with `template_content`.
        #[builder(into, default)]
        pub template_spec_version_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ResourceGroupTemplateDeploymentResult {
        /// The Debug Level which should be used for this Resource Group Template Deployment. Possible values are `none`, `requestContent`, `responseContent` and `requestContent, responseContent`.
        pub debug_level: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Deployment Mode for this Resource Group Template Deployment. Possible values are `Complete` (where resources in the Resource Group not specified in the ARM Template will be destroyed) and `Incremental` (where resources are additive only).
        ///
        /// > **Note:** If `deployment_mode` is set to `Complete` then resources within this Resource Group which are not defined in the ARM Template will be deleted.
        pub deployment_mode: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Resource Group Template Deployment. Changing this forces a new Resource Group Template Deployment to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The JSON Content of the Outputs of the ARM Template Deployment.
        pub output_content: pulumi_gestalt_rust::Output<String>,
        /// The contents of the ARM Template parameters file - containing a JSON list of parameters.
        ///
        /// > An example of how to pass variables into an ARM Template can be seen in the example.
        pub parameters_content: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Resource Group Template Deployment should exist. Changing this forces a new Resource Group Template Deployment to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Resource Group Template Deployment.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The contents of the ARM Template which should be deployed into this Resource Group. Cannot be specified with `template_spec_version_id`.
        pub template_content: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Template Spec Version to deploy. Cannot be specified with `template_content`.
        pub template_spec_version_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceGroupTemplateDeploymentArgs,
    ) -> ResourceGroupTemplateDeploymentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let debug_level_binding = args.debug_level.get_output(context);
        let deployment_mode_binding = args.deployment_mode.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_content_binding = args.parameters_content.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let template_content_binding = args.template_content.get_output(context);
        let template_spec_version_id_binding = args
            .template_spec_version_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:core/resourceGroupTemplateDeployment:ResourceGroupTemplateDeployment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "debugLevel".into(),
                    value: &debug_level_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deploymentMode".into(),
                    value: &deployment_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parametersContent".into(),
                    value: &parameters_content_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "templateContent".into(),
                    value: &template_content_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "templateSpecVersionId".into(),
                    value: &template_spec_version_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceGroupTemplateDeploymentResult {
            debug_level: o.get_field("debugLevel"),
            deployment_mode: o.get_field("deploymentMode"),
            name: o.get_field("name"),
            output_content: o.get_field("outputContent"),
            parameters_content: o.get_field("parametersContent"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            template_content: o.get_field("templateContent"),
            template_spec_version_id: o.get_field("templateSpecVersionId"),
        }
    }
}
