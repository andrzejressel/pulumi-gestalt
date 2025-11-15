#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineStorageConfigurationDataSettings {
    #[builder(into)]
    #[serde(rename = "defaultFilePath")]
    pub r#default_file_path: String,
    #[builder(into)]
    #[serde(rename = "luns")]
    pub r#luns: Vec<i32>,
}
