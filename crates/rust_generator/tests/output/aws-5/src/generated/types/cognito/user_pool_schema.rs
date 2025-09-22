#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserPoolSchema {
    /// Attribute data type. Must be one of `Boolean`, `Number`, `String`, `DateTime`.
    #[builder(into)]
    #[serde(rename = "attributeDataType")]
    pub r#attribute_data_type: String,
    /// Whether the attribute type is developer only.
    #[builder(into)]
    #[serde(rename = "developerOnlyAttribute")]
    pub r#developer_only_attribute: Option<bool>,
    /// Whether the attribute can be changed once it has been created.
    #[builder(into)]
    #[serde(rename = "mutable")]
    pub r#mutable: Option<bool>,
    /// Name of the attribute.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Configuration block for the constraints for an attribute of the number type. Detailed below.
    #[builder(into)]
    #[serde(rename = "numberAttributeConstraints")]
    pub r#number_attribute_constraints: Box<Option<super::super::types::cognito::UserPoolSchemaNumberAttributeConstraints>>,
    /// Whether a user pool attribute is required. If the attribute is required and the user does not provide a value, registration or sign-in will fail.
    #[builder(into)]
    #[serde(rename = "required")]
    pub r#required: Option<bool>,
    /// Constraints for an attribute of the string type. Detailed below.
    #[builder(into)]
    #[serde(rename = "stringAttributeConstraints")]
    pub r#string_attribute_constraints: Box<Option<super::super::types::cognito::UserPoolSchemaStringAttributeConstraints>>,
}
