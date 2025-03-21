/// Provides an Inspector Classic Assessment Target
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   bar:
///     type: aws:inspector:ResourceGroup
///     properties:
///       tags:
///         Name: foo
///         Env: bar
///   foo:
///     type: aws:inspector:AssessmentTarget
///     properties:
///       name: assessment target
///       resourceGroupArn: ${bar.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Inspector Classic Assessment Targets using their Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:inspector/assessmentTarget:AssessmentTarget example arn:aws:inspector:us-east-1:123456789012:target/0-xxxxxxx
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod assessment_target {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssessmentTargetArgs {
        /// The name of the assessment target.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Inspector Resource Group Amazon Resource Name (ARN) stating tags for instance matching. If not specified, all EC2 instances in the current AWS account and region are included in the assessment target.
        #[builder(into, default)]
        pub resource_group_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AssessmentTargetResult {
        /// The target assessment ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the assessment target.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Inspector Resource Group Amazon Resource Name (ARN) stating tags for instance matching. If not specified, all EC2 instances in the current AWS account and region are included in the assessment target.
        pub resource_group_arn: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AssessmentTargetArgs,
    ) -> AssessmentTargetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_arn_binding = args.resource_group_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:inspector/assessmentTarget:AssessmentTarget".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupArn".into(),
                    value: &resource_group_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AssessmentTargetResult {
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            resource_group_arn: o.get_field("resourceGroupArn"),
        }
    }
}
