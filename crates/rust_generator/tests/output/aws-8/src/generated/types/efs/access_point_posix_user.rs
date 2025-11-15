#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessPointPosixUser {
    /// POSIX group ID used for all file system operations using this access point.
    #[builder(into)]
    #[serde(rename = "gid")]
    pub r#gid: i32,
    /// Secondary POSIX group IDs used for all file system operations using this access point.
    #[builder(into)]
    #[serde(rename = "secondaryGids")]
    pub r#secondary_gids: Option<Vec<i32>>,
    /// POSIX user ID used for all file system operations using this access point.
    #[builder(into)]
    #[serde(rename = "uid")]
    pub r#uid: i32,
}
