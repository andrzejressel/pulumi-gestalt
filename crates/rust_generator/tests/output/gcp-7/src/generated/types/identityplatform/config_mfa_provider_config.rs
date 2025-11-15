#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigMfaProviderConfig {
    /// Whether MultiFactor Authentication has been enabled for this project.
    /// Possible values are: `DISABLED`, `ENABLED`, `MANDATORY`.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// TOTP MFA provider config for this project.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "totpProviderConfig")]
    pub r#totp_provider_config: Option<Box<super::super::types::identityplatform::ConfigMfaProviderConfigTotpProviderConfig>>,
}
