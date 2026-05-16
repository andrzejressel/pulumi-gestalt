#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustGatewayPolicyRuleSettingsCheckSession {
    /// Configure how fresh the session needs to be to be considered valid.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: String,
    /// Enable session enforcement for this rule.
    #[builder(into)]
    #[serde(rename = "enforce")]
    pub r#enforce: bool,
}
