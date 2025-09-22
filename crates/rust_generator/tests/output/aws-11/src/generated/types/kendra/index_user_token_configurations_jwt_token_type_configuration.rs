#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IndexUserTokenConfigurationsJwtTokenTypeConfiguration {
    /// The regular expression that identifies the claim. Minimum length of 1. Maximum length of 100.
    #[builder(into)]
    #[serde(rename = "claimRegex")]
    pub r#claim_regex: Option<String>,
    /// The group attribute field. Minimum length of 1. Maximum length of 100.
    #[builder(into)]
    #[serde(rename = "groupAttributeField")]
    pub r#group_attribute_field: Option<String>,
    /// The issuer of the token. Minimum length of 1. Maximum length of 65.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: Option<String>,
    /// The location of the key. Valid values are `URL` or `SECRET_MANAGER`
    #[builder(into)]
    #[serde(rename = "keyLocation")]
    pub r#key_location: String,
    /// The Amazon Resource Name (ARN) of the secret.
    #[builder(into)]
    #[serde(rename = "secretsManagerArn")]
    pub r#secrets_manager_arn: Option<String>,
    /// The signing key URL. Valid pattern is `^(https?|ftp|file):\/\/([^\s]*)`
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// The user name attribute field. Minimum length of 1. Maximum length of 100.
    #[builder(into)]
    #[serde(rename = "userNameAttributeField")]
    pub r#user_name_attribute_field: Option<String>,
}
