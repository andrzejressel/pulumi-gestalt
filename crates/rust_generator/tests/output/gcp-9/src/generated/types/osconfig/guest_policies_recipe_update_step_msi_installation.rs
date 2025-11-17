#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GuestPoliciesRecipeUpdateStepMsiInstallation {
    /// Return codes that indicate that the software installed or updated successfully. Behaviour defaults to [0]
    #[builder(into)]
    #[serde(rename = "allowedExitCodes")]
    pub r#allowed_exit_codes: Option<Vec<i32>>,
    /// The id of the relevant artifact in the recipe.
    #[builder(into)]
    #[serde(rename = "artifactId")]
    pub r#artifact_id: String,
    /// The flags to use when installing the MSI. Defaults to the install flag.
    #[builder(into)]
    #[serde(rename = "flags")]
    pub r#flags: Option<Vec<String>>,
}
