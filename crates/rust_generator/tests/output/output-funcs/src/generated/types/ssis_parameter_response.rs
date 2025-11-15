#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SsisParameterResponse {
    /// Parameter type.
    #[builder(into)]
    #[serde(rename = "dataType")]
    pub r#data_type: Option<String>,
    /// Default value of parameter.
    #[builder(into)]
    #[serde(rename = "defaultValue")]
    pub r#default_value: Option<String>,
    /// Parameter description.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Design default value of parameter.
    #[builder(into)]
    #[serde(rename = "designDefaultValue")]
    pub r#design_default_value: Option<String>,
    /// Parameter id.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<f64>,
    /// Parameter name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Whether parameter is required.
    #[builder(into)]
    #[serde(rename = "required")]
    pub r#required: Option<bool>,
    /// Whether parameter is sensitive.
    #[builder(into)]
    #[serde(rename = "sensitive")]
    pub r#sensitive: Option<bool>,
    /// Default sensitive value of parameter.
    #[builder(into)]
    #[serde(rename = "sensitiveDefaultValue")]
    pub r#sensitive_default_value: Option<String>,
    /// Parameter value set.
    #[builder(into)]
    #[serde(rename = "valueSet")]
    pub r#value_set: Option<bool>,
    /// Parameter value type.
    #[builder(into)]
    #[serde(rename = "valueType")]
    pub r#value_type: Option<String>,
    /// Parameter reference variable.
    #[builder(into)]
    #[serde(rename = "variable")]
    pub r#variable: Option<String>,
}
