#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetLaunchTemplateConfigOverrideInstanceRequirementsAcceleratorTotalMemoryMib {
    /// The maximum amount of accelerator memory, in MiB. To specify no maximum limit, omit this parameter.
    #[builder(into)]
    #[serde(rename = "max")]
    pub r#max: Option<i32>,
    /// The minimum amount of accelerator memory, in MiB. To specify no minimum limit, omit this parameter.
    #[builder(into)]
    #[serde(rename = "min")]
    pub r#min: Option<i32>,
}
