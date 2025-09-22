#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTaskSpecContainerSpecSecret {
    /// Represents the file GID. Defaults to `0`
    #[builder(into)]
    #[serde(rename = "fileGid")]
    pub r#file_gid: Option<String>,
    /// Represents represents the FileMode of the file. Defaults to `0o444`
    #[builder(into)]
    #[serde(rename = "fileMode")]
    pub r#file_mode: Option<i32>,
    /// Represents the final filename in the filesystem
    #[builder(into)]
    #[serde(rename = "fileName")]
    pub r#file_name: String,
    /// Represents the file UID. Defaults to `0`
    #[builder(into)]
    #[serde(rename = "fileUid")]
    pub r#file_uid: Option<String>,
    /// ID of the specific secret that we're referencing
    #[builder(into)]
    #[serde(rename = "secretId")]
    pub r#secret_id: String,
    /// Name of the secret that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID
    #[builder(into)]
    #[serde(rename = "secretName")]
    pub r#secret_name: Option<String>,
}
