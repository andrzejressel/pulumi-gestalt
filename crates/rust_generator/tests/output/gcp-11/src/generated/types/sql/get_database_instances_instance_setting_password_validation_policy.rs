#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDatabaseInstancesInstanceSettingPasswordValidationPolicy {
    /// Password complexity.
    #[builder(into)]
    #[serde(rename = "complexity")]
    pub r#complexity: String,
    /// Disallow username as a part of the password.
    #[builder(into)]
    #[serde(rename = "disallowUsernameSubstring")]
    pub r#disallow_username_substring: bool,
    /// Whether the password policy is enabled or not.
    #[builder(into)]
    #[serde(rename = "enablePasswordPolicy")]
    pub r#enable_password_policy: bool,
    /// Minimum number of characters allowed.
    #[builder(into)]
    #[serde(rename = "minLength")]
    pub r#min_length: i32,
    /// Minimum interval after which the password can be changed. This flag is only supported for PostgresSQL.
    #[builder(into)]
    #[serde(rename = "passwordChangeInterval")]
    pub r#password_change_interval: String,
    /// Number of previous passwords that cannot be reused.
    #[builder(into)]
    #[serde(rename = "reuseInterval")]
    pub r#reuse_interval: i32,
}
