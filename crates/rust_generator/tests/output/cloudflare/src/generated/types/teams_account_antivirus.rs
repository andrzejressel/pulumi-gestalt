#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TeamsAccountAntivirus {
    /// Scan on file download.
    #[builder(into)]
    #[serde(rename = "enabledDownloadPhase")]
    pub r#enabled_download_phase: bool,
    /// Scan on file upload.
    #[builder(into)]
    #[serde(rename = "enabledUploadPhase")]
    pub r#enabled_upload_phase: bool,
    /// Block requests for files that cannot be scanned.
    #[builder(into)]
    #[serde(rename = "failClosed")]
    pub r#fail_closed: bool,
    /// Set notifications for antivirus.
    #[builder(into)]
    #[serde(rename = "notificationSettings")]
    pub r#notification_settings: Option<Box<super::types::TeamsAccountAntivirusNotificationSettings>>,
}
