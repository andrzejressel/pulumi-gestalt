#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GuestPoliciesRecipeInstallStep {
    /// Extracts an archive into the specified directory.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "archiveExtraction")]
    pub r#archive_extraction: Option<Box<super::super::types::osconfig::GuestPoliciesRecipeInstallStepArchiveExtraction>>,
    /// Installs a deb file via dpkg.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dpkgInstallation")]
    pub r#dpkg_installation: Option<Box<super::super::types::osconfig::GuestPoliciesRecipeInstallStepDpkgInstallation>>,
    /// Copies a file onto the instance.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fileCopy")]
    pub r#file_copy: Option<Box<super::super::types::osconfig::GuestPoliciesRecipeInstallStepFileCopy>>,
    /// Executes an artifact or local file.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fileExec")]
    pub r#file_exec: Option<Box<super::super::types::osconfig::GuestPoliciesRecipeInstallStepFileExec>>,
    /// Installs an MSI file.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "msiInstallation")]
    pub r#msi_installation: Option<Box<super::super::types::osconfig::GuestPoliciesRecipeInstallStepMsiInstallation>>,
    /// Installs an rpm file via the rpm utility.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rpmInstallation")]
    pub r#rpm_installation: Option<Box<super::super::types::osconfig::GuestPoliciesRecipeInstallStepRpmInstallation>>,
    /// Runs commands in a shell.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "scriptRun")]
    pub r#script_run: Option<Box<super::super::types::osconfig::GuestPoliciesRecipeInstallStepScriptRun>>,
}
