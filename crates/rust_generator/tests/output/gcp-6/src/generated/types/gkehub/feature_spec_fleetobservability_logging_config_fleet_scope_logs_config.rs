#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureSpecFleetobservabilityLoggingConfigFleetScopeLogsConfig {
    /// Specified if fleet logging feature is enabled.
    /// Possible values are: `MODE_UNSPECIFIED`, `COPY`, `MOVE`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
}
