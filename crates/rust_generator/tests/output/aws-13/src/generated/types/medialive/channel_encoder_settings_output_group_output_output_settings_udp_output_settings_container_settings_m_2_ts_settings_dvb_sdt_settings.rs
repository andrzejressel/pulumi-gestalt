#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
