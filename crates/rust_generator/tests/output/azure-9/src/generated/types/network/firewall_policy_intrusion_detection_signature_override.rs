#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallPolicyIntrusionDetectionSignatureOverride {
    /// 12-digit number (id) which identifies your signature.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// state can be any of `Off`, `Alert` or `Deny`.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
}
