#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlexibleAppVersionDeploymentFile {
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// SHA1 checksum of the file
    #[builder(into)]
    #[serde(rename = "sha1Sum")]
    pub r#sha_1_sum: Option<String>,
    /// Source URL
    #[builder(into)]
    #[serde(rename = "sourceUrl")]
    pub r#source_url: String,
}
