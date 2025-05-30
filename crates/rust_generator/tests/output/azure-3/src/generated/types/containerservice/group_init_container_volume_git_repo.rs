#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GroupInitContainerVolumeGitRepo {
    /// Specifies the directory into which the repository should be cloned. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "directory")]
    pub r#directory: Box<Option<String>>,
    /// Specifies the commit hash of the revision to be cloned. If unspecified, the HEAD revision is cloned. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "revision")]
    pub r#revision: Box<Option<String>>,
    /// Specifies the Git repository to be cloned. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}
