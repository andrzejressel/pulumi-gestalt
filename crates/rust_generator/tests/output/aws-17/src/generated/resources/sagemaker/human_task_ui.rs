/// Provides a SageMaker Human Task UI resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:sagemaker:HumanTaskUI
///     properties:
///       humanTaskUiName: example
///       uiTemplate:
///         content:
///           fn::invoke:
///             function: std:file
///             arguments:
///               input: sagemaker-human-task-ui-template.html
///             return: result
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Human Task UIs using the `human_task_ui_name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/humanTaskUI:HumanTaskUI example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod human_task_ui {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HumanTaskUIArgs {
        /// The name of the Human Task UI.
        #[builder(into)]
        pub human_task_ui_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Liquid template for the worker user interface. See UI Template below.
        #[builder(into)]
        pub ui_template: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::sagemaker::HumanTaskUiUiTemplate,
        >,
    }
    #[allow(dead_code)]
    pub struct HumanTaskUIResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) assigned by AWS to this Human Task UI.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the Human Task UI.
        pub human_task_ui_name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Liquid template for the worker user interface. See UI Template below.
        pub ui_template: pulumi_gestalt_rust::Output<
            super::super::types::sagemaker::HumanTaskUiUiTemplate,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HumanTaskUIArgs,
    ) -> HumanTaskUIResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let human_task_ui_name_binding = args.human_task_ui_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let ui_template_binding = args.ui_template.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/humanTaskUI:HumanTaskUI".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "humanTaskUiName".into(),
                    value: &human_task_ui_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "uiTemplate".into(),
                    value: &ui_template_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        HumanTaskUIResult {
            id: o.get_field("id"),
            arn: o.get_field("arn"),
            human_task_ui_name: o.get_field("humanTaskUiName"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            ui_template: o.get_field("uiTemplate"),
        }
    }
}
