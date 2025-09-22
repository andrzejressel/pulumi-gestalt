#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobStatusErrorResult {
    /// Specifies where the error occurred, if present.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// A human-readable description of the error.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// A short error code that summarizes the error.
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: Option<String>,
}
