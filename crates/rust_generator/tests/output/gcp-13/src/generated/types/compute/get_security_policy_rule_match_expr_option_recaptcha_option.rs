#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSecurityPolicyRuleMatchExprOptionRecaptchaOption {
    /// A list of site keys to be used during the validation of reCAPTCHA action-tokens. The provided site keys need to be created from reCAPTCHA API under the same project where the security policy is created
    #[builder(into)]
    #[serde(rename = "actionTokenSiteKeys")]
    pub r#action_token_site_keys: Box<Vec<String>>,
    /// A list of site keys to be used during the validation of reCAPTCHA session-tokens. The provided site keys need to be created from reCAPTCHA API under the same project where the security policy is created.
    #[builder(into)]
    #[serde(rename = "sessionTokenSiteKeys")]
    pub r#session_token_site_keys: Box<Vec<String>>,
}
