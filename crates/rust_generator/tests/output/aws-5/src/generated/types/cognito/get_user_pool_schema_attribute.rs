#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetUserPoolSchemaAttribute {
    /// - Data type of the attribute (e.g., string, number).
    #[builder(into)]
    #[serde(rename = "attributeDataType")]
    pub r#attribute_data_type: String,
    /// - Whether the attribute is for developer use only.
    #[builder(into)]
    #[serde(rename = "developerOnlyAttribute")]
    pub r#developer_only_attribute: bool,
    /// - Whether the attribute can be changed after user creation.
    #[builder(into)]
    #[serde(rename = "mutable")]
    pub r#mutable: bool,
    /// - Name of the attribute.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "numberAttributeConstraints")]
    pub r#number_attribute_constraints: Vec<super::super::types::cognito::GetUserPoolSchemaAttributeNumberAttributeConstraint>,
    /// - Whether the attribute is required during user registration.
    /// * number_attribute_constraints - Constraints for numeric attributes.
    /// * string_attribute_constraints - Constraints for string attributes.
    #[builder(into)]
    #[serde(rename = "required")]
    pub r#required: bool,
    #[builder(into)]
    #[serde(rename = "stringAttributeConstraints")]
    pub r#string_attribute_constraints: Vec<super::super::types::cognito::GetUserPoolSchemaAttributeStringAttributeConstraint>,
}
