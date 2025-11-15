#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigSignIn {
    /// Whether to allow more than one account to have the same email.
    #[builder(into)]
    #[serde(rename = "allowDuplicateEmails")]
    pub r#allow_duplicate_emails: Option<bool>,
    /// Configuration options related to authenticating an anonymous user.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "anonymous")]
    pub r#anonymous: Option<Box<super::super::types::identityplatform::ConfigSignInAnonymous>>,
    /// Configuration options related to authenticating a user by their email address.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Option<Box<super::super::types::identityplatform::ConfigSignInEmail>>,
    /// (Output)
    /// Output only. Hash config information.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "hashConfigs")]
    pub r#hash_configs: Option<Vec<super::super::types::identityplatform::ConfigSignInHashConfig>>,
    /// Configuration options related to authenticated a user by their phone number.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "phoneNumber")]
    pub r#phone_number: Option<Box<super::super::types::identityplatform::ConfigSignInPhoneNumber>>,
}
