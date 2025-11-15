#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PatchDeploymentPatchConfigPostStepLinuxExecStepConfig {
    /// Defaults to [0]. A list of possible return values that the execution can return to indicate a success.
    #[builder(into)]
    #[serde(rename = "allowedSuccessCodes")]
    pub r#allowed_success_codes: Option<Vec<i32>>,
    /// A Cloud Storage object containing the executable.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "gcsObject")]
    pub r#gcs_object: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigPostStepLinuxExecStepConfigGcsObject>>,
    /// The script interpreter to use to run the script. If no interpreter is specified the script will
    /// be executed directly, which will likely only succeed for scripts with shebang lines.
    /// Possible values are: `SHELL`, `POWERSHELL`.
    #[builder(into)]
    #[serde(rename = "interpreter")]
    pub r#interpreter: Option<String>,
    /// An absolute path to the executable on the VM.
    #[builder(into)]
    #[serde(rename = "localPath")]
    pub r#local_path: Option<String>,
}
