#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettings {
    /// Used to insert watermarks of type Nielsen CBET. See Nielsen CBET Settings for more details.
    #[builder(into)]
    #[serde(rename = "nielsenCbetSettings")]
    pub r#nielsen_cbet_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenCbetSettings>>,
    /// Distribution types to assign to the watermarks. Options are `PROGRAM_CONTENT` and `FINAL_DISTRIBUTOR`.
    #[builder(into)]
    #[serde(rename = "nielsenDistributionType")]
    pub r#nielsen_distribution_type: Option<String>,
    /// Used to insert watermarks of type Nielsen NAES, II (N2) and Nielsen NAES VI (NW). See Nielsen NAES II NW Settings for more details.
    #[builder(into)]
    #[serde(rename = "nielsenNaesIiNwSettings")]
    pub r#nielsen_naes_ii_nw_settings: Option<Vec<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenNaesIiNwSetting>>,
}
