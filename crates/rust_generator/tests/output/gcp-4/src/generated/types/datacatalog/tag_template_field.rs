#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TagTemplateField {
    /// A description for this field.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The display name for this field.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Option<String>,
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "fieldId")]
    pub r#field_id: String,
    /// Whether this is a required field. Defaults to false.
    #[builder(into)]
    #[serde(rename = "isRequired")]
    pub r#is_required: Option<bool>,
    /// (Output)
    /// The resource name of the tag template field in URL format. Example: projects/{project_id}/locations/{location}/tagTemplates/{tagTemplateId}/fields/{field}
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The order of this field with respect to other fields in this tag template.
    /// A higher value indicates a more important field. The value can be negative.
    /// Multiple fields can have the same order, and field orders within a tag do not have to be sequential.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Option<i32>,
    /// The type of value this tag field can contain.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<super::super::types::datacatalog::TagTemplateFieldType>,
}
