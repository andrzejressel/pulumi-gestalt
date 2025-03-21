#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2TsSettingsDvbNitSettings {
    #[builder(into)]
    #[serde(rename = "networkId")]
    pub r#network_id: Box<i32>,
    #[builder(into)]
    #[serde(rename = "networkName")]
    pub r#network_name: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "repInterval")]
    pub r#rep_interval: Box<Option<i32>>,
}
