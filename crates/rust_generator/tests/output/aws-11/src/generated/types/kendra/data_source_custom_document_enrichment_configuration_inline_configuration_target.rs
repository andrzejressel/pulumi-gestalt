#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationTarget {
    /// The identifier of the target document attribute or metadata field. For example, 'Department' could be an identifier for the target attribute or metadata field that includes the department names associated with the documents.
    #[builder(into)]
    #[serde(rename = "targetDocumentAttributeKey")]
    pub r#target_document_attribute_key: Option<String>,
    /// The target value you want to create for the target attribute. For example, 'Finance' could be the target value for the target attribute key 'Department'. See target_document_attribute_value.
    #[builder(into)]
    #[serde(rename = "targetDocumentAttributeValue")]
    pub r#target_document_attribute_value: Option<Box<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationTargetTargetDocumentAttributeValue>>,
    /// `TRUE` to delete the existing target value for your specified target attribute key. You cannot create a target value and set this to `TRUE`. To create a target value (`TargetDocumentAttributeValue`), set this to `FALSE`.
    #[builder(into)]
    #[serde(rename = "targetDocumentAttributeValueDeletion")]
    pub r#target_document_attribute_value_deletion: Option<bool>,
}
