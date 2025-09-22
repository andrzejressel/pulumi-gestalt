#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FolderCustomModuleCustomConfigCustomOutput {
    /// A list of custom output properties to add to the finding.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<Vec<super::super::types::securitycenter::FolderCustomModuleCustomConfigCustomOutputProperty>>,
}
