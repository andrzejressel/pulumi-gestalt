#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSetting {
    /// Parameters that control the interactions with the CDN. See Archive CDN Settings for more details.
    #[builder(into)]
    #[serde(rename = "archiveCdnSettings")]
    pub r#archive_cdn_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArchiveCdnSettings>>,
    /// A director and base filename where archive files should be written. See Destination for more details.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingDestination>,
    /// Number of seconds to write to archive file before closing and starting a new one.
    #[builder(into)]
    #[serde(rename = "rolloverInterval")]
    pub r#rollover_interval: Option<i32>,
}
