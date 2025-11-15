#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApiOperationRequestRepresentationFormParameter {
    /// The default value for this Form Parameter.
    #[builder(into)]
    #[serde(rename = "defaultValue")]
    pub r#default_value: Option<String>,
    /// A description of this Form Parameter.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// One or more `example` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "examples")]
    pub r#examples: Option<Vec<super::super::types::apimanagement::ApiOperationRequestRepresentationFormParameterExample>>,
    /// The Name of this Form Parameter.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Is this Form Parameter Required?
    #[builder(into)]
    #[serde(rename = "required")]
    pub r#required: bool,
    /// The name of the Schema.
    #[builder(into)]
    #[serde(rename = "schemaId")]
    pub r#schema_id: Option<String>,
    /// The Type of this Form Parameter, such as a `string`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// The type name defined by the Schema.
    #[builder(into)]
    #[serde(rename = "typeName")]
    pub r#type_name: Option<String>,
    /// One or more acceptable values for this Form Parameter.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Option<Vec<String>>,
}
