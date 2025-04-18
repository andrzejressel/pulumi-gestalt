#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApiOperationResponseRepresentationFormParameter {
    /// The default value for this Form Parameter.
    #[builder(into, default)]
    #[serde(rename = "defaultValue")]
    pub r#default_value: Box<Option<String>>,
    /// A description of this Form Parameter.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// One or more `example` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "examples")]
    pub r#examples: Box<Option<Vec<super::super::types::apimanagement::ApiOperationResponseRepresentationFormParameterExample>>>,
    /// The Name of this Form Parameter.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Is this Form Parameter Required?
    #[builder(into)]
    #[serde(rename = "required")]
    pub r#required: Box<bool>,
    /// The name of the Schema.
    #[builder(into, default)]
    #[serde(rename = "schemaId")]
    pub r#schema_id: Box<Option<String>>,
    /// The Type of this Form Parameter, such as a `string`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The type name defined by the Schema.
    #[builder(into, default)]
    #[serde(rename = "typeName")]
    pub r#type_name: Box<Option<String>>,
    /// One or more acceptable values for this Form Parameter.
    #[builder(into, default)]
    #[serde(rename = "values")]
    pub r#values: Box<Option<Vec<String>>>,
}
