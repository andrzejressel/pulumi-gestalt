#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AppImageConfigJupyterLabImageConfigFileSystemConfig {
    /// The default POSIX group ID (GID). If not specified, defaults to `100`. Valid values are `0` and `100`.
    #[builder(into)]
    #[serde(rename = "defaultGid")]
    pub r#default_gid: Option<i32>,
    /// The default POSIX user ID (UID). If not specified, defaults to `1000`. Valid values are `0` and `1000`.
    #[builder(into)]
    #[serde(rename = "defaultUid")]
    pub r#default_uid: Option<i32>,
    /// The path within the image to mount the user's EFS home directory. The directory should be empty. If not specified, defaults to `/home/sagemaker-user`.
    /// 
    /// > **Note:** When specifying `default_gid` and `default_uid`, Valid value pairs are [`0`, `0`] and [`100`, `1000`].
    #[builder(into)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: Option<String>,
}
