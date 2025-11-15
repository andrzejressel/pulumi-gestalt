#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourceExecValidateFileRemote {
    /// SHA256 checksum of the remote file.
    #[builder(into)]
    #[serde(rename = "sha256Checksum")]
    pub r#sha_256_checksum: Option<String>,
    /// URI from which to fetch the object. It should contain
    /// both the protocol and path following the format `{protocol}://{location}`.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
}
