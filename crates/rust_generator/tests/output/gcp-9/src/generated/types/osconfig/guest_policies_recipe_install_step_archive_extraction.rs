#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GuestPoliciesRecipeInstallStepArchiveExtraction {
    /// The id of the relevant artifact in the recipe.
    #[builder(into)]
    #[serde(rename = "artifactId")]
    pub r#artifact_id: String,
    /// Directory to extract archive to. Defaults to / on Linux or C:\ on Windows.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Option<String>,
    /// The type of the archive to extract.
    /// Possible values are: `TAR`, `TAR_GZIP`, `TAR_BZIP`, `TAR_LZMA`, `TAR_XZ`, `ZIP`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
