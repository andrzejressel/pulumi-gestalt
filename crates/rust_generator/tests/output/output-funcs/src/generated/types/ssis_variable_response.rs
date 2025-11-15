#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SsisVariableResponse {
    /// Variable type.
    #[builder(into)]
    #[serde(rename = "dataType")]
    pub r#data_type: Option<String>,
    /// Variable description.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Variable id.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<f64>,
    /// Variable name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Whether variable is sensitive.
    #[builder(into)]
    #[serde(rename = "sensitive")]
    pub r#sensitive: Option<bool>,
    /// Variable sensitive value.
    #[builder(into)]
    #[serde(rename = "sensitiveValue")]
    pub r#sensitive_value: Option<String>,
    /// Variable value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
