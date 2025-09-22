#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceCustomDocumentEnrichmentConfigurationInlineConfiguration {
    /// Configuration of the condition used for the target document attribute or metadata field when ingesting documents into Amazon Kendra. See condition.
    #[builder(into)]
    #[serde(rename = "condition")]
    pub r#condition: Option<Box<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationCondition>>,
    /// `TRUE` to delete content if the condition used for the target attribute is met.
    #[builder(into)]
    #[serde(rename = "documentContentDeletion")]
    pub r#document_content_deletion: Option<bool>,
    /// Configuration of the target document attribute or metadata field when ingesting documents into Amazon Kendra. You can also include a value. Detailed below.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Option<Box<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationTarget>>,
}
