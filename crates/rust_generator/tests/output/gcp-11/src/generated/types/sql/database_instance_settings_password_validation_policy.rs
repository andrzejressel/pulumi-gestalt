#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatabaseInstanceSettingsPasswordValidationPolicy {
    /// Checks if the password is a combination of lowercase, uppercase, numeric, and non-alphanumeric characters.
    #[builder(into)]
    #[serde(rename = "complexity")]
    pub r#complexity: Option<String>,
    /// Prevents the use of the username in the password.
    #[builder(into)]
    #[serde(rename = "disallowUsernameSubstring")]
    pub r#disallow_username_substring: Option<bool>,
    /// Enables or disable the password validation policy.
    #[builder(into)]
    #[serde(rename = "enablePasswordPolicy")]
    pub r#enable_password_policy: bool,
    /// Specifies the minimum number of characters that the password must have.
    #[builder(into)]
    #[serde(rename = "minLength")]
    pub r#min_length: Option<i32>,
    /// Specifies the minimum duration after which you can change the password.
    #[builder(into)]
    #[serde(rename = "passwordChangeInterval")]
    pub r#password_change_interval: Option<String>,
    /// Specifies the number of previous passwords that you can't reuse.
    #[builder(into)]
    #[serde(rename = "reuseInterval")]
    pub r#reuse_interval: Option<i32>,
}
