#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BatchRuntimeConfig {
    /// Optional. Autotuning configuration of the workload.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "autotuningConfig")]
    pub r#autotuning_config: Box<Option<super::super::types::dataproc::BatchRuntimeConfigAutotuningConfig>>,
    /// Optional. Cohort identifier. Identifies families of the workloads having the same shape, e.g. daily ETL jobs.
    #[builder(into)]
    #[serde(rename = "cohort")]
    pub r#cohort: Option<String>,
    /// Optional custom container image for the job runtime environment. If not specified, a default container image will be used.
    #[builder(into)]
    #[serde(rename = "containerImage")]
    pub r#container_image: Option<String>,
    /// (Output)
    /// A mapping of property names to values, which are used to configure workload execution.
    #[builder(into)]
    #[serde(rename = "effectiveProperties")]
    pub r#effective_properties: Option<std::collections::HashMap<String, String>>,
    /// A mapping of property names to values, which are used to configure workload execution.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
    /// Version of the batch runtime.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
