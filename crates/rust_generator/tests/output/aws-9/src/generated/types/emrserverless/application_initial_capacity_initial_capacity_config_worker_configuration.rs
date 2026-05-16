#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationInitialCapacityInitialCapacityConfigWorkerConfiguration {
    /// The CPU requirements for every worker instance of the worker type.
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: String,
    /// The disk requirements for every worker instance of the worker type.
    #[builder(into)]
    #[serde(rename = "disk")]
    pub r#disk: Option<String>,
    /// The memory requirements for every worker instance of the worker type.
    #[builder(into)]
    #[serde(rename = "memory")]
    pub r#memory: String,
}
