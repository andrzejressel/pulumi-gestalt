#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebhookAuthenticationConfiguration {
    /// A valid CIDR block for `IP` filtering. Required for `IP`.
    #[builder(into)]
    #[serde(rename = "allowedIpRange")]
    pub r#allowed_ip_range: Option<String>,
    /// The shared secret for the GitHub repository webhook. Set this as `secret` in your `github_repository_webhook`'s `configuration` block. Required for `GITHUB_HMAC`.
    #[builder(into)]
    #[serde(rename = "secretToken")]
    pub r#secret_token: Option<String>,
}
