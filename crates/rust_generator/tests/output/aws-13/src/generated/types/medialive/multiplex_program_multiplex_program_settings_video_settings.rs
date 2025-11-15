#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MultiplexProgramMultiplexProgramSettingsVideoSettings {
    /// Constant bitrate value.
    #[builder(into)]
    #[serde(rename = "constantBitrate")]
    pub r#constant_bitrate: Option<i32>,
    /// Statmux settings. See Statmux Settings for more details.
    #[builder(into)]
    #[serde(rename = "statmuxSettings")]
    pub r#statmux_settings: Option<Box<super::super::types::medialive::MultiplexProgramMultiplexProgramSettingsVideoSettingsStatmuxSettings>>,
}
