#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccessPointRootDirectoryCreationInfo {
    /// POSIX group ID to apply to the `root_directory`.
    #[builder(into)]
    #[serde(rename = "ownerGid")]
    pub r#owner_gid: i32,
    /// POSIX user ID to apply to the `root_directory`.
    #[builder(into)]
    #[serde(rename = "ownerUid")]
    pub r#owner_uid: i32,
    /// POSIX permissions to apply to the RootDirectory, in the format of an octal number representing the file's mode bits.
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: String,
}
