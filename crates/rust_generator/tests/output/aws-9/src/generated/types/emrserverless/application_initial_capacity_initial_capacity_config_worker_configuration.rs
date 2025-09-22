#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
