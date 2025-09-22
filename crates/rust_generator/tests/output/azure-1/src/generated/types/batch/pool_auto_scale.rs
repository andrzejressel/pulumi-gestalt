#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PoolAutoScale {
    /// The interval to wait before evaluating if the pool needs to be scaled. Defaults to `PT15M`.
    #[builder(into)]
    #[serde(rename = "evaluationInterval")]
    pub r#evaluation_interval: Option<String>,
    /// The autoscale formula that needs to be used for scaling the Batch pool.
    #[builder(into)]
    #[serde(rename = "formula")]
    pub r#formula: String,
}
