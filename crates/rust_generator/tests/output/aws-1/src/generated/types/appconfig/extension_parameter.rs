#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ExtensionParameter {
    /// Information about the parameter.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The parameter name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Determines if a parameter value must be specified in the extension association.
    #[builder(into)]
    #[serde(rename = "required")]
    pub r#required: Option<bool>,
}
