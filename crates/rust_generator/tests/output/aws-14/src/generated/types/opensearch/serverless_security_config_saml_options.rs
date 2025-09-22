#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServerlessSecurityConfigSamlOptions {
    /// Group attribute for this SAML integration.
    #[builder(into)]
    #[serde(rename = "groupAttribute")]
    pub r#group_attribute: Option<String>,
    /// The XML IdP metadata file generated from your identity provider.
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: String,
    /// Session timeout, in minutes. Minimum is 5 minutes and maximum is 720 minutes (12 hours). Default is 60 minutes.
    #[builder(into)]
    #[serde(rename = "sessionTimeout")]
    pub r#session_timeout: Option<i32>,
    /// User attribute for this SAML integration.
    #[builder(into)]
    #[serde(rename = "userAttribute")]
    pub r#user_attribute: Option<String>,
}
