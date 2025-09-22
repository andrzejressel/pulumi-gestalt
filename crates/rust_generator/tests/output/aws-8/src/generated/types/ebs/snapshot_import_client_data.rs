#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SnapshotImportClientData {
    /// A user-defined comment about the disk upload.
    #[builder(into)]
    #[serde(rename = "comment")]
    pub r#comment: Option<String>,
    /// The time that the disk upload ends.
    #[builder(into)]
    #[serde(rename = "uploadEnd")]
    pub r#upload_end: Option<String>,
    /// The size of the uploaded disk image, in GiB.
    #[builder(into)]
    #[serde(rename = "uploadSize")]
    pub r#upload_size: Option<f64>,
    /// The time that the disk upload starts.
    #[builder(into)]
    #[serde(rename = "uploadStart")]
    pub r#upload_start: Option<String>,
}
