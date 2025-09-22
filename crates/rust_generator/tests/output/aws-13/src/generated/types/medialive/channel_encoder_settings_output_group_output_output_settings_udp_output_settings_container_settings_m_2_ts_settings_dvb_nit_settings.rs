#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2TsSettingsDvbNitSettings {
    #[builder(into)]
    #[serde(rename = "networkId")]
    pub r#network_id: i32,
    #[builder(into)]
    #[serde(rename = "networkName")]
    pub r#network_name: String,
    #[builder(into)]
    #[serde(rename = "repInterval")]
    pub r#rep_interval: Option<i32>,
}
