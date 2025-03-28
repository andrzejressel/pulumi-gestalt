/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleTenantTemplateDeployment:
///     type: azure:core:TenantTemplateDeployment
///     name: example
///     properties:
///       name: example
///       location: West Europe
///       templateSpecVersionId: ${example.id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:core:getTemplateSpecVersion
///       arguments:
///         name: myTemplateForTenant
///         resourceGroupName: myResourceGroup
///         version: v0.1
/// ```
///
/// ## Import
///
/// Tenant Template Deployments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:core/tenantTemplateDeployment:TenantTemplateDeployment example /providers/Microsoft.Resources/deployments/deploy1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tenant_template_deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TenantTemplateDeploymentArgs {
        /// The Debug Level which should be used for this Resource Group Template Deployment. Possible values are `none`, `requestContent`, `responseContent` and `requestContent, responseContent`.
        #[builder(into, default)]
        pub debug_level: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Azure Region where the Template should exist. Changing this forces a new Template to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Template. Changing this forces a new Template to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The contents of the ARM Template parameters file - containing a JSON list of parameters.
        #[builder(into, default)]
        pub parameters_content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Template.
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
    pub struct TenantTemplateDeploymentResult {
        /// The Debug Level which should be used for this Resource Group Template Deployment. Possible values are `none`, `requestContent`, `responseContent` and `requestContent, responseContent`.
        pub debug_level: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Azure Region where the Template should exist. Changing this forces a new Template to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Template. Changing this forces a new Template to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The JSON Content of the Outputs of the ARM Template Deployment.
        pub output_content: pulumi_gestalt_rust::Output<String>,
        /// The contents of the ARM Template parameters file - containing a JSON list of parameters.
        pub parameters_content: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Template.
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
        args: TenantTemplateDeploymentArgs,
    ) -> TenantTemplateDeploymentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let debug_level_binding = args.debug_level.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_content_binding = args.parameters_content.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let template_content_binding = args.template_content.get_output(context);
        let template_spec_version_id_binding = args
            .template_spec_version_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:core/tenantTemplateDeployment:TenantTemplateDeployment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "debugLevel".into(),
                    value: &debug_level_binding.drop_type(),
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
                    name: "parametersContent".into(),
                    value: &parameters_content_binding.drop_type(),
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
        TenantTemplateDeploymentResult {
            debug_level: o.get_field("debugLevel"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            output_content: o.get_field("outputContent"),
            parameters_content: o.get_field("parametersContent"),
            tags: o.get_field("tags"),
            template_content: o.get_field("templateContent"),
            template_spec_version_id: o.get_field("templateSpecVersionId"),
        }
    }
}
