#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AssessmentStatus {
    /// Specifies the cause of the assessment status.
    #[builder(into)]
    #[serde(rename = "cause")]
    pub r#cause: Option<String>,
    /// Specifies the programmatic code of the assessment status. Possible values are `Healthy`, `Unhealthy` and `NotApplicable`.
    #[builder(into)]
    #[serde(rename = "code")]
    pub r#code: String,
    /// Specifies the human readable description of the assessment status.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
}
