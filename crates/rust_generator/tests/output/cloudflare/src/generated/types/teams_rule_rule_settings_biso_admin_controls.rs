#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TeamsRuleRuleSettingsBisoAdminControls {
    /// Disable clipboard redirection.
    #[builder(into)]
    #[serde(rename = "disableClipboardRedirection")]
    pub r#disable_clipboard_redirection: Option<bool>,
    /// Disable copy-paste.
    #[builder(into)]
    #[serde(rename = "disableCopyPaste")]
    pub r#disable_copy_paste: Option<bool>,
    /// Disable download.
    #[builder(into)]
    #[serde(rename = "disableDownload")]
    pub r#disable_download: Option<bool>,
    /// Disable keyboard usage.
    #[builder(into)]
    #[serde(rename = "disableKeyboard")]
    pub r#disable_keyboard: Option<bool>,
    /// Disable printing.
    #[builder(into)]
    #[serde(rename = "disablePrinting")]
    pub r#disable_printing: Option<bool>,
    /// Disable upload.
    #[builder(into)]
    #[serde(rename = "disableUpload")]
    pub r#disable_upload: Option<bool>,
}
