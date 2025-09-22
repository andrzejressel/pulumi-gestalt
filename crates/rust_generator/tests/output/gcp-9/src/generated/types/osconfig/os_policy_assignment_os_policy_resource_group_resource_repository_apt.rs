#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourceRepositoryApt {
    /// Type of archive files in this repository.
    /// Possible values are: `ARCHIVE_TYPE_UNSPECIFIED`, `DEB`, `DEB_SRC`.
    #[builder(into)]
    #[serde(rename = "archiveType")]
    pub r#archive_type: String,
    /// List of components for this repository. Must
    /// contain at least one item.
    #[builder(into)]
    #[serde(rename = "components")]
    pub r#components: Vec<String>,
    /// Distribution of this repository.
    #[builder(into)]
    #[serde(rename = "distribution")]
    pub r#distribution: String,
    /// URI of the key file for this repository. The agent
    /// maintains a keyring at `/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg`.
    #[builder(into)]
    #[serde(rename = "gpgKey")]
    pub r#gpg_key: Option<String>,
    /// URI for this repository.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
}
