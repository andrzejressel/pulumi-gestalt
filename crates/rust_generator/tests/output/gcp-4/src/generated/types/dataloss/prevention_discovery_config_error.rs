#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigError {
    /// A list of messages that carry the error details.
    #[builder(into)]
    #[serde(rename = "details")]
    pub r#details: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigErrorDetails>>,
    /// The times the error occurred. List includes the oldest timestamp and the last 9 timestamps.
    #[builder(into)]
    #[serde(rename = "timestamp")]
    pub r#timestamp: Option<String>,
}
