#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAccessPointRootDirectoryCreationInfo {
    /// POSIX owner group ID
    #[builder(into)]
    #[serde(rename = "ownerGid")]
    pub r#owner_gid: i32,
    /// POSIX owner user ID
    #[builder(into)]
    #[serde(rename = "ownerUid")]
    pub r#owner_uid: i32,
    /// POSIX permissions mode
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: String,
}
