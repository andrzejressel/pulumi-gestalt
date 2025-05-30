#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_framework {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFrameworkArgs {
        #[builder(into, default)]
        pub control_sets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::auditmanager::GetFrameworkControlSet>>,
        >,
        #[builder(into)]
        pub framework_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the framework.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFrameworkResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub compliance_type: pulumi_gestalt_rust::Output<String>,
        pub control_sets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::auditmanager::GetFrameworkControlSet>>,
        >,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub framework_type: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFrameworkArgs,
    ) -> GetFrameworkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let control_sets_binding = args.control_sets.get_output(context);
        let framework_type_binding = args.framework_type.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:auditmanager/getFramework:getFramework".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "controlSets".into(),
                    value: &control_sets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frameworkType".into(),
                    value: &framework_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFrameworkResult {
            arn: o.get_field("arn"),
            compliance_type: o.get_field("complianceType"),
            control_sets: o.get_field("controlSets"),
            description: o.get_field("description"),
            framework_type: o.get_field("frameworkType"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
        }
    }
}
