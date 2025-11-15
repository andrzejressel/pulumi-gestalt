#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatasetDelimitedTextAzureBlobFsLocation {
    /// Is the `file_system` using dynamic expression, function or system variables? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "dynamicFileSystemEnabled")]
    pub r#dynamic_file_system_enabled: Option<bool>,
    /// Is the `filename` using dynamic expression, function or system variables? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "dynamicFilenameEnabled")]
    pub r#dynamic_filename_enabled: Option<bool>,
    /// Is the `path` using dynamic expression, function or system variables? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "dynamicPathEnabled")]
    pub r#dynamic_path_enabled: Option<bool>,
    /// The storage data lake gen2 file system on the Azure Blob Storage Account hosting the file.
    #[builder(into)]
    #[serde(rename = "fileSystem")]
    pub r#file_system: Option<String>,
    /// The filename of the file.
    #[builder(into)]
    #[serde(rename = "filename")]
    pub r#filename: Option<String>,
    /// The folder path to the file.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
}
