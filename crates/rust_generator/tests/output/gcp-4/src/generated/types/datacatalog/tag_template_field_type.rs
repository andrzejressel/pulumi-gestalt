#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TagTemplateFieldType {
    /// Represents an enum type.
    /// Exactly one of `primitive_type` or `enum_type` must be set
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "enumType")]
    pub r#enum_type: Option<Box<super::super::types::datacatalog::TagTemplateFieldTypeEnumType>>,
    /// Represents primitive types - string, bool etc.
    /// Exactly one of `primitive_type` or `enum_type` must be set
    /// Possible values are: `DOUBLE`, `STRING`, `BOOL`, `TIMESTAMP`.
    #[builder(into)]
    #[serde(rename = "primitiveType")]
    pub r#primitive_type: Option<String>,
}
