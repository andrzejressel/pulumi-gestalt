#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainRetentionPolicy {
    /// The retention policy for data stored on an Amazon Elastic File System (EFS) volume. Valid values are `Retain` or `Delete`.  Default value is `Retain`.
    #[builder(into, default)]
    #[serde(rename = "homeEfsFileSystem")]
    pub r#home_efs_file_system: Box<Option<String>>,
}
