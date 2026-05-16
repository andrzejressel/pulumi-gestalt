#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CanaryTimeline {
    /// Date and time the canary was created.
    #[builder(into)]
    #[serde(rename = "created")]
    pub r#created: Option<String>,
    /// Date and time the canary was most recently modified.
    #[builder(into)]
    #[serde(rename = "lastModified")]
    pub r#last_modified: Option<String>,
    /// Date and time that the canary's most recent run started.
    #[builder(into)]
    #[serde(rename = "lastStarted")]
    pub r#last_started: Option<String>,
    /// Date and time that the canary's most recent run ended.
    #[builder(into)]
    #[serde(rename = "lastStopped")]
    pub r#last_stopped: Option<String>,
}
