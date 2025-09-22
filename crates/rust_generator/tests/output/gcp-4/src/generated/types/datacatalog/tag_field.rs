#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TagField {
    /// Holds the value for a tag field with boolean type.
    #[builder(into)]
    #[serde(rename = "boolValue")]
    pub r#bool_value: Option<bool>,
    /// (Output)
    /// The display name of this field
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Option<String>,
    /// Holds the value for a tag field with double type.
    #[builder(into)]
    #[serde(rename = "doubleValue")]
    pub r#double_value: Option<f64>,
    /// Holds the value for a tag field with enum type. This value must be one of the allowed values in the definition of this enum.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "enumValue")]
    pub r#enum_value: Option<String>,
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "fieldName")]
    pub r#field_name: String,
    /// (Output)
    /// The order of this field with respect to other fields in this tag. For example, a higher value can indicate
    /// a more important field. The value can be negative. Multiple fields can have the same order, and field orders
    /// within a tag do not have to be sequential.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Option<i32>,
    /// Holds the value for a tag field with string type.
    #[builder(into)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Option<String>,
    /// Holds the value for a tag field with timestamp type.
    #[builder(into)]
    #[serde(rename = "timestampValue")]
    pub r#timestamp_value: Option<String>,
}
