#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PatchDeploymentPatchConfigZypper {
    /// Install only patches with these categories. Common categories include security, recommended, and feature.
    #[builder(into)]
    #[serde(rename = "categories")]
    pub r#categories: Option<Vec<String>>,
    /// List of packages to exclude from update.
    #[builder(into)]
    #[serde(rename = "excludes")]
    pub r#excludes: Option<Vec<String>>,
    /// An exclusive list of patches to be updated. These are the only patches that will be installed using 'zypper patch patch:' command.
    /// This field must not be used with any other patch configuration fields.
    #[builder(into)]
    #[serde(rename = "exclusivePatches")]
    pub r#exclusive_patches: Option<Vec<String>>,
    /// Install only patches with these severities. Common severities include critical, important, moderate, and low.
    #[builder(into)]
    #[serde(rename = "severities")]
    pub r#severities: Option<Vec<String>>,
    /// Adds the --with-optional flag to zypper patch.
    #[builder(into)]
    #[serde(rename = "withOptional")]
    pub r#with_optional: Option<bool>,
    /// Adds the --with-update flag, to zypper patch.
    #[builder(into)]
    #[serde(rename = "withUpdate")]
    pub r#with_update: Option<bool>,
}
