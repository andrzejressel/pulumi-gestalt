#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventSourcesConfigEventSource {
    /// Stores whether DevOps Guru is configured to consume recommendations which are generated from AWS CodeGuru Profiler. See `amazon_code_guru_profiler` below.
    #[builder(into)]
    #[serde(rename = "amazonCodeGuruProfilers")]
    pub r#amazon_code_guru_profilers: Option<Vec<super::super::types::devopsguru::EventSourcesConfigEventSourceAmazonCodeGuruProfiler>>,
}
