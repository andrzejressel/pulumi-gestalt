#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2TsSettingsDvbSdtSettings {
    #[builder(into)]
    #[serde(rename = "outputSdt")]
    pub r#output_sdt: Option<String>,
    #[builder(into)]
    #[serde(rename = "repInterval")]
    pub r#rep_interval: Option<i32>,
    #[builder(into)]
    #[serde(rename = "serviceName")]
    pub r#service_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "serviceProviderName")]
    pub r#service_provider_name: Option<String>,
}
