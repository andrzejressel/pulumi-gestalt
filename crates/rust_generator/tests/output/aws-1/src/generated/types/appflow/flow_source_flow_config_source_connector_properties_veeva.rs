#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowSourceFlowConfigSourceConnectorPropertiesVeeva {
    /// Document type specified in the Veeva document extract flow.
    #[builder(into)]
    #[serde(rename = "documentType")]
    pub r#document_type: Option<String>,
    /// Boolean value to include All Versions of files in Veeva document extract flow.
    #[builder(into)]
    #[serde(rename = "includeAllVersions")]
    pub r#include_all_versions: Option<bool>,
    /// Boolean value to include file renditions in Veeva document extract flow.
    #[builder(into)]
    #[serde(rename = "includeRenditions")]
    pub r#include_renditions: Option<bool>,
    /// Boolean value to include source files in Veeva document extract flow.
    #[builder(into)]
    #[serde(rename = "includeSourceFiles")]
    pub r#include_source_files: Option<bool>,
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: String,
}
