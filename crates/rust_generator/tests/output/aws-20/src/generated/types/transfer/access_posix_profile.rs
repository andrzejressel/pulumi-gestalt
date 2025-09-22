#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccessPosixProfile {
    /// The POSIX group ID used for all EFS operations by this user.
    #[builder(into)]
    #[serde(rename = "gid")]
    pub r#gid: i32,
    /// The secondary POSIX group IDs used for all EFS operations by this user.
    #[builder(into)]
    #[serde(rename = "secondaryGids")]
    pub r#secondary_gids: Option<Vec<i32>>,
    /// The POSIX user ID used for all EFS operations by this user.
    #[builder(into)]
    #[serde(rename = "uid")]
    pub r#uid: i32,
}
