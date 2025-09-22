#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BatchRuntimeConfigAutotuningConfig {
    /// Optional. Scenarios for which tunings are applied.
    /// Each value may be one of: `SCALING`, `BROADCAST_HASH_JOIN`, `MEMORY`.
    #[builder(into)]
    #[serde(rename = "scenarios")]
    pub r#scenarios: Option<Vec<String>>,
}
