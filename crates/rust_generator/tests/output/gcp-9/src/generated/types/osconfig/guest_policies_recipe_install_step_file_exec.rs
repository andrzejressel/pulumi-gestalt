#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GuestPoliciesRecipeInstallStepFileExec {
    /// A list of possible return values that the program can return to indicate a success. Defaults to [0].
    #[builder(into)]
    #[serde(rename = "allowedExitCodes")]
    pub r#allowed_exit_codes: Option<String>,
    /// Arguments to be passed to the provided executable.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Option<Vec<String>>,
    /// The id of the relevant artifact in the recipe.
    #[builder(into)]
    #[serde(rename = "artifactId")]
    pub r#artifact_id: Option<String>,
    /// The absolute path of the file on the local filesystem.
    #[builder(into)]
    #[serde(rename = "localPath")]
    pub r#local_path: Option<String>,
}
