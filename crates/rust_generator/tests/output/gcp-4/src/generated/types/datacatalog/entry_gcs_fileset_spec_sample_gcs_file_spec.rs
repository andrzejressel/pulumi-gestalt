#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EntryGcsFilesetSpecSampleGcsFileSpec {
    /// The full file path
    #[builder(into)]
    #[serde(rename = "filePath")]
    pub r#file_path: Option<String>,
    /// The size of the file, in bytes.
    #[builder(into)]
    #[serde(rename = "sizeBytes")]
    pub r#size_bytes: Option<i32>,
}
