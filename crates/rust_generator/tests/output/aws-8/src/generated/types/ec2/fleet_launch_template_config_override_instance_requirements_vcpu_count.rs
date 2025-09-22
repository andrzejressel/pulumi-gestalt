#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FleetLaunchTemplateConfigOverrideInstanceRequirementsVcpuCount {
    /// The maximum number of vCPUs. To specify no maximum limit, omit this parameter.
    #[builder(into)]
    #[serde(rename = "max")]
    pub r#max: Option<i32>,
    /// The minimum number of vCPUs. To specify no minimum limit, specify `0`.
    #[builder(into)]
    #[serde(rename = "min")]
    pub r#min: i32,
}
