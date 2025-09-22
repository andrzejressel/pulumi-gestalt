#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PatchDeploymentPatchConfigPreStep {
    /// The ExecStepConfig for all Linux VMs targeted by the PatchJob.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "linuxExecStepConfig")]
    pub r#linux_exec_step_config: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigPreStepLinuxExecStepConfig>>,
    /// The ExecStepConfig for all Windows VMs targeted by the PatchJob.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "windowsExecStepConfig")]
    pub r#windows_exec_step_config: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigPreStepWindowsExecStepConfig>>,
}
