#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MultiplexProgramMultiplexProgramSettingsVideoSettingsStatmuxSettings {
    /// Maximum bitrate.
    #[builder(into)]
    #[serde(rename = "maximumBitrate")]
    pub r#maximum_bitrate: Option<i32>,
    /// Minimum bitrate.
    #[builder(into)]
    #[serde(rename = "minimumBitrate")]
    pub r#minimum_bitrate: Option<i32>,
    /// Priority value.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
}
