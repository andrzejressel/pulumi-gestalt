#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConformancePackInputParameter {
    /// The input key.
    #[builder(into)]
    #[serde(rename = "parameterName")]
    pub r#parameter_name: String,
    /// The input value.
    #[builder(into)]
    #[serde(rename = "parameterValue")]
    pub r#parameter_value: String,
}
