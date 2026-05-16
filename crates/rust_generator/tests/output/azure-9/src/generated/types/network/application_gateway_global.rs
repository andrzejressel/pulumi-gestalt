#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationGatewayGlobal {
    /// Whether Application Gateway's Request buffer is enabled.
    #[builder(into)]
    #[serde(rename = "requestBufferingEnabled")]
    pub r#request_buffering_enabled: bool,
    /// Whether Application Gateway's Response buffer is enabled.
    #[builder(into)]
    #[serde(rename = "responseBufferingEnabled")]
    pub r#response_buffering_enabled: bool,
}
