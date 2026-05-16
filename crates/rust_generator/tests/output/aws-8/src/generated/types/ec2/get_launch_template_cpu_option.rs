#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLaunchTemplateCpuOption {
    #[builder(into)]
    #[serde(rename = "amdSevSnp")]
    pub r#amd_sev_snp: String,
    #[builder(into)]
    #[serde(rename = "coreCount")]
    pub r#core_count: i32,
    #[builder(into)]
    #[serde(rename = "threadsPerCore")]
    pub r#threads_per_core: i32,
}
