#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
