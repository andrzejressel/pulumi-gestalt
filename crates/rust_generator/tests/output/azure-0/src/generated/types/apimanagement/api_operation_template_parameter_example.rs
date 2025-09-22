#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApiOperationTemplateParameterExample {
    /// A long description for this example.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// A URL that points to the literal example.
    #[builder(into)]
    #[serde(rename = "externalValue")]
    pub r#external_value: Option<String>,
    /// The name of this example.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A short description for this example.
    #[builder(into)]
    #[serde(rename = "summary")]
    pub r#summary: Option<String>,
    /// The example of the representation.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
