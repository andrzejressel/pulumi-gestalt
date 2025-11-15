#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GuestPoliciesRecipeArtifact {
    /// Defaults to false. When false, recipes are subject to validations based on the artifact type:
    /// Remote: A checksum must be specified, and only protocols with transport-layer security are permitted.
    /// GCS: An object generation number must be specified.
    #[builder(into)]
    #[serde(rename = "allowInsecure")]
    pub r#allow_insecure: Option<bool>,
    /// A Google Cloud Storage artifact.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "gcs")]
    pub r#gcs: Option<Box<super::super::types::osconfig::GuestPoliciesRecipeArtifactGcs>>,
    /// Id of the artifact, which the installation and update steps of this recipe can reference.
    /// Artifacts in a recipe cannot have the same id.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// A generic remote artifact.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "remote")]
    pub r#remote: Option<Box<super::super::types::osconfig::GuestPoliciesRecipeArtifactRemote>>,
}
