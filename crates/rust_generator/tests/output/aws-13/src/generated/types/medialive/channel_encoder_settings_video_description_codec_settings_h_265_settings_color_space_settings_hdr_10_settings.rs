#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsHdr10Settings {
    /// Sets the MaxCLL value for HDR10.
    #[builder(into)]
    #[serde(rename = "maxCll")]
    pub r#max_cll: Option<i32>,
    /// Sets the MaxFALL value for HDR10.
    #[builder(into)]
    #[serde(rename = "maxFall")]
    pub r#max_fall: Option<i32>,
}
