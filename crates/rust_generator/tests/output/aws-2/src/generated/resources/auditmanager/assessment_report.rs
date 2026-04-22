/// Resource for managing an AWS Audit Manager Assessment Report.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = assessment_report::create(
///         "test",
///         AssessmentReportArgs::builder()
///             .assessment_id("${testAwsAuditmanagerAssessment.id}")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Audit Manager Assessment Reports using the assessment report `id`. For example:
///
/// ```sh
/// $ pulumi import aws:auditmanager/assessmentReport:AssessmentReport example abc123-de45
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod assessment_report {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssessmentReportArgs {
        /// Unique identifier of the assessment to create the report from.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub assessment_id: pulumi_gestalt_rust::Input<String>,
        /// Description of the assessment report.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// Name of the assessment report.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AssessmentReportResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier of the assessment to create the report from.
        ///
        /// The following arguments are optional:
        pub assessment_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the user who created the assessment report.
        pub author: pulumi_gestalt_rust::Output<String>,
        /// Description of the assessment report.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the assessment report.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Current status of the specified assessment report. Valid values are `COMPLETE`, `IN_PROGRESS`, and `FAILED`.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AssessmentReportArgs,
    ) -> AssessmentReportResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AssessmentReportArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> AssessmentReportResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AssessmentReportArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> AssessmentReportResult {
        let assessment_id_binding = args.assessment_id.get_output(ctx);
        let description_binding = args.description.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:auditmanager/assessmentReport:AssessmentReport".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assessmentId".into(),
                    value: &assessment_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        AssessmentReportResult {
            id: o.get_id(),
            urn: o.get_urn(),
            assessment_id: o.get_field("assessmentId"),
            author: o.get_field("author"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            status: o.get_field("status"),
        }
    }
}
