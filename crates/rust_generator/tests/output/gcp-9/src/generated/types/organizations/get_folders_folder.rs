#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFoldersFolder {
    /// The timestamp of when the folder was created
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: String,
    /// The timestamp of when the folder was requested to be deleted (if applicable)
    #[builder(into)]
    #[serde(rename = "deleteTime")]
    pub r#delete_time: String,
    /// The display name of the folder
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: String,
    /// Entity tag identifier of the folder
    #[builder(into)]
    #[serde(rename = "etag")]
    pub r#etag: String,
    /// The id of the folder
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The parent id of the folder
    #[builder(into)]
    #[serde(rename = "parent")]
    pub r#parent: String,
    /// The lifecycle state of the folder
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: String,
    /// The timestamp of when the folder was last modified
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: String,
}
