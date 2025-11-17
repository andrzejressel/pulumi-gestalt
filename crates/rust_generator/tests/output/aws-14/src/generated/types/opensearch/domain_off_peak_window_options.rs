#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainOffPeakWindowOptions {
    /// Enabled disabled toggle for off-peak update window.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "offPeakWindow")]
    pub r#off_peak_window: Option<Box<super::super::types::opensearch::DomainOffPeakWindowOptionsOffPeakWindow>>,
}
