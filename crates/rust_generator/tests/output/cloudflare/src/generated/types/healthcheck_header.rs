#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HealthcheckHeader {
    /// The header name.
    #[builder(into)]
    #[serde(rename = "header")]
    pub r#header: String,
    /// A list of string values for the header.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}
