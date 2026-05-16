#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustGatewaySettingsLoggingSettingsByRuleTypeL4 {
    /// Whether to log all activity.
    #[builder(into)]
    #[serde(rename = "logAll")]
    pub r#log_all: bool,
    #[builder(into)]
    #[serde(rename = "logBlocks")]
    pub r#log_blocks: bool,
}
