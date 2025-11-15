#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuntimeMetric {
    /// (Output)
    /// Contains runtime daemon metrics, such as OS and kernels and
    /// sessions stats.
    #[builder(into)]
    #[serde(rename = "systemMetrics")]
    pub r#system_metrics: Option<std::collections::HashMap<String, String>>,
}
