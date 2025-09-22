#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourceExecValidate {
    /// Optional arguments to pass to the source during
    /// execution.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Option<Vec<String>>,
    /// A remote or local file. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "file")]
    pub r#file: Box<Option<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceExecValidateFile>>,
    /// The script interpreter to use. Possible values
    /// are: `INTERPRETER_UNSPECIFIED`, `NONE`, `SHELL`, `POWERSHELL`.
    #[builder(into)]
    #[serde(rename = "interpreter")]
    pub r#interpreter: String,
    /// Only recorded for enforce Exec. Path to an
    /// output file (that is created by this Exec) whose content will be recorded in
    /// OSPolicyResourceCompliance after a successful run. Absence or failure to
    /// read this file will result in this ExecResource being non-compliant. Output
    /// file size is limited to 100K bytes.
    #[builder(into)]
    #[serde(rename = "outputFilePath")]
    pub r#output_file_path: Option<String>,
    /// An inline script. The size of the script is limited to
    /// 1024 characters.
    #[builder(into)]
    #[serde(rename = "script")]
    pub r#script: Option<String>,
}
